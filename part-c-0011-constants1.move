module mustafa::learn_move {

    use std::debug::print;

    const THIS_YEAR: u16 = 2025;

    fun lets_move() {

        print(&THIS_YEAR);

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

