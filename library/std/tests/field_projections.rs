#![allow(incomplete_features, dead_code)]
#![feature(field_projections)]

use std::field::field_of;

struct Foo {
    x: usize,
}

impl field_of!(Foo, x) {
    fn foo() {}
}

fn bar(_: field_of!(Foo, x)) {}

fn baz(x: field_of!(Foo, x)) {
    bar(x)
}

fn main() {
    <field_of!(Foo, x)>::foo()
}
