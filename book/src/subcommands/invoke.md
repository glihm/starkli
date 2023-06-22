# invoke subcommand

Send an invoke transaction from an account contract.

To send a transaction, you must provide a signer and account, followed
by the contract address, the selector and arguments.

You can use the `--watch` option to wait for the transaction to confirm.

```bash
$ starkli invoke --watch \
                 --keystore key_1 \
                 --account account_1 \
                 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 \
                 name_set \
                 0x737461726b6e6574
```

To only estimate the fees of the transaction without actually sending the transaction, you can run:

```bash
$ starkli invoke --estimate-only \
                 --keystore key_1 \
                 --account account_1 \
                 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 \
                 name_set \
                 0x737461726b6e6574

0.000001281906213888 ETH
```
