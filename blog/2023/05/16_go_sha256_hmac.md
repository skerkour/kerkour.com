+++
date = 2023-05-16T07:00:00Z
title = "Generating a SHA256 HMAC in Go"
type = "post"
tags = ["cryptography", "go", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/sha256-hmac-golang"

[extra]
lang = "en"
+++

SHA256 being vulnerable to [length-extension attacks](https://crypto.stackexchange.com/questions/3978/understanding-the-length-extension-attack), you need to use the special HMAC construction to securely sign data with SHA256 and a secret key.

Here is how to do it in Go.


```go
package main

import (
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
)

func main() {
  // create a random 64 bytes (512 bites) secret
	secret := make([]byte, 64)
	_, err := rand.Read(secret)
	if err != nil {
		fmt.Println("error generating a random secret:", err)
		return
	}

	data := []byte("Hello World")

	// create a new HMAC by defining the hash type and the key
	hmac := hmac.New(sha256.New, secret)

	// compute the HMAC
	hmac.Write([]byte(data))
	dataHmac := hmac.Sum(nil)

	hmacHex := hex.EncodeToString(dataHmac)
	secretHex := hex.EncodeToString(secret)

	fmt.Printf("HMAC_SHA256(key: %s, data: %s): %s", secretHex, string(data), hmacHex)
}
```
