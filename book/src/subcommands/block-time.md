# block-time subcommand

Get Starknet block timestamp only.

By default, it's the block time of the latest block.

```bash
$ starkli block-time

2023-06-21T23:23:33+00:00
```

You can specify a block with a `BLOCK_ID` argument,
which can be a `TAG`, `HASH` or `NUMBER`.

```bash
$ starkli block-time 88

2021-11-15T17:41:46+00:00
```

Use `--unix` option to output a unix timestamp:

```bash
$ starkli block-time --unix 88

1636998106
```

Or the RFC 2822 output with `--rfc2822`:

```bash
$ starkli block-time --rfc2822 88

Mon, 15 Nov 2021 17:41:46 +0000
```
