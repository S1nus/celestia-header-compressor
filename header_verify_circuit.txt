| Name               | Type     | Description                                                     |
|--------------------|----------|-----------------------------------------------------------------|
| Consensus:         |          |                                                                 |
| Block              | u64      | Block version                                                   |
| App                | u64      | App version                                                     |
| ChainID            | String   | Fixed identifier of chain                                       |
| Height             | u64      | Block height                                                    |
| Time:              |          |                                                                 |
| wall               | uint64   |                                                                 |
| ext                | uint64   |                                                                 |
| loc:               | Location |                                                                 |
| name               | string   |                                                                 |
| zone:              | []zone   | (various other fields that need serialization and verification) |
| ...                |          |                                                                 |
| LastBlockID:       | BlockID  |                                                                 |
| Hash               | HexBytes |                                                                 |
| PartSetHeader:     |          |                                                                 |
| Total              | u32      |                                                                 |
| Hash               | HexBytes |                                                                 |
| LastCommitHash     | HexBytes |                                                                 |
| DataHash           | HexBytes |                                                                 |
| ValidatorsHash     | HexBytes |                                                                 |
| NextValidatorsHash | HexBytes |                                                                 |
| ConsensusHash      | HexBytes |                                                                 |
| AppHash            | HexBytes |                                                                 |
| LastResultsHash    | HexBytes |                                                                 |
| EvidenceHash       | HexBytes |                                                                 |
| ProposerAddress    | address  |                                                                 |
