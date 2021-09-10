# evm-contracts
EVM contracts indexer


# rinkeby logs

```
curl -v -H"Content-Type: application/json" http://localhost:8545 --data '[{"jsonrpc":"2.0", "method":"eth_getLogs", "params": [{
   "fromBlock": "0x75b4fc", "address": "0xF9C39ec11055508BddA0Bc2a0234aBbbC09a3DeC"}], "id": 1}]'
```