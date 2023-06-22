# transaction subcommand

Get Starknet transaction by hash.

```bash
$ starkli transaction 0x06a9f49148992175694e5bb5a34a352d775059117fcf987d4478f7d0f729860c

{
  "transaction_hash": "0x6a9f49148992175694e5bb5a34a352d775059117fcf987d4478f7d0f729860c",
  "max_fee": "0x513c37f06f8",
  "version": "0x1",
  "signature": [
    "0x566ebf70fdedc2fa34a459299127ec6976ff93cd703009ba43a146ff0296c87",
    "0x6cfe2ca282d4e32a3ef883298e73e349f64869b346d29481d9eca298cb83daf"
  ],
  "nonce": "0x10",
  "type": "INVOKE",
  "sender_address": "0x136b3bdee77cfcb343a2f30ee08fd5340fc83688b3514bc89bea4011fad720f",
  "calldata": [
    "0x1",
    "0x259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478",
    "0x166d775d0cf161f1ce9b90698485f0c7a0e249af1c4b38126bddb37859737ac",
    "0x0",
    "0x1",
    "0x1",
    "0x737461726b6e6574"
  ]
}
```
