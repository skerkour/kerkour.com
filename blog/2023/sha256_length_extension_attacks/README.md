# [Breaking SHA256: length extension attacks in practice](https://kerkour.com/sha256-length-extension-attacks)


## Usage

```bash
$ go run ./ -verbose
SecretKey: 459b26fb72fbc187e424d0b73c64eff2a170576e929f0255dc719f7f51d9d6c6
Original Data: user_id=1&role=user
Original Signature: 6c67e88ac5a246ce0f19da4eb279c56b3d9ba3e51879e33541e42b27dea7fe53
Verify(SecretKey, OriginalData): true

---------------------------------------------------------------------------------------------------

SHA256 state:
00000000  73 68 61 03 6c 67 e8 8a  c5 a2 46 ce 0f 19 da 4e  |sha.lg....F....N|
00000010  b2 79 c5 6b 3d 9b a3 e5  18 79 e3 35 41 e4 2b 27  |.y.k=....y.5A.+'|
00000020  de a7 fe 53 00 00 00 00  00 00 00 00 00 00 00 00  |...S............|
00000030  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
00000040  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
00000050  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
00000060  00 00 00 00 00 00 00 00  00 00 00 40              |...........@|

Malicious Data: &something=true&role=admin
Malicious Message (OriginalData || padding || MaliciousData):
00000000  75 73 65 72 5f 69 64 3d  31 26 72 6f 6c 65 3d 75  |user_id=1&role=u|
00000010  73 65 72 80 00 00 00 00  00 00 00 00 00 00 01 98  |ser.............|
00000020  26 73 6f 6d 65 74 68 69  6e 67 3d 74 72 75 65 26  |&something=true&|
00000030  72 6f 6c 65 3d 61 64 6d  69 6e                    |role=admin|

Malicious Signature: cedf9f0ee04d26731c6641390a761ab21786345be1f4c04072e3b501e475d195
Verify(SecretKey, maliciousMessage): true
```
