# storage subcommand

Get storage value for a slot at a contract.

Contract address and storage are required.

If you have used the `Storage` struct in your contract,
the key can be obtained computing the `starknet_keccak` of the field name.

```rust
struct Storage {
    _name: felt252,
}
```

The `starknet_keccak` it's used by `starkli selector` command:

```bash
$ starkli selector _name

0x03a858959e825b7a94eb8d55c738f59c7bf4685267af5064bed5fd9c6bbc26de
```

Then check the storage for the contract:

```bash
$ starkli storage 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 \
                  0x03a858959e825b7a94eb8d55c738f59c7bf4685267af5064bed5fd9c6bbc26de
```
