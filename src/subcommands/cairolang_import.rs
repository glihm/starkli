use anyhow::Result;
use clap::Parser;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde::Deserialize;

use starknet::{
    core::{
        chain_id::{MAINNET, TESTNET, TESTNET2},
        types::FieldElement,
    },
    providers::AnyProvider,
    providers::Provider,
    signers::SigningKey,
};

use crate::{account::AccountConfig, ProviderArgs};

#[derive(Debug, Deserialize)]
struct CairolangAccount {
    private_key: String,
    public_key: String,
    salt: String,
    address: String,
    deployed: bool,
}

type CairolangContent = HashMap<String, HashMap<String, CairolangAccount>>;

const CAIROLANG_ACCOUNT_FILE_DEFAULT: &str =
    "~/.starknet_accounts/starknet_open_zeppelin_accounts.json";

#[derive(Debug, Parser)]
pub struct CairolangImport {
    #[clap(flatten)]
    provider: ProviderArgs,
    #[clap(
        long,
        help = "Overwrite the keystore file and account config file if they already exist"
    )]
    force: bool,
    #[clap(long, required = true, help = "Path to save the keystore JSON file")]
    keystore_file: PathBuf,
    #[clap(long, required = true, help = "Path to save the account config file")]
    account_file: PathBuf,
    #[clap(long, help = "Path to cairo-lang config file")]
    cairolang_file: Option<PathBuf>,
    #[clap(help = "Name of the cairo-lang account to import")]
    cairolang_account: String,
}

impl CairolangImport {
    pub async fn run(self) -> Result<()> {
        // TODO: clone is not that good right? Maybe an heavy copy.
        // Do we care for this tmp impl?
        let provider = self.provider.clone().into_provider();
        let chain_id = provider.chain_id().await?;

        let network = if chain_id == MAINNET {
            "alpha-mainnet"
        } else if chain_id == TESTNET {
            "alpha-goerli"
        } else if chain_id == TESTNET2 {
            "alpha-goerli2"
        } else {
            anyhow::bail!("Unsupported network");
        };

        let account = self.account_from_file(network)?;

        self.keystore_save(&account)?;

        self.account_fetch(&account, &provider).await?;

        Ok(())
    }

    /// Extracts an account from Cairolang config file.
    fn account_from_file(
        &self,
        network_identifier: &str,
    ) -> Result<CairolangAccount, anyhow::Error> {
        let default_path =
            PathBuf::from(shellexpand::tilde(CAIROLANG_ACCOUNT_FILE_DEFAULT).to_string());

        let path = match &self.cairolang_file {
            Some(p) => p,
            None => &default_path,
        };

        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                anyhow::bail!(
                    "Cairo-lang account file not found or not accessible.
If you have used the default configuration of cairo-lang, please remove the
--cairolang-file argument, to automatically use the default file ({}).
",
                    CAIROLANG_ACCOUNT_FILE_DEFAULT
                )
            }
        };

        let mut json = String::new();

        let content: CairolangContent = match file.read_to_string(&mut json) {
            Ok(_) => match serde_json::from_str(&json) {
                Ok(c) => c,
                Err(_) => anyhow::bail!("Cairo-lang account file is not formatted as expected."),
            },
            Err(_) => anyhow::bail!("Cairo-lang account file couldn't be read."),
        };

        let account_name = &self.cairolang_account;

        if let Some(network) = content.get(network_identifier) {
            if let Some(account) = network.get(account_name) {
                Ok(CairolangAccount {
                    private_key: String::from(&account.private_key),
                    public_key: String::from(&account.public_key),
                    salt: String::from(&account.salt),
                    address: String::from(&account.address),
                    deployed: account.deployed,
                })
            } else {
                anyhow::bail!(
                    "No account '{}' found for network '{}'.",
                    account_name,
                    network_identifier
                )
            }
        } else {
            anyhow::bail!("No account found for network '{}'.", network_identifier)
        }
    }

    // I don't like to copy paste... I can refactor some code to be available
    // like src/account.rs and create src/keystore.rs for instance,
    // would that be ok?

    /// Saves the keystore from private key.
    fn keystore_save(&self, account: &CairolangAccount) -> Result<(), anyhow::Error> {
        if self.keystore_file.exists() && !self.force {
            anyhow::bail!("Keystore JSON file already exists.");
        }

        let private_key = FieldElement::from_hex_be(account.private_key.trim())?;
        let password = rpassword::prompt_password("Enter password: ")?;

        let key = SigningKey::from_secret_scalar(private_key);
        key.save_as_keystore(&self.keystore_file, &password)?;

        println!(
            "Created new encrypted keystore file: {}",
            std::fs::canonicalize(&self.keystore_file)?.display()
        );
        println!("Public key: {:#064x}", key.verifying_key().scalar());

        Ok(())
    }

    /// Fetches account config from an already deployed account contract.
    async fn account_fetch(
        &self,
        account: &CairolangAccount,
        provider: &AnyProvider,
    ) -> Result<(), anyhow::Error> {
        if self.account_file.exists() && !self.force {
            anyhow::bail!("Account config file already exists.");
        }

        let account_config = AccountConfig::fetch(&account.address, provider).await?;

        let mut file = std::fs::File::create(&self.account_file)?;
        serde_json::to_writer_pretty(&mut file, &account_config)?;
        file.write_all(b"\n")?;

        eprintln!(
            "Downloaded new account config file: {}",
            std::fs::canonicalize(&self.account_file)?.display()
        );

        Ok(())
    }
}
