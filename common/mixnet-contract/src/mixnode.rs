// due to code generated by JsonSchema
#![allow(clippy::field_reassign_with_default)]

use crate::{IdentityKey, SphinxKey};

use cosmwasm_std::{coin, Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNode {
    pub host: String,
    pub mix_port: u16,
    pub verloc_port: u16,
    pub http_api_port: u16,
    pub sphinx_key: SphinxKey,
    /// Base58 encoded ed25519 EdDSA public key.
    pub identity_key: IdentityKey,
    pub version: String,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub enum Layer {
    Gateway,
    One,
    Two,
    Three,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixNodeBond {
    pub bond_amount: Coin,
    pub total_delegation: Coin,
    pub owner: Addr,
    pub layer: Layer,
    pub mix_node: MixNode,
}

impl MixNodeBond {
    pub fn new(bond_amount: Coin, owner: Addr, layer: Layer, mix_node: MixNode) -> Self {
        MixNodeBond {
            total_delegation: coin(0, &bond_amount.denom),
            bond_amount,
            owner,
            layer,
            mix_node,
        }
    }

    pub fn identity(&self) -> &String {
        &self.mix_node.identity_key
    }

    pub fn bond_amount(&self) -> Coin {
        self.bond_amount.clone()
    }

    pub fn owner(&self) -> &Addr {
        &self.owner
    }

    pub fn mix_node(&self) -> &MixNode {
        &self.mix_node
    }
}

impl Display for MixNodeBond {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "amount: {} {}, owner: {}, identity: {}",
            self.bond_amount.amount, self.bond_amount.denom, self.owner, self.mix_node.identity_key
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct PagedMixnodeResponse {
    pub nodes: Vec<MixNodeBond>,
    pub per_page: usize,
    pub start_next_after: Option<IdentityKey>,
}

impl PagedMixnodeResponse {
    pub fn new(
        nodes: Vec<MixNodeBond>,
        per_page: usize,
        start_next_after: Option<IdentityKey>,
    ) -> Self {
        PagedMixnodeResponse {
            nodes,
            per_page,
            start_next_after,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct MixOwnershipResponse {
    pub address: Addr,
    pub has_node: bool,
}
