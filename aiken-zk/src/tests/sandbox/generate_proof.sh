#!/bin/bash

CIRCUIT_PATH="$1"

CIRCUIT_NAME=$(basename -s ".circom" "${CIRCUIT_PATH}")

VERIFICATION_KEY_PATH="$2"

INPUT_PATH="$3"

OUTPUT_PATH="build/"
mkdir -p ${OUTPUT_PATH}


circom "${CIRCUIT_PATH}" --r1cs --wasm --sym -p bls12381 -o ${OUTPUT_PATH}

node "${OUTPUT_PATH}${CIRCUIT_NAME}_js/generate_witness.js" "${OUTPUT_PATH}${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${INPUT_PATH}"  ${OUTPUT_PATH}witness.wtns

snarkjs groth16 prove "${VERIFICATION_KEY_PATH}" ${OUTPUT_PATH}witness.wtns ${OUTPUT_PATH}proof.json ${OUTPUT_PATH}public.json

node curve_compress/compressedProof.js ${OUTPUT_PATH}proof.json > proof.ak
