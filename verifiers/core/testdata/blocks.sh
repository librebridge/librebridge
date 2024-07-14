#!/bin/bash

for block in {6308067..6308167}
do
    cast block $block --rpc-url https://eth-sepolia.api.onfinality.io/public --json
done
