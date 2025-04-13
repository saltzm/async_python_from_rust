# Go Example for async_rust_from_python

This is a Go implementation similar to the Python example, demonstrating how to call the Rust async library from Go.

## Prerequisites

1. Install Go: https://golang.org/doc/install
2. Make sure the Rust library is compiled:
   ```bash
   cargo build
   ```

## Files

- `my_library.go`: Go wrapper for the Rust library
- `main.go`: Example program that uses the wrapper

## Running the example

1. Make sure the Rust library is built:
   ```bash
   cargo build
   ```

2. Run the Go program:
   ```bash
   go run main.go my_library.go
   ```

## How it works

This Go example demonstrates:

1. CGO bindings to the Rust library
2. Concurrent operation execution using goroutines
3. Sequential operation execution
4. Proper resource cleanup using finalizers

The implementation uses Go's concurrency primitives (goroutines, channels, WaitGroup) to provide a clean API over the Rust library.

## Potential issues

- If you get CGO errors, make sure the Rust library is properly built and the dynamic library is accessible in the path specified in the CGO directives.
- On macOS, you might need to adjust the library path and name in the CGO directive in `my_library.go`:
  ```go
  // #cgo LDFLAGS: -L./target/debug -lasync_rust_from_python
  ```
- On Linux or Windows, you may need to adjust the library name as appropriate. 