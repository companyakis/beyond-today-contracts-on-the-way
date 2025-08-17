module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let (q1_sales, q2_sales, q3_sales, q4_sales) : (u32, u32, u32, u32) = (32_450_000, 18_650_000, 41_475_500, 21_650_454);

        print(&q1_sales);
        print(&q2_sales);
        print(&q3_sales);
        print(&q4_sales);
       
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

// [debug] 32450000
// [debug] 18650000
// [debug] 41475500
// [debug] 21650454
