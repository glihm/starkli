# chain-id subcommand

Get Starknet network ID.

```bash
$ starkli chain-id

0x534e5f474f45524c49 (SN_GOERLI)
```

To avoid the decoded string, use `--no-decode` option:

```bash
$ starkli chain-id --no-decode

0x534e5f474f45524c49
```

To output in decimal, use `--dec` option:

```bash
$ starkli chain-id --no-decode --dec

1536727068981429685321
```
