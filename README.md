The Chaum-Pedersen protocol is a zero-knowledge proof used in cryptographic systems. It allows one party (the prover) to prove to another party (the verifier) that they know a secret, without revealing the secret itself.

Here's a simplified version of how the protocol works, along with the math involved:

Commitment: The prover selects a random number ```r```. They compute ```t = g^r mod p``` and send ```t``` to the verifier. Here, ```g``` is a generator of a cyclic group of order ```q``` with modulus ```p```.

Challenge: The verifier sends a random number ```c``` to the prover as a challenge.

Response: The prover computes ```s = r + c*x mod q``` and sends ```s``` to the verifier. Here, ```x``` is the prover's secret.

Verification: The verifier checks if ```g^s mod p``` equals ```t * y^c mod p```. Here, ```y = g^x mod p``` is the prover's public key. If the equation holds, the verifier accepts the proof.

The protocol is zero-knowledge because the verifier learns nothing about the prover's secret x from the proof. The protocol is also interactive and requires the prover and verifier to exchange several messages.

The Chaum-Pedersen protocol is based on the hardness of the Discrete Logarithm Problem (DLP). It assumes that it's computationally infeasible to compute ```x``` given ```g```, ```p```, and ```g^x mod p```, which ensures the security of the protocol.


You can run the tests using:
```
cargo test
```


Also check out the gRPC-server branch other than main.
