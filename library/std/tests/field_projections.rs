#![allow(incomplete_features)]
#![feature(field_projections)]

use std::field::field_of;

struct Foo {
    x: usize,
}

type X = field_of!(Foo, x);

fn main() {}
