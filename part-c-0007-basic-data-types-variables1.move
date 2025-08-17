module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let age: u8 = 10;
        print(&age);

        age = 17;
        print(&age);

        let year: u16 = 2025;
        print(&year);

        let sales: u64 = 323_434_680;
        print(&sales);
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 10
// [debug] 17
// [debug] 2025
// [debug] 323434680
