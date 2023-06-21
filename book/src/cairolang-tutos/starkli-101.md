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
$ starkli signer keystore new ./key_1
```

For more details about wallet and keys, please refere to [signer section](../subcommands/signer.md).

Then, this key can be used to initialize an account. Take a moment to note the address
of your account upon deploy, outputed by Starkli.

```bash
$ starkli account oz init --keystore ./key_1 account_1
```

### Once the account is initialized (still local for now), you must pre-fund this account.
To pre-fund, you have several options, but the easiest is to pick-up one of Starknet wallet (ArgentX or Braavos), and from
those wallets you can send funds to the address Starkli gave you at the previous command. If you want to have an estimate
of the gas fees to deploy your account, run the next step.

### To deploy the account, simply run:
```
$ starkli account deploy account_1 --keystore ./key_1
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
// ** contract1.cairo **

#[contract]
mod Contract1 {

    struct Storage {
        _name: felt252,
    }

    #[event]
    fn NameGreeted(name: felt252) {}

    #[event]
    fn NameChanged(previous: felt252, current: felt252) {}

    #[constructor]
    fn constructor(name: felt252) {
        _name::write(name);
    }

    #[view]
    fn name_get() -> felt252 {
        let n = _name::read();
        NameGreeted(n);
        n
    }

    #[external]
    fn name_set(name: felt252) {
        let previous = _name::read();
        _name::write(name);
        NameChanged(previous, name);
    }

}
```

Let's give an example with only docker required on your machine:

```
$ sudo docker run --rm -it -v $(pwd):/cairo --entrypoint starknet-compile starknet/cairo:1.1.0 /cairo/contract1.cairo /cairo/contract1.json --replace-ids
```

If you are not familiar with docker, you can check Scarb documentation, and run `scarb build`.

Once the contract is compiled into Sierra (the `.json` file), we need it's class hash.
It's important to note that the class hash depends on the contract's content. So if you are doing this
tutorial, we recommend that you change at least some lines of code to generate a new class hash.

```bash
$ starkli class-hash ./contract1.json

0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea
```

This will output the class hash of your compiled contract.

## Declare the new class

When you develop your own contract, you are generating a new class hash as mentioned earlier.
For this reason, you have to declare the new contract class in order to Starknet
to store the code associated with your contract.

**If the class hash already exists, Starknet will refuse the declare transaction. Please modify
the code of the contract if you want to follow. Add a function, add an event (event if not used),
add a variable, etc...**

```bash
$ starkli declare --watch --keystore key_1 --account account_1 contract1.json

Declaring Cairo 1 class: 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea
Compiling Sierra class to CASM with compiler version v1.1.0...
CASM class hash: 0x0339b10696de96d2b921f3f19c1af4bc95750ed6cb6b058f938fe73556879d95
Contract declaration transaction: 0x072cd111c4a57886dc168c16e2643eac58798bbf23324575bf03725d2d84f94d
Waiting for transaction 0x072cd111c4a57886dc168c16e2643eac58798bbf23324575bf03725d2d84f94d to confirm...
Transaction not confirmed yet...
Transaction 0x072cd111c4a57886dc168c16e2643eac58798bbf23324575bf03725d2d84f94d confirmed
Class hash declared:
0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea
```

Starkli will then output the Cairo 1 class hash (the one we can see with the previous `class-hash` command),
but also the transaction associated with the declare. You can use still use the `--watch` option from Starkli,
or monitor your transaction on an explorer with the `transaction hash`.

## Deploy your contract

Once your new class is declared, you have to deploy an instance of your contract.
Starknet is dividing the logic (contract class code you declared at the previous step)
and the state (storage variables) of a contract.

To initialize a new state associated with the declared code, you can use the deploy transaction.
To deploy, you will need the class hash for which you want a new instance of. This class hash was seen
at the step above, using the `class-hash` command of Starkli or the output from `declare`.

Finally, you will need to pass the constructor arguments to your contract. If you don't provide
constructor arguments but the contract is expecting some, you'll have a beautiful error printed to the screen.

If your arguments are felts, and you are using short strings, Starkli has a tool for that using [to-cairo-string](../subcommands/to-cairo-string.md).

In the case of `contract1`, let's use a string to initialize the constructor with a name: `starkli`.

```bash
$ starkli to-cairo-string starkli

0x737461726b6c69

$ starkli deploy --watch --keystore key_1 --account account_1 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea 0x737461726b6c69

Deploying class 0x0392d83f853eb1b6f57aa7de4e9dc8ffc660239ff2ecb1fb8a9749ef0d36a2ea with salt 0x023815f8c5dba41d29bf6023568de206233e099bdd10fbac33f43eae89d4ec94...
Contract deployment transaction: 0x05bdc93ef3e8050ca61c31039683906db67d67e3fdd41315f414a52dc16a3ab8
```

And once the transaction is confirmed, check the deployment transaction on the explorer to access the deployed address of your contract.

You'll aslo be able to interact with this contract using it's `view` called `name_get` directly with Starkli:

```bash
$ starkli call 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 name_get

[
    "0x00000000000000000000000000000000000000000000000000737461726b6c69"
]


$ starkli parse-cairo-string 0x00000000000000000000000000000000000000000000000000737461726b6c69

starkli
```

If you want to execute a transaction to call an external, you can achieve it using the `invoke` command:

```bash
$ starkli to-cairo-string starknet

0x737461726b6e6574

$ starkli invoke --watch --keystore key_1 --account account_1 0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478 name_set 0x737461726b6e6574

Invoke transaction: 0x06a9f49148992175694e5bb5a34a352d775059117fcf987d4478f7d0f729860c
```

If you prefer the explorer mode, you can use
[starkscan](https://testnet.starkscan.co/contract/0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478#read-write-contract-sub-read)
or
[voyager](https://goerli.voyager.online/contract/0x0259ae94e14641568687da0a42611f648ce16b9a08159488561d6a66250c0478#readContract).
