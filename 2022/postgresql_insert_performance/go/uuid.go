package main

import (
	"database/sql/driver"
	"errors"
	"fmt"

	"github.com/google/uuid"
	"github.com/jackc/pgtype"
)

var errUndefined = errors.New("cannot encode status undefined")
var errBadStatus = errors.New("invalid status")

// UUID pgx type wrapper for google/uuid.UUID
type UUID struct {
	UUID   uuid.UUID
	Status pgtype.Status
}

// Set implements pgtype.Value interface
func (u *UUID) Set(src interface{}) error {
	if src == nil {
		*u = UUID{Status: pgtype.Null}
		return nil
	}

	if value, ok := src.(interface{ Get() interface{} }); ok {
		value2 := value.Get()
		if value2 != value {
			return u.Set(value2)
		}
	}

	switch value := src.(type) {
	case uuid.UUID:
		*u = UUID{UUID: value, Status: pgtype.Present}
	case [16]byte:
		*u = UUID{UUID: value, Status: pgtype.Present}
	case []byte:
		if len(value) != 16 {
			return fmt.Errorf("[]byte must be 16 bytes to convert to UUID: %d", len(value))
		}
		*u = UUID{Status: pgtype.Present}
		copy(u.UUID[:], value)
	case string:
		uid, err := uuid.Parse(value)
		if err != nil {
			return err
		}
		*u = UUID{UUID: uid, Status: pgtype.Present}
	default:
		// If all else fails see if pgtype.UUID can handle it. If so, translate through that.
		pgUUID := &pgtype.UUID{}
		if err := pgUUID.Set(value); err != nil {
			return fmt.Errorf("cannot convert %v to UUID", value)
		}

		*u = UUID{UUID: pgUUID.Bytes, Status: pgUUID.Status}
	}

	return nil
}

// Get implements pgtype.Value interface
func (u UUID) Get() interface{} {
	switch u.Status {
	case pgtype.Present:
		return u.UUID
	case pgtype.Null:
		return nil
	default:
		return u.Status
	}
}

// AssignTo implements pgtype.Value interface
func (u *UUID) AssignTo(dst interface{}) error {
	switch u.Status {
	case pgtype.Present:
		switch v := dst.(type) {
		case *uuid.UUID:
			*v = u.UUID
			return nil
		case *[16]byte:
			*v = u.UUID
			return nil
		case *[]byte:
			*v = make([]byte, 16)
			copy(*v, u.UUID[:])
			return nil
		case *string:
			*v = u.UUID.String()
			return nil
		default:
			if nextDst, retry := pgtype.GetAssignToDstType(v); retry {
				return u.AssignTo(nextDst)
			}
			return fmt.Errorf("unable to assign to %T", dst)
		}

	case pgtype.Null:
		return pgtype.NullAssignTo(dst)
	}

	return fmt.Errorf("cannot assign %v into %T", u, dst)
}

// DecodeText implemnts pgtype.TextDecoder interface
func (u *UUID) DecodeText(_ *pgtype.ConnInfo, src []byte) error {
	if src == nil {
		*u = UUID{Status: pgtype.Null}
		return nil
	}

	parsedUUID, err := uuid.ParseBytes(src)
	if err != nil {
		return err
	}

	*u = UUID{UUID: parsedUUID, Status: pgtype.Present}
	return nil
}

// DecodeBinary implements pgtype.BinaryDecoder interface
func (u *UUID) DecodeBinary(_ *pgtype.ConnInfo, src []byte) error {
	if src == nil {
		*u = UUID{Status: pgtype.Null}
		return nil
	}

	if len(src) != 16 {
		return fmt.Errorf("invalid length for UUID: %v", len(src))
	}

	*u = UUID{Status: pgtype.Present}
	copy(u.UUID[:], src)
	return nil
}

// EncodeText implements pgtype.TextEncoder interface
func (u UUID) EncodeText(_ *pgtype.ConnInfo, buf []byte) ([]byte, error) {
	switch u.Status {
	case pgtype.Null:
		return nil, nil
	case pgtype.Undefined:
		return nil, errUndefined
	}

	return append(buf, u.UUID.String()...), nil
}

// EncodeBinary implements pgtype.BinaryEncoder interface
func (u UUID) EncodeBinary(_ *pgtype.ConnInfo, buf []byte) ([]byte, error) {
	switch u.Status {
	case pgtype.Null:
		return nil, nil
	case pgtype.Undefined:
		return nil, errUndefined
	}

	return append(buf, u.UUID[:]...), nil
}

// Scan implements the database/sql.Scanner interface.
func (u *UUID) Scan(src interface{}) error {
	if src == nil {
		*u = UUID{Status: pgtype.Null}
		return nil
	}

	switch src := src.(type) {
	case string:
		return u.DecodeText(nil, []byte(src))
	case []byte:
		return u.DecodeText(nil, src)
	}

	return fmt.Errorf("cannot scan %T", src)
}

// Value implements the database/sql/driver.Valuer interface.
func (u UUID) Value() (driver.Value, error) {
	return pgtype.EncodeValueText(u)
}

// MarshalJSON implements encoding/json.Marshaler interface
func (u UUID) MarshalJSON() ([]byte, error) {
	switch u.Status {
	case pgtype.Present:
		return []byte(`"` + u.UUID.String() + `"`), nil
	case pgtype.Null:
		return []byte("null"), nil
	case pgtype.Undefined:
		return nil, errUndefined
	}

	return nil, errBadStatus
}

// UnmarshalJSON implements encoding/json.Unmarshaler interface
func (u *UUID) UnmarshalJSON(b []byte) error {
	var uid uuid.NullUUID
	err := uid.UnmarshalJSON(b)
	if err != nil {
		return err
	}

	status := pgtype.Null
	if uid.Valid {
		status = pgtype.Present
	}
	*u = UUID{UUID: uid.UUID, Status: status}

	return nil
}
