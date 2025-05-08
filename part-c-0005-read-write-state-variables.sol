// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract CompanyAkis {

    uint public myNumber;


    function setMyNumber(uint _n) external {

        myNumber = _n;
    }


    function getMyNumber() external view returns (uint) {

        return myNumber;
    }

}
