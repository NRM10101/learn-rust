
# Rust Control Flow

## 1. **if Expressions**
In Rust, `if` is used to execute code conditionally.

## Note
    1. Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code
    2. The if and else arms should have value types that are compatible.

### Syntax:
```rust
if condition {
    // Code
} else if another_condition {
    // Code
} else {
    // Code
}
```

### Example:
```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("Less than 5");
    } else if number == 5 {
        println!("Equal to 5");
    } else {
        println!("Greater than 5");
    }
}
```

---

## 2. **Loops**
Rust provides `loop`, `while`, and `for` loops for repeating code.

### Infinite Loop with `loop`:
```rust
loop {
    println!("This runs forever unless broken!");
    break; // Stops the loop
}
```
**break**:
    The `break` statement is used to exit a loop immediately. When break us encountered, the loop terminates immediately.
**continue**:
    The `continue` statement is used to skip the rest of the current iteration and proceed to the next iteration.When the `continue` statement is executed, the loop's current iteration end, and the next iteration begins.

### Conditional Loop with `while`:
```rust
fn main() {
    let mut number = 3;

    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}
```

### Iterative Loop with `for`:

we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
```

---

## 3. **Pattern Matching with `match`**
`match` is a powerful control flow construct that compares a value against multiple patterns.

### Syntax:
```rust
match value {
    pattern1 => { /* Code */ },
    pattern2 => { /* Code */ },
    _ => { /* Fallback Code */ },
}
```

### Example:
```rust
fn main() {
    let number = 6;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 => println!("A prime number"),
        4..=10 => println!("Between 4 and 10"),
        _ => println!("Something else"),
    }
}
```

---

## 4. **Using `if let`**
`if let` is a concise way to match a single pattern.

### Example:
```rust
fn main() {
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("Value is: {}", value);
    }
}
```

---

## 5. **Using `while let`**
`while let` runs a loop as long as a pattern matches.

### Example:
```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

---

## 6. **Returning Values from Loops**
Loops in Rust can return values using `break`.

### Example:
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Break with a value
        }
    };

    println!("The result is: {}", result);
}
```

---

## 7. **Labeled Breaks and Continues**
Labeled loops help control flow in nested loops.

### Example:
```rust
fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        loop {
            println!("Entered the inner loop");
            break 'outer; // Break the outer loop
        }

        println!("This will never be printed");
    }
}
```

---

## 8. **Match Guards**
`match` allows additional conditions using guards.

### Example:
```rust
fn main() {
    let number = Some(4);

    match number {
        Some(x) if x > 5 => println!("Greater than 5"),
        Some(x) => println!("Value: {}", x),
        None => println!("No value"),
    }
}
```

---

## 9. **Combining Control Flow with Iterators**
Rust loops work seamlessly with iterators.

### Example:
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    for num in numbers.iter().filter(|&&x| x % 2 == 0) {
        println!("Even number: {}", num);
    }
}
```

---

## 10. **Early Return with `return`**
Functions can exit early with `return`.

### Example:
```rust
fn main() {
    let x = 5;

    if x < 10 {
        return println!("Exiting early");
    }

    println!("This won't be printed if x < 10");
}
```

---
