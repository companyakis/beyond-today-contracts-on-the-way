// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract CompanyAkis {

    bool public completed = false;

    bool public started = true;

    bool public ok;

    constructor(bool _condition) {

        ok = _condition;
    }

}
