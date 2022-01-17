# Rust panics in print-macros when Linux process looses terminal

## Reproduce

Run this in debugger.

Go to terminal of debugger.

Pause execution (CTRL + Z).

`bg` to run it in background

Close the terminal e.g. `exit` or STRG + D

The debugger jumps immediately in a panic in `std::io::stdio::print_to`.

## But 

`write!(std::io::stdout(), "Hello world!\n").unwrap()` instead of `println!` works without panic.
