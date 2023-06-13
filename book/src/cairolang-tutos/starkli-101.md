# Starkli 101

In this tutorial, you will be guided from scratch to deploy contracts on Starknet.

1. Preparing your cryptographic keys and account.
2. How to compile your Cairo contract with a simple contract example.
3. Declaring a new contract class on Starknet.
4. Deploying an instance of a contract on Starknet.

## Prepare an account

To interact with the blockchain, you need an account to sign transactions.
This tutorial assumes that you have no wallet and no account setup.

You may want to check [cairolang-import](../subcommands/cairolang-import.md)
if you already have an account used with cairo-lang previously.

### First, you need a pair of cryptographic keys.

```bash
starkli signer keystore new ./key_1
```

For more details about wallet and keys, please refere to [signer section](../subcommands/signer.md).

Then, this key can be used to initialize an account. Take a moment to note the address
of your account upon deploy, outputed by Starkli.

```bash
starkli account oz init --keystore ./key_1 account_1
```

### Once the account is initialized (still local for now), you must pre-fund this account.
To pre-fund, you have several options, but the easiest is to pick-up one of Starknet wallet (ArgentX or Braavos), and from
those wallets you can send funds to the address Starkli gave you at the previous command. If you want to have an estimate
of the gas fees to deploy your account, run the next step.

### To deploy the account, simply run:
```
starkli account deploy account_1 --keystore ./key_1
```

The prompt will be blocked to ensure that you press ENTER when your pre-fund transaction is validated.
Once you press ENTER, Starkli will show you the transaction hash related to your account deployment and will
track the progress of the transaction.

You are then all set, your cryptographic keys are generated and encrypted locally with your password.
And your account is deployed on-chain, meaning that you can now start sending transaction!

For more details about accounts, please refere to [account section](../subcommands/account.md).

## Compile your Cairo contract

The next step is to compile a Cairo contract.

To compile cairo contract you have several options:
1. Use the compiler directly (with docker recommended). Please refer to [Starkware](https://github.com/starkware-libs/cairo)
   documentation if you want to install cairo compiler.
2. Use [Scarb](https://docs.swmansion.com/scarb/docs), the cairo package manager.

A simple contract to compile:

```rust
// ** contract 1 **                                                                                                                                                

#[contract]
mod Contract1 {

    struct Storage {
        _name: felt252,
    }

    #[event]
    fn StarkliGreeting(name: felt252) {}

    #[constructor]
    fn constructor(name: felt252) {
        _name::write(name);
    }
                                                                                                                                                                   
    #[view]
    fn name_get() -> felt252 {
        let n = _name::read();
        StarkliGreeting(n);
        n
    }
}
```

Let's give an example with only docker required on your machine:

```
sudo docker run --rm -it -v $(pwd):/cairo --entrypoint starknet-compile starknet/cairo:1.1.0 /cairo/contract1.cairo /cairo/contract1.json --replace-ids
```

If you are not familiar with docker, you can check Scarb documentation, and run `scarb build`.

Once the contract is compiled into Sierra (the `.json` file), we need it's class hash.
It's important to note that the class hash depends on the contract's content. So if you are doing this
tutorial, we recommend that you change at least some lines of code to generate a new class hash.

```bash
starkli class-hash ./contract1.json
```

This will output the class hash of your compiled contract.

## Declare the new class

When you develop your own contract, you are generating a new class hash as mentioned earlier.
For this reason, you have to declare the new contract class in order to Starknet
to store the code associated with your contract.

If the class hash already exists, Starknet will refuse the declare transaction. Please refer
to the previous section if you encounter this error.

```bash
starkli declare --keystore key_1 --account account_1 contract1.json
```

Starkli will then output the Cairo 1 class hash (the one we can see with the previous `class-hash` command),
but also the transaction associated with the declare. You can use the transaction hash to monitor it's progress
or use the `--watch` option of starkli.

## Deploy your contract

Once your new class is declared, you have to deploy an instance of your contract.
Starknet is dividing the logic (contract class code you declared at the previous step)
and the state (storage variables) of a contract.

To initialize a new state associated with the declared code, you can use the deploy transaction.
To deploy, you will need the class hash for which you want a new instance. This class hash was seen
2 steps before, using the `class-hash` command of Starkli.

Finally, you will need to pass the constructor arguments to your contract. If you don't provide
constructor arguments but the contract is expecting some, you'll have a beautiful error printed to the screen.

If your arguments are felts, and you are using short strings, Starkli has a tool for that using `to-cairo-string` or `parse-cairo-string`.

In the case of `contract1`, let's use a string to initialize the constructor with a name we will see in the event.

```bash
starkli to-cairo-string starkli
0x737461726b6c69

starkli deploy --keystore key_1 --account account_1 --watch 0x07bb0bf839488d45745632c095b1bac9d1d04e16549bbf7301d2611e08fcd126 0x737461726b6c69
```

And once it's confirmed, you'll be able to interact with this contract using it's `view` called `name_get` on
[starkscan](https://testnet.starkscan.co/contract/0x05866804a5088134cf4110fe2bf8985e15443667e8d38733a5f4e48cf3277724#read-write-contract)
or
[voyager](https://goerli.voyager.online/contract/0x05866804a5088134cf4110fe2Bf8985E15443667E8d38733a5F4E48CF3277724#readContract).

