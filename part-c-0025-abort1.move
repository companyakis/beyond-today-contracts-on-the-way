module mustafa::learn_move {

    //use std::debug::print;

    const ERROR_COST_EXCEEDED: u64 = 1001;
    const ERROR_LOWER_SALES: u64 = 1002;

    const MAX_COST: u64 = 52_000;
    const MIN_SALES: u64 = 65_000;

    public fun cost_and_sales_controller(cost: u64, sales: u64) {

        if (cost > MAX_COST) {

            abort ERROR_COST_EXCEEDED;
        };

        if (sales < MIN_SALES) {

            abort ERROR_LOWER_SALES;
        };

    }
   
    #[test]
    #[expected_failure(abort_code = ERROR_COST_EXCEEDED)]
    public fun testing_cost() {

        cost_and_sales_controller(67000, 45000);
        
    }

    #[test]
    #[expected_failure(abort_code = ERROR_LOWER_SALES)]
    public fun testing_sales() {

        cost_and_sales_controller(50000, 64000);
    }

}

