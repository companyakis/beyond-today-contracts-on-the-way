address hello_blockchain {
    module Daily {
        use std::debug::print;
        use hello_blockchain::Math;

        fun sum_nums(a: u64, b: u64): u64 {
            Math::sum(a, b)
        }

        #[test]
        fun testing_daily() {
            let s1 = sum_nums(7550, 75555);
            print(&s1); 
        }
    }
}
