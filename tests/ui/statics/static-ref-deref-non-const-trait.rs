//! Regression test for https://github.com/rust-lang/rust/issues/25901
//!
//! Tests that using a non-const `Deref` impl to coerce `&A` to `&B`
//! in a `static` produces a proper error.
struct A;
struct B;

static S: &'static B = &A;
//~^ ERROR the trait bound `A: const Deref` is not satisfied

use std::ops::Deref;

impl Deref for A {
    type Target = B;
    fn deref(&self) -> &B {
        static B_: B = B;
        &B_
    }
}

fn main() {}
