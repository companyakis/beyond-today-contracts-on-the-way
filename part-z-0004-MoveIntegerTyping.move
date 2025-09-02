// Now it's your turn to add a variable to our island to find out how many trees are on it.

// To do this, add a nb_tree variable of type u8 to your Move my_shore module.

module robinson::my_shore {

    struct GlobalData has key {
        nb_tree: u8,
    }

}


// Integer types

// Move supports six unsigned integer types: u8, u16, u32, u64, u128, and u256.

// If the type is not specified, the compiler will attempt to infer the type from the context. In the absence of clear inference, the default type is u64.
