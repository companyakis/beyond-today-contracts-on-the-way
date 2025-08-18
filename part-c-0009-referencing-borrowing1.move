module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let current_year: u16 = 2025;

        let ref_current_year: &u16 = &current_year;

        print(&current_year);

        print(ref_current_year);

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 2025
// [debug] 2025
