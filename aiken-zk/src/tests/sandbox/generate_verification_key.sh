#!/bin/bash

CIRCUIT_PATH="$1"
CEREMONY_PATH="$2"
RAND_1="$3"
RAND_2="$4"

CIRCUIT_NAME=$(basename -s ".circom" "${CIRCUIT_PATH}")
OUTPUT_PATH="build/"

mkdir -p ${OUTPUT_PATH}


# Compiles circuit to wasm
circom "${CIRCUIT_PATH}" --r1cs --wasm --sym -p bls12381 -o ${OUTPUT_PATH}

# Groth16 setup
snarkjs groth16 setup ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs ${CEREMONY_PATH} ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey

echo $RAND_1 | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey --name="1st Contributor Name" -v

echo $RAND_2 | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0002.zkey --name="Second contribution Name" -v

snarkjs zkey beacon ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey verification_key.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
