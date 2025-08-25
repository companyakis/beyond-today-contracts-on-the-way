module mustafa::learn_move {

    use std::debug::print;

    const ERROR_COST_EXCEEDED: u64 = 1001;

    public fun cost_controller(cost: u64) {

        print(&cost);

        assert!(cost <= 52_000, ERROR_COST_EXCEEDED);

        print(&cost);

    }
   
    #[test]
    public fun testing() {

        // cost_controller(67000);
        cost_controller(45000);
    }

}

