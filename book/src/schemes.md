# Starkli Schemes

In starknet, everything is felt. So by default, starkli is taking any argument as felt.
To help serializing the arguments, starkli introduces schemes.

Schemes are used with the following syntax: `scheme_name:value`.

More awesome schemes are coming soon! :)

# Short String (str)

For now Cairo only has short string.

```
starkli call <CONTRACT_ADDRESS> str:starkli
```

You can use the [to-cairo-string](./subcommands/to-cairo-string.md) command
if you want to convert it manually.

# u256

`u256` are represented as a struct of 2 `u128` in Cairo. One `u256` doesn't fit into
one felt, but `u128` does.

For this reason, a serialized `u256` in Cairo is represented as two felts:

```rust
let u: u256 = 1_u256;

// this will be serialized as `[0x1, 0x0]`.
```

Using starkli:

```bash
starkli call <CONTRACT_ADDRESS> u256:1

# without the scheme:
starkli call <CONTRACT_ADDRESS> 1 0
```

# Constants

To have handy constants accessible quickly, starkli has the `const:` scheme that can include:

`u256_max` and `felt_max` for now, and those are the canonical way to write those schemes:

```
starkli call <CONTRACT_ADDRESS> const:u256_max

starkli call <CONTRACT_ADDRESS> const:felt_max
```
