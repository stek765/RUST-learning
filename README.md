# 📘 RUST-learning

Welcome to my Rust learning journey!  
This repository contains projects and exercises inspired by the official book **"The Rust Programming Language"** (aka *The Book*).

---

## 📚 Chapters Overview

| Chapter                          | Folder Name              | Description |
|----------------------------------|---------------------------|-------------|
| 1. Getting Started               | `1) hello_cargo`          | A simple “Hello, world!” to get familiar with `cargo`. |
| 2. Programming a Guessing Game  | `2) guessing_game`        | CLI game using random numbers and user input. |
| 3. Common Programming Concepts  | `3) basics`               | Variables, functions, loops, ownership basics, etc. |
| 4. Ownership                    | `4) ownership`            | Rust’s unique memory model and the ownership rules. |
| 5. Structs                      | `5) structs`              | Creating and using your own data types. |
| 6. Enums and Pattern Matching   | `6) enums`                | Handling data with `enum` and `match`. |
| 7. Managing Growing Projects    | `7) modules_project`      | Organizing code into modules and multiple files. |
| 8. Collections                  | `8) collections`          | Vectors, strings, hash maps and more. |
| 9. Error Handling               | `9) error_handling`       | Using `Result`, `panic!`, and managing errors. |
| 10. Generics, Traits, Lifetimes | `10) generics_traits`     | Writing flexible and reusable code. |
| 11. Testing                     | `11) testing`             | Unit tests and integration tests. |
| 12. Building a CLI App          | `12) in_out_project`      | Recreating a simplified version of the `grep` command. |

---

## ▶️ Running the CLI App (`12) in_out_project`)

This chapter's project replicates a basic version of the `grep` terminal command.

### Run the program:

```bash
cargo run -- to_find poem.txt
```

Replace `to_find` with the word you're looking for, and `poem.txt` with the path to your file.

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) & `cargo`

---

## 🧠 Notes

This is a personal learning repo following *The Book*. Feel free to explore or fork if you're learning too!

---

## 📖 References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)