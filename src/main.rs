use std::{fs, ops::Index};
use std::str::FromStr;
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
use tendermint::{
    crypto::default::signature::Verifier as SigVerifier,
    crypto::signature::Verifier,
    Signature,
    PublicKey,
};
use tendermint_proto::crypto::PublicKey as RawPubkey;
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
        //r#type: SignedMsgType::Precommit.into(),
        r#type: SignedMsgType::Proposal.into(),
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
    let chain_id = head.header.chain_id.clone();
    let v0 = get_canonical_vote(&head, 0, chain_id);
    let bid = v0.block_id.unwrap();
    let part_set_header = bid.part_set_header.unwrap();
    let part_set_header_bytes = part_set_header.encode_length_delimited_to_vec();
    let hash = part_set_header.hash;
    let total = part_set_header.total;
    println!("PARTSET HEADER BYTES\n{:?}", part_set_header_bytes);
    println!("HASH\n{:?}", hash);
    println!("TOTAL\n{:?}", total);
}

fn verify_sigs() {

    let file = fs::read("header.json")
        .unwrap();
    let resp: Response = serde_json::from_slice(file.as_slice()).unwrap();
    let head = resp.result;
    let chain_id = head.header.chain_id.clone();
    let v0 = get_canonical_vote(&head, 0, chain_id);
    let mut buf = v0.encode_length_delimited_to_vec();
    let sig = Signature::try_from(head.commit.signatures[0].signature.clone())
        .unwrap();
    let pubkey = head.validator_set.validators[0].clone().pub_key.unwrap();
    let sum = pubkey.sum.unwrap();
    let mut pkbytes = match sum {
        tendermint_proto::crypto::public_key::Sum::Ed25519(e) => e,
        tendermint_proto::crypto::public_key::Sum::Secp256k1(_) => {
            panic!("Not supported");
        }
    };
    let pk = PublicKey::from_raw_ed25519(pkbytes.as_slice()).unwrap();
    let v = SigVerifier::verify(pk, buf.as_slice(), &sig);
    match v {
        Ok(_) => {println!("Verification SUCCESS!");},
        Err(_) => {println!("Verification FAILED");}
    }
}