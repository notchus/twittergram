#!/bin/bash

# Run Rust schema generator
cargo run --bin schema --manifest-path ../shared/Cargo.toml

# Convert JSON schema to TypeScript
npx json2ts -i ../shared/schema.json -o ../frontend/shared/models.ts