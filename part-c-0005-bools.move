module hello_blockchain::learning {

        use std::debug::print;

        fun why_move() {

            let ready: bool = true;

            let completed: bool = false;

            print(&ready);
            print(&completed);
 
        }

        #[test]
        fun testing() {

            why_move();
        }
}

