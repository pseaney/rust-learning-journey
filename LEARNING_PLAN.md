This issue tracks my progress through intermediate and advanced Rust topics. Each item links to examples or notes in this repo.

### ✅ Core Concepts
- [x] Ownership, Borrowing, Structs, Enums
- [x] Traits and Trait Objects
- [x] Unit and Integration Testing
- [x] Modular Crate Design

---

### 🚀 Intermediate to Advanced Topics

#### 🔹 Smart Pointers and Ownership Patterns
- [ ] `Box<T>` and heap allocation
- [ ] `Rc<T>` and shared ownership
- [ ] `RefCell<T>` and interior mutability
- [ ] Build a singly linked list with `Box`
- [ ] Refactor with `Rc` and `RefCell`

#### 🔹 Advanced Traits and Trait Objects
- [x] Trait bounds with generics
- [x] Returning `impl Trait` vs `Box<dyn Trait>`
- [ ] Trait inheritance and default methods
- [ ] Object safety and dynamic dispatch

#### 🔹 Macros and Code Generation
- [ ] `macro_rules!` basics
- [ ] Repetition, pattern matching, hygiene
- [ ] Procedural macros and `#[derive]`

#### 🔹 Concurrency and Parallelism
- [ ] `std::thread`, `Mutex`, `Arc`
- [ ] Channels and message passing
- [ ] `async/await` with `tokio` or `async-std`

#### 🔹 Error Handling and Result Composition
- [ ] `Result`, `Option`, and `?`
- [ ] Combinators: `map`, `and_then`, `unwrap_or_else`
- [ ] Custom error types with `thiserror` or `anyhow`

#### 🔹 Crate Design and Workspace Management
- [x] Multi-crate workspace setup
- [ ] Re-exports and visibility (`pub use`, `pub(crate)`)
- [ ] Publishing to crates.io

#### 🔹 Real-World Projects
- [ ] CLI tool with `clap`
- [ ] REST API with `axum` or `actix-web`
- [ ] Async file downloader or task runner
- [ ] Capstone project (TBD)