# 1 - Introduction

## Summary

??


enjeux:
- reconnaissance passive vs active
- rust setup
- fist rust program: simple password cracker
- rust crates
- rust result
- rust raii

hook:

plan:



expliquer pourquoi Rust est le parfait langage

ce livre s'adresse aux developpeurs qui voudraient en apprendre plus sur la securite
ou aux devSecOps. Sec guys qui voudraient en apprendre plus sur la programmation autre que python.

expliquer pourquoi la structure du livre match les phases d'un hack:
Reconnaissance
Enumeration
Exploitation
Maintaining
Access
CleanUp
Progression
Exfiltration


c'est donc tout naturellement que notre livre va suivre ces etapes, en 3 parties


et une fois l'infrastructure adverse breached, on...

sha1 cracker: ce qu'on remarque c'est que l'on a pas besoin de fermer le fichier, il est closed on drop
the file is automatically closed when the variable goes out of scope
