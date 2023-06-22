use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::{
    core::{
        serde::unsigned_field_element::UfeHex,
        types::{BlockId, BlockTag, FieldElement, FunctionCall},
        utils::get_contract_address,
    },
    macros::{felt, selector},
    providers::{AnyProvider, Provider},
};

pub const KNOWN_ACCOUNT_CLASSES: [KnownAccountClass; 2] = [KnownAccountClass {
    class_hash: felt!("0x048dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292"),
    variant: AccountVariantType::OpenZeppelin,
},

KnownAccountClass {
    class_hash: felt!("0x006ea5324f5d3f919a7ff007acfad6c421d724cf0cbcf0f6105945565518a572"),
    variant: AccountVariantType::OpenZeppelin,
},

];

#[derive(Serialize, Deserialize)]
pub struct AccountConfig {
    pub version: u64,
    pub variant: AccountVariant,
    pub deployment: DeploymentStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AccountVariant {
    OpenZeppelin(OzAccountConfig),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DeploymentStatus {
    Undeployed(UndeployedStatus),
    Deployed(DeployedStatus),
}

pub struct KnownAccountClass {
    pub class_hash: FieldElement,
    pub variant: AccountVariantType,
}

pub enum AccountVariantType {
    OpenZeppelin,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct OzAccountConfig {
    pub version: u64,
    #[serde_as(as = "UfeHex")]
    pub public_key: FieldElement,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct UndeployedStatus {
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub salt: FieldElement,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct DeployedStatus {
    #[serde_as(as = "UfeHex")]
    pub class_hash: FieldElement,
    #[serde_as(as = "UfeHex")]
    pub address: FieldElement,
}

impl AccountConfig {
    pub fn deploy_account_address(&self) -> Result<FieldElement> {
        let undeployed_status = match &self.deployment {
            DeploymentStatus::Undeployed(value) => value,
            DeploymentStatus::Deployed(_) => {
                anyhow::bail!("account already deployed");
            }
        };

        match &self.variant {
            AccountVariant::OpenZeppelin(oz) => Ok(get_contract_address(
                undeployed_status.salt,
                undeployed_status.class_hash,
                &[oz.public_key],
                FieldElement::ZERO,
            )),
        }
    }

    /// Fetches account config from an already deployed account contract.
    pub async fn fetch(address: &str, provider: &AnyProvider) -> Result<Self, anyhow::Error> {
        let address = FieldElement::from_hex_be(address)?;

        let class_hash = provider
            .get_class_hash_at(BlockId::Tag(BlockTag::Pending), address)
            .await?;

        let known_class = match KNOWN_ACCOUNT_CLASSES
            .iter()
            .find(|class| class.class_hash == class_hash)
        {
            Some(class) => class,
            None => {
                eprintln!(
                    "{} is not a known account class hash. \
                     If you believe this is a bug, submit a PR to:",
                    format!("{:#064x}", class_hash).bright_yellow()
                );
                eprintln!("    https://github.com/xJonathanLEI/starkli");
                anyhow::bail!("unknown class hash: {:#064x}", class_hash);
            }
        };

        match known_class.variant {
            AccountVariantType::OpenZeppelin => {
                let public_key = provider
                    .call(
                        FunctionCall {
                            contract_address: address,
                            entry_point_selector: selector!("getPublicKey"),
                            calldata: vec![],
                        },
                        BlockId::Tag(BlockTag::Pending),
                    )
                    .await?[0];

                Ok(AccountConfig {
                    version: 1,
                    variant: AccountVariant::OpenZeppelin(OzAccountConfig {
                        version: 1,
                        public_key,
                    }),
                    deployment: DeploymentStatus::Deployed(DeployedStatus {
                        class_hash,
                        address,
                    }),
                })
            }
        }
    }
}
