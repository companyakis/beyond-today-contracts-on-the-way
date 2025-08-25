module mustafa::learn_move {

    use std::debug::print;
    use std::signer;

    public fun lets_move(s: &signer) {

        let addr: &address = signer::borrow_address(s);

        print(addr);
    }
   
    #[test(account = @0x1)]
    public fun testing(account: signer) {

        lets_move(&account);  // [debug] @0x1
    }

}

