address hello_blockchain {

    module Daily {

        //use std::debug::print;
        // use std::string::{String, utf8};

        fun learning_daily() {

            // one way to mark variable as intentionally unused - by using underscore _
            
            let _year: u16 = 2025;

            let _: bool = true;
        }


        #[test]
        fun testing_daily() {
            
            learning_daily(); 
        }
    }
}

