# call subcommand

Call contract functions without sending transactions.

To call a contract `view` without sending a transaction you don't need
an account / signer.

Starkli also automatically compute the `selector` to make the call:

```bash
$ starkli call 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 name_get

[
    "0x000000000000000000000000000000000000000000000000737461726b6e6574"
]
```

If the `view` has more argument, they must follow the name of the `view`.
Starkli will not convert any argument, you must follow the `view` signature
for the types.

```bash
$ starkli call 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 \
               view_with_args \
               0x123 \
               0x6633
```
