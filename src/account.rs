use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::{
    core::{
        serde::unsigned_field_element::UfeHex, types::FieldElement, utils::get_contract_address,
    },
    macros::felt,
};

pub const KNOWN_ACCOUNT_CLASSES: [KnownAccountClass; 2] = [
    KnownAccountClass {
        class_hash: felt!("0x048dd59fabc729a5db3afdf649ecaf388e931647ab2f53ca3c6183fa480aa292"),
        variant: AccountVariantType::OpenZeppelin,
        description: "OpenZeppelin account contract v0.6.1 compiled with cairo-lang v0.11.0.2",
    },
    KnownAccountClass {
        class_hash: felt!("0x03131fa018d520a037686ce3efddeab8f28895662f019ca3ca18a626650f7d1e"),
        variant: AccountVariantType::Braavos,
        description: "Braavos proxy compiled with v0.10.3",
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
    Braavos(OzAccountConfig),
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
    pub description: &'static str,
}

pub enum AccountVariantType {
    OpenZeppelin,
    Braavos,
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
            AccountVariant::OpenZeppelin(oz) | AccountVariant::Braavos(oz) => {
                Ok(get_contract_address(
                    undeployed_status.salt,
                    undeployed_status.class_hash,
                    &[oz.public_key],
                    FieldElement::ZERO,
                ))
            }
        }
    }
}

impl Display for AccountVariantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountVariantType::OpenZeppelin => write!(f, "OpenZeppelin"),
            AccountVariantType::Braavos => write!(f, "Braavos"),
        }
    }
}
