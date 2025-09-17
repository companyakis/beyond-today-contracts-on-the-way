address hello_blockchain {

    module Daily {

        use std::debug::print;
        use std::string::{String, utf8};

        fun learning_daily() {
            
            let (age, year, name): (u8, u16, String) = (17, 2025, utf8(b"Ayhan"));

            print(&age);
            print(&year);
            print(&name);
        }


        #[test]
        fun testing_daily() {
            
            learning_daily(); 
        }
    }
}

// [debug] 17
// [debug] 2025
// [debug] "Ayhan"
