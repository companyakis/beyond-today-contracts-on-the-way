// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

contract CompanyAkis {

    function examResult(uint8 _score) public pure returns (string memory) {

        if (_score >=85 && _score <=100) {

            return "Awesome result!";
        }

        else if (_score >=70 && _score < 84) {

            return "Not bad, but you can do better!";
        }

        else {

            return "You should study more!";
        }
    }

 
}
