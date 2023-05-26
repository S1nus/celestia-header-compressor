use std::{fs, ops::Index};
use std::str::FromStr;
use tendermint::Signature;
use tendermint_proto::v0_37::{
    types::{
        BlockId as RawBlockId, 
        Header as RawHeader,
        Commit as RawCommit,
        Vote as RawVote,
        CanonicalVote as RawCanonicalVote,
        ValidatorSet as RawValidatorSet,
        PartSetHeader,
        CanonicalPartSetHeader,
        CanonicalBlockId,
        BlockId,
        SignedMsgType,
    },
    version::Consensus as RawConsensusVersion,
};
use serde::{Deserialize, Serialize};
use prost::Message;
use ed25519_dalek::{PublicKey, Signature};

#[derive(Deserialize, Serialize)]
struct DAH {
    row_roots: Vec<String>,
    column_roots: Vec<String>,
}
#[derive(Deserialize, Serialize)]
struct CelestiaHeader {
    header: RawHeader,
    commit: RawCommit,
    validator_set: RawValidatorSet,
    dah: DAH,
}

#[derive(Deserialize)]
struct Response {
    jsonrpc: String,
    result: CelestiaHeader,
    id: u32,
}

fn get_vote(head: &CelestiaHeader, index: usize) -> RawVote {
    RawVote {
        r#type: SignedMsgType::Precommit.into(),
        height: head.commit.height,
        round: head.commit.round,
        block_id: head.commit.block_id.clone(),
        timestamp: head.commit.signatures[index].timestamp.clone(),
        validator_address: head.commit.signatures[index].validator_address.clone(),
        validator_index: i32::try_from(index).unwrap(),
        signature: head.commit.signatures[index].signature.clone(),
    }
}

fn get_canonical_vote(head: &CelestiaHeader, index: usize, chain_id: String) -> RawCanonicalVote {
    RawCanonicalVote {
        r#type: SignedMsgType::Precommit.into(),
        height: head.commit.height,
        round: head.commit.round.into(),
        block_id: match &head.commit.block_id {
            Some(b) => Some(canonize_block_id(&b)),
            None => None,
        },
        timestamp: head.commit.signatures[index].timestamp.clone(),
        chain_id: chain_id,
    }
}

fn canonize_block_id(b: &BlockId) -> CanonicalBlockId {
    let p = match &b.part_set_header {
        Some(p) => Some(CanonicalPartSetHeader {
            total: p.total,
            hash: p.hash.clone(),
        }),
        None => None,
    };
    CanonicalBlockId {
        hash: b.hash.clone(),
        part_set_header: p,
    }
}

fn main() {

    let file = fs::read("header.json")
        .unwrap();
    let resp: Response = serde_json::from_slice(file.as_slice()).unwrap();
    let head = resp.result;
    let chain_id = String::from("celestia");
    let v0 = get_canonical_vote(&head, 0, chain_id);
    let buf = v0.encode_to_vec();
    let sig = head.commit.signatures[0];
    println!("{:02X?}", buf);
}