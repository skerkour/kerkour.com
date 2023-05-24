# [Breaking SHA256: length extension attacks in practice](https://kerkour.com/sha256-length-extension-attacks)


## Usage

```bash
$ go run ./ -verbose
SecretKey: 7365637265747365637265747365637265747365637265747365637265747365
Legitimate Data: user_id=1&role=user
Legitimate Signature SHA256(SecretKey || LegitimateData): 5b0b4b2472778fea87faac08a72a47d24538bff9d7f19a3a85d069893e2b08ab
Verify LegitimateSignature == SHA256(SecretKey || LegitimateData): true

---------------------------------------------------------------------------------------------------

Malicious Data: &something=true&role=admin
Malicious Message (LegitimateData || padding || MaliciousData):
00000000  75 73 65 72 5f 69 64 3d  31 26 72 6f 6c 65 3d 75  |user_id=1&role=u|
00000010  73 65 72 80 00 00 00 00  00 00 00 00 00 00 01 98  |ser.............|
00000020  26 73 6f 6d 65 74 68 69  6e 67 3d 74 72 75 65 26  |&something=true&|
00000030  72 6f 6c 65 3d 61 64 6d  69 6e                    |role=admin|

Malicious Signature: 8c37e11e8397b39cba72fa0e4769716c69a7ba9e29cfaf00d4601e086e85dd8f
Verify MaliciousSignature == SHA256(SecretKey, MaliciousMessage): true
```
