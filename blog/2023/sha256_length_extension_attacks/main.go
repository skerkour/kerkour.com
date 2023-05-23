package main

import (
	"crypto/subtle"
	"encoding/binary"
	"encoding/hex"
	"flag"
	"fmt"
	"log"
)

var verbose bool

func main() {
	flag.BoolVar(&verbose, "verbose", false, "verbose")
	flag.Parse()

	secretKey, err := hex.DecodeString("459b26fb72fbc187e424d0b73c64eff2a170576e929f0255dc719f7f51d9d6c6")
	if err != nil {
		log.Fatal("SecretKey is not valid Hex")
	}

	originalData := []byte("user_id=1&role=user")
	originalSignature := sign(secretKey, originalData)

	maliciousData := []byte("&something=true&role=admin")
	maliciousMessage := generateMaliciousMessage(uint64(len(secretKey)), originalData, maliciousData)
	maliciousSignature := forgeSignature(uint64(len(secretKey)), uint64(len(originalData)), originalSignature, maliciousData)

	fmt.Printf("SecretKey: %s\n", hex.EncodeToString(secretKey))
	fmt.Printf("Original Data: %s\n", string(originalData))
	fmt.Printf("Original Signature: %s\n", hex.EncodeToString(originalSignature))
	fmt.Printf("Verify(SecretKey, OriginalData): %v\n", verifySignature(secretKey, originalSignature, originalData))
	fmt.Println("---------------------------------")
	fmt.Printf("Malicious Data: %s\n", string(maliciousData))
	if verbose {
		fmt.Println("Malicious Message (OriginalData || padding || MaliciousData):")
		fmt.Println(hex.Dump(maliciousMessage))
	}
	fmt.Printf("Malicious Signature: %s\n", hex.EncodeToString(maliciousSignature))
	fmt.Printf("Verify(SecretKey, maliciousMessage): %v\n", verifySignature(secretKey, maliciousSignature, maliciousMessage))
}

func forgeSignature(secretKeyLength uint64, originalDataLength uint64, originalSignature []byte, maliciousData []byte) (forgedSignature []byte) {
	digest, err := loadSha256(originalSignature, secretKeyLength+originalDataLength)
	if err != nil {
		log.Fatalf("loading SHA256 hash: %s", err)
	}

	digest.Write(maliciousData)
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

	return
}

func generatePadding(secretKeyLength uint64, originalDataLength uint64) (padding []byte) {
	messageLength := secretKeyLength + originalDataLength
	zerosLength := 64 - 8 - 1 - (messageLength % 64)

	padding = make([]byte, 1+zerosLength+8)

	padding[0] = 0x1 << 7
	binary.BigEndian.PutUint64(padding[1+zerosLength:], messageLength*8)

	// fmt.Println("PADDING:")
	// fmt.Printf("Message Length: %d\n", messageLength)
	// fmt.Printf("Zeros: %d\n", zerosLength)
	// fmt.Println(hex.Dump(padding))
	// dumpBinary(padding)

	return
}

// dumpBinary prints 00000000 00000000 00000000 00000001
func dumpBinary(data []byte) {
	for i, n := range data {
		fmt.Printf("%08b ", n)
		if (i+1)%4 == 0 && i != 0 {
			fmt.Println("")
		}
	}
	fmt.Println("")
}

func verifySignature(secretKey []byte, signatureToVerify []byte, data []byte) (isValid bool) {
	isValid = false
	signature := sign(secretKey, data)

	if subtle.ConstantTimeCompare(signature, signatureToVerify) == 1 {
		isValid = true
	}

	return
}

var digestState []byte

func sign(secretKey []byte, data []byte) (signature []byte) {
	message := make([]byte, 0, len(secretKey)+len(data))
	message = append(message, secretKey...)
	message = append(message, data...)

	digest := NewSha256()
	digest.Write(message)

	hash := digest.checkSum()
	var err error
	digestState, err = digest.MarshalBinary()
	if err != nil {
		panic(err)
	}

	// if print {
	// 	fmt.Println("DIGEST STATE:")
	// 	fmt.Println(hex.Dump(digestState))
	// }

	signature = hash[:]
	return
}

func loadSha256(hashBytes []byte, secretKeyAndDataLength uint64) (hash *digest, err error) {
	digestBinary := make([]byte, 0, marshaledSize)
	digestBinary = append(digestBinary, []byte(magic256)...)
	digestBinary = append(digestBinary, hashBytes...)
	digestBinary = append(digestBinary, make([]byte, chunk)...)
	digestBinary = binary.BigEndian.AppendUint64(digestBinary, 64)

	hash = NewSha256()
	err = hash.UnmarshalBinary(digestBinary)

	// fmt.Println("DIGEST BINARY:")
	// fmt.Println(hex.Dump(digestBinary))
	return
}
