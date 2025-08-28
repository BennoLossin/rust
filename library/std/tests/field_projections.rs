#![allow(incomplete_features)]
#![feature(field_projections)]

use std::field::{Field, UnalignedField, field_of};
use std::mem::offset_of;
use std::ptr;

pub struct Foo {
    pub x: usize,
    pub y: usize,
}

pub fn project_ref<F: Field>(r: &F::Base) -> &F::Type
where
    F::Type: Sized,
{
    unsafe { &*ptr::from_ref(r).byte_add(F::OFFSET).cast() }
}

#[test]
fn foo() {
    let foo = Foo { x: 42, y: 24 };
    let x = project_ref::<field_of!(Foo, x)>(&foo);
    let y = project_ref::<field_of!(Foo, y)>(&foo);
    assert_eq!(*x, 42);
    assert_eq!(*y, 24);
    assert_eq!(<field_of!(Foo, x)>::OFFSET, offset_of!(Foo, x));
    assert_eq!(Z, 0);
    bar::<Z>();
    bar::<X>();
}

const Z: usize = <field_of!(Foo, z)>::OFFSET;
type Z = <field_of!(Foo, z) as UnalignedField>::Type;
type X = <field_of!(Foo, x) as UnalignedField>::Type;

fn bar<T>() {}

fn main() {}
