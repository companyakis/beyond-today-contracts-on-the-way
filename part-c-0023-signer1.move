module mustafa::learn_move {

    use std::debug::print;
    use std::signer;

    public fun lets_move(s: &signer) {

        let addr: address = signer::address_of(s);

        print(&addr);
    }
   
    #[test(account = @0x1)]
    public fun testing(account: signer) {

        lets_move(&account);  // [debug] @0x1
    }

}

// The address in Move.toml is your module's real identity and deployment location, 
// while @0x1 is a placeholder used to simulate a transaction initiator's identity during testing.
