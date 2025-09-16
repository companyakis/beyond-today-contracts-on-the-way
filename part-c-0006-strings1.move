module hello_blockchain::learning {

        use std::debug::print;

        use std::string::{String, utf8};

        fun fast_move() {

            let why_move: String = utf8(b"Move for secure and fast apps");

            print(&why_move);
 
        }

        #[test]
        fun testing() {

            fast_move(); // [debug] "Move for secure and fast apps"

        }
}

