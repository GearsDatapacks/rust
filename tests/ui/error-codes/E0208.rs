#![feature(rustc_attrs)]

#[rustc_variance]
struct Foo<'a, T> { //~ ERROR [-, o]
    t: &'a mut T,
}

fn main() {}
