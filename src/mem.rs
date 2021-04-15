// use std::mem;

/*
    The Drop Trait
*/

pub struct Person {
    pub name: String,
    pub age: u8,
}

// What happens internally with the above code when a Person goes out of scope?

// Explicit drop statement

#[test]
fn test_drop_person() {
    // define a person

    // drop person
}

// from C++ programming: rule of 3
// https://en.wikipedia.org/wiki/Rule_of_three_(C%2B%2B_programming)

// Manual Drop implementation

// impl Drop for Person {
// }

/*
    Size of operator
*/

use std::mem::size_of;

pub fn sizeof_operator() {
    // let t = (3, 3i8);
    let _size = size_of::<(i32, i8)>();
}

/*
    mem::replace
    https://doc.rust-lang.org/std/mem/fn.replace.html
*/

/*
    Example from the reading:
    https://rust-unofficial.github.io/too-many-lists/first-final.html
*/
