// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::{sol, SolCall, SolInterface, SolValue};
use anyhow::{Context, Result};
use apps::{BonsaiProver, TxSender};
use clap::Parser;
use k256::{
    ecdsa::{
        signature::{Keypair, Signer},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
use methods::IS_EVEN_ELF;
use rand_core::OsRng;
use risc0_ethereum_view_call::config::{
    ChainSpec, ForkCondition, EIP1559_CONSTANTS_DEFAULT, GNOSIS_CHAIN_SPEC,
};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallEnv, EvmHeader, ViewCall,
};
use risc0_zkvm::serde::to_vec;
use risc0_zkvm::Journal;

// `IEvenNumber` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface IEvenNumber {
        function set(uint256 x, bytes32 post_state_digest, bytes calldata seal);
    }
}

sol! {
    /// ERC-20 balance function signature.
    interface POAP {
        function tokenDetailsOfOwnerByIndex(address owner, uint256 index) external view returns (uint256, uint256);
    }
}

// /// Arguments of the publisher CLI.
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     // /// Ethereum chain ID
//     // #[clap(long)]
//     // chain_id: u64,

//     // /// Ethereum Node endpoint.
//     // #[clap(long, env)]
//     // eth_wallet_private_key: String,
//     /// Ethereum Node endpoint.
//     #[arg(short, long, env = "RPC_URL")]
//     rpc_url: String,
//     // /// Application's contract address on Ethereum
//     // #[clap(long)]
//     // contract: String,

//     // /// The input to provide to the guest binary
//     // #[clap(short, long)]
//     // input: U256,
// }

fn get_verification_inputs(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
    poap_index: U256,
) -> Result<Vec<u8>> {
    /// Address of the USDT contract on Ethereum Sepolia
    const contract: Address = address!("22C1f6050E56d2876009903609a2cC3fEf83B415");

    /// Caller address
    const caller: Address = address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42");

    let call: POAP::tokenDetailsOfOwnerByIndexCall = POAP::tokenDetailsOfOwnerByIndexCall {
        owner: address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42"),
        index: poap_index,
    };

    // Create a view call environment from an RPC endpoint and a block number. If no block number is
    // provided, the latest block is used. The `with_chain_spec` method is used to specify the
    // chain configuration.
    let env = EthViewCallEnv::from_rpc(&std::env::var("RPC_URL").unwrap(), None)?
        .with_chain_spec(&GNOSIS_CHAIN_SPEC);
    let number = env.header().number();
    let commitment = env.block_commitment();

    // Preflight the view call to construct the input that is required to execute the function in
    // the guest. It also returns the result of the call.
    let (view_call_input, returns) = ViewCall::new(call, contract)
        .with_caller(caller)
        .preflight(env)?;
    println!(
        "For block {} `{}` returns: {} - {}",
        number,
        POAP::tokenDetailsOfOwnerByIndexCall::SIGNATURE,
        returns._0,
        returns._1
    );

    let sig_data_inputs = (
        verifying_key.to_encoded_point(true),
        message,
        signature,
        poap_index,
    );

    println!("Running the guest with the constructed input:");

    let input = InputBuilder::new()
        .write(&view_call_input)
        .unwrap()
        .write(&sig_data_inputs)
        .unwrap()
        .bytes();

    Ok(input)
}

fn main() -> Result<()> {
    // // Create a new `TxSender`.
    // let tx_sender = TxSender::new(
    //     args.chain_id,
    //     &args.rpc_url,
    //     &args.eth_wallet_private_key,
    //     &args.contract,
    // )?;

    // Get inputs
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"This is a message that will be signed, and verified within the zkVM";
    let signature: Signature = signing_key.sign(message);
    let poap_index: U256 = U256::from(0);
    let input =
        get_verification_inputs(signing_key.verifying_key(), message, &signature, poap_index)
            .unwrap();

    // ABI encode the input for the guest binary, to match what the `is_even` guest
    // code expects.

    // Send an off-chain proof request to the Bonsai proving service.
    let (journal, post_state_digest, seal) = BonsaiProver::prove(IS_EVEN_ELF, &input)?;

    let journal = Journal::new(journal);
    let (bytes_1, bytes_2, bytes_3): (Vec<u8>, String, U256) = journal.decode().unwrap();
    println!("Commitment: {:?}", bytes_1);
    println!("Signature Hash: {}", bytes_2);
    println!("Event Id: {}", bytes_3);

    // Decode the journal. Must match what was written in the guest with
    // `env::commit_slice`.
    // let x = U256::abi_decode(&journal, true).context("decoding journal data")?;

    // // Encode the function call for `IEvenNumber.set(x)`.
    // let calldata = IEvenNumber::IEvenNumberCalls::set(IEvenNumber::setCall {
    //     x,
    //     post_state_digest,
    //     seal: seal.into(),
    // })
    // .abi_encode();

    // Send the calldata to Ethereum.
    // let runtime = tokio::runtime::Runtime::new()?;
    // runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}

pub struct InputBuilder {
    input: Vec<u32>,
}

impl InputBuilder {
    pub fn new() -> Self {
        InputBuilder { input: Vec::new() }
    }

    pub fn write(mut self, input: impl serde::Serialize) -> Result<Self> {
        self.input.extend(to_vec(&input)?);
        Ok(self)
    }

    pub fn bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&self.input).to_vec()
    }
}
