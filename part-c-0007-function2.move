module hello_blockchain::learning {

        use std::debug::print;

        fun sum(x: u64, y: u64, z: u64): u64 {

            let result = x + y + z;

            result
        }

        #[test]
        fun testing() {

            let sum_result = sum(55_000, 550_000, 5_500_000);

            print(&sum_result); // [debug] 6105000
        }
}

