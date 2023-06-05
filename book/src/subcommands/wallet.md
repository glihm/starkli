# Wallet

A wallet is a software managing cryptographic keys associated to a blockchain
user. A user has to sign transactions in order to interact with the blockchain.

Starkli proposes two ways to interact with the key pair:

1. A managed and secure keystore based on [web3 secret storage definition](
https://ethereum.org/en/developers/docs/data-structures-and-encoding/web3-secret-storage/)
where the private key is strongly encrypted locally with a passphrase of your
choice.

2. A raw generation of a key pair printed in the screen at generation.

## keystore
Keystore management related commands. The keystore is a safe way to
store your keys, and interact with them.

You have to choose and remember a passphrase (or password) to protect your keys.
Please attempt to choose a strong password.

Be aware that web3 secret storage definition is based on JSON files.
You must carefully store and identify those files, to avoid accidental
removal or loss.

It's a good practice and safe to backup keystore files in several locations,
as the private key inside the keystore file is encrypted.
    
### new
Randomly generate a new keystore.

`starkli wallet keystore new ~/.keystore/key_1`

In the example above, strakli will create `key_1` **if it doesn't exist**.
However, starkli **will not** create the `~/.keystore` directory automatically.

If the keystore file `~/.keystore/key_1` already exists, starkli returns an
error `keystore file already exists`. To overcome this error, you can choose to
add the `--force` option.  
**Be careful** to not overwrite a keystore file with important keys.

You are then prompted to enter a passphrase (or password), which is used
to derive a symmetric cryptographic key to strongly encrypt your newly
generated private key.

```bash
Enter password:
Created new encrypted keystore file: ~/.keystore/key_1
Public key: 0x048cb21c13975711aae42cb20dfba9c20e5ff70f0429b55d35c79e2f8e8d5cf9
```

### from-key
Create a keystore file from an existing private key.

If you already have a private key, you can use this method to import
your private key. It's important to note that the public key can be
computed from the private key. So, importing the private key will automatically
add the public key.

`starkli wallet keystore from-key ~/.keystore/imported_key_1`

You will then be prompted to enter the private key and a new passphrase.
Note that the passphrase can be totally different from the passphrase
you have used for the same private key, in other wallets.

```bash
Enter private key: 
Enter password: 
Created new encrypted keystore file: ~/.keystore/key_1
Public key: 0x048cb21c13975711aae42cb20dfba9c20e5ff70f0429b55d35c79e2f8e8d5cf9
```

### inspect
Check the public key of an existing keystore file.

Prints to the screen the public key in the given keystore file.

`starkli wallet keystore inspect ~/.keystore/key_1`

You will be prompted to enter your passphrase, and the public key will be
displayed.

```bash
Enter password: 
Public key: 0x048cb21c13975711aae42cb20dfba9c20e5ff70f0429b55d35c79e2f8e8d5cf9
```

### inspect-private
Check the private key of an existing keystore file.

`starkli wallet keystore inspect-private ~/.keystore/key_1`

As mentioned earlier, the private key is encrypted. Without your passphrase
the wallet software can't decrypt your private key.

```bash
Enter password: 
Private key: 0x0546e02959e78748b0619ee1b19ccc5ae9592b12986037038e23d9f49ced1094
```

## gen-keypair

Randomly generate a new key pair.

With this raw approach, the key pair is displayed on the screen.

```bash
starkli wallet gen-keypair

Private key : 0x07d76232672f820687bc3b5524120f60bd58bced07bb4265273e0e05f2e0c464
Public key  : 0x0051327fb5c1e0cb8fb25cb1f17f53012c9e985ae10fa08ecc5e1ca034eeda4f
```
