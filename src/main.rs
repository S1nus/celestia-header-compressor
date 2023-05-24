use std::fs;
use std::str::FromStr;
use tendermint::block::signed_header::SignedHeader;
use tendermint::block::{Commit, Id};
use tendermint::account::{Id as AccountId};
use tendermint::validator::Set as ValidatorSet;
use tendermint::{Time};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as,DisplayFromStr};

#[derive(Deserialize)]
struct HeaderResponse {
    jsonrpc: String,
    result: CelestiaHeader,
    id: usize,
}

/// AppHash is usually a SHA256 hash, but in reality it can be any kind of data
#[derive(Clone, PartialEq, Eq, Default, Debug, Hash, Deserialize)]
pub struct AppHash(Vec<u8>);

#[derive(Serialize, Deserialize)]
struct DAH {
    row_roots: Vec<Vec<u8>>,
    column_roots: Vec<Vec<u8>>,
}

#[derive(Deserialize)]
struct CelestiaHeader {
    header: Header,
    commit: Commit,
    validators: ValidatorSet,
    dah: DAH,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Version {
    /// Block version
    #[serde_as(as = "DisplayFromStr")]
    pub block: u64,

    /// App version
    #[serde_as(as = "DisplayFromStr")]
    pub app: u64,
}

pub const SHA256_HASH_SIZE: usize = 32;

/// Hash digests
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Deserialize)]
pub enum Hash {
    /// SHA-256 hashes
    Sha256([u8; SHA256_HASH_SIZE]),
    /// Empty hash
    #[default]
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Header {
    /// Header version
    pub version: Version,

    /// Chain ID
    pub chain_id: String,

    /// Current block height
    pub height: u64,

    /// Current timestamp
    pub time: Time,

    /// Previous block info
    pub last_block_id: Option<Id>,

    /// Commit from validators from the last block
    pub last_commit_hash: Option<Hash>,

    /// Merkle root of transaction hashes
    pub data_hash: Option<Hash>,

    /// Validators for the current block
    pub validators_hash: Hash,

    /// Validators for the next block
    pub next_validators_hash: Hash,

    /// Consensus params for the current block
    pub consensus_hash: Hash,

    /// State after txs from the previous block
    pub app_hash: AppHash,

    /// Root hash of all results from the txs from the previous block
    pub last_results_hash: Option<Hash>,

    /// Hash of evidence included in the block
    pub evidence_hash: Option<Hash>,

    /// Original proposer of the block
    pub proposer_address: AccountId,
}

fn main() {
    let data = fs::read("header.json")
        .unwrap();
    let resp: HeaderResponse = serde_json::from_slice(data.as_slice()).unwrap();
    println!("rpc {:?}", resp.jsonrpc);
}