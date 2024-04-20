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

    struct BlockCommitment {
        bytes32 blockHash;
        uint256 blockNumber;
    }

    struct ProofData {
        uint256 eventId;
        uint256 semaphoreId;
        bytes32 nullifier;
    }

    constructor(IRiscZeroVerifier _verifier, ISemaphore _semaphore, uint256 _eventId) {
        verifier = _verifier;
        semaphore = _semaphore; 
        eventId = _eventId;
        bytes32 group_id = keccak256(abi.encode(address(this), _eventId));
        semaphore.createGroup(group_id, 20, address(this));
    }

    function joinGroup(
        bytes memory journal,
        bytes32 postStateDigest,
        bytes calldata seal
    ) public {
        require(verifier.verify(seal, imageId, postStateDigest, sha256(journal)));
        (
            ProofData memory proofData,
            BlockCommitment memory blockCommitment
        ) = abi.decode(journal, (ProofData, BlockCommitment));
    }
}
