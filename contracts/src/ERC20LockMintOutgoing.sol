// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import "./LibreBridgeCore.sol";
import "./IAppContract.sol";
import "./LBMintableERC20.sol";

contract ERC20LockMintOutgoing is Initializable, OwnableUpgradeable, UUPSUpgradeable, IAppContract {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    LibreBridgeCore bridgeCore;

    address remoteAppContract;

    function adminBridgeCore(address _bridgeCore) public onlyOwner {
        bridgeCore = LibreBridgeCore(_bridgeCore);
    }

    function initialize(address initialOwner, address _bridgeCore, address _remoteAppContract) public initializer {
        bridgeCore = LibreBridgeCore(_bridgeCore);

        remoteAppContract = _remoteAppContract;

        __Ownable_init(initialOwner);
        __UUPSUpgradeable_init();
    }

    function handleMessage(address fromAppContract, bytes calldata message) external {
        require(fromAppContract == remoteAppContract, "Only support target requirement");

        (address fromToken, address targetToken, address receiver, uint256 amount) =
            abi.decode(message, (address, address, address, uint256));

        LBMintableERC20 erc20 = LBMintableERC20(targetToken);
        require(erc20.remoteToken() == fromToken, "fromToken must equal to remoteToken of MintableERC20");

        erc20.mint(receiver, amount);
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
