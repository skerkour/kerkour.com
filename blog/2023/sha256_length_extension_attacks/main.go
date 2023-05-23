package main

import (
	"crypto/sha256"
	"crypto/subtle"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"log"
)

func main() {
	secretKey, err := hex.DecodeString("459b26fb72fbc187e424d0b73c64eff2a170576e929f0255dc719f7f51d9d6c6")
	if err != nil {
		log.Fatal("SecretKey is not valid Hex")
	}

	originalData := []byte("user_id=1&role=user")
	originalSignature := sign(secretKey, originalData)

	maliciousData := []byte("&something=true&role=admin")
	maliciousMessage := generateMaliciousMessage(uint64(len(secretKey)), originalData, maliciousData)
	maliciousSignature := forgeSignature(uint64(len(secretKey)), uint64(len(originalData)), originalSignature, maliciousMessage)

	fmt.Printf("SecretKey: %s\n", hex.EncodeToString(secretKey))
	fmt.Printf("Original Data: %s\n", string(originalData))
	fmt.Printf("Original Signature: %s\n", hex.EncodeToString(originalSignature))
	fmt.Printf("Verify(SecretKey, OriginalData): %v\n", verifySignature(secretKey, originalSignature, originalData))
	fmt.Println("---------------------------------")
	fmt.Printf("Malicious Data: %s\n", string(maliciousData))
	fmt.Printf("Malicious Signature: %s\n", hex.EncodeToString(maliciousSignature))
	fmt.Printf("Verify(SecretKey, MaliciousMessage): %v\n", verifySignature(secretKey, maliciousSignature, maliciousMessage))
}

func forgeSignature(secretKeyLength uint64, originalDataLength uint64, originalSignature []byte, maliciousMessage []byte) (forgedSignature []byte) {
	digest, err := loadSha256(originalSignature, secretKeyLength+originalDataLength)
	if err != nil {
		log.Fatalf("loading SHA256 hash: %s", err)
	}

	digest.Write(maliciousMessage)
	hash := digest.Sum(nil)
	forgedSignature = hash[:]

	return
}

func generateMaliciousMessage(secretKeyLength uint64, originalData []byte, maliciousData []byte) (message []byte) {
	padding := generatePadding(secretKeyLength, uint64(len(originalData)))
	message = make([]byte, 0, len(originalData)+len(padding)+len(maliciousData))

	message = append(message, originalData...)
	message = append(message, padding...)
	message = append(message, maliciousData...)

	fmt.Println(hex.Dump(message))

	return
}

// TODO
func generatePadding(secretKeyLength uint64, originalDataLength uint64) (padding []byte) {
	len := secretKeyLength + originalDataLength
	// padding = make([]byte, (secretKeyLength + originalDataLength) % 64)

	var tmp [64 + 8]byte // padding + length buffer
	// tmp[0] = 0x80
	tmp[0] = 0x1
	var t uint64
	if len%64 < 56 {
		t = 56 - len%64
	} else {
		t = 64 + 56 - len%64
	}

	// Length in bits.
	len <<= 3
	padlen := tmp[:t+8]
	binary.BigEndian.PutUint64(padlen[t+0:], len)

	padding = padlen

	// fmt.Println(hex.Dump(padding))

	return
}

func verifySignature(secretKey []byte, signatureToVerify []byte, data []byte) (isValid bool) {
	isValid = false
	signature := sign(secretKey, data)

	if subtle.ConstantTimeCompare(signature, signatureToVerify) == 1 {
		isValid = true
	}

	return
}

func sign(secretKey []byte, data []byte) (signature []byte) {
	message := make([]byte, 0, len(secretKey)+len(data))
	message = append(message, secretKey...)
	message = append(message, data...)

	hash := sha256.Sum256(message)

	signature = hash[:]
	return
}

func loadSha256(hashBytes []byte, secretKeyAndDataLength uint64) (hash *digest, err error) {
	digestBinary := make([]byte, 0, marshaledSize)
	digestBinary = append(digestBinary, []byte(magic256)...)
	digestBinary = append(digestBinary, hashBytes...)
	digestBinary = append(digestBinary, make([]byte, chunk)...)
	digestBinary = binary.BigEndian.AppendUint64(digestBinary, secretKeyAndDataLength)

	hash = NewSha256()
	err = hash.UnmarshalBinary(digestBinary)
	return
}
