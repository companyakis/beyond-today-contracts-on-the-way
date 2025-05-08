// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract CompanyAkis {

    // state, local and global variables

    string public me = "Mustafa Buyukdereli"; // state

    uint16 public year = 2025; // state

    uint256 public time = block.timestamp; // global

    address public owner = msg.sender; // global

    function proverbKeeper() external pure returns (string memory) {

        string memory proverb = "A rolling stone gathers no moss!";

        return proverb; // local
    }

}
