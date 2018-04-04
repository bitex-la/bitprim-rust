#!/bin/bash

rm -rf tests/btc-testnet-files/*
reset
cargo test $1 -- --test-threads=1 --nocapture

