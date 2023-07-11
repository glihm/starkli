# Cairolang import

**IMPORTANT: this command is not merged yet**

Imports a private key and an associated account contract from
`cairo-lang` configuration file.

When you use `cairo-lang` to generate a new account, the cryptographic
keys are written in the same file as the configuration of your auto-generated
OpenZeppelin account contract.

In Starkli, wallet and account are two separated entities.
For this reason, you have to explicitely provide two files were the keys
will be stored, and where the account configuration will be stored.
Please refer to the [accounts and wallet chapter](../accounts-wallets.md)
for more details.

`cairolang-import` command is provided in order to seamlessly migrate your
existing wallets and contracts to Starkli.

As in `cairo-lang`, ensure your [network environment](../getting-started.md#interacting-with-starknet) variable is correctly set.

```bash
# You have run this command with cairo-lang
starknet new_account --account account_1

# You can then run this with Starkli
$ starkli cairolang-import \
          --keystore-file ~/.keystore/key_1.json \
          --account-file ~/.account/account_1.json \
          account_1
```

If you did not use the default path to store you account configuration when working
with `cairo-lang`, you can use the `cairolang-file` argument to provide the path
of your file.

```bash
# You have run this command with cairo-lang
starknet new_account --account account_2 --account_dir /my/path

# You can then run this with Starkli
$ starkli cairolang-import \
          --keystore-file ~/.keystore/key_2.json \
          --account-file ~/.account/account_2.json \
          --cairolang-file /my/path/starknet_open_zeppelin_accounts.json \
          account_2
```

Finally, if you want to overwrite the keystore and account config file if they already exist,
you can use the `--force` option. **Please be careful to not overwrite important files**.

When you run the command, you will be prompted to enter a password,
used to encrypt your private key.

```
Enter password:
Created new encrypted keystore file: ~/.keystore/key_1
Public key: 0x077efc5a30bdfe579af083c2f3297c52a0b675278431fea59b9a40421793809a
```

Once you see the public key, you can be sure that your keys are correctly
imported and safely encrypted locally.

After this, Starkli will attempt to fetch your contract configuration
from the blockchain. If for any reason this process fails, or if your
account is not deployed, the account configuration will not be written.
But your keys are already imported successfully.

You will be able to interact with the blockchain and your account with Starkli,
please see [account](account.md) related command.
