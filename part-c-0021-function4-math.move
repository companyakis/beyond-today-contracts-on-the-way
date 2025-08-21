module mustafa::calculations {

    public entry fun sum_triple(x: u128, y: u128, z: u128) {

       let _ = x + y + z;
    }
}

// Private function: A hidden machine setting inside a factory, known only to the factory workers. 
// No one from the outside can access it.

// Public function: A tool that another machine inside the factory can use. 
// This tool cannot be requested from the outside, but it can be used by other machines.

// Public entry function: A process initiation button at the factory gate. 
// A person from the outside (a front-end) can press this button to initiate a process inside. 
// However, the process is still executed inside the factory (the blockchain) by the machines (backend functions).
