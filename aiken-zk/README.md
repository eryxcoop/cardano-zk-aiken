# Explanation (so far)
This repo contains a tool that facilitates the integration of Aiken validators with Zero Knowledge proofs. It allows the user to write Aiken programs that contain some offchain block. This creates a new source code so that instead of being executed on-chain, the program will receive a proof of that execution and validate it. This is useful to move computation out of the chain and/or to hide information.

The compilation of the code will also create a circom component that represents the desired computation, compile and execute it and generate a verifying key. This verifying key will be included in the new source code, along with some code to verify a proof for that specific program. This proof has to be generated in order to invoque/execute this Aiken program onchain.

The proving system used for this process is Groth16.

### What does the tool actually do (so far)
Takes an Aiken source code (some_program.ak) as an input. This src can have **one** of the following new language tokens in the validator body:
* ```offchain addition(x, y, z)```: verifies that ```x + y == z```.
* ```offchain subtraction(x, y, z)```: vverifies that ```x - y == z```.
* ```offchain multiplication(x, y, z)```: verifies that ```x * y == z```.
* ```offchain fibonacci(x, y, z, w)```: verifies that the fibonacci sequence with initial values ```[x,y]``` and ```z``` elements ends with ```w```. In this case, ```z``` **must** be a literal number.
* ```offchain if(x, y, z, w)```: verifies that ```y == z if (x == 1) | y == w if (x == 0)```. ```x``` must be 1 or 0. 
* ```offchain assert_eq(x, y)```: verifies that ```x == y```

```x```,```y```,```z``` and ```w``` must be 
* **numeric literals** such as ```4``` or ```0xa3```.
* **single variable names** such as ```my_number```.

### Public and private parameters
This new addition to the language allows you to declare some of the arguments as private. Some examples are
* offchain addition(priv x, pub y, z)
* offchain assert_eq(priv x, y)

If the visibility modifier is not present, the argument is assumed to be **public**.

The program ```aiken-zk``` is used like:

```aiken-zk build my_original_program.ak my_zk_program.ak```, where ```my_original_program.ak``` is the aiken source code with one of the tokens mentioned above, and ```my_zk_program.ak``` is the generated aiken code that includes a validator for such token.

# Prerequisites
To run the aiken-zk compiler you must have the following tools:
* Rust > 1.87.0 - https://www.rust-lang.org/tools/install
* Node >= 20.10.0 - https://nodejs.org/en/download
* Snark.js >= 0.7.5 - https://www.npmjs.com/package/snarkjs
* Circom = 2.1.9 - https://docs.circom.io/getting-started/installation/ (```git checkout v2.1.9```)
* Aiken >= 1.1.17 - https://aiken-lang.org/installation-instructions

The idea in the future is to reduce the amount of dependencies. 

# Alternative: use docker
To avoid installing all the dependencies listed above, you can use docker. Run

```docker pull bweisz/aiken-zk:latest```

then run 

```docker run -it bweisz/aiken-zk:latest``` to create a container and start running bash commands inside it.

Inside the container, run ```./start.sh``` to install some dependencies and build the tool.
Then, you can proceed to automated or manual testing. 


# Automated testing
You can avoid the first 2 steps if you are going with the Docker version, since they've been done in the ```./start.sh``` command. To run the tests:
1. Go to the ```src/tests/sandbox/curve_compress``` and run ```npm install```. You'll only have to do this once.
2. Run ```cargo build```.
3. Run ```cargo test```.

# Manual testing

### Building
To run an example yourself, go to the directory ```milestone_2_example```.

You can avoid this step if you are going with the Docker version:
Enter the sub-directory ```curve_compress``` and run ```npm install```.

Then go back and run the following:

```cargo run -- build validators_with_offchain/example.ak validators/output.ak```

The command will generate a modified aiken file in ```validators/output.ak```. This is the final source code, over which you can use the main aiken compiler 

```aiken check```

You'll notice that the generated aiken code has some extra imports, some weird looking ```SnarkVerificationKey``` at the end, and that the new token was replaced by a function call to the verification algorithm. The thing is, this is now pure aiken code, whereas the original code wasn't. You could try to build the original code with the aiken compiler, but it would definitively fail. The ```aiken-zk``` performs a pre-compilation phase

The compilation output includes additional files needed for proof generation (that will be used on testing and deployment steps):
- circom circuit: generated circuit that matches the function behaviour (```addition``` in the example)
- verification key: the blueprint of the zk circuit generated that it's also included on the output aiken program. Be careful, different compilations from the same program generates different verification keys


### Verifying the proof
The previous generated source code (validators/output.ak) includes a test that missing a valid proof to success.

Aiken-zk provides a command to generate the proof given the following elements:
- circom circuit from building step
- verification key from building step
- inputs: the inputs for the function provided by the user. Inputs a,b,c for the addition example where a+b should be equal to c

So, if you run the following:

```cargo run -- prove output.circom verification_key.zkey inputs.json proof.ak```

You'll get a proof.ak ready to be copy&paste on the test ```test_example```

Then, running an ```aiken check``` should execute successfully.