# selector subcommand

Calculate select from name.

The selector is an identifier through which the function is callable in
transactions or in other classes. The selector is the
[starknet_keccak](
https://docs.starknet.io/documentation/architecture_and_concepts/Hashing/hash-functions/#starknet-keccak)
hash of the function name, encoded in ASCII.

```bash
starkli selector my_func
0x0084829ac21528acb5aa5ddebd65c58d9738238cabb243f35da83e7ba516c480
```
