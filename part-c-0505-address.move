// Now, let's focus on your island's location. To do this, we must include a shore_location variable of the address type in the Move my_shore module.

module robinson::my_shore {

    struct GlobalData has key {
        nb_tree: u8,
        has_river: bool,
        shore_location: address,
    }

}


/*

Types of Addresses
Two Types of Addresses: There are two kinds of these special codes: named and numerical.

Named Address Example: Think of it like giving a name to a place. Just like we call our house by a name, in programming, we can name a location. For example, we can say my_house: address = @0x1;.

Numerical Address Example: Instead of a name, we can use regular numbers like 42, 0xCAFE, or 2021 to represent a location.

Using Named Addresses: We can use the named addresses wherever we need to mention a location. 
For instance, let a1: address = @0x1; is a quick way of saying the location is 0x0000000000000000000000000000000000000000000000000000000000000001.

*/
