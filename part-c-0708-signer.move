// Now it's our turn to add the signer as a parameter to our function shores and check if the signer of the function shores is equal to address of @0x42 using signer::address_of(&s) == @0x42.

module robinson::my_shore {

    use std::signer;

    struct GlobalData has key {
        nb_tree: u8,
        has_river: bool,
        shore_location: address,
        daily_visitors: vector<u64>,
    }
    
    fun shores(s: signer) {
        assert!(signer::address_of(&s) == @0x42, 0);
    }
 }
