use std::fs;
use std::str::FromStr;
use tendermint_proto::v0_37::{
    types::{
        BlockId as RawBlockId, 
        Header as RawHeader,
        Commit as RawCommit,
        ValidatorSet as RawValidatorSet,
    },
    version::Consensus as RawConsensusVersion,
};
use serde::{Deserialize, Serialize};

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

fn main() {
    let data = r#"
{
            "version": {
                "block": "11",
                "app": "1"
            },
            "chain_id": "blockspacerace-0",
            "height": "517435",
            "time": "2023-05-18T16:52:21.114866275Z",
            "last_block_id": {
                "hash": "0F8465E11DDE3F1A794117E1C1BC521F80B1EBA694B83C1E90777CA36A66C1A0",
                "parts": {
                    "total": 1,
                    "hash": "8087F8624305DD88B00FCD6F19AAC7E319FA5AF03919CEF4E57BC58563499E25"
                }
            },
            "last_commit_hash": "ABAACE23204A783B197A8628C646DE3ABA105FCDB4C0B65182331FC64307342B",
            "data_hash": "A407A4ACF12B54AF0E96400CCD698E524F4E3DC3D4F6E1249813D9F78A0EF0FD",
            "validators_hash": "AA6B66F4F8224415CD48BD64F1016FF84D15CFAE8E6EDF391292ED26207C9334",
            "next_validators_hash": "AA6B66F4F8224415CD48BD64F1016FF84D15CFAE8E6EDF391292ED26207C9334",
            "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
            "app_hash": "ACCB2033E95DDBC051CF2F03E09B423B3D17A5C1DF3F6E675BC89084788FE94A",
            "last_results_hash": "1A20C58F41547BF8CA9B9FBE323A0B57CFF60D701F8D15E25887B5264072AB8B",
            "evidence_hash": "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855",
            "proposer_address": "2AA44B697AB328C549087868F916B7A60C26B4FF"
        }
    "#;

    let head: RawHeader = serde_json::from_str(data).unwrap();
    println!("{:?}", head.height);

    let file = fs::read("header.json")
        .unwrap();
    let resp: Response = serde_json::from_slice(file.as_slice()).unwrap();
    println!("{:?}", resp.result.header.chain_id);
}