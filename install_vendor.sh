#!/bin/bash

BITPRIM_VERSION=0.9
conan install bitprim-node-exe/$BITPRIM_VERSION@bitprim/stable -o currency=BTC
rm bn
rm deploy_manifest.txt

for file in vendor/bitprim_btc/*
do
	file_name="${file##*/}"
	folder="${file_name:3:-2}"
	cp "$(find ~/.conan/data/$folder/$BITPRIM_VERSION -name $file_name)" vendor/bitprim_btc/${file##*/}
done
