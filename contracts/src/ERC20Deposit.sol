// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import "./LibreBridgeCore.sol";

contract ERC20Deposit is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    using SafeERC20 for IERC20;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    LibreBridgeCore bridgeCore;

    address remoteERC20AppContract;

    function adminBridgeCore(address _bridgeCore) public onlyOwner {
        bridgeCore = LibreBridgeCore(_bridgeCore);
    }

    function initialize(address initialOwner, address _bridgeCore, address _remoteERC20AppContract)
        public
        initializer
    {
        bridgeCore = LibreBridgeCore(_bridgeCore);

        remoteERC20AppContract = _remoteERC20AppContract;

        __Ownable_init(initialOwner);
        __UUPSUpgradeable_init();
    }

    function depositERC20(uint256 targetChain, IERC20 fromToken, address targetToken, uint256 amount) public {
        fromToken.safeTransferFrom(msg.sender, address(this), amount);

        bytes memory message = abi.encode(targetToken, amount);

        bytes32 domain = bridgeCore.computeDomainThisChain(targetChain, address(this), remoteERC20AppContract);
        uint256 nonce = bridgeCore.domainNonces(domain);

        bridgeCore.passMessage(targetChain, remoteERC20AppContract, nonce, message);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
