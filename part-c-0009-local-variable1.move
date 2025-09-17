address hello_blockchain {

    module Daily {
        use std::debug::print;

        fun learning_daily() {
            
            let year: u16 = 2000;
            print(&year);
        }

        fun foo() {

            let year: u16 = 2025;
            print(&year);
        }

        #[test]
        fun testing_daily() {
            
            learning_daily(); // 2000

            foo(); // 2025
        }
    }
}
