
# Rust Common Programming Concepts

This document provides an overview of Rust's fundamental programming concepts, highlighting features that make Rust unique.

---

## **1. Variables and Mutability**
By default, variables in Rust are **immutable**. To make a variable mutable, use the `mut` keyword.

```rust
fn main() {
    let x = 5; // Immutable
    println!("x = {}", x);

    let mut y = 10; // Mutable
    y += 5;
    println!("y = {}", y);
}
```

## **`const`**

### Definition:
- `const` is used to define **constants** that are always immutable and known at compile time.

### Key Features:
1. **Immutable by Default**: Cannot be changed after declaration.
2. **Global or Scoped**: Can be declared in global or block scope.
3. **Compile-time Requirement**: The value must be a compile-time constant.
4. **Type Annotation Required**: Explicit type annotation is mandatory.

### Example:
```rust
const MAX_SCORE: u32 = 100; // Compile-time constant

fn main() {
    println!("Max Score: {}", MAX_SCORE);
    // MAX_SCORE = 200; // Error: cannot assign to a constant
}
```

### Use Case:
- Use `const` for fixed values that don’t change during the program's lifecycle, such as mathematical constants or configuration values.

---

## **Shadowing**

### Definition:
- Shadowing allows you to **redeclare a variable** with the same name, effectively creating a new variable that "shadows" the previous one.

### Key Features:
1. **Does Not Mutate**: Unlike `mut`, shadowing creates a new variable without mutating the original.
2. **Allows Type Changes**: The new variable can have a different type.
3. **Scoped**: The shadowed variable is valid only in its block.

### Examples:

#### Basic Example:
```rust
fn main() {
    let x = 5; // Immutable variable
    let x = x + 1; // Shadowing (creates a new variable)
    let x = x * 2; // Shadowing again
    println!("Final value of x: {}", x); // Output: 12
}
```

#### Type Change with Shadowing:
```rust
fn main() {
    let spaces = "   "; // String type
    let spaces = spaces.len(); // Integer type (shadowing changes type)
    println!("Spaces length: {}", spaces); // Output: 3
}
```

### Use Case:
- Use shadowing to transform or refine a variable without mutating it, ensuring immutability while allowing reassignments.

---

## **Comparison: `const` vs. Shadowing**

| Feature                | `const`                           | Shadowing                       |
|------------------------|------------------------------------|---------------------------------|
| **Mutability**         | Always immutable                  | Creates new variables, not mutable |
| **Scope**              | Global or block                   | Block-scoped                   |
| **Type Change**        | Not allowed                       | Allowed during shadowing       |
| **Initialization**     | Compile-time constant required    | Value can be computed at runtime |
| **Usage**              | Fixed values                     | Transform or refine variable values |

---

## **Key Points**

- **`const`** is for defining **compile-time constants**.
- **Shadowing** allows redeclaring variables for **runtime transformations** or **type changes**.




---

## **2. Data Types**

### Scalar Types:
- **Integers:** `i32`, `u32`, etc.
- **Floating-point:** `f32`, `f64`
- **Boolean:** `bool`
- **Character:** `char`

```rust
let int: i32 = 42;
let float: f64 = 3.14;
let is_true: bool = true;
let letter: char = 'R';
```

### Compound Types:
- **Tuples:**
  ```rust
  let tup: (i32, f64, char) = (42, 3.14, 'R');
  let (x, y, z) = tup; // Destructuring
  ```
- **Arrays:**
  ```rust
  let arr = [1, 2, 3, 4, 5];
  println!("First element: {}", arr[0]);
  ```

---

## **3. Ownership and Borrowing**

### Ownership:
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved
    // println!("{}", s1); // Error: s1 is no longer valid
}
```

### Borrowing:
```rust
fn main() {
    let s = String::from("hello");
    print_string(&s); // Borrowing
    println!("{}", s); // Still valid
}

fn print_string(s: &String) {
    println!("{}", s);
}
```

---

## **4. Functions**
Functions in Rust use snake_case naming and require explicit return types for non-unit values.

```rust
fn main() {
    let sum = add(5, 10);
    println!("Sum: {}", sum);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // No semicolon = return value
}
```

---

## **5. Control Flow**

### Conditional Statements:
```rust
fn main() {
    let number = 5;

    if number > 0 {
        println!("Positive");
    } else {
        println!("Non-positive");
    }
}
```

### Loops:
```rust
loop {
    println!("Loop forever!");
    break; // Exit loop
}

let mut x = 0;
while x < 5 {
    println!("{}", x);
    x += 1;
}

for i in 0..5 {
    println!("{}", i);
}
```

---

## **6. Pattern Matching**
Rust’s `match` is a powerful control flow construct.

```rust
fn main() {
    let number = 7;

    match number {
        1 => println!("One"),
        2..=5 => println!("Between 2 and 5"),
        _ => println!("Something else"),
    }
}
```

---

## **7. Structs and Enums**

### Structs:
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point({}, {})", p.x, p.y);
}
```

### Enums:
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        _ => println!("Other direction"),
    }
}
```

---

## **8. Collections**

### Vectors (`Vec`):
```rust
let mut vec = vec![1, 2, 3];
vec.push(4);
println!("{:?}", vec);
```

### HashMap:
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", "value");
println!("{:?}", map);
```

---

## **9. Error Handling**

### Result:
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### Panic:
```rust
fn main() {
    panic!("Something went wrong!");
}
```

---

## **10. Iterators and Closures**

### Iterators:
```rust
let numbers = vec![1, 2, 3];
let sum: i32 = numbers.iter().sum();
println!("Sum: {}", sum);
```

### Closures:
```rust
let add = |a, b| a + b;
println!("{}", add(5, 10));
```

---

## **11. Traits**
Traits define shared behavior, similar to interfaces.

```rust
trait Greet {
    fn greet(&self);
}

struct Person;

impl Greet for Person {
    fn greet(&self) {
        println!("Hello!");
    }
}

fn main() {
    let p = Person;
    p.greet();
}
```

---

## **12. Concurrency**

### Threads:
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

---

This document summarizes the most common concepts in Rust programming.
