# 2 - Attack surface discovery
<!--  mapping, Multi-threaded -->
<!-- #### a simple multi-threaded port scanner -->

## Summary

How to perform effective reconnaissance? In this chapter we will build a multi-threaded scanner in order to automate the mapping of the target.


enjeux:
- expliquer la methodologie et quoi faire lors de la phase de reconnaissance
- expliquer pourquoi scanner les ports
- rust fearless concurrency
- implementer un sub domain finder et scanner les ports pour chaques domaines
- multithreading

hook:

plan:
* Why reconnaissance
* Methodology of reconnaissance
* challenges of reconnaissance -> comment filtrer la donnee
* Parallelism in Rust
* Introducing tricoder (Implementing out first scanner)
* Going multi-thread with Rayon





Soyons honnete, le code que nous venons d’ecrire est moche


shadowing

## Code spec

on va faire un scanner qui sert a mapper les targets.

le faire de maniere functionnel

pipeline

subdomain enumeration
filter resolving domains
port scan resolving domains
check if port is HTTP(s)


rendre tout ca parallel