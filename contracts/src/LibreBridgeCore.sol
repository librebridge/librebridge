// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";

import "./IAppContract.sol";

contract LibreBridgeCore is Initializable, PausableUpgradeable, OwnableUpgradeable, UUPSUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address initialOwner, uint256 _thisChainId, bytes32 _imageId) public initializer {
        thisChainId = _thisChainId;
        imageId = _imageId;

        __Pausable_init();
        __Ownable_init(initialOwner);
        __UUPSUpgradeable_init();
    }

    uint256 public thisChainId;
    bytes32 public imageId;
    IRiscZeroVerifier public verifier;

    function adminSetThisChainId(uint256 _thisChainId) public onlyOwner {
        thisChainId = _thisChainId;
    }

    function adminSetImageId(bytes32 _imageId) public onlyOwner {
        imageId = _imageId;
    }

    function adminSetVerifier(address _verifier) public onlyOwner {
        verifier = IRiscZeroVerifier(_verifier);
    }

    /// Nonce of domain
    mapping(bytes32 => uint256) public domainNonces;
    /// chainId => blockhash => blocknumber
    mapping(uint256 => mapping(bytes32 => uint256)) blockNumberOfChain;

    event PassMessage(
        bytes32 messageHash,
        uint256 fromChainId,
        uint256 indexed toChainId,
        address indexed fromAppContract,
        address indexed toAppContract,
        bytes message
    );

    function passMessage(uint256 toChain, address toAppContract, uint256 nonce, bytes calldata message) public {
        bytes32 domain = computeDomain(thisChainId, toChain, msg.sender, toAppContract);

        require(domainNonces[domain] == nonce, "Target nonce must be same");

        bytes32 messageHash = keccak256(abi.encode(thisChainId, nonce, domain));

        emit PassMessage(messageHash, thisChainId, toChain, msg.sender, toAppContract, message);

        domainNonces[domain] += 1;
    }

    function receiveMessage(
        uint256 latestBlockHeight,
        bytes32 latestBlockHash,
        uint256 txBlockHeight,
        bytes32 txBlockHash,
        uint256 beginBlockHeight,
        bytes32 beginBlockHash,
        uint256 fromChainId,
        uint256 toChainId,
        address fromAppContract,
        IAppContract toAppContract,
        bytes calldata seal,
        bytes calldata message
    ) public {
        require(toChainId == thisChainId, "Target chainid must be this chain");
        require(latestBlockHeight > txBlockHeight && txBlockHeight > beginBlockHeight, "Failed to verify block number");

        require(blockNumberOfChain[fromChainId][beginBlockHash] == beginBlockHeight, "Failed to verify block");

        bytes memory journal = abi.encode(
            latestBlockHeight,
            latestBlockHash,
            txBlockHeight,
            txBlockHash,
            beginBlockHeight,
            beginBlockHash,
            fromChainId,
            toChainId,
            fromAppContract,
            toAppContract,
            message
        );

        // Verfy proof
        verifier.verify(seal, imageId, sha256(journal));

        blockNumberOfChain[fromChainId][latestBlockHash] = latestBlockHeight;
        blockNumberOfChain[fromChainId][txBlockHash] = txBlockHeight;
        blockNumberOfChain[fromChainId][beginBlockHash] = beginBlockHeight;

        toAppContract.handleMessage(fromAppContract, message);
    }

    function computeDomain(uint256 fromChainId, uint256 toChainId, address fromAppContract, address toAppContract)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(fromChainId, toChainId, fromAppContract, toAppContract));
    }

    function computeDomainThisChain(uint256 toChainId, address fromAppContract, address toAppContract)
        public
        view
        returns (bytes32)
    {
        return computeDomain(thisChainId, toChainId, fromAppContract, toAppContract);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
