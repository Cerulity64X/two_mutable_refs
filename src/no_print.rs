use std::rc::Rc;

/// pointer_casting_double_mut but with no intermediate print statements. Easier to read.
pub unsafe fn pointer_casting_double_mut_np() {
    let mut x: i32 = 0;

    // First reference is safe.
    let first_mref: &mut i32 = &mut x;
    // Second "clones" the reference, which is unsafe as it requires pointer casting.
    let second_mref: &mut i32 = unsafe{(first_mref as *mut i32).as_mut()}.unwrap();

    // These will add to the same variable.
    *first_mref += 4;
    *second_mref += 6;

    println!("Value of variable: {x}");
}

/// rc_cloning_double_mut but with no intermediate print statements. Easier to read.
pub unsafe fn rc_cloning_double_mut_np() {
    // Safe, smart references...
    let mut starting_ref: Rc<i32> = Rc::new(0);
    let mut cloned_ref: Rc<i32> = starting_ref.clone();

    // Unchecked conversion to "stupid" &mut when multiple Rc or Weak references exist is unsafe but possible.
    let first_mref: &mut i32 = unsafe{Rc::get_mut_unchecked(&mut starting_ref)};
    let second_mref: &mut i32 = unsafe{Rc::get_mut_unchecked(&mut cloned_ref)};

    // These will add to the same variable.
    *first_mref += 4;
    *second_mref += 6;

    println!("Value of the starting reference: {starting_ref}");
}
/// pointer_casting_multi_mut but with no intermediate print statements. Easier to read.
pub unsafe fn pointer_casting_multi_mut_np(ref_count: usize) {
    let mut starting_var: i32 = 0;
    let mut refs: Vec<&mut i32> = vec![];

    for i in 0..ref_count {
        refs.push(unsafe{(&mut starting_var as *mut i32).as_mut().unwrap()});
    }

    let mut n: i32 = 0;
    for mref in refs {
        *mref += n;
        n += 1;
    }
    println!("Value of the variable: {starting_var}");
}
