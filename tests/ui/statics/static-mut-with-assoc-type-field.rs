//! Regression test for https://github.com/rust-lang/rust/issues/29821
//!
//! Tests that `static mut` items with fields whose type is an
//! associated type projection compile successfully.
//@ build-pass

pub trait Foo {
    type FooAssoc;
}

pub struct Bar<F: Foo> {
    id: F::FooAssoc,
}

pub struct Baz;

impl Foo for Baz {
    type FooAssoc = usize;
}

static mut MY_FOO: Bar<Baz> = Bar { id: 0 };

fn main() {}
