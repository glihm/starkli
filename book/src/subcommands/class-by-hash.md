# class-by-hash subcommand

Get contract class by hash.

```bash
$ starkli class-by-hash 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea

{
  "sierra_program": [
    "0x1",
    ...
  ],
  "contract_class_version": "0.1.0",
  "entry_points_by_type": {
    "CONSTRUCTOR": [
      {
        "selector": "0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194",
        "function_idx": 2
      }
    ],
    "EXTERNAL": [
      {
        "selector": "0x166d775d0cf161f1ce9b90698485f0c7a0e249af1c4b38126bddb37859737ac",
        "function_idx": 1
      },
      {
        "selector": "0x293c9b0657d7591853c62ddc495b09ff833e04ad61f066dd7c8cc3a5b6b303d",
        "function_idx": 0
      }
    ],
    "L1_HANDLER": []
  },
  "abi": "[{\"type\": \"function\", \"name\": \"constructor\", \"inputs\": [{\"name\": \"name\", \"type\": \"core::felt252\"}], \"outputs\": [], \"state_mutability\": \"external\"}, {\"type\": \"function\", \"name\": \"name_get\", \"inputs\": [], \"outputs\": [{\"type\": \"core::felt252\"}], \"state_mutability\": \"view\"}, {\"type\": \"function\", \"name\": \"name_set\", \"inputs\": [{\"name\": \"name\", \"type\": \"core::felt252\"}], \"outputs\": [], \"state_mutability\": \"external\"}, {\"type\": \"event\", \"name\": \"NameGreeted\", \"inputs\": [{\"name\": \"name\", \"type\": \"core::felt252\"}]}, {\"type\": \"event\", \"name\": \"NameChanged\", \"inputs\": [{\"name\": \"previous\", \"type\": \"core::felt252\"}, {\"name\": \"current\", \"type\": \"core::felt252\"}]}]"
}
```
