module hello_blockchain::learning {

        use std::debug::print;
        use std::string::{String, utf8};

        fun why_move(): String {

            let why: String = utf8(b"Why Move? For secure and fast Web3...");
            return why
        }

        #[test]
        fun testing() {

            let why_test = why_move();

            print(&why_test); // [debug] "Why Move? For secure and fast Web3..."
        }
}
