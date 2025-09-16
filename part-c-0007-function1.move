module hello_blockchain::learning {

        use std::debug::print;

        use std::string::utf8;

        fun say_hi() {

            print(&utf8(b"Hi there! Mustafa Buyukdereli was here!"));
        }

        #[test]
        fun testing() {

            say_hi(); // [debug] "Hi there! Mustafa Buyukdereli was here!"
        }
}

