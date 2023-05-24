use std::{fs, ops::Index};
use std::str::FromStr;
use tendermint_proto::v0_37::{
    types::{
        BlockId as RawBlockId, 
        Header as RawHeader,
        Commit as RawCommit,
        Vote as RawVote,
        ValidatorSet as RawValidatorSet,
        SignedMsgType,
    },
    version::Consensus as RawConsensusVersion,
};
use serde::{Deserialize, Serialize};
use prost::Message;

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

fn main() {

    let file = fs::read("header.json")
        .unwrap();
    let resp: Response = serde_json::from_slice(file.as_slice()).unwrap();
    let head = resp.result;
    let v0 = get_vote(&head, 0);
    let buf = v0.encode_to_vec();
    println!("{:02X?}", buf);
}