#![feature(get_mut_unchecked)]
use std::rc::Rc;

// All of this code is unsafe
fn main() {
    unsafe{pointer_casting_double_mut()};
}

/// Casts the first mutable reference to a pointer, then converts it back into an &mut (unsafe), effectively cloning it.
unsafe fn pointer_casting_double_mut() {
    println!("Creating variable...");
    let mut x: i32 = 0;
    println!("Value of variable: {x}");
    println!("Creating first reference...");
    let first_mref: &mut i32 = &mut x;
    println!("UNSAFE: Creating second reference (casting existing reference to pointer, then converting to &mut)...");
    let second_mref: &mut i32 = (first_mref as *mut i32).as_mut().unwrap();
    println!("Adding 4 to the first reference...");
    *first_mref += 4;
    println!("Adding 6 to the second reference...");
    *second_mref += 6;
    println!("Value of variable: {x}");
}

/// Creates an Rc, clones it, then gets a mutable reference from both (unchecked, unsafe).
unsafe fn rc_cloning_double_mut() {
    println!("Creating main reference (Rc)...");
    let mut starting_ref: Rc<i32> = Rc::new(0);
    println!("Value at reference: {starting_ref}");
    println!("Cloning reference...");
    let mut cloned_ref: Rc<i32> = starting_ref.clone();
    println!("UNSAFE: Getting first mutable reference (get_mut will return None when there are existing Rc or Weak references)...");
    let first_mref: &mut i32 = unsafe{Rc::get_mut_unchecked(&mut starting_ref)};
    println!("UNSAFE: Getting second mutable reference...");
    let second_mref: &mut i32 = unsafe{Rc::get_mut_unchecked(&mut cloned_ref)};
    println!("Adding 4 to the first reference...");
    *first_mref += 4;
    println!("Adding 6 to the second reference...");
    *second_mref += 6;
    println!("Value of the starting reference: {starting_ref}");
}
