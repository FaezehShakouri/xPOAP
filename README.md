# zkPOAPFeedback

zkPOAPFeedback is a privacy-focused platform tailored for individuals holding POAP (Proof of Attendance Protocol) tokens, offering them a unique opportunity to provide feedback within a club. This system guarantees complete privacy by enabling users to submit evidence of their POAP ownership without compromising their anonymity or the privacy of their POAP tokens.

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
