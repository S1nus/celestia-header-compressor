use std::fs;
use tendermint::block::signed_header::SignedHeader;
use tendermint::block::{Header, Commit};
use tendermint::validator::Set as ValidatorSet;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct HeaderResponse {
    jsonrpc: String,
    result: CelestiaHeader,
    id: usize,
}

#[derive(Serialize, Deserialize)]
struct DAH {
    row_roots: Vec<Vec<u8>>,
    column_roots: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
struct CelestiaHeader {
    header: Header,
    commit: Commit,
    validators: ValidatorSet,
    dah: DAH,
}

fn main() {
    let data = fs::read("header.json")
        .unwrap();
    let resp: HeaderResponse = serde_json::from_slice(data.as_slice()).unwrap();
    println!("rpc {:?}", resp.jsonrpc);
}