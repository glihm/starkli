# Subcommands


List of available subcommands:  

| subcommand                                  | description                                                             |
|:-------------------------------------------:|:-----------------------------------------------------------------------:|
| [selector](selector.md)                     | Calculate selector from name                                            |
| [class-hash](class-hash.md)                 | Calculate class hash from any contract artifacts (Sierra, casm, legacy) |
| [to-cairo-string](to-cairo-string.md)       | Encode string into felt with the Cairo short string representation      |
| [parse-cairo-string](parse-cairo-string.md) | Decode string from felt with the Cairo short string representation      |
| [mont](mont.md)                             | Print the montgomery representation of a field element                  |
| transaction                                 | Get Starknet transaction by hash                                        |
| block-number                                | Get latest block number                                                 |
| block-hash                                  | Get latest block hash                                                   |
| block                                       | Get Starknet block                                                      |
| block-time                                  | Get Starknet block timestamp only                                       |
| state-update                                | Get state update from a certain block                                   |
| transaction-receipt                         | Get transaction receipt by hash                                         |
| chain-id                                    | Get Starknet network ID                                                 |
| nonce                                       | Get nonce for a certain contract                                        |
| storage                                     | Get storage value for a slot at a contract                              |
| class-hash-at                               | Get contract class hash deployed at a certain address                   |
| class-by-hash                               | Get contract class by hash                                              |
| class-at                                    | Get contract class deployed at a certain address                        |
| syncing                                     | Get node syncing status                                                 |
| wallet                                      | Wallet management commands                                              |
| account                                     | Account management commands                                             |
| deploy                                      | Deploy contract via the Universal Deployer Contract                     |
| completions                                 | Generate shell completions script                                       |


