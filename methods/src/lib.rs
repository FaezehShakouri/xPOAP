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
use k256::{
    ecdsa::{
        signature::{Keypair, Signer},
        Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};
use risc0_ethereum_view_call::{
    ethereum::EthViewCallEnv, EvmHeader, ViewCall,
};
use rand_core::OsRng;
use risc0_ethereum_view_call::config::GNOSIS_CHAIN_SPEC;
use risc0_zkvm::{default_executor, ExecutorEnv, SessionInfo};
use tracing_subscriber::EnvFilter;

sol! {
    interface POAP {
        function tokenDetailsOfOwnerByIndex(address owner, uint256 index) external view returns (uint256, uint256);
    }
}

sol!(
    struct ProofData {
        uint256 eventId;
        uint256 semaphoreId;
        bytes32 nullifier;
    }
);

fn get_verification_inputs(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
    poap_index: U256,
    elf_binary: &[u8],
) -> Result<SessionInfo> {

    const CONTRACT: Address = address!("22C1f6050E56d2876009903609a2cC3fEf83B415");
    const CALLER: Address = address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42");

    let call: POAP::tokenDetailsOfOwnerByIndexCall = POAP::tokenDetailsOfOwnerByIndexCall {
        owner: address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42"),
        index: poap_index,
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let env = EthViewCallEnv::from_rpc(&std::env::var("RPC_URL").unwrap(), None)?
        .with_chain_spec(&GNOSIS_CHAIN_SPEC);
    let number = env.header().number();

    let (input, returns) = ViewCall::new(call, CONTRACT)
        .with_caller(CALLER)
        .preflight(env)?;
    println!(
        "For block {} `{}` returns: token_id: {} - event_id: {}",
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
        exec.execute(env?, elf_binary)
            .context("failed to run executor")?
    };

    Ok(session_info)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_poap_owner() {
        let signing_key = SigningKey::random(&mut OsRng); 
        let message = b"This is a message that will be signed, and verified within the zkVM";
        let signature: Signature = signing_key.sign(message);
        let poap_index: U256 = U256::from(0);

        let session_info =
        get_verification_inputs(signing_key.verifying_key(), message, &signature, poap_index, super::IS_POAP_OWNER_ELF)
                .unwrap();
        
        let proof_data = ProofData::abi_decode(&session_info.journal.as_ref(), false).unwrap();
    }
}
