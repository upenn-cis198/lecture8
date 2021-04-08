use std::fs::File;
use std::os::unix::io::FromRawFd;

// Why do we need unsafe blocks in Rust.
// 1) We may want to avoid redundant checks (iterators use this).
// 2) We may be talking to foreign code which Rust cannot trust to be safe.
// 3) It is difficult (but not impossible) to implement certain data structures
//    in Rust: e.g. doubly linked lists.
// Excellent reading:
//     https://rust-unofficial.github.io/too-many-lists/

/*
    Unsafe block
*/

pub fn unsafe_block() {
    let x = 3;
    unsafe {
        // DO unsafe things here.
        // make a pointer to T
        let x_ref: *const usize = &x;

        // Here: do whatever your heart desires...
        // E.g. convert the pointer to a mutable reference
        // and directly write to the memory,
        // even though x is immutable.

        // (Reading is unsafe too, not jus twriting)
        println!("{}", *x_ref);
    }

    // Unsafe functions are labeled as unsafe,
    // 'unsafe fn'
    // Must be called ONLY in unsafe blocks.

    // See for example the pointer API:
    // https://doc.rust-lang.org/std/primitive.pointer.html
    // Making a raw pointer is fine
    // Modifying/accessing the memory at the pointer is highly unsafe.

    // Can return a value
    let _file = unsafe {
        // pub unsafe fn from_raw_fd(fd: RawFd) -> File
        File::from_raw_fd(1)
    };

    // Unsafe pointer/memory access is dangerous!
    // Most things in memory get moved around, for example:
    let mut v = vec![100, 100, 100];
    let _v_start_ref: *const usize = &v[0];
    for i in 0..1000000 {
        // At some point, v may run out of space
        // and internally be moved somewhere else in memory
        // when calling .push()
        // If that occurs, my v_start_ref becomes invalid
        // (basically it points to garbage)
        v.push(i);
    }

    // The above compiles because v_start_ref is not ever accessed.
    // If I try to *use* v_start_ref, that's unsafe.
}

/// Documentation
/// # Safety
/// This function is only safe if the caller ensures ....
pub unsafe fn unsafe_block_unsafe() {
    // Note: adding 'unsafe fn' implicitly adds unsafe { ... } around the function
    // body, don't also need to mark the body as unsafe

    let mut x = 3;
    let x_ptr: *mut usize = &mut x;
    *x_ptr = 4;

    // ...do something bad...
    // that I can't guarantee internally is correct
}

// Building safe abstractions.
// A trait, or type could use unsafe, but have a safe API, unsafe_block() function
// above is an example of this.
pub fn call_unsafe_bloc() {
    // Not unsafe!
    unsafe_block();

    // Error: unsafe_block_unsafe();
    //        ^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    // unsafe_block_unsafe();

    unsafe {
        // I know what I'm doing
        unsafe_block_unsafe();
    }
}

// So you may wrap your own unsafe code in safe abstractions for consumption.

// Why are some functions unsafe?
// Unsafe functions: the compiler does not check some invariant or contract
// that the function expects to be true, not meeting this may lead to memory
// errors, or undefined behavior.

// Either we don't want to check this invariant (runtime cost) or we cannot check
// some invariant.

// Good example: C-style raw union type
// https://doc.rust-lang.org/reference/items/unions.html
// Like an enum, but doesn't come with a label about which enum variant it is,
// doesn't have 'match', and doesn't involve a runtime check every time
// it's accessed to figure out which enum variant it is.
// SO: accessing a union is unsafe.

// Undefined behavior
// Where does undefined behavior come from?
// UB is a language level idea, language semantics defined expected behavior,
// for some code. Some actions are defined as undefined behavior:
// e.g.
// Using a freed pointer,
// Overflowing an integer,
// Reading past the end of an array.
// Example undefined behavior:
// - convert a random integer, say 1005003 to a pointer *mut usize,
// - read and write to the memory at that pointer.

// Why do we allow undefined behavior in languages?

// Rust's idea: undefined behavior should only be in unsafe code.

/*
    ========== INTERLUDE: ID MANAGER CASE STUDY ==========
    (Homework 3 challenging aspects)

    ========== End of Lecture 8 Part 1 ==========
*/

// Rust must trust:

// From book:
pub fn normal_code() {
    let i = 10;
    trusted_function(&i);
    println!("{}", i * 100);

    // Compiler can optimize this code! To
    // trusted_function(&10);
    // println!("{}", 1000);
}

pub fn trusted_function(shared: &i32) {
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
pub fn raw_pointers() {
    let x: i64 = 0xFFFF32ADF;
    let pr: &i64 = &x;

    // Raw address 0xFFFF32adf is now being pointed to.
    let px: *mut i64 = x as *mut i64;
    // Get a pointer to x.
    let pxx = pr as *const i64;

    // Can only dereference pointes in unsafe blocks:
    unsafe {
        println!("{}", *px);
        println!("{}", *pxx);
    }

    // You must always exclitly dereference raw pointers.
    // No pointer arithmetic operators. methods offset and wrappering_offset.
}

// Useful fuctions:
// null and null_mut
// https://doc.rust-lang.org/std/ptr/fn.null_mut.html

// Imagine you want to call time(2):
// https://docs.rs/libc/0.2.43/libc/fn.time.html

type TimeT = i64;

/// Documentation
/// # Safety
/// This function is only safe if the caller ensures ....
pub unsafe fn time(_time: *mut TimeT) -> TimeT {
    unimplemented!()
}

use std::ptr::null_mut;

pub fn call_time() {
    let _t = unsafe { time(null_mut()) };
}

use std::mem::size_of;

pub fn sizeof_operator() {
    // let t = (3, 3i8);
    let _size = size_of::<(i32, i8)>();
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
// Examples:
// https://doc.rust-lang.org/std/os/raw/index.html
// https://docs.rs/libc/0.2.93/libc/

// Imagine you have a nice rust function like this:
// How do you call it from C?
// pub fn fizz_buzz(){
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
