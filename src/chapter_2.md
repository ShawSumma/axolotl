# Ownership

## Similarities to Rust Ownership
Like rust axolotl has a concept of ownership, where one object processes the rights to a given resource, however in order to maintain simplicity unlike rust references, any reference in axolotl is one that cannot be stored in any way. By doing this the complexity of the borrow checker is kept at a minimum, while still allowing references to be used to increase performance. 

## Scopes
Each function, and builtin command posses its own scope, there is also a global scope. A borrow can only occur  in one direction, principally from a higher scope, to a child scope. 

### Scope Termination
whenever an object goes out of scope its complete method is called, however no memory management needs to be performed in the complete method, as each scope is operated as an arena, and can thus be block deallocated once each complete method is called. The complete methods exist not as a means of mamnaging memory but as a way to close file descriptors.

## Mutability

rust's philosophy on mutability is used, principally that 