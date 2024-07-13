// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

contract LBMintableERC20 is ERC20, ERC20Burnable {
    address public outgoing;

    address public remoteToken;

    uint8 decimal;

    constructor(address _outgoing, address _remoteToken, string memory _name, string memory _symbol, uint8 _decimals)
        ERC20(_name, _symbol)
    {
        outgoing = _outgoing;
        remoteToken = _remoteToken;

        decimal = _decimals;
    }

    function decimals() public view override returns (uint8) {
        return decimal;
    }

    function mint(address to, uint256 amount) public {
        require(msg.sender == outgoing, "Only outgoing contract can call this function");

        _mint(to, amount);
    }
}
