# Lecture 8: Systems Programming

## Reminders

- Homework 3 final due date is tomorrow.

    - Read Piazza threads!

    - Remember to run `cargo clippy` and `cargo fmt`.

    - Make sure to `.gitignore` the `/target` directory.

    - Email before the deadline to use late days.

- Connect with your final project team! (Form a group chat?) Team assignments are on Piazza.

- Homework 4 and final project (proposal + final) posted soon.

## Outline of lecture

Sometimes in Rust we want really low-level control over how our program runs.
Examples:

- Manipulating and passing around direct references to memory (pointers);

- Making system calls;

- Interfacing with C code;

- Dealing with low-level IO (e.g. file handles and pipes; network hosts, ports, and sockets); and

- Spawning and handling threads and processes.

Today we will go over some of the things in the Rust toolbox for these programming tasks, which will be useful for Homework 4:

- Unsafe code;

- Smart pointer types; and (if time)

- C code wrappers and system calls.
