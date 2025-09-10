# 🦀 rust-todo-cli

A simple, fast, and minimal command-line TODO app built with Rust.  
Stores tasks in a plain text file – no database, no setup required.

---

## 📦 Features

-  Add, list, and delete TODOs
-  Uses only a `todo.txt` file for storage
-  Super fast and portable – written in Rust
-  No external dependencies

---

## 🚀 Installation

### 1. Clone the repository

```bash
git clone https://github.com/your-username/rust-todo-cli.git
cd rust-todo-cli
```
### 2. Build the project

```bash
cargo build --release
```

---

### Usage

```bash
./target/release/rust-todo-cli <command> [arguments...]
```

###Commands

| Command          | Description                    | Example                             |
| ---------------- | ------------------------------ | ----------------------------------- |
| `add <task>`     | Add a new task                 | `rust-todo-cli add "Buy groceries"` |
| `list`           | List all tasks                 | `rust-todo-cli list`                |
| `delete <index>` | Delete task by index (1-based) | `rust-todo-cli delete 2`            |
| `done <index>`   | Complete task by index (1-based) | `rust-todo-cli done 2`            |

---

## 📁 How It Works

- All tasks are stored in a simple text file named `todo.txt` in the current working directory.
- Each line in the file represents one task.

### Example `todo.txt`
```
Buy groceries
Complete Rust project
Call Alice
```

## Example work flow

```
$ rust-todo-cli add "Write blog post"
Task added!

$ rust-todo-cli list
1. Buy groceries
2. Complete Rust project
3. Call Alice
4. Write blog post

$ rust-todo-cli delete 2
Task "Complete Rust project" deleted!

```


