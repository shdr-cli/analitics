#! /bin/bash
cd rust
cargo build --release
cp target/release/rust.exe ../Сборка/
cp ../.env ../Сборка/
