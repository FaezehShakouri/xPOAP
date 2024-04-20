// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.
import {ISemaphore} from "./ISemaphore.sol";

contract EvenNumber {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.IS_EVEN_ID;
    ISemaphore public immutable semaphore;
    uint256 public immutable eventId;
    mapping(bytes32 => bool) public nullifires;
    uint256 public immutable groupId;

    struct BlockCommitment {
        bytes32 blockHash;
        uint256 blockNumber;
    }

    struct ProofData {
        uint256 eventId;
        uint256 semaphoreId;
        bytes32 nullifier;
    }

    struct Signal {
        uint256 signal;
        uint256 scope;
        uint256 merkleTreeRoot;
        uint256 nullifierHash;
        uint256[8] proof;
    }

    constructor(
        IRiscZeroVerifier _verifier,
        address _semaphore,
        uint256 _eventId
    ) {
        verifier = _verifier;
        semaphore = ISemaphore(_semaphore);
        eventId = _eventId;
        groupId = uint256(keccak256(abi.encode(address(this), _eventId)));
        semaphore.createGroup(groupId, 20, address(this));
    }

    function joinGroup(
        bytes memory journal,
        bytes32 postStateDigest,
        bytes calldata seal
    ) public {
        require(
            verifier.verify(seal, imageId, postStateDigest, sha256(journal))
        );
        (
            ProofData memory proofData,
            BlockCommitment memory blockCommitment
        ) = abi.decode(journal, (ProofData, BlockCommitment));

        require(nullifires[proofData.nullifier] == false, "DUPLICATE_PROOF");
        // require(proofData.eventId == eventId, "INVALID_EVENT_ID");

        nullifires[proofData.nullifier] = true;
        semaphore.addMember(groupId, proofData.semaphoreId);
    }

    function validateSignal(Signal memory signal) external {
        ISemaphore(semaphore).verifyProof(
            groupId,
            signal.merkleTreeRoot,
            signal.signal,
            signal.nullifierHash,
            signal.scope,
            signal.proof
        );
    }
}
