module mustafa::learn_move {

    use std::debug::print;
    //use std::string::{String, utf8};

    fun lets_move() {

        let my_proverb: vector<u8> = b"A rolling stone gathers no moss...";

        print(&my_proverb); // 0x4120726f6c6c696e672073746f6e652067617468657273206e6f206d6f73732e2e2e
    }

    #[test]
    fun testing() {

        lets_move();
    }

}
