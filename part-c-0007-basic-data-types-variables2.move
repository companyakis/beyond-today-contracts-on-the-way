module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let is_completed: bool = false;

        let ready = true;

        print(&(is_completed == ready)); // false

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

