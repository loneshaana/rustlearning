# Data Structures In Rust

## Table of Contents

### 1. [Introduction](#introduction)
### 1.1 [What is Rust?](#what-is-rust)
Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without garbage collection, making it a popular choice for performance-critical applications.

### 1.2 [Why Rust?](#why-rust)
Rust is known for its strong emphasis on safety and performance. It prevents null pointer dereferencing, buffer overflows, and data races, making it a reliable choice for developing secure and efficient software.

### 1.3 [Setting Up Rust](#setting-up-rust)
To set up Rust, you need to install `rustup`, the Rust toolchain installer. Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to get started.

### 1.4 [Hello World in Rust](#hello-world-in-rust)
Creating a "Hello, World!" program in Rust is simple. Create a new file with a `.rs` extension and add the following code:
```rust
fn main() {
    println!("Hello, World!");
}
```
Compile and run the program using `rustc` and the executable file generated.
### 2. [Primitive Data Types](#primitive-data-types)
### 2.1 [Integers](#integers)
Rust provides several integer types, each with a specific size. The integer types are signed (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) and unsigned (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`). The size of `isize` and `usize` depends on the architecture of the machine (32-bit or 64-bit).

### 2.2 [Floating-Point Numbers](#floating-point-numbers)
Rust supports two floating-point types: `f32` and `f64`. These types follow the IEEE-754 standard for floating-point arithmetic.

### 2.3 [Booleans](#booleans)
The boolean type in Rust is `bool`, which can have one of two values: `true` or `false`.

### 2.4 [Characters](#characters)
The `char` type in Rust represents a single Unicode scalar value. It is four bytes in size and can represent a wide range of characters, including alphabets, numerals, and special symbols.

### 2.5 [Tuples](#tuples)
Tuples in Rust are a way to group multiple values of different types into a single compound type. Tuples have a fixed length and can contain elements of different types.

### 2.6 [Arrays](#arrays)
Arrays in Rust are collections of elements of the same type. Arrays have a fixed length, and their size must be known at compile time. Arrays are defined using square brackets, with the type of elements and the number of elements specified.
### 3. [Compound Data Types](#compound-data-types)
### 3.1 [Structs](#structs)
Structs in Rust are used to create custom data types that group related values. There are three types of structs: classic structs, tuple structs, and unit structs.

#### Classic Structs
Classic structs have named fields. Here's an example:
```rust
struct Point {
    x: i32,
    y: i32,
}
```

#### Tuple Structs
Tuple structs have unnamed fields. Here's an example:
```rust
struct Color(i32, i32, i32);
```

#### Unit Structs
Unit structs don't have any fields. They are useful for generics. Here's an example:
```rust
struct Unit;
```

### 3.2 [Enums](#enums)
Enums in Rust are used to define a type by enumerating its possible values. Each variant of an enum can optionally have associated data.

Here's an example:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### 3.3 [Option](#option)
The `Option` type is used when a value can be either something or nothing. It is defined by the standard library as:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

### 3.4 [Result](#result)
The `Result` type is used for error handling and represents either success (`Ok`) or failure (`Err`). It is defined by the standard library as:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
### 4. [Custom Data Types](#custom-data-types)
### 4. [Custom Data Types](#custom-data-types)

Rust allows you to define custom data types using structs, enums, and type aliases. These custom types help you model your data more accurately and make your code more readable and maintainable.

#### 4.1 [Type Aliases](#type-aliases)
Type aliases allow you to create a new name for an existing type. This can make complex types easier to work with and improve code readability. Here's an example:
```rust
type Kilometers = i32;

let distance: Kilometers = 5;
```

#### 4.2 [Newtype Pattern](#newtype-pattern)
The newtype pattern involves creating a tuple struct with a single element to give a meaningful name to a type. This can be useful for type safety and clarity. Here's an example:
```rust
struct UserId(i32);

fn get_user_id() -> UserId {
    UserId(42)
}
```

#### 4.3 [Phantom Types](#phantom-types)
Phantom types are used to create types that carry extra compile-time information without affecting runtime behavior. This is achieved using the `PhantomData` marker. Here's an example:
```rust
use std::marker::PhantomData;

struct PhantomStruct<T> {
    _marker: PhantomData<T>,
}

let _phantom: PhantomStruct<i32> = PhantomStruct { _marker: PhantomData };
```

These custom data types allow you to create more expressive and type-safe Rust programs.
### 5. [Data Structures](#data-structures)
### 5.1 [Vectors](#vectors)
Vectors are resizable arrays in Rust. They are defined using the `Vec<T>` type, where `T` is the type of elements in the vector. Vectors can grow and shrink in size at runtime.

Here's an example:
```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
v.push(3);
```

### 5.2 [HashMaps](#hashmaps)
HashMaps in Rust are collections of key-value pairs. They are defined using the `HashMap<K, V>` type, where `K` is the type of keys and `V` is the type of values.

Here's an example:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### 5.3 [HashSets](#hashsets)
HashSets are collections of unique values. They are defined using the `HashSet<T>` type, where `T` is the type of elements in the set.

Here's an example:
```rust
use std::collections::HashSet;

let mut books = HashSet::new();
books.insert("The Catcher in the Rye");
books.insert("To Kill a Mockingbird");
```

### 5.4 [LinkedLists](#linkedlists)
LinkedLists are collections of elements arranged in a linear order, where each element points to the next. They are defined using the `LinkedList<T>` type, where `T` is the type of elements in the list.

Here's an example:
```rust
use std::collections::LinkedList;

let mut list: LinkedList<i32> = LinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);
```

### 5.5 [BinaryHeap](#binaryheap)
BinaryHeap is a priority queue implemented with a binary heap. It is defined using the `BinaryHeap<T>` type, where `T` is the type of elements in the heap.

Here's an example:
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(4);
heap.push(1);
heap.push(7);
```

