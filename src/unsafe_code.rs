use std::fs::File;
use std::os::unix::io::FromRawFd;

// Why do we need unsafe blocks in Rust.
// 1) We may want to avoid redundant checks (iterators use this).
// 2) We may be talking to foreign code which Rust cannot trust to be safe.
// 3) It is difficult (impossible?) to implement certain data structures in Rust:
//    e.g. doubly linked lists.

// Unsafe block.

fn unsafe_block(){
    unsafe {
        // DO unsafe things here.

    }

    // Can return a value
    let file = unsafe {
        // unsafe fn from_raw_fd(fd: RawFd) -> File
        File::from_raw_fd(1)
    };
}

// Building safe abstractions.
// A trait, or type could use unsafe, but have a safe API, unsafe_block() function
// above is an example of this.
fn call_unsafe_bloc(){
    unsafe_block();
}

// So you may wrap your own unsafe code in safe abstractions for consumption.


// Why are some functions unsafe?
// Unsafe functions: the compiler does not check some invariant or contract
// that the function expects to be true, not meeting this may lead to memory
// errors, or undefined behavior.

// Either we don't want to check this invariant (runtime cost) or we cannot check
// some invariant.


// Undefined behavior
// Where does undefined behavior come from?
// UB is a language level idea, language semantics defined expected behavior,
// for some code. Some actions are defined as undefined behavior:
// e.g.
// Using a freed pointer,
// Overflowing an integer,
// Reading past the end of an array.

// Why do we allow undefined behavior in languages?

// Rust must trust:

// From book:
fn normal_code() {
    let i = 10;
    trusted_function(&i);
    println!("{}", i * 100);
    return;

    // Compiler can optimize this code! To
    // trusted_function(&10);
    // println!("{}", 1000);
}

fn trusted_function(shared: &i32) {
    unsafe {
        let mutable = shared as *const i32 as *mut i32;
        *mutable = 20;
    }
}

// This breaks the optimization rust did! Code no longer has the same meaning
// before and after the calls!

// Almost all optmizations could lead to cases like this... So we either chose never
// to optimize code, or live knowning undefined behavior could mess up our programs...
// So Rust assumes a program is well behaved.


// What else can we do in unsafe blocks:

// 1. Call unsafe functions
// 2. dereference raw pointers
// 3. Access static mut global variables.
// 4. Call FFI Functions.

// Notice it doesn't "turn off" the borrow checker, or any other safety check for Rust.

// Raw Pointers. Basically a C or C++ pointer.
// Pointers can be null.
fn raw_pointers() {
    let x: i64 = 0xFFFF32adf;
    let pr: &i64 = &x;

    // Raw address 0xFFFF32adf is now being pointed to.
    let px: *mut i64 = x as *mut i64;
    // Get a pointer to x.
    let pxx = pr as *const i64;

    // Can only dereference pointes in unsafe blocks:
    unsafe {
        *pxx;
    }

    // You must always exclitly dereference raw pointers.
    // No pointer arithmetic operators. methods offset and wrappering_offset.
}

// Useful fuctions:
// null and null_mut
// https://doc.rust-lang.org/std/ptr/fn.null_mut.html

// Imagine you want to call time(2):
// https://docs.rs/libc/0.2.43/libc/fn.time.html
type time_t = i64;
unsafe fn time(time: *mut time_t) -> time_t {
    unimplemented!()
}

use std::ptr::null_mut;

fn call_time(){
    let t = unsafe {
        time(null_mut())
    };
}

use std::mem::size_of;

fn sizeof_operator() {
    // let t = (3, 3i8);
    let size = size_of::<(i32, i8)>();
}

// Talk about linking.
// Talk about static vs shared objects.
// Show example of linking in C?
// nm
// ldd

// What is glibc.
// Rust: libc
// Rust: Nix
// Talk about system calls.

// FFI
// Most languages interface through C.

// We must be able to establish a correspondence to C code.
// Check out std::os::raw

// Show example of linking Rust code to C:

// Imagine you have a nice rust function like this:
// How do you call it from C?
// fn fizz_buzz(){
//     for i in 0..100{
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("{}: fizzbuzz", i);
//         }
//         if i % 3 == 0 {
//             println!("{}: fizz", i);
//         }
//         if i % 5 == 0 {
//             println!("{}: buzz", i);
//         }
//     }

//     return;
// }

// Statically link Rust code?
