module mustafa::learn_move {

    use std::debug::print;
    use std::string::{String, utf8};

    fun lets_move() {

        let my_proverb: String = utf8(b"A rolling stone gathers no moss...");

        print(&my_proverb);
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

