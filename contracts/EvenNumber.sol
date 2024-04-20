// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

contract EvenNumber {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.IS_EVEN_ID;

    struct BlockCommitment {
        bytes32 blockHash;
        uint256 blockNumber;
    }

    struct ProofData {
        uint256 eventId;
        uint256 semaphoreId;
        bytes32 nullifier;
    }

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
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
    }
}
