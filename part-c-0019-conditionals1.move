module mustafa::learn_move {

    use std::debug::print;
   
    fun lets_move() {

        let daily_sales: u16 = 45_000;

        let premium: u16 = if (daily_sales > 30_000) { // "let mut usage" is NOT supported yet by APTOS CLI!

            1000

        } else if (daily_sales < 30_000 && daily_sales > 20_000) {

            750

        } else {

            0
        };

        print(&premium); // [debug] 1000

    }

    #[test]
    fun testing() {

        lets_move();
    }

}
