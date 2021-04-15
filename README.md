# Lecture 8: Systems Programming

## Reminders (April 8)

- Homework 3 final due date is tomorrow.

    - Read Piazza threads!

    - Remember to run `cargo clippy` and `cargo fmt`.

    - Make sure to `.gitignore` the `/target` directory.

    - Email before the deadline to use late days.

- Connect with your final project team! (Form a group chat?) Team assignments are on Piazza.

- Homework 4 and final project (proposal + final) posted soon.

## Reminders (April 15)

- Homework 3 grading is underway. Out of 130. Common mistakes today if time.

    Comments are in the dedicated git pull request

- Final project proposal due **tomorrow April 16**

    I will not delay the deadline for this, but I will allow this to be more interactive, i.e. you first post your idea / what you are thinking about, I can comment on it and you will have a chance to update it. But you need to show that you have thought about / are working on it

    (Use the GitHub classroom link to create a team, and then join the appropriate team)

- Homework 4 available and due **Thursday, April 22.**

    `strace` utility (with extra credit)

- Homework make-up option (choose 1 assignment)

- Only 1-2 more lectures after today -- TBD topic preferences. Options:

    1. Concurrent and parallel programming

    2. Useful crates and tools, e.g. web programming

    3. Advanced features and future of Rust

        - async/await related features
        - const generics

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

### Optional additional material (didn't get to in lecture)

- Manual memory management (the `Drop` trait, `mem::replace`)

- HW3 common mistakes
