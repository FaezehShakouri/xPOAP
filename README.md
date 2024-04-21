# myPOAP

## The Problem

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

Leveraging Risc-zero technology, zkPOAPFeedback establishes a secure and Privacy-Preserving ecosystem. Here, users can confirm their ownership of POAP tokens acquired through various events or engagements.

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

### Configuring Bonsai

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
