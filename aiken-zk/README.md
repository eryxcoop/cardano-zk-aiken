# Explanation (so far)
This repo contains a tool that facilitates the integration of Aiken validators with Zero Knowledge proofs. It allows the user to write Aiken programs that contain some ```offchain``` block. Instead of being executed on-chain, this fragment of the code will be replaced by a Groth16 validator.

### What does the tool actually do (so far)
Takes an Aiken source code (some_program.ak) as an input. This src can have **one** of the following new language tokens in the validator body:
* ```offchain addition(x, y, z)```: verifies that ```x + y == z```.
* ```offchain subtraction(x, y, z)```: vverifies that ```x - y == z```.
* ```offchain multiplication(x, y, z)```: verifies that verifies that ```x * y == z```.
* ```offchain fibonacci(x, y, z, w)```: verifies that the fibonacci sequence with initial values ```[x,y]``` and ```z``` elements ends with ```w```. In this case, ```z``` **must** be a literal number.
* ```offchain if(x, y, z, w)```: verifies that ```y == z if (x == 1) | y == w if (x == 0)```. ```x``` must be 1 or 0. 
* ```offchain assert_eq(x, y)```: verifies that ```x == y```

```x```,```y```,```z``` and ```w``` must be 
* **numeric literals** such as ```4``` or ```0xa3```.
* **single variable names** such as ```my_number```.

The program ```aiken-zk``` is used like:

```aiken-zk my_original_program.ak my_zk_program.ak```, where ```my_original_program.ak``` is the aiken source code with one of the tokens mentioned above, and ```my_zk_program.ak``` is the generated aiken code that includes a validator for such token.

# Prerequisites
To run the aiken-zk compiler you must have the following tools:
* Rust > 1.87.0 - https://www.rust-lang.org/tools/install
* Node >= 20.10.0 - https://nodejs.org/en/download
* Snark.js >= 0.7.5 - https://www.npmjs.com/package/snarkjs
* Circom >= 2.1.9 - https://docs.circom.io/getting-started/installation/
* Aiken >= 1.1.17 - https://aiken-lang.org/installation-instructions

# Automated testing
To run the tests 
1. Go to the ```src/tests/sandbox/curve_compress``` and run ```npm install```.You'll only have to do this once.
2. Run ```cargo build```.
3. Run ```cargo test```.

# Manual testing

### Building
To run an example yourself, go to the directory ```milestone 2 - example```. First, enter the sub-directory ```curve_compress``` and run ```npm install```. Then go back and run the following:

```cargo run -- build validators_with_offchain/example.ak validators/output.ak```

The command will generate a modified aiken file in ```validators/output.ak```. This is the final source code, over which you can use the main aiken compiler 

```aiken check```

You'll notice that the generated aiken code has some extra imports, some weird looking ```SnarkVerificationKey``` at the end, and that the new token was replaced by a function call to the verification algorithm. The thing is, this is now pure aiken code, whereas the original code wasn't. You could try to build the original code with the aiken compiler, but it would definitively fail. The ```aiken-zk``` performs a pre-compilation phase

### Verifying the proof
