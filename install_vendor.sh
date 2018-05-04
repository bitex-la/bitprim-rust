#!/bin/bash

BITPRIM_VERSION=0.9
conan install bitprim-node-exe/$BITPRIM_VERSION@bitprim/stable -o currency=BTC
rm bn
rm deploy_manifest.txt

for file in vendor/bitprim_btc/*
do
	file_name="${file##*/}"
	folder=~/.conan/data/${file_name:3:-2}
	if [ -e  $folder ]
	then
		if [ -e  $folder/$BITPRIM_VERSION ]
		then
			cp "$(find $folder/$BITPRIM_VERSION -name $file_name)" vendor/bitprim_btc/${file##*/}
		else
			last_version="$(ls -1 $folder | tail -n 1)"
			cp "$(find $folder/$last_version -name $file_name)" vendor/bitprim_btc/${file##*/}
		fi
	fi
done
