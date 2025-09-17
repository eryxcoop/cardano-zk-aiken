#!/bin/bash

cd src/tests/sandbox/curve_compress && npm install
cd ../../../../milestone_example/curve_compress && npm install
cd ../../ && cargo build && cargo test