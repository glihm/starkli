# declare subcommand

Declare a contract class.

To declare a contract, it must be compiled first. Refer to the
[tutorial](../cairolang-tutos/starkli-101.md) if you don't know how to compile a contract.

The program is compiled into it's Sierra representation, which is a `JSON` file.

```bash
$ starkli declare --watch \
                  --keystore key_1 \
                  --account account_1 \
                  contract1.json
```

To only estimate the fees of this transaction, use `--estimate-only`:
```bash
$ starkli declare --estimate-only \
                  --keystore key_1 \
                  --account account_1 \
                  contract1.json
```
