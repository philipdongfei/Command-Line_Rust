# Truth or Consequences

## Getting Started with "Hello, world!"


## Organizing a Rust Project Directory


## Creating and Running a Project with Cargo


## Writing and Running Integration Tests

*Outside-in* or *integration testing* is when you write tests that run your programs as the user might, and that's what we'll do for this program.

### Adding a Project Dependency

### Understanding Program Exit Values

The Portable Operating System Interface (POSIX) standards dictate that the standard exit code is 0 to indicate success (think *zero* errors) and any number from 1 to 255 otherwise. 

The tests are not necessarily run in the same order they are declared in the code. This is because Rust is a safe language for writing *concurrent* code, which means code can be run across multiple threads. The testing takes advantage of this concurrency to run many tests in parallel, so the test results may appear in a different order each time you run them. This is a feature, not a bug. If you would like to run the tests in order, you can run them on a single thead via **cargo test \-\- \-\-test\-threads=1**

### Testing the Program Output

Learning to read test output is a skill in itself and takes practice. The preceding test result is trying very hard to show you how the *expected* output differs from the *actual* output. While this is a trivial program, I hope you can see the value in automatically checking all aspects of the programs we write.

### Exit Values Make Programs Composable



## Summary

This chapter introduced you to some key ideas about organizing a Rust project and some basic ideas about command-line programs. To recap:

- The Rust compiler **rustc** compiles Rust source code into a machine-executable file on Windows, macOS, and Linux.
- The Cargo tool helps create a new Rust project and also compiles, runs, and tests the code.
- Command-line tools like **ls**, **cd**, **mkdir**, and **rm** often accept command-line arguments like file or directory names as well as options like **\-f** or **\-p**.
- POSIX-compatible programs should exit with a value of 0 to indicate success and any value between 1 and 255 to indicate an error.
- By default, **cargo new** creates a new Rust program that prints "Hello, world!"
- You learned to add crate dependencies to Cargo.toml and use the crates in your code.
- You created a *tests* directory to organize testing code, and you use **\#\[test\]** to mark functions that should be execuated as tests.
- You learned how to test a program's exit status as well as how to check the text printed to STDOUT.
- You learned how to write, run, and test alternate binaries in a Cargo project by creating source code files in the *src/bin* directory.
- You wrote your own implementations of the **true** and **false** programs along with tests to verify that they succeed and fail as expected. You saw that by default a Rust program will exit with the value zero and that the **std::process::exit** function can be used to explicitly exit with a given code. Additionally, the **std::process::abort** function can be used to exit with a nonzero error code.

