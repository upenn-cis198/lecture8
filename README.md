# Lecture 8: Systems Programming

## Reminders

- Homework 3 final due date is tomorrow.

    - Read Piazza threads!

    - Remember to run `cargo clippy` and `cargo fmt`.

    - Make sure to `.gitignore` the `/target` directory.

    - Email before the deadline to use late days.

- Connect with your final project team! (Form a group chat?) Team assignments are on Piazza.

- Homework 4 and final project (proposal + final) posted soon.

## Outline

Sometimes in Rust we want really low-level control over how our program runs.
Examples:

- Manipulating and passing around direct references to memory (pointers);

- Making system calls;

- Interfacing with C code;

- Dealing with low-level IO (e.g. file handles and pipes; network hosts, ports, and sockets); and

- Spawning and handling threads and processes.

In this lecture we will go over some of the things in the Rust toolbox for these programming tasks, which will be useful for Homework 4.
Like the last few, this lecture will be split into multiple parts.

### Part 1

- Unsafe code (`unsafe` blocks and functions)

- Introduction to smart pointer types: `IDManger` case study

### Part 2

- C code wrappers and system calls (`libc`, `nix`)

- Smart pointer types continued (`Box`, `Cell`, `Rc`, `RefCell`, etc.)

- Manual memory management (the `Drop` trait, `mem::replace`)
