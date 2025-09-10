# Explanation (so far)

This repo contains a tool that facilitates the integration of Aiken validators with Zero Knowledge proofs. It allows the
user to write Aiken programs that contain some offchain block. This creates a new source code so that instead of being
executed on-chain, the program will receive a proof of that execution and validate it. This is useful to move
computation out of the chain and/or to hide information.

The compilation of the code will also create a circom component that represents the desired computation, compile and
execute it and generate a verifying key. This verifying key will be included in the new source code, along with some
code to verify a proof for that specific program. This proof has to be generated in order to invoque/execute this Aiken
program onchain.

The proving system used for this process is Groth16.

## Development cycle

To take advantage of this new added capability, the following development cycle can be used:

- Code your Aiken validator including the new offchain keyword.

- Convert that code, this will generate both an Aiken code as well as a circuit and a verification key
  which will be used to execute and verify the offchain portion of your computation.

- Compile the aiken code and deploy it to the chain, so it is available to be unlocked.

- Distribute the code with offchain, the circuit and the verification key to users so they can perform
  the offchain computation and generate the zk-proof necessary to unlock the on-chain validator.

- Now, as someone who wants to use the onchain code:
    - Take the offchain portion, perform the computation you need. This is when you will use your private parameters
      along
      the public ones. As a product of this task you will have a proof file.

    - This file will be added to the usual execution/testing/unlocking of the onchain validator.

# Prerequisites

To run the aiken-zk compiler you must have the following tools:

* Rust > 1.87.0 - https://www.rust-lang.org/tools/install
* Node >= 20.10.0 - https://nodejs.org/en/download
* Snark.js >= 0.7.5 - https://www.npmjs.com/package/snarkjs
* Circom = 2.1.9 - https://docs.circom.io/getting-started/installation/ (```git checkout v2.1.9```)
* Aiken >= 1.1.17 - https://aiken-lang.org/installation-instructions

The idea in the future is to reduce the amount of dependencies.

## Alternative: use docker

To avoid installing all the dependencies listed above, you can use docker. Run

```docker pull bweisz/aiken-zk:latest```

then run

```docker run -it bweisz/aiken-zk:latest``` to create a container and start running bash commands inside it.

Inside the container, run ```./start.sh``` to install some dependencies and build the tool.
Then, you can proceed to automated or manual testing.

## Testing setup

You can avoid the first 2 steps if you are going with the Docker version, since they've been done in
the ```./start.sh``` command. To run the tests:

1. Go to the ```src/tests/sandbox/curve_compress``` and run ```npm install```. You'll only have to do this once.
2. Run ```cargo build```.
3. Run ```cargo test```.

# Capabilities description

## Offchain statement

It extends the Aiken language with a new ```offchain``` keyword. This means that a programmer can write an Aiken
source code with **one** of the tokens presented below.

### Parameters convention
* ```x```, ```y```, ```z```, ```w```: integer literal, integer variable name 
* ```t```, ```r```: integer literal
* ```A```, ```B```: list variable name

#### Types description
* **integer literal** such as ```4``` or ```0xa3```.
* **integer variable name** such as ```my_number```.
* **list variable name** such as ```my_list```. It cannot be a list literal as ```[1,2]```. 


### Supported tokens
* ```offchain addition(x, y, z)```: verifies that $x + y = z$.
* ```offchain subtraction(x, y, z)```: verifies that $x - y = z$.
* ```offchain multiplication(x, y, z)```: verifies that $x * y = z$.
* ```offchain fibonacci(x, y, t, z)```: verifies that the fibonacci sequence with initial values $[x,y]$ and $t$
  elements ends with $z$. In this case, $t$ **must** be a literal number.
* ```offchain if(x, y, z, w)```: verifies that $y = z \,\,\,\rm{if} (x = 1)\, |\, y = w \,\,\,\rm{if} (x = 0)$. $x$ must be 1 or 0.
* ```offchain assert_eq(x, y)```: verifies that $x = y$
* ```offchain sha256(t, A, B)```: verifies that $sha256(A) = B$ with $size(A) = t$ and $size(B) = 256$. A and B being lists of **bits**.
* ```offchain poseidon(t, A, x)```: verifies that $poseidon(A) = x$ with $size(A) = t$ and A being a list of integers and x being an integer.
* ```offchain polynomial_evaluations(t, A, r, B, C)```: verifies that the polynomial $P$ represented by the coefficients $A$ match $\forall i \in [1\ldots r]$ $P(B[i]) = C[i]$.
* ```offchain merkle_tree_checker(merklePathLength, leaf, merkleRoot , pathElements, pathIndices)```: verifies that the merkle path formed by $pathElements$, in which each element is on the left or the right (indicated by $0$s and $1$s respectively in $pathIndices$), where $leaf$ is the value of the leaf and $merkleRoot$ is the root of the tree, is correct. $merklePathLength$ is the length of the merkle path without the root and **must** be a constant in compilation time. **Warning: do not use in production since the hash function is being mocked for now.**


