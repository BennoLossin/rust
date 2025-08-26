#![allow(incomplete_features, dead_code)]
#![feature(field_projections)]

use std::field::{Field, UnalignedField, field_of};
use std::mem::offset_of;
use std::ptr;

struct Foo {
    x: usize,
}

pub fn project_ref<F: Field>(r: &F::Base) -> &F::Type
where
    F::Type: Sized,
{
    unsafe { &*ptr::from_ref(r).byte_add(F::OFFSET).cast() }
}

fn main() {
    let x = Foo { x: 42 };
    let _: &usize = project_ref::<field_of!(Foo, x)>(&x);
}
