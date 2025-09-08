module hello_blockchain::learning {

        use std::debug::print;

        fun why_move() {

            let ready: bool = true;

            let completed: bool = false;

            let r1: bool = ready && completed;

            let r2: bool = ready || completed;

            let r3: bool = !ready && completed;

            let r4: bool = !(ready && completed) || completed;

            print(&r1);
            print(&r2);
            print(&r3);
            print(&r4);
 
        }

        #[test]
        fun testing() {

            why_move();
        }
}

// [debug] false
// [debug] true
// [debug] false
// [debug] true