* ```offchain custom(path/to/circom/component, [pi0, pi1, ...])``` allows you to use your own circom **components** in aiken code. You must provide a path to the circom file that includes the defined component, and pass a list for the public inputs (in the same order that are defined in the template, without the ```pub``` keyword, ignoring private parameters). The amount of public inputs must be the same in the aiken code and the component definition. 
 

### Public and private parameters

This new addition to the language allows you to declare some of the arguments as private. Some examples are:

* ```offchain addition(priv, pub y, z)```
* ```offchain addition(priv, priv, priv)```
* ```offchain assert_eq(priv, y)```

Any visibility combination is possible. If the visibility modifier is not present, the argument is assumed to be
**public**.

## Aiken changes

In order to use this zk capabilities your Aiken code must define the ZK type:

```
pub type ZK<redeemer_type> {
    redeemer: redeemer_type,
    proofs: List<Proof>,
}
```

And the redeemer variable must be named as ```zk_redeemer```.

## Convertion to Aiken

A written source code that includes an offchain statement is not compilable by an Aiken compiler.

So, in order to compile the Aiken code for testing or deployment purposes we provide a tool to generate valid Aiken
code.

This step replaces the offchain code with a ZK Groth16 verifier. In a later step, a proof will be needed to prove
the onchain execution of the piece of code executed offchain.

The tool provides the following command for this convertion:

```cargo run -- build code_with_offchain.ak aiken_code.ak```

This command will generate a modified aiken file in ```aiken_code.ak```. This is the final source code, over which
you can use the main aiken compiler either to run the tests or deploy to blockchain.

```aiken build```

You'll notice that the generated aiken code has some extra imports, some weird looking ```SnarkVerificationKey``` at the
end, and that the new token was replaced by a function call to the verification algorithm. The thing is, this is now
pure aiken code, whereas the original code wasn't. You could try to build the original code with the aiken compiler, but
it would definitively fail. The ```aiken-zk``` performs a pre-compilation phase.

The compilation output includes additional files needed for proof generation (that will be used on testing and
deployment steps):

- circom circuit: generated circuit that matches the function behaviour (```addition``` in the example)
- verification key: the blueprint of the zk circuit generated that it's also included on the output aiken program. Be
  careful, different compilations from the same program generates different verification keys

## Proof generation

The tool supports code generation for Aiken testing and MeshJS unlocking. It hides the proof generation details to the
contract consumer.

Aiken-zk provides a ```prove``` command, with variants ```prove aiken``` and ```prove meshJS``` depending on how do you
need
to present it.

Any command asks for the same parameters ir order:

- circom circuit: from building step
- verification key: from building step
- inputs: the inputs for the function provided by the user. For example, inputs a,b,c for the offchain addition
  statement where a+b should be equal to c

### Aiken testing

As the redeemer has type ZK<Redeemer>, a proof is needed to complete the variable values on testing.

So, if you run the following:

```cargo run -- prove aiken output.circom verification_key.zkey inputs.json proof.ak```

You'll get a proof.ak ready to be copy&paste on the Aiken test.

For example, a redeemer could be:

```
ZK { 
    redeemer: a_redeemer_without_zk,
    proofs: [ 
        Proof {
            piA: "complete with generated piA",
            piB: "complete with generated piB",
            piC: "complete with generated piC",
        }
    ] 
},
```

#### MeshJs contract unlocking

This step assumes that you made a contract deployment and you have its transaction hash.

Once you have a script for unlocking purposes, run the following command to generate a library
to be used on it:

```cargo run -- prove meshjs output.circom verification_key.zkey inputs.json zk_redeemer.ts```

Your unlocking code script might look like:

```javascript
txBuilder
    .spendingPlutusScript("V3")
...
.
txInRedeemerValue(REDEEMER_WITHOUT_ZK)
...
.
complete()
```

Then import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
 ```

And use the exported function.

```javascript
txBuilder
    .spendingPlutusScript("V3")
...
.
txInRedeemerValue(mZKRedeemer(REDEEMER_WITHOUT_ZK))
...
.
complete()
```

This function will wrap your redeemer with additional information about the proof.

Now you have all you need to deploy the script into the blockchain.