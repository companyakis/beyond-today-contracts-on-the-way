/*
Your Goal: Hello World 
It's time to do Hello World in Solidity! 

Create a public bytes32 state variable msg1 which stores a string literal containing "Hello World".
Create a public string state variable msg2 which stores a string literal that requires over 32 bytes to store.

*/

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Contract {

    bytes32 public msg1 = "Hello World";

    string public msg2 = "Hello Hello Hello Hello Hello Hello Hello Hello";
	
}

