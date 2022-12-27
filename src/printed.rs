use std::rc::Rc;

/// Casts the first mutable reference to a pointer, then converts it back into an &mut (unsafe), effectively cloning it.
pub unsafe fn pointer_casting_double_mut() {
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
pub unsafe fn rc_cloning_double_mut() {
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

/// Same technique as pointer_casting_double_mut, but stores the references in a Vec.
/// Every ref will have its index + 1 added to it, ending up with a `triangle number`.
pub unsafe fn pointer_casting_multi_mut(ref_count: usize) {
    println!("Creating variable...");
    let mut starting_var: i32 = 0;
    println!("Value of variable: {starting_var}");

    println!("Creating reference vector...");
    let mut refs: Vec<&mut i32> = vec![];

    println!("UNSAFE: Creating references (casting *mut to &mut is unsafe, see pointer_casting_double_mut as well)");
    for i in 0..ref_count {
        println!("Created reference {}...", i + 1);
        refs.push(unsafe{(&mut starting_var as *mut i32).as_mut().unwrap()});
    }

    println!("Adding to each reference...");
    let mut n: i32 = 1;
    for mref in refs {
        *mref += n;
        println!("Added {n} to reference {n}");
        n += 1;
    }

    println!("Value of the variable: {starting_var}");
}
