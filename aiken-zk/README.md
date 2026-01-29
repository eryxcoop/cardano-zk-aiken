# Aiken-zk

This repo contains a tool that facilitates the integration of Aiken validators with Zero Knowledge proofs. It allows the
user to write Aiken programs that contain some offchain block of code. This creates a new source code so that instead of being
executed on-chain, the program will receive a proof of that execution and validate it. This is useful to move
computation out of the chain and/or to hide information.

The compilation of the code will also create a circom component that represents the desired computation, compile and
execute it and generate a verifying key. This verifying key will be included in the new source code, along with some
code to verify a proof for that specific program. This proof has to be generated in order to validate this Aiken
program onchain.

The proving system used for this process is Groth16.

## Prerequisites

To run the aiken-zk compiler you must have the following tools:

* Rust > 1.87.0 - https://www.rust-lang.org/tools/install
* Node >= 20.10.0 - https://nodejs.org/en/download
* Snark.js >= 0.7.5 - https://www.npmjs.com/package/snarkjs (Install it globally)
* Circom = 2.2.3 - https://docs.circom.io/getting-started/installation/ (```git checkout v2.2.3```)
* Aiken >= 1.1.17 - https://aiken-lang.org/installation-instructions

## Installation
Step on the ```aiken-zk``` subfolder and run ```cargo install --path .``` command (dot included). This will install 
```aiken-zk``` globally so you can use it from anywhere. The available commands are 
* ```aiken-zk new <project_name>``` --> Use this to create a new project
* ```aiken-zk build <args described below>```
* ```aiken-zk prove <args described below>```

## Tutorial
You can find a full-workflow tutorial for this tool in [this medium post](https://medium.com/eryxcoop/aiken-zk-tutorial-d11b440a7d1a).

## Development cycle
To take advantage of this new added capability, the following development cycle can be used:

1. Code your Aiken validator including the new ```offchain``` keyword.

2. Build that code using ```aiken-zk```, this will generate an Aiken code, a circuit and a verification key
  which will be used to execute and verify the offchain portion of your computation.

3. Compile the aiken code and deploy it to the chain, so it is available to be unlocked.

4. Distribute the code with offchain, the circuit and the verification key to users, so they can perform
  the offchain computation and generate the zk-proof necessary to unlock the on-chain validator.

5. Now, as someone who wants to use the onchain code:
   * Perform the offchain computation you need. This is when you will use your private parameters along the public ones. As a product of this task you will have a proof file.

   * This file will be added to the usual execution/testing/unlocking of the onchain validator.

## Testing

To run the tests just go to the ```aiken-zk``` sub-directory and run 
```bash
./test.sh
```  
This will install the javascript dependencies, build the code and run the tests. 

# Capabilities description

## Offchain statement

This tool extends the Aiken language with a new ```offchain``` keyword. This means that a programmer can write an Aiken
source code with **one** (and **only** one) of the tokens presented below.

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
* ```offchain polynomial_evaluations(t, A, r, B, C)```: verifies that the polynomial $P$ with grade $t$ represented by the coefficients $A$ matches $\forall i \in [1\ldots r]$ $P(B[i]) = C[i]$.
* ```offchain merkle_tree_checker(merklePathLength, leaf, merkleRoot , pathElements, pathIndices)```: verifies that the merkle path formed by $pathElements$, in which each element is on the left or the right (indicated by $0$s and $1$s respectively in $pathIndices$), where $leaf$ is the value of the leaf and $merkleRoot$ is the root of the tree, is correct. $merklePathLength$ is the length of the merkle path without the root and **must** be a constant in compilation time. **Warning: do not use in production since the hash function is being mocked for now.**


* ```offchain custom(path/to/circom/component.circom, [pi0, pi1, ...])``` allows you to use your own circom **components** in aiken code. You must provide a path to a circom file with a ```main``` component. Then, as you can see in the definition, you must pass a list of public inputs (in the same order that are defined in the template, without the ```pub``` keyword, ignoring private parameters). **The amount of public inputs must be the same in the aiken code and the component definition**. By default, the tool will assume that each public input is a single number, but what if you know that some aiken variable is storing a list of numbers instead of a number? Well, in that case you must prepend the symbol ```@``` to the variable. 
 
Let's see an example: 
```rust
let array = [1,2,3,4,5]
let num = 3
offchain custom("path/to/circom/component.circom", [@array, num])
```
This will let our compiler know that the ```array``` variable contains a list. If the path is defined correctly, the workflow for custom circuits will now follow the same steps as the example. When using custom circuits, you must provide an array with the public inputs alone. 
 

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

Also, the redeemer variable **must** be named ```redeemer```.

## Convertion to Aiken

A written source code that includes an offchain statement is not compilable by an Aiken compiler. So, in order to compile
the Aiken code for testing or deployment purposes we provide a tool to generate valid Aiken code. This step replaces the 
offchain code with a ZK Groth16 verifier. In a later step, a proof will be needed to prove on-chain the execution of the
piece of code (executed offchain).

The tool provides the following command for this convertion:

```aiken-zk build code_with_offchain.ak aiken_code.ak```

This command will generate a modified aiken file in ```aiken_code.ak```. This is the final source code, over which
you can use the main aiken compiler either to run the tests or deploy to blockchain.

```aiken build```

You'll notice that the generated aiken code has some extra imports, some weird looking ```SnarkVerificationKey``` at the
end, and that the new token was replaced by a function call to the verification algorithm. The thing is, this is now
pure aiken code, whereas the original code wasn't. You could try to build the original code with the aiken compiler, but
it would definitively fail. The ```aiken-zk``` performs a pre-compilation phase.

The compilation output includes additional files needed for proof generation (that will be used on testing and
deployment steps):

- **circom circuit**: generated circuit that matches the function behaviour (```addition``` in the example). If you're using
    a custom Circom circuit, ```aiken-zk``` won't generate this artifact automatically and will use yours instead.
- **verification key**: the blueprint of the zk circuit generated that it's also included on the output aiken program. Be
  careful, different compilations from the same program generates different verification keys.

## Proof generation

The tool supports code generation for Aiken testing and MeshJS unlocking. It hides the proof generation details to the
contract consumer.

Aiken-zk provides a ```prove``` command, with variants ```prove aiken``` and ```prove meshJS``` depending on how do you
need to present it.

Any command asks for the same parameters ir order:

* **circom circuit**: from building step.
* **verification key**: from building step.
* **inputs**: the inputs for the function provided by the user. For example, inputs a,b,c for the offchain addition
  statement where a+b should be equal to c. This is a json file that you as a user have to create in order to feed
  the proof generation algorithm. This file can contain sensitive data, so if you're thinking about putting the pre-image
  of a hash in this file, you are correct.

### Aiken testing

Since we defined a type ZK<Redeemer> for the redeemer, a proof is needed to complete the variable values on testing.

So, if you run the following:

```aiken-zk prove aiken output.circom verification_key.zkey inputs.json proof.ak```

You'll get a proof.ak ready to be copy&pasted on the Aiken test.

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

```aiken-zk prove meshjs output.circom verification_key.zkey inputs.json zk_redeemer.ts```

Your unlocking code script might look like:

```javascript
txBuilder.spendingPlutusScript("V3")
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
txBuilder.spendingPlutusScript("V3");
...
.
txInRedeemerValue(mZKRedeemer(REDEEMER_WITHOUT_ZK))
...
.
complete()
```

This function will wrap your redeemer with additional information about the proof.

Now you have all you need to deploy the script into the blockchain.