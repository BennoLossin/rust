#![allow(dead_code)]

use std::field::Field; //~ ERROR: use of unstable library feature `field_projections`
use std::ptr;

fn project_ref<F: Field>( //~ ERROR: use of unstable library feature `field_projections`
    r: &F::Base, //~ ERROR: use of unstable library feature `field_projections`
) -> &F::Type //~ ERROR: use of unstable library feature `field_projections`
where
    F::Type: Sized, //~ ERROR: use of unstable library feature `field_projections`
{
    unsafe { &*ptr::from_ref(r).byte_add(F::OFFSET).cast() } //~ ERROR: use of unstable library feature `field_projections`
}

fn main() {}
