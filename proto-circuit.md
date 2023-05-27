# Protobuf deserialization:

## CanonicalPartSetHeader:
- sovCanonical:
between length 0 and 127, it's 1. otherwise, it's 2.
for CanonicalPartSetHeader hash, the length always == 32 (?)
if so, then this is simply always 1

## PartSetHeader note:
* PartSetHeader hash seems to be the hash of the protobuf of the entire block... light clients cannot verify this, since they don't have the whole block to begin with.
* This means we don't have to verify it- we can simply verify that the validators signed it, and no other heights.
