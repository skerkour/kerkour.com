+++
date = 2021-11-04T06:00:00Z
title = "Signatures: The foundations of modern end-to-end encryption"
type = "post"
tags = ["cryptography", "programming", "hacking"]
authors = ["Sylvain Kerkour"]
url = "/signatures-modern-end-to-end-encryption"

[extra]
lang = "en"

comment ="""

You might be thinking asymmetric encryption is probably enough to start encrypting
your emails. In reality, asymmetric encryption is quite limited due to the restricted
length of messages it can encrypt. The speed of asymmetric encryption and decryption
Asymmetric encryption and hybrid encryption
is also slow in comparison to symmetric encryption. This is due to asymmetric con-
structions implementing math operations, as opposed to symmetric primitives that
often just manipulate bits.




Reddit crypto: Hello, I wrote this blog post to exaplain to my developer friends what really is modern e2ee.
"""
+++

<!-- introduction sur le chiffrement de bout en bout:


Symmetric encryption is a solve problem. Today, challenges are about keys distribution nad proof of identity.

Parler des logiciels connus qui utilisent e2ee



Quand on parlede chiffrement de bout en bout, les gens pensent a PGP, asymetricx encryption, RSA...

So e2ee should be just RSA +  a symmetrix cipher AES right? Not a all. C'est comme ca que PGP marche, et c'est pour ca que tous les cryptographers disent que c'est de la merde. -->



Now that most of our communications are digital, a problem arises: **How to keep our messages private despite all the intermediaries?** Internet Service Providers (ISPs) and Service providers (Facebook, Telegram, Line, WeChat...) are all in a position of Man-In-The-Middle (MITM) and are able to inspect, record, and even modify our communications without our consent or knowledge.

And this is before talking about malicious actors.

> This post is an excerpt from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)


![Communication without encryption](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/communication.svg)
<!-- illustration:
Envoyer un message, montrer tout les points ou on peut se faire espionner

Alice -> ISP -> service provide (ex: Chat) -> ISP -> Bob
 -->


You may think that you have nothing to hide, so it doesn't matter. Think twice.

- What can happen if all your messages and your web browsing history are stored forever and accessible by the employees of those companies? While in the first place I'm certainly not comfortable with having strangers looking at my messages, the point is that over time, the chances of a leak or a hack are 100% as everything digital can be copied at (almost) the speed of light. Thus all your communication should be considered (soon to be) public.
- You may have nothing to hide today. But if history taught us **one** thing, it's that even if you consider yourself "normal", a crazy dictator can seize power (or be elected) and start imprisoning or exterminating entire chunks of the population because of their hobbies, hair color, or size.


This is where **end-to-end encryption (E2EE)** comes into play. With E2EE, only the intended recipients are able to decrypt and read the messages. Thus, none of the intermediaries can inspect, store or modify your private messages.



![Communication with encryption](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/encrypted_communication.svg)
<!--
Illustration:

meme schema d'echange de message que ci-dessus, mais avec le message chiffre pour montrer que seulement bob peut le lire
 -->



Before going further, I want to clarify a few things.

When we talk about a "message", it's not necessarily an email or a chat message. It can also be a network packet, so anything you do online, from visiting websites to buying shoes passing by gaming.

The original message is called **plaintext**, and the encrypted message is called **ciphertext**.



**How can we encrypt a message in a way that only our dear friend Bob is able to decrypt it?**


{{< subscribe_form >}}

## Public-key cryptography

<!-- aka asymetric cryptography
When you hear -->

Please welcome Public-key cryptography (a.k.a. asymetric cryptography).


The principle is simple. Encryption keys come in pairs:
- A **public key** is a key that should be shared with others so they can use it to encrypt data intended for you, and only you.
- A **private key** is a secret that should never be shared with anyone and that allows you to decrypt data that was previously encrypted with the public key.

The tuple `(private key, public key)` is called a **keypair**.


When I want to send something to bob, I encrypt my message with his public key, and only he is able to decrypt the message.


![Public-key cryptography](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/public_key.svg)
<!-- Illustration:
public key (identity) + private key.
use bob's public key to encrypt message.
Bob use his private key to decrypt message.
 -->


Because I need to know Bob's public key before sending him a message, his public key is kind of his digital **identity**. Usually, I can get Bob's public key through the same app I'm using to send him a message, but I need to verify (using another channel, like a face-to-face meeting) with him that the public key the service served me is Bob's one and not a malicious one.


Because only the owner of the private key is able to decrypt content encrypted with the public key, from a cryptographic point of view, **1 public key = 1 identity**.


