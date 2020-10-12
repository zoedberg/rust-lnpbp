Change Log
==========

v0.1.0
------

### Library overview
- **Paradigms**: generic APIs for L1/L3 best practices
  * **Client-side validation**: _______
  * **Single-use-seals**: _______
  * **Strict encoding**: _____
- **Bitcoin protocol**: extensions to `bitcoin` crate and L2/L3 APIs
  * **Deterministic bitcoin commitments** (DBC) based on LNPBP1-4 standard
  * **
  * **Short bitcoin identifiers** based on LNPBP-4 standard
  * **Resolver API**: _______
  * **Chains**, chain parameters and universal asset identifiers: ______
  * **Script types**: _________
  * **Transaction-output-based single-use-seals**: _______
- **RGB**: confidential smart-contract system for Bitcoin & Lightning Network
  based on client-side validation paradigm (LNPBP______ standards)
  * **Schema** ______
  * **Contracts** _____
  * **Scripting** _____
  * **Confidentiality** _____
  * **Scalability** _____
  The library implements RGB Core v1 release candidate set of standards
- **Lightning networking protocol**: generalized P2P and RPC networking APIs
  based on the original Lightning standard; early preview
  * Universal P2P node ids supporting IPv4, IPv6, Torv2 and Torv3 addresses and
    public keys
  * Feature vectors _____
  * ______

### Breaking changes since RC2
- Updated taproot-based hashed tag system (BIP-340) according to the most
  recent specs.

No other changes since v0.1.0-rc2 except updated upstream dependencies: now they
are all based on published crates without forks.


v0.1.0-rc.2
-----------

### Breaking changes:
- Changed embedded procedure names for RGB VM
- Removed requirement for PSBT to contain fee key in RGB anchor creation (it 
  needs to be a properly constructed PSBT with `witness_utxo`/`non_witness_utxo` 
  data)

### Other changes:
- More embedded procedures for RGB VM
- Schema serde serialization (YAML, JSON etc)
- Serde serialization for all RGB contract structures
- Strict encoding and decoding of Curve25519 public keys and Ed25519 signatures
- Implementation of Curve25519 public keys and Ed25519 signatures as RGB state 
  and metadata
- Bech types for Pedersen commitments, Bulletproofs, Curve25519 data
- Tweaking factor is added into PSBT information during anchor creation
- Added bitcoin protocol resolvers API


v0.1.0-rc.1
-----------

### Breaking changes:
- RGB protocol & schema versioning with feature bits
- Consignment versioning
- Changed Bech32 encodings of RGB data structures; added deflation encoding
- Implemented RGB public state extensions
- Refactored LNP addressing and it's encoding
- Completed Tor v2 and v3 addresses support
- RGB data structures naming refactoring
- Changed bulletproofs commitments which will enable future aggregation
- Introduced Chain and ChainParam types instead of old network versioning

### Other changes:
- Test coverage >70%
- Code docs >50%


v0.1.0-beta.4
-------------

### Breaking changes:
- Updated upstream crates (bitcoin, bitcoin_hashes, secp256k1, 
  grin_secp256k1zpk, miniscript, lightning) with many PRs merged
- EmbedCommitVerify now can mutate container data (used for returning tweaking 
  factors)
- Upgrading `rand` version to the most recent one (blocked previously by 
  grin_secp256k1zpk dependency)
- Changied txout seals to use u32 vouts instead of u16
- Changed txout blinding factor to be u64 instead of u32

### Other changes:
- Test coverage >50% (zero-knowledge functionality & RGB contracts structures)
- Returning tweaking factors
- Minimal support for Tor V2 addresses; improved internet address parsing


v0.1.0-beta.3
-------------

### Breaking changes
- Single-use-seals blinding factor changed from 32-bit to 64-bit of entropy
- Transaction output indexes in single-use-seal definitions are now 32-bit, as 
  in Bitcoin Core / rust-bitcoin (previously were 16-bit)

### New features
- Initial Tor V2 address support
- Test cases for BP mod strict encoding


v0.1.0-beta.2
-------------

### Features overview
- Complete validation workflow with new Validator object
- Virtual machines for RGB contracts (interface + embedded VM)
- `Consignment` now has a version field, so in the future more space-saving 
  variants can be created (like removing txid from anchors and using short 
  universal bitcoin IDs when BP node adoption will increase)
- Anchor contains txid field; so validation can be performed with just Bitcoin 
  Core (no Electrum or BP node is required). This also speeded up validation 
  performance significantly.

### Breaking changes
- Change of `TransitionId` hash tag value (previously-generated transition ids 
  will be invalid)
- Change of `GenesisId`  hash tag value (previously-generated contract/assets 
  ids will be invalid)
- `TransitionId` type is replaced with `NodeId`
- `NodeId` and `ContractId` are now equal by value; `ContractId` is `NodeId` 
  wrapper
- `ancestors()` method moved from `Transition` to `Node` trait; genesis returns 
  an empty array
- Consignment endpoints contain `NodeId` information
