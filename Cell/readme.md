## Cell

### shareable mutable container.

Rust memory safety is based on the rule: Given an Object T, it is possible to have one of the following
- Several immutable references(&T) to the object (also know aliasing)
- one mutable reference(&mut T) to the object (also known as mutability)


This is enforced by the Rust compiler. However, there are situtations where this rule is not flexible enough. Sometimes it is required to have multiple references to an object and yet mutate it.

Shareable mutable containers exist to permit mutability in a controlled manner, even in the presence of aliasing. Cell<T> , RefCell<T> and oNceCell<T> allow doing this in a single-threaded way- they do not implement Sync. 
If you need to do aliasing and mutation among multiple threads, Mutex<T>, RwLock<T>, onceLock<T> or atomic types are the correct data structures to do so.

### Cell<T> implements interior mutablity by moving values in and out of the cell. That is an &mut T to the inner value can never be obtained, and the value itself cannot be directly obtained without replacing it with something else. Both of these rules ensure that there is never more than one referecen pointing to the inner value.


## How to compile
cargo +nightly check