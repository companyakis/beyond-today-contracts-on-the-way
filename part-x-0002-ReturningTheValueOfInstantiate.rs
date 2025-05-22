/*
For this exercise, we're simply going to elaborate on the instantiate outline we've already created.

Add a Result<R, E> return to the function. "R" will be Response and "E" will be ContractError, which we can import as a Rust Crate

Have the function return Ok(Response::default()).

*/

use cosmwasm_std::entry_point;
use cosmwasm_std::{Response};
use crate::error::ContractError;

#[entry_point]
pub fn instantiate() -> Result<Response, ContractError> {
  
    Ok(Response::default())
}



/*
The instantiate entry point returns a Rust Result, taking a CosmWasm Response struct and an error type.

Response
The Response struct returns a few items that allow the contract to communicate back to the caller. It has several types in it for sending messages and data back.

Messages
CosmWasm is based on the Actor Model  design pattern.
In this pattern, Actors do not talk directly to one another (i.e., do not call functions directly) but rather send messages to one another
*/
