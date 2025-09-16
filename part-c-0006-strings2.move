module hello_blockchain::learning {

        use std::debug::print;

        //use std::string::utf8;

        fun fast_move() {

            let why_move: vector<u8> = b"Move for secure and fast apps";

            print(&why_move);
 
        }

        #[test]
        fun testing() {

            fast_move(); // [debug] 0x4d6f766520666f722073656375726520616e6420666173742061707073

        }
}

