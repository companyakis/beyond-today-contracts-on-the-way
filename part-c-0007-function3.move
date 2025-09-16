module hello_blockchain::learning {

        use std::debug::print;
    
        fun max_num(x: u64, y: u64): u64 {

            if (x > y) {
                
                x
            } else {

                y
            }
        }

        #[test]
        fun testing() {

            let r1 = max_num(55_000, 550_000);

            print(&r1); // [debug] 550000

            let r2 = max_num(44444, 44444);

            print(&r2); // [debug] 44444
        }
}

