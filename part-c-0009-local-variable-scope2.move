address hello_blockchain {

    module Daily {
        use std::debug::print;

        fun learning_daily() {
            
            let year: u16 = 2000;
            print(&year);

            let age: u8 = 17;

            {
                let year: u16 = 2080;
                print(&year);

                age += 15;
                print(&age);
            }
        }


        #[test]
        fun testing_daily() {
            
            learning_daily(); 
        }
    }
}

/*[debug] 2000
[debug] 2080
[debug] 32*/
