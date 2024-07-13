// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

interface IAppContract {
    function handleMessage(address fromAppContract, bytes calldata message) external;
}
