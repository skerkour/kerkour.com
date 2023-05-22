+++
date = 2023-04-01T07:00:00Z
title = "How to zip and unzip files in Go"
type = "post"
tags = ["programming", "go"]
authors = ["Sylvain Kerkour"]
url = "/zip-unzip-files-in-golang"

[extra]
lang = "en"
+++

Compressing files into a `.zip` archive and unzipping it in Go is super easy thanks to its extensive and easy-to-use standard library.

Here is how.

## Zip files in Go

```go
// zip.go
package main

import (
	"archive/zip"
	"io"
	"io/fs"
	"log"
	"os"
	"path/filepath"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Println("Usage: go run zip.go archive.zip file1 [file2...]")
		os.Exit(1)
	}

	archive := os.Args[1]
	files := os.Args[2:]

	err := zipFiles(archive, files...)
	if err != nil {
		log.Fatal(err)
	}
}

func zipFiles(archive string, files ...string) (err error) {
	// Create the ZIP file
	zipFile, err := os.Create(archive)
	if err != nil {
		return
	}
	defer zipFile.Close()

	zipWriter := zip.NewWriter(zipFile)
	defer zipWriter.Close()

	for _, filePath := range files {
		err = processFile(zipWriter, filePath)
		if err != nil {
			return
		}
	}

	return
}

func processFile(zipWriter *zip.Writer, filePath string) error {
	var err error
	var fileInfo fs.FileInfo
	var header *zip.FileHeader
	var headerWriter io.Writer
	var file *os.File

	fileInfo, err = os.Stat(filePath)
	if err != nil {
		return err
	}

	header, err = zip.FileInfoHeader(fileInfo)
	if err != nil {
		return err
	}

	header.Method = zip.Deflate

	header.Name, err = filepath.Rel(filepath.Dir("."), filePath)
	if err != nil {
		return err
	}

	if fileInfo.IsDir() {
		header.Name += "/"
	}

	headerWriter, err = zipWriter.CreateHeader(header)
	if err != nil {
		return err
	}

	if fileInfo.IsDir() {
		return nil
	}

	file, err = os.Open(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = io.Copy(headerWriter, file)
	return err
}
```

To run this program, save it to a file called "zip.go" and execute the following command in your terminal:

```shell
$ go run zip.go your_zipfile.zip file1 file2 file3...
```


## Unzip files in Go


```go
package main

import (
	"archive/zip"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	if len(os.Args) != 3 {
		fmt.Println("Usage: go run unzip.go <zipfile> <destination>")
		os.Exit(1)
	}

	zipFile := os.Args[1]
	destDir := os.Args[2]

	if err := unzip(zipFile, destDir); err != nil {
		fmt.Println("Error unzipping file:", err)
		os.Exit(1)
	}

	fmt.Println("Successfully unzipped:", zipFile)
}

func unzip(zipFile, destDir string) error {
	reader, err := zip.OpenReader(zipFile)
	if err != nil {
		return err
	}
	defer reader.Close()

	destDir = filepath.Clean(destDir)

	for _, file := range reader.File {
		if err := extractFile(file, destDir); err != nil {
			return err
		}
	}

	return nil
}

func extractFile(file *zip.File, destDir string) error {
	destPath := filepath.Join(destDir, file.Name)
	destPath = filepath.Clean(destPath)

	// Check for file traversal attack
	if !strings.HasPrefix(destPath, destDir) {
		return fmt.Errorf("invalid file path: %s", file.Name)
	}

	if file.FileInfo().IsDir() {
		if err := os.MkdirAll(destPath, file.Mode()); err != nil {
			return err
		}
	} else {
		if err := os.MkdirAll(filepath.Dir(destPath), os.ModePerm); err != nil {
			return err
		}

		destFile, err := os.OpenFile(destPath, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
		if err != nil {
			return err
		}
		defer destFile.Close()

		srcFile, err := file.Open()
		if err != nil {
			return err
		}
		defer srcFile.Close()

		if _, err := io.Copy(destFile, srcFile); err != nil {
			return err
		}
	}

	return nil
}
```

To run this program, save it to a file called "unzip.go" and execute the following command in your terminal:

```shell
$ go run unzip.go path/to/your/zipfile.zip path/to/destination/directory
```
