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

#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![no_main]

use std::char::from_digit;
use alloy_primitives::{address, Address, Uint, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_ethereum_view_call::{
    config::GNOSIS_CHAIN_SPEC, ethereum::EthViewCallInput, BlockCommitment, ViewCall,
};
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use light_poseidon::{parameters::bn254_x5, Poseidon, PoseidonHasher};

const CONTRACT: Address = address!("22C1f6050E56d2876009903609a2cC3fEf83B415");
const CALLER: Address = address!("6f22b9f222D9e9AF4481df55B863A567dfe1dd42");

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

fn main() {
    // Read the input from the guest environment.
    let call_input: EthViewCallInput = env::read();
    let (encoded_verifying_key, message, signature, poap_index): (
        EncodedPoint,
        Vec<u8>,
        Signature,
        U256,
    ) = env::read();

    // Hash of signature.
    let signature_hash = alloy_primitives::keccak256(signature.to_bytes());

    // Get verifying key.
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // Verify the signature.
    verifying_key
        .verify(&message, &signature)
        .expect("Signature verification failed!");

    // ViewCall to get event_id.
    let call: POAP::tokenDetailsOfOwnerByIndexCall = POAP::tokenDetailsOfOwnerByIndexCall {
        owner: CALLER,
        index: poap_index,
    };

    // Converts the input into a `ViewCallEnv` for execution.
    let view_call_env = call_input.into_env().with_chain_spec(&GNOSIS_CHAIN_SPEC);

    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    let block_commitment = view_call_env.block_commitment();

    // Execute the view call.
    let returns = ViewCall::new(call, CONTRACT)
        .with_caller(CALLER)
        .execute(view_call_env)
        .clone();
    println!("View call result: token_id: {}, event_id: {}", returns._0, returns._1);

    // Create and commit proof data.
    env::commit(
        &ProofData {
            eventId: returns._1,
            semaphoreId: U256::from(0),
            nullifier: signature_hash,
        }
        .abi_encode(),
    );

    // Commit block commitment.
    env::commit(&block_commitment.abi_encode());
}
