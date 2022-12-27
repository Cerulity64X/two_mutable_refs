//! All of this code is unsafe.
//! This code is not for any type of production.
//! This is for experimentation, education and demonstration.
//! The `printed` module contains code followable at runtime.
//! The `no_print` module contains more readable source code.
#![feature(get_mut_unchecked)]
use std::rc::Rc;

use printed::pointer_casting_multi_mut;

mod printed;
mod no_print;

fn main() {
    unsafe {pointer_casting_multi_mut(10)};
}
