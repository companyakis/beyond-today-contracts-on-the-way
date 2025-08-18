module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let current_year: u16 = 2025;

        let mut_ref_current_year: &mut u16 = &mut current_year;

        *mut_ref_current_year = 2026; // * value at - dereferencing

        print(mut_ref_current_year);

        print(&current_year);

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 2026
// [debug] 2026
