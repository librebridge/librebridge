// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

library LibreBridgeLib {
    function computeMessageHash(uint256 fromChainId, uint256 toChainId, uint256 nonce, bytes32 domain)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(fromChainId, toChainId, nonce, domain));
    }

    function computeMessageHashThisChain(uint256 toChainId, uint256 nonce, bytes32 domain)
        public
        view
        returns (bytes32)
    {
        return computeMessageHash(block.chainid, toChainId, nonce, domain);
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
        return computeDomain(block.chainid, toChainId, fromAppContract, toAppContract);
    }
}
