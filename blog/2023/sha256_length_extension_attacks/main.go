package main

import (
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
	originalSignature := sign(secretKey, originalData, true)

	maliciousData := []byte("&something=true&role=admin")
	maliciousMessage := generateMaliciousMessage(uint64(len(secretKey)), originalData, maliciousData)
	maliciousSignature := forgeSignature(uint64(len(secretKey)), uint64(len(originalData)), originalSignature, maliciousMessage, maliciousData)

	fmt.Printf("SecretKey: %s\n", hex.EncodeToString(secretKey))
	fmt.Printf("Original Data: %s\n", string(originalData))
	fmt.Printf("Original Signature: %s\n", hex.EncodeToString(originalSignature))
	fmt.Printf("Verify(SecretKey, OriginalData): %v\n", verifySignature(secretKey, originalSignature, originalData))
	fmt.Println("---------------------------------")
	fmt.Printf("Malicious Data: %s\n", string(maliciousData))
	fmt.Printf("Malicious Signature: %s\n", hex.EncodeToString(maliciousSignature))
	fmt.Printf("Verify(SecretKey, originalData || maliciousData): %v\n", verifySignature(secretKey, maliciousSignature, append(originalData, maliciousData...)))
}

func forgeSignature(secretKeyLength uint64, originalDataLength uint64, originalSignature []byte, maliciousMessage []byte, maliciousData []byte) (forgedSignature []byte) {
	digest, err := loadSha256(originalSignature, secretKeyLength+originalDataLength)
	if err != nil {
		log.Fatalf("loading SHA256 hash: %s", err)
	}

	// digest := NewSha256()
	// digest.Write(maliciousMessage)

	digest.Write(maliciousData)
	hash := digest.Sum(nil)
	forgedSignature = hash[:]

	return
}

func generateMaliciousMessage(secretKeyLength uint64, originalData []byte, maliciousData []byte) (message []byte) {
	padding := generatePadding(secretKeyLength, uint64(len(originalData)))
	message = make([]byte, 0, len(originalData)+len(padding))
	// message = make([]byte, 0, len(originalData)+len(padding)+len(maliciousData))

	message = append(message, originalData...)
	message = append(message, padding...)
	// message = append(message, maliciousData...)

	// fmt.Println(hex.Dump(message))
	// dumpBinary(message)

	return
}

func generatePadding(secretKeyLength uint64, originalDataLength uint64) (padding []byte) {
	messageLength := secretKeyLength + originalDataLength
	zerosLength := 64 - 8 - 1 - (messageLength % 64)

	padding = make([]byte, 1+zerosLength+8)

	// var tmp [64 + 8]byte // padding + length buffer
	// // tmp[0] = 0x80
	// // binary.LittleEndian.
	// tmp[0] = 0x1 << 7
	// var t uint64
	// if zerosLength%64 < 56 {
	// 	t = 56 - zerosLength%64
	// } else {
	// 	t = 64 + 56 - zerosLength%64
	// }

	// // Length in bits.
	// zerosLength <<= 3
	// // padlen := tmp[:t+8]
	// binary.BigEndian.PutUint64(tmp[t:], zerosLength)

	padding[0] = 0x1 << 7
	binary.BigEndian.PutUint64(padding[1+zerosLength:], messageLength*8)

	fmt.Println("PADDING:")
	fmt.Printf("Message Length: %d\n", messageLength)
	fmt.Printf("Zeros: %d\n", zerosLength)
	fmt.Println(hex.Dump(padding))
	dumpBinary(padding)

	// fmt.Println(hex.Dump(padding))

	return
}

func dumpBinary(data []byte) {
	for i, n := range data {
		fmt.Printf("%08b ", n) // prints 00000000 11111101
		if (i+1)%4 == 0 && i != 0 {
			fmt.Println("")
		}
	}
	fmt.Println("")
}

func verifySignature(secretKey []byte, signatureToVerify []byte, data []byte) (isValid bool) {
	isValid = false
	fmt.Println("verifySignature")
	signature := sign(secretKey, data, true)

	if subtle.ConstantTimeCompare(signature, signatureToVerify) == 1 {
		isValid = true
	}

	return
}

var digestState []byte

func sign(secretKey []byte, data []byte, print bool) (signature []byte) {
	message := make([]byte, 0, len(secretKey)+len(data))
	message = append(message, secretKey...)
	message = append(message, data...)

	digest := NewSha256()
	digest.Write(message)

	hash := digest.Sum(nil)
	var err error
	digestState, err = digest.MarshalBinary()
	if err != nil {
		panic(err)
	}

	if print {
		fmt.Println("DIGEST STATE:")
		fmt.Println(hex.Dump(digestState))
	}

	// hash := sha256.Sum256(message)

	signature = hash[:]
	return
}

func loadSha256(hashBytes []byte, secretKeyAndDataLength uint64) (hash *digest, err error) {
	digestBinary := make([]byte, 0, marshaledSize)
	digestBinary = append(digestBinary, []byte(magic256)...)
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[0]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[1]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[2]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[3]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[4]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[5]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[6]))
	// digestBinary = binary.BigEndian.AppendUint32(digestBinary, uint32(hashBytes[7]))

	digestBinary = append(digestBinary, hashBytes...)
	digestBinary = append(digestBinary, make([]byte, chunk)...)
	digestBinary = binary.BigEndian.AppendUint64(digestBinary, secretKeyAndDataLength)

	hash = NewSha256()
	hash.Write([]byte("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"))
	err = hash.UnmarshalBinary(digestState)

	fmt.Println("DIGEST BINARY:")
	fmt.Println(hex.Dump(digestBinary))
	return
}
