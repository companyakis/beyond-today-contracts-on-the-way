module mustafa::learn_move {

    use std::debug::print;

    fun lets_move() {

        let (sales_year, total_sales, achieved) : (u16, u32, bool) = (0, 0, false);

        print(&sales_year);
        print(&total_sales);
        print(&achieved);
    }

    #[test]
    fun testing() {

        lets_move();
    }

}

//[debug] 0
//[debug] 0
//[debug] false
