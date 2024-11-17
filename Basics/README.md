
# Rust Cargo Folder Structure

This document provides an overview of the folder structure created by Cargo, Rust's build system and package manager.

---

## **Basic Folder Structure**

```
project_name/
├── Cargo.toml       # Metadata and dependencies
├── Cargo.lock       # Dependency versions (generated after the first build)
├── src/             # Source code directory
│   ├── main.rs      # Entry point for binary projects
│   └── lib.rs       # Entry point for library projects (optional)
├── target/          # Build output directory (generated automatically)
└── .gitignore       # Preconfigured for Rust projects
```

---

## **Detailed Breakdown**

### **1. Cargo.toml**
- This file is the **manifest** for your Rust project.
- Contains metadata like the project name, version, authors, and dependencies.
  
Example:
```toml
[package]
name = "project_name"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"    # Example dependency
```

---

### **2. Cargo.lock**
- This file is **automatically generated** after building the project for the first time.
- Locks the specific versions of dependencies to ensure consistent builds.

---

### **3. src/**
- Contains your Rust source code.
- For a **binary project**, the entry point is `main.rs`.
- For a **library project**, the entry point is `lib.rs`.

#### Structure examples:

1. **Binary Project**:
    ```
    src/
    └── main.rs
    ```
    Example:
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

2. **Library Project**:
    ```
    src/
    └── lib.rs
    ```
    Example:
    ```rust
    pub fn greet() {
        println!("Hello from the library!");
    }
    ```

---

### **4. target/**
- This directory is **created automatically** during the first build.
- Stores compiled binaries, libraries, and intermediate build files.
- Common subdirectories:
  - `debug/`: Contains debug builds.
  - `release/`: Contains optimized builds.

You can ignore this directory using `.gitignore` to avoid committing it to version control.

---

### **5. .gitignore**
- Automatically created if you have Git initialized.
- Preconfigured to ignore files and folders like:
  - `target/`
  - `.DS_Store`
  - Any build artifacts or temporary files.

Example:
```
/target
*.log
```

---

## **Optional Additions**

### **tests/**
- Contains integration tests.
- Each file in this directory is a separate test case.

Example:
```
tests/
└── integration_test.rs
```
Contents:
```rust
#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}
```

---

### **examples/**
- Contains example programs that demonstrate how to use your crate.

Example:
```
examples/
└── example_program.rs
```

---

### **benches/**
- Used for benchmarking with libraries like `criterion`.

---

## **Expanded Project Example**
For a more complex project:
```
project_name/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs        # For binary project
│   └── lib.rs         # For shared library (optional)
├── tests/             # Integration tests
│   └── integration_test.rs
├── examples/          # Example programs
│   └── example_program.rs
├── benches/           # Benchmarks
│   └── benchmark.rs
├── target/            # Build artifacts
└── .gitignore
```

---

## **Key Points**
1. Cargo standardizes project structure, making it easier for Rust developers to collaborate and understand codebases.
2. **Start simple** with `Cargo.toml` and `src/` and expand to include `tests/`, `examples/`, etc., as your project grows.
3. The `target/` folder can be safely ignored for version control.
