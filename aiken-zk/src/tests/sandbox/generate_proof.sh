#!/bin/bash

CIRCUIT_PATH="$1"

CIRCUIT_NAME=$(basename -s ".circom" "${CIRCUIT_PATH}")

VERIFICATION_KEY_PATH="$2"

INPUT_PATH="$3"

OUTPUT_PATH="$4"

BUILD_PATH="build/"
mkdir -p ${BUILD_PATH}

circom "${CIRCUIT_PATH}" --r1cs --wasm --sym -p bls12381 -o ${BUILD_PATH}

node "${BUILD_PATH}${CIRCUIT_NAME}_js/generate_witness.js" "${BUILD_PATH}${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm" "${INPUT_PATH}"  ${BUILD_PATH}witness.wtns

snarkjs groth16 prove "${VERIFICATION_KEY_PATH}" ${BUILD_PATH}witness.wtns ${BUILD_PATH}proof.json ${BUILD_PATH}public.json

node curve_compress/compressedProof.js ${BUILD_PATH}proof.json > "${OUTPUT_PATH}"
