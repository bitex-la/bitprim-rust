#!/bin/bash

# Should be last version
BITPRIM_VERSION=0.9
conan install bitprim-node-exe/$BITPRIM_VERSION@bitprim/stable -o currency=BTC
rm bn
rm deploy_manifest.txt

for file in vendor/bitprim_btc/*
do
	path_file="$(find ~/.conan/data/bitprim-blockchain/$BITPRIM_VERSION -name "${file##*/}")"
	cp $path_file vendor/bitprim_btc/${file##*/}
done
