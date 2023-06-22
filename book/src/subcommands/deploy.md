# deploy subcommand

Deploy contract via the Universal Deployer Contract.

To deploy a contract, you must specify the class hash of a previously
[declared](declare.md) contract.

The constructor arguments must follow the class hash, and must respect
the signature of the constructor.

```bash
$ starkli deploy --watch \
                 --keystore key_1 \
                 --account account_1 \
                 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea \
                 0x737461726b6c69
```

To only estimate the fees of this transaction, use `--estimate-only`:

```bash
$ starkli deploy --estimate-only \
                 --keystore key_1 \
                 --account account_1 \
                 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea \
                 0x737461726b6c69

0.000004380858954784 ETH
```
