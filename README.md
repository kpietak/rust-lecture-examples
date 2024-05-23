# Examples for Rust lecture

### Set up and run

To run the examples follow the steps:
1. Install `rustup` environment (if not set up yet), see https://www.rust-lang.org/learn/get-started for more details.
1. Clone or download the repository.
1. Enter the main directory:
  ```
  cd rust-lecture-examples
  ```
1. Run a chosen example:
  ```
  cargo run --example <example_name>
  ```
1. If an example contains unit tests, you can run them using:
  ```
  cargo test --example <example_name> -- --nocapture
  ```
  The `nocapture` option allows to print test output (written using `println!` macro) to the terminal. 
1. If you want to run particular test specify its name:
  ```
  cargo test --example <example_name> tests::<test_name> -- --nocapture
  ```
  for example:
  ```
  cargo test --example smart-pointers tests::refcell_interior_mutability -- --nocapture
  ```

### Content

The complete list of examples you will find in `examples` directory. 

Here's the (incomplete yet) description of the examples:

- `smart-pointers.rs` illustrates how to use types such as `Box`, `Rc` and `RefCell`; 
- `static-dispatch-benchamrk.rs` and `dynamic-dispatch-benchmark.rs` provides a simple benchmark comparing static and dynamic dispatch; to see the proper result run them with:
  ```
  cargo run --example static-dispatch-benchmark.rs --release
  ```



