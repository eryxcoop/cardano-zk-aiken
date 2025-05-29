#!/bin/bash

CIRCUIT_PATH="$1"

CIRCUIT_NAME=$(basename -s ".circom" "${CIRCUIT_PATH}")

INPUT_PATH="$2"

OUTPUT_PATH="build/"
mkdir -p ${OUTPUT_PATH}

CEREMONY_PATH="$3"

# Compiles circuit to wasm
circom "${CIRCUIT_PATH}" --r1cs --wasm --sym -p bls12381 -o ${OUTPUT_PATH}

# Generates witness
node ${OUTPUT_PATH}${CIRCUIT_NAME}_js/generate_witness.js ${OUTPUT_PATH}${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${INPUT_PATH}  ${OUTPUT_PATH}witness.wtns

# Groth16 setup
snarkjs groth16 setup ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs ${CEREMONY_PATH} ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey

echo "sdkadkaskdas" | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey --name="1st Contributor Name" -v

echo "dasfaefaef" | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0002.zkey --name="Second contribution Name" -v 

snarkjs zkey beacon ${OUTPUT_PATH}${CIRCUIT_NAME}_0002.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"

snarkjs zkey export verificationkey ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey ${OUTPUT_PATH}verification_key.json

# Generates proof
snarkjs groth16 prove ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey ${OUTPUT_PATH}witness.wtns ${OUTPUT_PATH}proof.json ${OUTPUT_PATH}public.json

# Verify
snarkjs groth16 verify ${OUTPUT_PATH}verification_key.json ${OUTPUT_PATH}public.json ${OUTPUT_PATH}proof.json

# Serialize circuit information
# snarkjs r1cs export json ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs.json

