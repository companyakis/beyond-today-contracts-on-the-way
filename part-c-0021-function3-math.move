module mustafa::calculations {

    // "Here, we're saying that only the module named learn_move can access the sum_triple function."

    friend mustafa::learn_move;

    public(friend) fun sum_triple(x: u128, y: u128, z: u128) : u128 {

        x + y + z
    }
}