<!-- This concept is pushed to the extreme in some protocols where your public key is literarily your identity, such as in scuttlebutt. This is problematic, because if you lose you private key or it leaks, you will need to build a new identity and re-build your social graph from 0 -->



One example of such an algorithm is [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)).


Is it enough to secure our communication?

Wait a minute!

Reality is quite different: public-key encryption is limited in the length of the messages it can encrypt and is painfully slow.




## Hybrid encryption

Hybrid encryption takes the best of symmetric encryption and asymmetric encryption: messages are encrypted with symmetric encryption (fast, any length, safe...), and only the ephemeral symmetric secret key (short, with a length of 256 bits - 32 bytes most of the time) is encrypted using asymmetric encryption.

The symmetric key is said to be ephemeral because it is discarded by both parties once the message is encrypted / decrypted and a new key is generated to encrypt each message.

Nowadays, the only recommended way to do symmetric encryption is to use [Authenticated Encryption with Associated Data (**AEAD**)](https://en.wikipedia.org/wiki/Authenticated_encryption) that allows us to detect if the ciphertext is modified.


![Hybrid encryption](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/hybrid.svg)
<!-- TODO: illustration

generate temporary key
encrypt message with
encrypt tempo key with public key
send message + encrypted tempo key

RSA + AEAD
-->


With this scheme, **1 public key still equals 1 identity**, but we can now encrypt messages of any length at max speed.


Yet, the situation is still not perfect. To offer good security, RSA keys tend to be large (3072 bits or more), and RSA encryption is not that easy to get right (principally related to padding), which is a big source of bugs.



## Diffie–Hellman key exchange

Diffie–Hellman key exchange (more commonly called key exchange) is a method to establish a **shared secret** between two parties through a public channel.

The same shared secret can be derived from Alice's public key and Bob's private key than from Bob's public key and Alice's private key. Thus, both Alice and Bob can compute the same shared secret using their respective private keys and the other one's public key.

Nowadays, the recommended key exchange functions to use are [Elliptic-curve Diffie–Hellman (**ECDH**)](https://en.wikipedia.org/wiki/Elliptic-curve_Diffie%E2%80%93Hellman), which is way simpler to implement than RSA encryption.


However,  shared secrets computed through ECDH key exchange can't be used directly for symmetric encryption. Most AEAD algorithms expect a uniformly random symmetric key which shared secrets are not. Thus, to "increase their entropy", we pass the output of the key exchange function into a **Key Derivation Function (KDF)** to generate a shared secret key that can be used for symmetric encryption.

![Key exchange](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/key_exchange.svg)
<!-- TODO: illustration
encrypt message with
encrypt tempo key with public key
send message + encrypted tempo key

RSA + AEAD
-->

An example of ECDH function is [`x25519`](https://en.wikipedia.org/wiki/Curve25519).


So, is E2EE simply key exchange + AEAD?

Hold on! What happens if our private key is leaked?

If one of the intermediaries recorded all our messages and our private key leaked, the malicious actor would be able to **decrypt all the messages**! Past, present, and future.

This is basically how [PGP](https://en.wikipedia.org/wiki/Pretty_Good_Privacy) works and the principal reason it's criticized by cryptographers.

As managing keys is known to be hard, it's not a matter of *"if"*, but of *"when"*.


{{< subscribe_form >}}

## Forward Secrecy

Forward Secrecy (also known as Perfect Forward Secrecy) is a feature of protocols that guarantees that if a key leaks at the moment `T`, messages sent before, at `T-1`, `T-2`, `T-3`... can't be decrypted.

![Forward secrecy](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/forward_secrecy.svg)
<!-- Illustration: Forward secrecy -->


To implement forward secrecy, we could simply create many keypairs, use one keypair per message and delete it after the message is received.

But then we would lose our feature that **1 public key = 1 identity**: we would need to verify with Bob for each message that each public key is legitimate and actually comes from Bob, and not a MITM attacker, which is impracticable.


<!-- Comme on l'a vu, une cle = 1 identite, changer de keypair for each message pose probleme: on n'aurait plus de moyen de definir de maniere cryptographique que ces messages viennent de la meme personne. -->

Unless...

## Signatures


![Signatures](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/signatures.svg)
<!-- illustration: signatures -->


Signatures allow a person in possession of a private key to authenticate a document or a message. By signing the message or document, the private key owner attests to its validity. Then, everybody who has access to the public key can verify that the signature matches the document.

Thus, **Signatures are the perfect tool to build a digital identity**.

Let see how to use signatures with encryption to secure our communications.


## End-to-end encryption



![End-to-end encryption](https://kerkour.com/2021/signatures-modern-end-to-end-encryption/e2ee.svg)
<!-- illustration
generate tmp key + sign it
send tmp key + public key
fetch tmp key and verify signature
-->

**1.** Bob Generates a signature keypair and a key exchange (ephemeral) keypair. He signs the key exchange keypair with the key exchange public key and then publishes both public keys plus the signature.

**2.** Alice fetches both public keys and the signature. She verifies that the signatures match the key exchange keypair. If the signature matches, then we are sure that the key exchange public key comes from Bob.

**3.** Alices generates a key exchange (ephemeral) keypair. She performs a key exchange with her private key and Bob's public key to generate a shared secret and pass it into a KDF to generate a symmetric secret key. She uses this secret key to encrypt her message. She then signs the key exchange public key and can now destroy the private key exchange private key.

**4.** Alices sends her public key exchange key, encrypted message, and signature to Bob.

**5.** Bob verifies that the signature is valid with Alice's public signing key. If everything is good, he can now use the public key exchange key that Alice just sent him to perform a key exchange with his key exchange private key and pass the shared secret into a KDF to generate exactly the same symmetric secret key as Alice. With that secret key, he can finally decrypt the message.


One interesting thing to note is that Alice only signs the public key exchange key and not the whole encrypted message because the integrity and authenticity of the message are guaranteed thanks to **AEAD** encryption. If any bit of the encrypted message or public key is modified by a malicious actor, the decryption operations will fail and return an error.

Key exchange keypairs are called **ephemeral** because they are no longer used after the message is sent or decrypted. On the other hand, signing keys are called **long-term** keys as they need to be renewed only when a leak happens (or is suspected).

It's a lot of effort to send a message, but it's totally worth it. We now have a single identity key: the public signing key, and we can use as many encryption keys as we want. We just need to sign those encryption keys.

Furthermore, we could use this signing key for many other things, such as signing documents, contracts...



<!--
On utilise des temporary keys bundles
Thus, modern protocols today use signing keys as identities.

With a signing key you can sign everything you want: temporary keys, files...


You may be wondering, but why shuld I trust the public key distributed by the server?
You shouldn't! This pattern is known as TOFU (Trust of first use) and is simply an usability compromise. And this is why secure chat all offer the way to verify if the keys match, either in person, or over another channel repute comme secure.
 -->



In short, **Modern end-to-end encryption = Signatures + Key exchange + AEAD**

**Signatures** are the long-term identity keys and are used to sign ephemeral key exchange keys.

Ephemeral **key exchange keys** are used to encrypt symmetric AEAD keys.

**AEAD** keys are used to encrypt the messages.



So, this is how the [Signal protocol](https://signal.org/docs/) works?


No. The protocol above is not perfect. For example, performing 1 key exchange per message is inefficient. Furthermore, the protocol is subject to [replay attacks](https://en.wikipedia.org/wiki/Replay_attack).


This is why advanced protocols like Signal use more techniques such as [the double ratchet](https://signal.org/docs/specifications/doubleratchet/) or [ephemeral key bundles](https://signal.org/docs/specifications/x3dh/) to provide strong security guarantees, such as protection against replay attacks.



Examples of primitives:
- Signatures: `ed25519`
- Key exchange: `x25519`
- Key Derivation Function: `HSalsa20` or `Blake2b`
- AEAD: `XChaCha20-Poly1305` or `AES-256-GCM`



This is for the theory. In practice, you have to keep in mind that while E2EE **should** be used, it's not a silver bullet and a motivated attacker can still eavesdrop your communications:
- A lot of people prefer to have their chat and emails backed up, and those backups are most of the time not encrypted.
- Devices, apps and app stores can be compromised, and messages or keys can be exfiltrated directly from the devices, bypassing all forms of encryption.
- For web-based services, HTTPS content can be intercepted and/or modified due to hacked or malicious [Certificate Authorities (CAs)](https://en.wikipedia.org/wiki/Certificate_authority) and a script can be injected to exfiltrate keys or messages.
- Anybody can take a screenshot or even a picture of the screen.

With E2EE we have to trust fewer third-parties, but at the end of the day, we still have to trust devices manufacturers, app developers, and a few others. This is where Open Source comes into play to bring transparency and accountability about what runs on our devices.


**Want to see what it looks like in code? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust) where we build our own end-to-end encrypted protocol to secure the communication of a Remote Access Tool.**
