module mustafa::learn_move {

    use std::debug::print;
    use std::string::{utf8};

    fun lets_move() {

        let my_proverb: vector<u8> = b"A rolling stone gathers no moss...";

        print(&utf8(my_proverb)); // [debug] "A rolling stone gathers no moss..."
    }

    #[test]
    fun testing() {

        lets_move();
    }

}
