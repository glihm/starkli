# Starkli 101

In this tutorial, you will be guided from scratch to deploy contracts on Starknet.

1. Preparing your cryptographic keys and account.
2. How to compile your Cairo contract with a simple contract example.
3. Declaring a new contract class on Starknet.
4. Deploying an instance of a contract on Starknet.

## Prepare an account and a signer

To interact with the blockchain, you need an account to sign transactions.
This tutorial assumes that you have no wallet and no account setup.

### Initialize a signer

The signer is in charge of signing the transactions. You can create a new pair of
cryptographic key, or importing an existing private key. More details in the
[signer section](../subcommands/signer.md).

Let's create here a new signer.

```bash
$ starkli signer keystore new /path/to/key_1.json
```

This will prompt you to enter a password and save your encrypted private key
into the `key_1.json` file.

Keystore is an other word to refer to the way your signer (keys) are stored.

To then use this account at each command, you can export the environment variable such as:

```bash
$ export STARKNET_KEYSTORE=/path/to/key_1.json
```

We then initialize a new account, using OpenZeppelin class already declared on Starknet:

```bash
$ starkli account oz init /path/to/account_1.json
```

Note that we didn't need to pass the `--keystore` option if the `STARKNET_KEYSTORE` is already set.
**Take note of your account address, we need then to pre-fund it.**

Again, to avoid passing the account to each command of starkli, we can export
the `STARKNET_ACCOUNT` variable.

### Once the account is initialized (still local for now), you must pre-fund this account.

To pre-fund, you have several options, but the easiest is to pick-up one of Starknet wallet (ArgentX or Braavos), and from
those wallets you can send funds to the address Starkli gave you at the previous command.

Once some ETHs are sent to your account, proceed to the next step.

### To deploy the account, simply run:

In this example, we pass explicitely the path to the `account_1.json`.
You can rely on `STARKNET_ACCOUNT` environment variable, but for the deploy,
it makes sense to explicitely defines which account we want to deploy.

```
$ starkli account deploy --account /path/to/account_1.json
```

The prompt will be blocked to ensure that you press ENTER when your pre-fund transaction is validated.
Once you press ENTER, Starkli will show you the transaction hash related to your account deployment and will
track the progress of the transaction.

You are then all set, your cryptographic keys are generated and encrypted locally with your password.
And your account is deployed on-chain, meaning that you can now start sending transaction!

For more details about accounts, please refere to [account section](../subcommands/account.md).

From here, remember that you can define environment variables to avoid passing explicitely your account and your
keystore file at each command. If you have a node, you can also define it here. If not, starkli
will fallback on the gateway for now.

```bash
$ export STARKNET_ACCOUNT=/path/to/account_1.json
$ export STARKNET_KEYSTORE=/path/to/key_1.json
$ export STARKNET_RPC=https://my.node.starknet
```

From here, Starkli will automatically load those files.
If you need to override this behavior for a specific command,
you can still use the `--keystore` and `--account` arguments.

## Compile your Cairo contract

The next step is to compile a Cairo contract.

To compile cairo contract you have several options:

1. Use the compiler directly (with docker recommended). Please refer to [Starkware](https://github.com/starkware-libs/cairo)
   documentation if you want to install cairo compiler.
2. Use [Scarb](https://docs.swmansion.com/scarb/docs), the cairo package manager, which is easy to configure.

A simple contract to compile:

```rust
// ** contract1.cairo **

#[starknet::interface]
trait MyContractInterface<T> {
    fn name_get(self: @T) -> felt252;
    fn name_set(ref self: T, name: felt252);
}

#[starknet::contract]
mod contract1 {

    #[storage]
    struct Storage {
        name: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        NameChanged: NameChanged,
    }

    #[derive(Drop, starknet::Event)]
    struct NameChanged {
        previous: felt252,
        current: felt252,
    }

    #[constructor]
    fn constructor(ref self: ContractState, name: felt252) {
        self.name.write(name);
    }

    #[external(v0)]
    impl Contract1 of super::MyContractInterface<ContractState> {
        ///
        fn name_get(self: @ContractState) -> felt252 {
            self.name.read()
        }

        ///
        fn name_set(ref self: ContractState, name: felt252) {
            let previous = self.name.read();
            self.name.write(name);
            self.emit(NameChanged{ previous, current: name });
        }
    }
}
```

To compile the contract, you can use [scarb](https://docs.swmansion.com/scarb).

Ensure that into the `Scarb.toml` you have a dependency matching your scarb version.
For instance `starknet="2.0.1"` for a scarb version like:
```
scarb --version

scarb 0.5.1 (798acce7f 2023-07-05)
cairo: 2.0.1 (https://crates.io/crates/cairo-lang-compiler/2.0.1)
```

## Declare the new class

When you develop your own contract, you are generating a new class hash as mentioned earlier.
For this reason, you have to declare the new contract class in order to Starknet
to store the code associated with your contract.

**If the class hash already exists, Starknet will refuse the declare transaction. Please modify
the code of the contract if you want to follow. Add a function, add an event (event if not used),
add a variable, etc...**

```bash
$ starkli declare --watch ./target/dev/contract1.json
```

Starkli will then output the Cairo 1 class hash (which can also be retrived using `starkli class-hash <FILE.json>` command),
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
To work with short strings, starkli proposes some `schemes`, which will handle the serialization
of arguments for you: `str:starkli`.

More about schemes [here](../schemes.md).

This will automatically generate the underlying felt you can obtain using [to-cairo-string](../subcommands/to-cairo-string.md).

```bash
$ starkli class-hash ./target/dev/contract1.json
0x068f2d2fce1d69bd9469d9364d0e64225ea58e6e209737f8473eb33c941dabb1

$ starkli deploy --watch 0x068f2d2fce1d69bd9469d9364d0e64225ea58e6e209737f8473eb33c941dabb1 str:starkli
```

In the output of this command, starkli will give you the transaction hash and also the address where the
contract will be deployed.

Let's then interact with the contract with `call` and `invoke`.

```bash
$ starkli call 0x06734d6f8209f96c6bafec231a738b967926b00095f29dec141e64620c5ff929 name_get
[
    "0x00000000000000000000000000000000000000000000000000737461726b6c69"
]


$ starkli parse-cairo-string 0x00000000000000000000000000000000000000000000000000737461726b6c69
starkli

$ starkli invoke --watch 0x06734d6f8209f96c6bafec231a738b967926b00095f29dec141e64620c5ff929 name_set str:starknet
```

If you prefer the explorer mode, you can use
[voyager](https://goerli.voyager.online/contract/0x06734d6f8209f96c6bafec231a738b967926b00095f29dec141e64620c5ff929#readContract) or
[starkscan](https://testnet.starkscan.co/contract/0x06734d6f8209f96c6bafec231a738b967926b00095f29dec141e64620c5ff929#read-write-contract-sub-read).
