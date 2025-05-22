/*
Let's create an empty instantiate function. Here are the steps we're interested in:

Write a use declaration to get access to the entry_point attribute from the CW library.
Write the signature for a public Rust function called instantiate. For now, we only want to write the function with an empty parameters list, 
and empty body and no return, like this pub fn my_function() {}. We'll fill in the rest as we go along.

*/

use cosmwasm_std::entry_point;

#[entry_point]
pub fn instantiate() {
  
}


/*
CosmWasm is a smart contract platform built for the Cosmos ecosystem. It allows you to write high performance and secure smart contracts 
in a multitude of languages which compile to Wasm...

You can think of smart contract code as the blueprint for the contract itself. It does not truly become a useful part of a blockchain until it has been instantiated. 
Once it is established on the blockchain it can be interacted with.

In this exercise and the next few we're going to work on the instantiate entry point (as well as two others).

In Rust (and other C/C++ and Java-like languages), there is the concept of an "entry point". 
This is typically called main and is the first function called when a compiled application is executed.
When compiling a traditional Rust app, the compiler expects there to be a main function from which the rest of the application will start and run.

There is no main function in CosmWasm - so how does it know instantiate is an entry point

You can think of smart contract code as the blueprint for the contract itself. It does not truly become a useful part of a blockchain until it has been instantiated. 
Once it is established on the blockchain it can be interacted with.

In this exercise and the next few we're going to work on the instantiate entry point (as well as two others).

In Rust (and other C/C++ and Java-like languages), there is the concept of an "entry point". 
This is typically called main and is the first function called when a compiled application is executed. 
When compiling a traditional Rust app, the compiler expects there to be a main function from which the rest of the application will start and run.

There is no main function in CosmWasm - so how does it know instantiate is an entry point

*/
