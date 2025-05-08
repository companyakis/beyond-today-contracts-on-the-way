// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract CompanyAkis {

    // can be set inside the constructor but cannot be modified afterwards

    uint16 public immutable startupYear;

    constructor(uint16 _y) {

        startupYear = _y;
    }
}
