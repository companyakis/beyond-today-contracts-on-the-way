module mustafa::learn_move {

    use std::debug::print;

    fun sum(x: u128, y: u128, z: u128) : u128 {

        x + y + z
    }

    fun lets_move() {

        let result = sum(25000, 250000, 255000000);

        print(&result); // [debug] 255275000
    }
   
    
    #[test]
    fun testing() {

        lets_move(); 
    }

}

