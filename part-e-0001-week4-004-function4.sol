/*
 Your Goal: Overload Double
Create another pure external/public function double which takes two uint parameters.
Double both of the arguments and return both of them in the same order they were passed into the function.
*/

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Contract {

    function double(uint _num) external pure returns (uint) {

        return _num * 2;
    }


    function double(uint _a, uint _b) external pure returns (uint, uint) {

        return (_a * 2, _b * 2);
    }

}
