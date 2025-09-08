module hello_blockchain::learning {

        use std::debug::print;

        fun why_move() {

            // from u8 to u128... non-negative values!

            let age: u8 = 10;
            let year: u16 = 2025;
            let sales = 452_654; // u64
            let big_num: u128 = 34_454_625_547;

            print(&age);
            print(&year);
            print(&sales);
            print(&big_num);
        }

        #[test]
        fun testing() {

            why_move();
        }
}

// [debug] 10
// [debug] 2025
// [debug] 452654
// [debug] 34454625547
