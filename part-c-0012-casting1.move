address hello_blockchain {

    module Daily {

        use std::debug::print;
        // use std::string::{String, utf8};

        fun learning_daily() {

            let x: u8 = 17;
            let y: u16 = 2000;

            //let sum = x + y; // error: cannot use `u16` with an operator which expects a value of type `u8`

            let sum = (x as u16) + y;
        
            print(&sum); // [debug] 2017
        }


        #[test]
        fun testing_daily() {
            
            learning_daily(); 
        }
    }
}

