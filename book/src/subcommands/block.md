# block subcommand

Get Starknet block.

By default, it returns the latest block with transactions hashes only.

```bash
$ starkli block

{
  "status": "ACCEPTED_ON_L2",
  "block_hash": "0x27081f785bb3c5f9182960e7ef45aad0476642e82180291ef1759c1a429c857",
  "parent_hash": "0x22736a995e3096e844a581497c655552d24d0319af6c5e81484ff9557894b1c",
  "block_number": 821990,
  "new_root": "0x5e837b6957f8e63a61ad9369816650876c19c2a88f9269a31c20cf3588bd39d",
  "timestamp": 1687389385,
  "sequencer_address": "0x1176a1bd84444c89232ec27754698e5d2e7e1a7f1539f12027f28b23ec9f3d8",
  "transactions": [
    "0x1b9b3ae3d89aa88a48c13394dfdbc984bceb2e4c085b1ddce07fca41a013c2",
    ...
    "0x576673c2b700fc09b69ebb768f647e028a3b1b585eeb1056764b5145dfba66f"
  ]
}
```

You can specify a block with a `BLOCK_ID` argument,
which can be a `TAG`, `HASH` or `NUMBER`.

```bash
$ starkli block 88
```

If you also want the full content of the block, use the `--full` option.

```bash
$ starkli block --full 88
```
