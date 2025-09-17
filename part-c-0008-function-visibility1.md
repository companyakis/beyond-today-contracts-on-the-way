Functions in move can be declared as:

private - By default the functions in move are private that means they can only be called within the same module and cannot be access outside the module by other modules and scripts.

public - A public function can be called by any function in any module or script.

public(friend) - A public(friend) function can be called by any function in the same module and by the function of the module which are explicitly defined in the friend list.

entry - Entry function are the function where move program starts its execution. They can be combined with public and public(friend) modifier.

