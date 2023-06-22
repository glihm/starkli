# class-at subcommand

Get contract class deployed at a certain address.

```bash
$ starkli class-at 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478

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
