#!/bin/bash

if [ -z "$1" ] ; then
	echo 'You need to pass a version number'
	exit 0
fi
BITPRIM_VERSION=$1

if [ -z "$2" ] ; then
	echo 'You need to pass a currency'
	exit 0
fi
CURRENCY=$2

FOLDERS=(bitprim-blockchain bitprim-consensus bitprim-core bitprim-database bitprim-network 
bitprim-node-cint bitprim-node boost gmp icu secp256k1)
FILES=(libbitprim-blockchain.a libbitprim-consensus.a libbitprim-core.a libbitprim-database.a
libbitprim-network.a libbitprim-node-cint.a libbitprim-node.a libboost_filesystem.a
libboost_iostreams.a libboost_log.a libboost_program_options.a libboost_regex.a
libboost_system.a libboost_thread.a libgmp.a libsecp256k1.a)

conan install bitprim-node-exe/$BITPRIM_VERSION@bitprim/stable -o currency=${CURRENCY^^}
conan install bitprim-node-cint/$BITPRIM_VERSION@bitprim/stable -o currency=${CURRENCY^^}
rm bn
rm deploy_manifest.txt

mkdir -p vendor/bitprim_${CURRENCY,,}

# ex. find_files folder
function find_files {
	for file in ${FILES[*]}
	do
		found=$(find $1 -name $file -print -quit)
		if [[ $found ]]
		then
			cp $found vendor/bitprim_${CURRENCY,,}/$file
		fi
	done
}

for folder in ${FOLDERS[*]}
do
	package_path=~/.conan/data/$folder
	if [ -e  $package_path ]
	then
		if [ -e  $package_path/$BITPRIM_VERSION ]
		then
			find_files $package_path/$BITPRIM_VERSION
		else
			last_version="$(ls -1 $package_path | tail -n 1)"
			find_files $package_path/$last_version
		fi
	fi
done
