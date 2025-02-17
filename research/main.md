# Research and selection of a proving system
This document contains a comparative study of different proving systems in order to select the best alternative for verifying ZK-Snark proofs in Aiken. This research is the first [milestone](https://milestones.projectcatalyst.io/projects/1300084) of this catalyst [project](https://projectcatalyst.io/funds/13/cardano-open-developers/designing-an-api-for-zk-snark-proof-verification-in-aiken-eryx). Our main conclusion is that the best fit for this project are either groth16 or plonk proving systems.

The study focuses on popular implementations of proving systems such as: groth16 *(gnark / arkworks / snarkjs)*, Plonk *(gnark / arkworks / snarkjs)*, Fflonk *(snarkjs)*, Halo2, Barretenberg, Jolt, Honk, XNova, Polymath, Plonky2, Plonky3, Stone, Stwo, Risc0, sp1 and boojum.

To choose the best alternative we consider properties such as: proof size, proving/verification time, and the reliability of the protocol based on its current usage. Proving systems can be classified based on which polynomial commitment scheme (PCS) they use. This is important because proving systems using the same PCS share properties that allow to simplify the analysis.

Let's start the analysis by exploring the available plutus primitives.

## Leveraging builtins
As a first step, we'll analyze which proving systems leverage the current capabilities of Cardano. In the [appendix](#appendix) there's a list of relevant zk-friendly primitives in PlutusTx. Most available primitives focus on elliptic curve operations and pairings over **BLS12-381**. These were specifically added to reduce the costs of on-chain verification. Other important primitives include the computation of hash functions like **Blake2b** or **SHA256**.

We'll restrict to those proving systems that rely on commitment schemes that use either **BLS12-381** operations or **FRI** with some of the supported hashes. This rules out many popular provers that don't have battle tested implementations for the **BLS12-381** curve, such as Halo2.

## Challenges of using FRI based proving systems
Now we'll analyze FRI-based proving systems. A large number of mainstream proving systems and zk-VMs use FRI as a commitment scheme. This commitment scheme has the advantage of not requiring a trusted setup, but the downside of having a larger proof size.

The proof size depends on several factors:
- **FRI settings**: parameters such as the number of queries and the blow-up factor. These parameters affect the security of the protocol.
- **Degree of polynomials**: different arithmetic representations can lead to different proof sizes.
- **Optimizations**: common optimizations such as batch openings for the vector commitment scheme also help to reduce the proof size.
 
Let's analyze how large would a FRI proof be in an optimistic setting.

### FRI protocol
The FRI protocol has two phases:
- **Commit phase:** the prover commits to $k$ polynomials $(f_i)_{i=0}^{k-1}$ of degrees $2^{n - i}$, for some $k<n$. The commitment to a each polynomial $f_i$ is the root of a Merkle tree of evaluations of $f_i$ at a domain $D_i$ of size $2^{n - i + b}$. The values $k$ and $b$ are parameters of the protocol.

- A **query phase** where the verifier chooses $s$ random indexes and gets the evaluations of all the polynomials $f_i$ at elements in $D_i$ indexed by them. Along with those evaluations, the prover also sends batched Merkle authentication paths for them. These authentication paths are the main contributors to the size of a FRI based proving system.

Let's say the target security bits of the system is around 80. The most optimistic parameters are given by what's called *the conjectured security*. This states that the currently known attacks are the only possible attacks to the FRI protocol, and claims that the security bits of a FRI proof is given by $b \cdot s$. On top of it one can also perform proof of work to increase the security bits. Some standard configurations can be found in the default settings of popular proving systems such as Plonky2. Examples are $s=28$ and $b=3$ and $s=31$ and $b=2$.

### Estimating the proof size
The following is a Python script that computes an estimate of the expected number of hashes in a FRI query phase:

```python
from functools import lru_cache
from math import comb

def P(k, n):
    # Binomial probability
    return comb(n, k) / 2**n

@lru_cache(None)
def E(h, s):
    """
    Returns the expected number of hashes in an optimized batch
    authentication path for a Merkle tree with height `h`.
    """
    if s == 1:
        return h
    if h == 0:
        # No hashes needed for depth 0
        return 0
    
    expected_value = 0

    # If all sampled values fall into one of the two main subtrees,
    # then the root of the other main subtree needs to be added to 
    # the authentication path to reach the root of the tree.
    expected_value += P(0, s) * (E(h - 1, s) + 1)
    expected_value += P(s, s) * (E(h - 1, s) + 1)

    # If sampled values split across both the two main subtrees,
    # then the batch authentication path is the concatenation of
    # the authentication paths for the subtrees.
    for k in range(1, s):
        expected_value += P(k, s) * (E(h - 1, k) + E(h - 1, s - k))

    return expected_value

# Example usage
if __name__ == "__main__":
    n = 10 # log of number of constraints
    b = 2  # log blow up factor
    s = 31 # number of queries
    m = 4

    # Sum all the hashes for the batch authentication paths of all
    # the polynomials f_i in the FRI query phase.
    fri_number_of_hashes = sum([E(h, s) for h in range(m, n + b)])
    print(f"Expected number of hashes in FRI query phase: {fri_number_of_hashes:.4f}")
    
    # Expected number of hashes in FRI query phase: 549.7972
```

This means that a bare minimum of 550 hashes will be needed for any secure FRI proof. Tipically, hashes consist of 32 bytes, which makes the size of any FRI proof at least 17 KB. Which is a minimum for this toy example with 1024 constraints. Usually this is multiplied by several factors and proofs sizes are at least 500 KB for any useful program.

These proof sizes largely exceed the current maximum allowed size of a transaction in Cardano (See [Appendix](#appendix)). For this reason we rule out the following FRI-based proving systems for on-chain proof verification:  Plonky2, Plonky3, Stone, Stwo, Risc0, sp1 and boojum.

## Suitable proving systems
The choice for on-chain verification boils down to the following proving systems: **groth16** and **plonk**. Note that recursion might allow for different off-chain verification strategies in the future, for example by using a groth16 circuit of a Plonky2 verifier. Some projects in this direction are [this plonky2 verifier in circom](https://github.com/polymerdao/plonky2-circom) and [this plonky2 verifier in gnark](https://github.com/succinctlabs/gnark-plonky2-verifier).

### Groth16
[Groth16](https://eprint.iacr.org/2016/260.pdf) is one of the most efficient zk-SNARKs in terms of proof size and verification time. It has been widely used in blockchain applications because it keeps costs low. The tradeoff is the need for a trusted setup.

This is a preprocessing step required in some zk-SNARKs producing two main outputs: a proving key, used to generate proofs, and a verifying key, used to verify them. In Groth16, this setup is circuit-specific, meaning a new SRS is needed for every different circuit. The main concern with trusted setups is that if the randomness used in the ceremony is compromised, the system’s security could be at risk.

To address the risks of a compromised trusted setup a multi-party computation (MPC) is generally used to generate it. In this process, multiple participants contribute randomness, ensuring that as long as one participant remains honest and discards their secret, the final setup remains secure. The ceremony produces the necessary elliptic curve points, which are then used to construct the proving and verifying keys.

The most relevant elements to analyze the on-chain execution of the groth16 verifier are:
- The size of the verification key.
- The complexity and operations of the verification algorithm.

#### Verification key size
If the number of public inputs of the circuit is $M$, the verification key consists of:
- 3 elliptic curve points on $\mathbb{G}_2$.
- $M + 1$ elliptic curve points on $\mathbb{G}_1$.

As we're using the **BLS12-381** elliptic curve, each element in $\mathbb{G}_1$ can be compressed to 48 bytes, and the ones in $\mathbb{G}_2$ to 96 bytes. Therefore, the verification key size is $96 \cdot 3  + 48 \cdot (M + 1)$ bytes.

#### Verifier complexity and proof size
Proof verification is relatively fast, and all the operations needed for verification are supported by the Plutus **BLS12-381** builtins:
- $M$ scalar multiplications in $G_1$ (`bls12_381_g1_scalar_mul`)
- $M$ group additions in $G_1$ (`bls12_381_g1_add`)
- 4 Miller loops (`bls12_381_miller_loop`)
- 2 multiplications in the extension field (`bls12_381_mul_miller_loop_result`)
- 1 check that two elements in the extension field represent the same coset (`bls12_381_final_verify`)

The proof consists of:
- 2 elliptic curve points in $\mathbb{G}_1$,
- 1 elliptic curve point in $\mathbb{G}_2$.

Therefore, the size of a compressed groth16 proof over **BLS12-381** is $48\cdot 2 + 96 = 192$ bytes. Note that the only constructors of `G1Element` and `G2Element` in Aiken are `bls12_381_G1_uncompress` and `bls12_381_G2_uncompress` that take 48 and 96 byte strings as input. The **BLS12-381** builtin functions take as arguments objects of type `G1Element` and `G2Element`.

### Plonk
[Plonk](https://eprint.iacr.org/2019/953.pdf) is a popular zk-SNARK known for its flexibility and efficiency. In this context, we'll analyze Plonk with KZG over the **BLS12-381** curve. Unlike Groth16, it doesn't require a new trusted setup for each circuit. Although new circuits need a specific verification key, this key can be generated without the need of a trusted ceremony. This makes it more practical for many applications while still relying on a structured reference string (SRS) via the KZG commitment scheme.

Plonk's trusted setup can also be produced with a MPC ceremony like in the Groth16's case. As long as at least one participant discards their secret, the setup remains secure. This can mitigate concerns about trust assumptions.

#### Proof size
In Plonk, verification keys and proof sizes are independent of the number of public inputs. The verification key consists of:

- $9$ elliptic curve points on $\mathbb{G}_1$.
- $2$ elliptic curve points on $\mathbb{G}_2$.

Total verification key size: $96 \cdot 2 + 48 \cdot 9 = 624$ bytes.

#### Verifier complexity
The exact complexity of the Plonk verifier varies depending on the tricks and optimizations used. But, if the circuit has $M$ public inputs, then the verification of a proof involves approximately the following operations:

- ~$18$ scalar multiplications in $G_1$ (`bls12_381_g1_scalar_mul`)
- ~$18$ group additions in $G_1$ (`bls12_381_g1_add`)
- 2 Miller loops (`bls12_381_miller_loop`)
- 1 check that two elements in the extension field represent the same coset (`bls12_381_final_verify`)
- $O(M)$ operations on the scalar field. These are usually negligible compared to the above.

Pairings and elliptic curve operations are supported by their corresponding plutus **BLS12-381** builtins, shown between brackets.

The proof consists of:

- 9 elliptic curve points in $\mathbb{G}_1$,
- 6 scalar field element.

Total proof size: $48 \cdot 9 + 32 \cdot 6 = 624$ bytes, independent of $M$.


### Verification costs in Cardano
A rough estimate of how this operations translate to Cardano's cost model can be found in this [document](https://hackmd.io/@_XlWbpTTRNaI4BeB7d8qig/Bk4nCkWaj), related to [CIP-0381](https://cips.cardano.org/cip/CIP-0381) where the BLS12-381 builtins where suggested.

## Conclusion
Groth16 and Plonk over BLS12381 are currently the only suitable proving system for on-chain verification that have battle tested implementations. The main reason for this being the smaller proof size and verification keys of this proving systems. The most mature implementations of these proving systems are gnark, snarkjs and arkworks. In the future, other FRI based off-chain provers could be leveraged with the help of recursion, for example by wrapping a STARK in a SNARK proof. However, as a first step groth16 and plonk proving systems are the main options.

# Appendix

### Cardano's current ZK capabilities and limitations
- List of ZK friendly PlutusTx primitives 
    - [BLS12381 curve builtins](https://plutus.cardano.intersectmbo.org/haddock/1.34.0.0/plutus-tx/PlutusTx-Builtins-Internal.html)
        - data BuiltinBLS12_381_G1_ElementSource
        - data BuiltinBLS12_381_G1_Element
        - bls12_381_G1_equals
        - bls12_381_G1_add
        - bls12_381_G1_neg
        - bls12_381_G1_scalarMul
        - bls12_381_G1_compress
        - bls12_381_G1_uncompress
        - bls12_381_G1_hashToGroup
        - bls12_381_G1_compressed_zero
        - bls12_381_G1_compressed_generator
        - data BuiltinBLS12_381_G2_ElementSource
        - data BuiltinBLS12_381_G2_Element
        - bls12_381_G2_equals
        - bls12_381_G2_add
        - bls12_381_G2_neg
        - bls12_381_G2_scalarMul
        - bls12_381_G2_compress
        - bls12_381_G2_uncompress
        - bls12_381_G2_hashToGroup
        - bls12_381_G2_compressed_zero
        - bls12_381_G2_compressed_generator
        - data BuiltinBLS12_381_MlResult
        - bls12_381_millerLoop
        - bls12_381_mulMlResult
        - bls12_381_finalVerify
    - Hash builtins
        - sha2_256
        - sha3_256
        - blake2b_224
        - blake2b_256
        - keccak_256
        - ripemd_160
        
Note: There are no builtins for Poseidon hashes, which is a popular choice for many proving systems supporting recursion.

### Maximum transaction size
Currently the maximum allowed size for any transaction is set to 16384 bytes.
```bash
$ curl -H "project_id: {REPLACE_BLOCKFROST_PROJECT_ID}" https://cardano-mainnet.blockfrost.io/api/v0/epochs/latest/parameters | jq | grep max_tx_size

  "max_tx_size": 16384,
```
