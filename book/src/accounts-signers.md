# Accounts and Signer

Starkli makes a neat separation between accounts and signer.

## Signer

A signer (or wallet), refers to a software or a physical device used to store and manage
user's cryptographic keys, and the private key particularly. In fact, this private key
is used to sign transactions and thus interact with the blockchain.  

Starkli relies on [starknet-rs](https://github.com/xJonathanLEI/starknet-rs)
which is using  [web3 secret storage definition](
https://ethereum.org/en/developers/docs/data-structures-and-encoding/web3-secret-storage/)
to store the cryptographic keys locally.  

In web3 secret storage definition, a `keystore` is a JSON file
where your cryptographic keys are stored.

In this file, your private key is always stored encrypted,
and only decrypted on the fly when you execute a starkli
command providing the `keystore` file as argument.

For this reason, starkli is not assuming any default keystore
file (yet?), and you must provide it explicitly at each command.

See [signer subcommands](subcommands/signer.md).


---

## Account

Starknet supports account abstraction natively, which means that
any account on Starknet **is** a smart contract.  

Contracts (and thus accounts) are primarly defined by four things:
* [class](https://docs.starknet.io/documentation/architecture_and_concepts/Contracts/contract-classes/)
* [address](https://docs.starknet.io/documentation/architecture_and_concepts/Contracts/contract-address/)
* [storage](https://docs.starknet.io/documentation/architecture_and_concepts/Contracts/contract-storage/)
* [ABI](https://docs.starknet.io/documentation/architecture_and_concepts/Contracts/contract-abi/)

From those four attributes, and especially with the class, we can clearly
identify which kind of account we are dealing with.

For instance, OpenZeppelin account contract has a wellknown class:
`0x048dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292` than can be used
to verify the deployed account.

To this, we can add more information like:

* Address of the contract
* Salt used when address is pre-computed
* Public key associated to the private key used to sign the transaction to declare and deploy the contract

All of this is summarized in starkli into the account configuration file.
This file allows starkli having some context about the account to be used when
a command requires such information.

See [account subcommands](subcommands/account.md).
