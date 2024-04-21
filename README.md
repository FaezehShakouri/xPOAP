# zkPoapCommunities

### Background

POAPs are everywhere and a lot of people own them. Here is a description from the official website [https://poap.xyz](poap.xyz):

`POAP, short for "Proof of Attendance Protocol," allows you to mint memories as digital mementos we call "POAPs." Give POAPs to people for sharing a memory with you.`

Technically, POAPs are simply NFTs on a blockchain. This means that the protocol is permissionless and composable by design. For example, someone could create a gated chat application on top of the POAP protocol to allow holders of EthGlobal POAPs to chat and interact.

However, there is a fundamental privacy issue with the way the POAP protocol is currently implemented. Specifically, for holders to prove they own a POAP issued by EthGlobal, they need to disclose their wallet address. This, in turn, exposes more information about the holders than is necessary for the chat application.

### Solutions

#### A new protocol 🤔

An obvious solution to the privacy problem is to design and implement a new protocol with privacy built into its design. While this approach might work, it would require significant duplicate engineering efforts and would need the promotion of an entirely new protocol. **And then what about all the POAPs already issued? do people then have to migrate to the new protocol?**

#### ZK to the rescue 💯

An ideal solution would work like this: The existing POAP protocol continues to function as usual, allowing people to issue and own POAPs just as they always have. However, when a holder wants to demonstrate ownership of a POAP from a specific event, they can do so without revealing their wallet address. Additionally, applications could integrate with the privacy-preserving protocol in a permission-less manner.

**In this repository you'll find a PoC implementation of the second approach using RISC0**

## How It Works

### High level overview

Holders would generate a zero-knowledge proof of ownership for a POAP from a specific issuer off-chain. They could then use this proof to join a `Semaphore` group. By leveraging the `Semaphore` protocol, application developers could enable anonymous interactions using existing tools. Additionally, a `Semaphore` identity offers the added benefit of allowing holders to interact with the system without needing to regenerate a proof each time.

### What exactly is being proven is zero knowledge?

#### 1 - POAP data

Each POAP is associated with an `eventId`. And all the required information is stored on-chain. More specifically the following `view` on the POAP contract is all we need:

```solidity
function tokenDetailsOfOwnerByIndex(address owner, uint256 index) external view returns (uint256, uint256);
```

This view takes in an `owner` address and an `index`. It then returns the corresponding `tokenId` and `eventId`. From this information, we'll want to expose the `eventId` and keep everything else (`owner`, `tokenId`, `index`) private.
For this part, We'll make use of the `view-call` library of `risc0` to query the blockchain and generate a proof of the validity of data.

#### 2 - Ownership of the wallet

So far we have generated a proof that there exists an `owner` holding a POAP issued in a given event. Now the holder needs to proof that they actually have the private key of the owner wallet.

For this, the user is required to signed predetermined message, like the following and pass it to the risc0 guest program:

`This is a message that will be signed, and verified within the zkVM. It is intended to prove ownership of a POAP with eventId xxxx`

the guest will then extract the signer of this message from the provided signature and makes the blockchain query described in step 1.

### What is the nullifier?

To prevent a holder from joining multiple times, the hash of their signature is used as a nullifier. However, even though this prevents the same owner from joining more than once, it does not prevent a new owner from using the same token if it is transferred. To address this, we need to freeze the block number at which the blockchain query is made, effectively disabling transfers.

## Considerations

This project is PoC and has known and unknown bugs.
Also without loss of generality, the source code has some hardcoded data in it.

## Project Structure

Below are the primary files in the project directory

```text
.
├── Cargo.toml                        // Configuration for Cargo and Rust
├── foundry.toml                      // Configuration for Foundry
├── apps
│   ├── Cargo.toml
│   └── src
│       └── lib.rs                    // Utility functions
│       └── bin
│           └── publisher.rs          // Main app to publish program results into your app contract
├── contracts
│   ├── POAPGroup.sol                 // Get proof data and join to club
|   ├── ISemaphore.sol                //
│   └── ImageID.sol                   // Generated contract with the image ID for zkPOAPFeedback
├── methods
│   ├── Cargo.toml
│   ├── guest
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── bin
│   │           └── is_poap_owner.rs  // Guest program for checking ownership of POAPs
│   └── src
│       └── lib.rs                    // Compiled image IDs and tests for the guest program (is_poap_owner)
└── tests
    ├── POAPGroup.t.sol               // Tests for the basic example contract
    └── Elf.sol                       // Generated contract with paths the guest program ELF files.
```

## Dependencies

First, [install Rust] and [Foundry], and then restart your terminal.

```sh
# Install Rust
curl https://sh.rustup.rs -sSf | sh
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
```

Next, you will need to install the `cargo risczero` tool.
We'll use [`cargo binstall`][cargo-binstall] to get `cargo-risczero` installed, and then install the `risc0` toolchain.
See [RISC Zero installation] for more details.

```sh
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

## Getting Started

### Build the Code

- Builds for zkVM program, the publisher app, and any other Rust code.

  ```sh
  cargo build
  ```

- Build your Solidity smart contracts

  > NOTE: `cargo build` needs to run first to generate the `ImageID.sol` contract.

  ```sh
  forge build
  ```

### Run the Tests

- Tests your zkVM program.

  ```sh
  cargo test
  ```

- Test your Solidity contracts, integrated with your zkVM program.

  ```sh
  RISC0_DEV_MODE=true forge test -vvv
  ```

[Bonsai]: https://dev.bonsai.xyz/
[Foundry]: https://getfoundry.sh/
[Get Docker]: https://docs.docker.com/get-docker/
[Groth16 SNARK proof]: https://www.risczero.com/news/on-chain-verification
[RISC Zero Verifier]: https://github.com/risc0/risc0/blob/release-0.21/bonsai/ethereum/contracts/IRiscZeroVerifier.sol
[RISC Zero installation]: https://dev.risczero.com/api/zkvm/install
[RISC Zero zkVM]: https://dev.risczero.com/zkvm
[RISC Zero]: https://www.risczero.com/
[Sepolia]: https://www.alchemy.com/overviews/sepolia-testnet
[app contract]: ./contracts/
[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall#cargo-binaryinstall
[coprocessor]: https://www.risczero.com/news/a-guide-to-zk-coprocessors-for-scalability
[deployment guide]: /deployment-guide.md
[developer FAQ]: https://dev.risczero.com/faq#zkvm-application-design
[image-id]: https://dev.risczero.com/terminology#image-id
[install Rust]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[journal]: https://dev.risczero.com/terminology#journal
[publisher]: ./apps/README.md
[zkVM program]: ./methods/guest/
