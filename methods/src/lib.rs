//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

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

#[allow(unused_imports)]
use alloy_primitives::{address, Address, U256};
use alloy_sol_types::{sol, SolCall, SolValue};
use anyhow::{Context, Result};
use clap::Parser;
use k256::{
    ecdsa::{
        signature::{Keypair, Signer},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
use risc0_ethereum_view_call::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthViewCallEnv, BlockCommitment, EvmHeader, ViewCall,
};
use std::collections::BTreeMap;

use rand_core::OsRng;
use risc0_ethereum_view_call::config::{
    ChainSpec, ForkCondition, EIP1559_CONSTANTS_DEFAULT, GNOSIS_CHAIN_SPEC,
};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv, SessionInfo};
use tracing_subscriber::EnvFilter;

sol! {
    /// ERC-20 balance function signature.
    interface POAP {
        function tokenDetailsOfOwnerByIndex(address owner, uint256 index) external view returns (uint256, uint256);
    }
}

/// Given an secp256k1 verifier key (i.e. public key), message and signature,
/// runs the ECDSA verifier inside the zkVM and returns a receipt, including a
/// journal and seal attesting to the fact that the prover knows a valid
/// signature from the committed public key over the committed message.
fn prove_ecdsa_verification(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
    poap_index: U256,
) -> Result<SessionInfo> {
    /// Address of the USDT contract on Ethereum Sepolia
    const contract: Address = address!("22C1f6050E56d2876009903609a2cC3fEf83B415");

    /// Caller address
    const caller: Address = address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42");

    /// Function to call
    let call: POAP::tokenDetailsOfOwnerByIndexCall = POAP::tokenDetailsOfOwnerByIndexCall {
        owner: address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42"),
        index: poap_index,
    };

    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Create a view call environment from an RPC endpoint and a block number. If no block number is
    // provided, the latest block is used. The `with_chain_spec` method is used to specify the
    // chain configuration.
    let env = EthViewCallEnv::from_rpc(&std::env::var("RPC_URL").unwrap(), None)?
        .with_chain_spec(&GNOSIS_CHAIN_SPEC);
    let number = env.header().number();
    let commitment = env.block_commitment();

    // Preflight the view call to construct the input that is required to execute the function in
    // the guest. It also returns the result of the call.
    let (input, returns) = ViewCall::new(call, contract)
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
    let session_info = {
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .write(&sig_data_inputs)
            .unwrap()
            .build()
            .context("Failed to build exec env");
        let exec = default_executor();
        exec.execute(env?, IS_EVEN_ELF)
            .context("failed to run executor")?
    };

    Ok(session_info)
}

sol!(
    struct ProofData {
        uint256 eventId;
        uint256 semaphoreId;
        bytes32 nullifier;
    }
);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_poap() {
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let message = b"This is a message that will be signed, and verified within the zkVM";
        let signature: Signature = signing_key.sign(message);
        let poap_index: U256 = U256::from(0);
        let session_info =
            prove_ecdsa_verification(signing_key.verifying_key(), message, &signature, poap_index)
                .unwrap();

        println!("Session info: {:?}", session_info.journal.as_ref());

        let proof_data = ProofData::abi_decode(&session_info.journal.as_ref(), true).unwrap();

        println!("Proof data: {}", proof_data.eventId);
        println!("Proof data: {}", proof_data.semaphoreId);
        println!("Proof data: {}", proof_data.nullifier);

        // let bytes = session_info.journal.as_ref();
        // println!("------------ {:?}", &bytes);
        // assert_eq!(&bytes[..64], &commitment.abi_encode());
    }
}
