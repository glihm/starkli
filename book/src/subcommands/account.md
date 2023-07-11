# account subcommand

Account management commands.

## fetch

Fetch account config from an already deployed account contract.

Starkli is still under construction, and some automatic fetches are still now available (Braavos/ArgentX mostly).

In the meantime, you can create your own account configuration file with the following:

```json
{
  "version": 1,
  "variant": {
    "type": "open_zeppelin",
    "version": 1,
    "public_key": "<PUBLIC_KEY>"
  },
  "deployment": {
    "status": "deployed",
    "class_hash": "<PROVIDER_CLASS_HASH>",
    "address": "<ACCOUNT_ADDRESS>"
  }
}
```

You can have your `ACCOUNT_ADDRESS` directly in your wallet, it's the `0x...` displayed at the top of it.

Your `PUBLIC_KEY` can be seen when your export your private key, or directly interacting with your contract with
`get_signers` or `get_public_key`. Or an easiest way to get it, just use the [signer from-key](./signer.md#from-key) command
to import your private key. The public key will be printed.

The class hash depends on your providers, here are some of them:

```
Braavos: 0x03131fa018d520a037686ce3efddeab8f28895662f019ca3ca18a626650f7d1e
ArgentX: 0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918
```

Once it's done, save this file for instance in `~/.accounts/my_account.json`.
And if the account is already deployed, you can start using starkli commands.

Do not forget to import the private key corresponding to this account using the [signer from-key](./signer.md#from-key) command.

Do not forget to export the environment variable to avoid passing `--account` to each command:

```bash
export STARKNET_ACCOUNT=~/.accounts/my_account.json
```

## OpenZeppelin (oz)

Specific command related to OpenZeppelin standard account.

### init

Initializes an account configuration file with default OpenZeppelin class hash.

```bash
$ starkli account oz init /path/to/account.json
```

Then you can use the `account deploy` command to deploy it.

