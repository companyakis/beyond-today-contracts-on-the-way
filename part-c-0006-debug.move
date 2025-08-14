module mustafa::learn_move {

    use std::debug;

    fun sample_greet() {

        debug::print(&b"Hi there!");
    }

    #[test]
    fun testing() {

        sample_greet();
    }

}