### 5.6 [BTreeMap](#btreemap)
BTreeMap is an ordered map based on a B-Tree. It is defined using the `BTreeMap<K, V>` type, where `K` is the type of keys and `V` is the type of values.

Here's an example:
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(1, "a");
map.insert(2, "b");
map.insert(3, "c");
```

### 5.7 [BTreeSet](#btreeset)
BTreeSet is an ordered set based on a B-Tree. It is defined using the `BTreeSet<T>` type, where `T` is the type of elements in the set.

Here's an example:
```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(1);
set.insert(2);
set.insert(3);
```
### 6. [Algorithms](#algorithms)
### 6. [Algorithms](#algorithms)

### 6.1 [Sorting](#sorting)
Rust provides several sorting algorithms in the standard library. The most commonly used is the `sort` method, which sorts a vector in place.

Here's an example:
```rust
let mut vec = vec![5, 3, 1, 4, 2];
vec.sort();
```

### 6.2 [Searching](#searching)
Rust provides methods for searching within slices. The `binary_search` method performs a binary search on a sorted slice.

Here's an example:
```rust
let slice = [1, 2, 3, 4, 5];
let result = slice.binary_search(&3);
```

### 6.3 [Iterators](#iterators)
Iterators in Rust are used to process sequences of elements. The `Iterator` trait provides various methods for transforming and consuming iterators.

Here's an example:
```rust
let vec = vec![1, 2, 3, 4, 5];
let sum: i32 = vec.iter().sum();
```

### 6.4 [Recursion](#recursion)
Rust supports recursive functions, which are functions that call themselves. Recursion is useful for solving problems that can be broken down into smaller subproblems.

Here's an example:
```rust
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

### 6.5 [Dynamic Programming](#dynamic-programming)
Dynamic programming is a technique for solving problems by breaking them down into simpler subproblems and storing the results of subproblems to avoid redundant computations.

Here's an example of the Fibonacci sequence using dynamic programming:
```rust
fn fibonacci(n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}
```
