# Getting started

Starkli is designed to be easy to use and effective, both
for developers and to be scripted. In this section you will find
some general behavior and design choices made for Starkli.

## Interacting with Starknet

In Starkli, most of the commands are interacting with the blockchain,
and thus require a provider to do so.

### JSON RPC provider
To setup a node URL, you can set the `STARKNET_RPC` environment variable.
Once set, you don't have to use `--rpc` argument in every and each command
you use in order to interact with the blockchain.

However, if you have to override the environment variable value for
very specific commands, adding the `--rpc` command takes precedence
and overrides the `STARKNET_RPC` value for this specific command only.

Specifying an URL for a node is enough, you don't have to provide the network
as it is automatically set.

```
$ export STARKNET_RPC=https://starknet-mainnet.xxx

$ starkli chain-id
0x534e5f4d41494e (SN_MAIN)

$ starkli chain-id --rpc https://starknet-goerli.xxx
0x534e5f474f45524c49 (SN_GOERLI)
```

### Gateway
In the current implementation, the Starknet gateway is also supported.
The gateway is the address of a sequencer on a server, which is currently used
to support the network before the total decentralization of Starknet.
However the use of the gateway is **strongly discouraged** as it will be deprecated.

To use the gateway you can first unset `STARKNET_RPC` as the two configurations
are exclusive and the RPC **always takes precedence** in Starkli, and will become the only
provider once the gateway will be deprecated. So if you use both, you'll have a warning
at each command you run.

Then, set `STARKNET_NETWORK` with one of the following values:
* `mainnet`
* `goerli-1`
* `goerli-2`

```
$ export STARKNET_NETWORK=mainnet

$ starkli chain-id
0x534e5f4d41494e (SN_MAIN)

$ starkli chain-id --network goerli-1
0x534e5f474f45524c49 (SN_GOERLI)
```


## Explicit accounts and wallets

As detailed in the [accounts and wallet chapter](./accounts-wallets.md),
Starkli makes a clear distinction between accounts contracts and wallets.

For this reason, each command associated with one of account or wallet requires an
explicit path to the corresponding file.

This is up to you to write some scripts combined to environment variables
to automatize / facilitate the use of those files.

Some examples:

Inspects the public key of a given keystore file.  
`$ starkli wallet keystore inspect ~/.keystore/key_1`

Fetches an account contracts configuration from the blockchain,
storing the configuration into the `account_1` file.  
`$ starkli account fetch --output ~/.accounts/account_1 0x1234`
