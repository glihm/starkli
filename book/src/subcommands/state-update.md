# state-update subcommand

Get state update from a certain block.

By default, it's the state update for the latest block.

```bash
$ starkli state-update

{
  "block_hash": "0x35c08b58eaf8fd9db014f7a1ccd99d143eb4c38c3f6e32c2e01f054a863d277",
  "new_root": "0x782f901f3d869b21cb4ea43962a0890e9e6699df5ba8740b6cf23111a533edc",
  "old_root": "0x1d1178fe327384a631109d2062e2b53be4f9627d1e0bfb879a14cbfab88c1e3",
  "state_diff": {
    "storage_diffs": [
      {
        "address": "0x61fdcf831f23d070b26a4fdc9d43c2fbba1928a529f51b5335cd7b738f97945",
        "storage_entries": [
          {
            "key": "0xc6a8895c81c502dbda64e2111af3127204a93e6eac70300751892bb41ba2bb",
            "value": "0x2a8828e47d38e6c35"
          },
          ...
        ]
      }
    ],
    "deprecated_declared_classes": [],
    "declared_classes": [],
    "deployed_contracts": [],
    "replaced_classes": [],
    "nonces": [
      {
        "contract_address": "0x5476feeefe85e5fd6b318ed08fcca706bb80880be1d22dba71f49a49fa1320b",
        "nonce": "0x1c1"
      },
      ...
    ]
  }
}
```

You can specify a block with a `BLOCK_ID` argument,
which can be a `TAG`, `HASH` or `NUMBER`.

```bash
$ starkli state-update 88
```
