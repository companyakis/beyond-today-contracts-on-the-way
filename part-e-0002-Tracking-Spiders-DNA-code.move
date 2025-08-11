// Add a struct called SpiderDna with a single field called dna_digits of type u64 to the spider_nest module.

module 0xcafe::spider_nest {

    struct SpiderDna has key {

        dna_digits: u64,
    }
}


/*
On Aptos, all data written to global storage needs to be declared in a struct that has the key attribute.

Unless explicitly declared in a struct with the key attribute, all variables declared and modified in a function are local variables and
thus not stored on the blockchain (in global storage). Also note that the struct name is capitalized by convention.

Move supports multiple different types of unsigned integers. The values of these integers are always zero or larger. 
The different types of integers have different maximum values they can store.

*/
