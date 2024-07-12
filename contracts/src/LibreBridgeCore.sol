// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

contract LibreBridgeCore is Initializable, PausableUpgradeable, OwnableUpgradeable, UUPSUpgradeable {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address initialOwner) public initializer {
        __Pausable_init();
        __Ownable_init(initialOwner);
        __UUPSUpgradeable_init();
    }

    uint256 public thisChainId;

    mapping(bytes32 => uint256) public domainNonces;

    event PassMessage(
        bytes32 indexed messageHash, uint256 toChain, address fromAppContract, address toAppContract, bytes message
    );

    function passMessage(uint256 toChain, address toAppContract, uint256 nonce, bytes calldata message) public {
        bytes32 domain = computeDomain(thisChainId, toChain, msg.sender, toAppContract);

        require(domainNonces[domain] == nonce, "Target nonce must be same");

        bytes32 messageHash = keccak256(abi.encode(thisChainId, nonce, domain));

        emit PassMessage(messageHash, toChain, msg.sender, toAppContract, message);

        domainNonces[domain] += 1;
    }

    function computeDomain(uint256 fromChainId, uint256 toChainId, address fromAppContract, address toAppContract)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(fromChainId, toChainId, fromAppContract, toAppContract));
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
