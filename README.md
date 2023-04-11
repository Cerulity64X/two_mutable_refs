# Two Mutable References
Rust experiment showcasing multiple ways of creating multiple mutable references to the same variable at runtime. This is meant to be an experiment, and is not targeted towards production.

One of Rust's pillars of memory safety is that you cannot (safely) create more than one mutable reference to the same variable.\
However, there are multiple unsafe ways to circumvent these bounds.\
This repository aims to showcase multiple ways of breaking these rules.

# Usage
The code shown in this repository should likely never be used in production-ready code.\
All functions in the repository are unsafe, and break one of the fundamental rules Rust enforces.

# Techniques
Pointer casting: Casting an &mut to a *mut, then casting the *mut back to an &mut (which is unsafe) effectively clones the reference.\
Rc cloning: Creating an Rc and cloning it gives two smart references. You can then get the underlying reference from both. Pointer casting can usually be used instead.\

# Utilizations
We can create an entire Vec of mutable references all pointing to the same value. This repo demonstrates this with the pointer casting technique.
