module mustafa::learn_move {

    use std::debug::print;

    use mustafa::calculations;

    fun lets_move() {

        let result = calculations::sum_triple(15, 150, 150000);

        print(&result); // [debug] 150165

    }
   
    #[test]
    fun testing() {

        lets_move(); 
    }

}

