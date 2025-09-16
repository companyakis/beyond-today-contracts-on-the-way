module hello_blockchain::learning {

        use std::debug::print;
    
        fun sum_or_diff(x: u64, y: u64): (u64, u64) {

            if (x >= y) {

                (x + y, x - y)

            } else {

                (x + y, 0)
            } 
        }

        #[test]
        fun testing() {

            let (s1, d1) = sum_or_diff(550, 497);
            print(&s1);
            print(&d1);

            let (s2, d2) = sum_or_diff(400, 9999);
            print(&s2);
            print(&d2);
        }
}

// [debug] 1047
// [debug] 53
// [debug] 10399
// [debug] 0
