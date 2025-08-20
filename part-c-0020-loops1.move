module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

     let num: u8 = 1; // "let mut" usage is not supported yet by APTOS cli!

     while (num <= 50) {

          print(&num);

          num += 3;
     }

    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 1
// [debug] 4
// [debug] 7
// [debug] 10
// [debug] 13
// [debug] 16
// [debug] 19
// [debug] 22
// [debug] 25
// [debug] 28
// [debug] 31
// [debug] 34
// [debug] 37
// [debug] 40
// [debug] 43
// [debug] 46
// [debug] 49
