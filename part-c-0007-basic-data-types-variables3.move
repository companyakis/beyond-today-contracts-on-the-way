module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let my_address: address = @mustafa; // This symbol tells the compiler to use the value of the mustafa address from the Move.toml file.

        print(&my_address);

       
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

