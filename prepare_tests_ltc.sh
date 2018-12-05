#!/bin/bash

rm -rf tests/ltc-testnet-files/database
rm -rf tests/ltc-testnet-files/log
reset
cargo test runs_500_blocks_sync -- --test-threads=1 --nocapture
if [ $? -ne 0 ]
then
  echo "Failed to run test. Database is likely invalid. Fix stuff and try again."
else
  echo "Copying prepared database"
  mv tests/ltc-testnet-files/database tests/ltc-testnet-files/prepared_database
  echo "Ok all done"
fi
