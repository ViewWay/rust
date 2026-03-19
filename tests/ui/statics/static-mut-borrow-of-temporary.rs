//! Regression test for https://github.com/rust-lang/rust/issues/46604
//!
//! Tests that a `static` binding with a mutable borrow of a temporary
//! produces the correct errors.
static buf: &mut [u8] = &mut [1u8, 2, 3, 4, 5, 7]; //~ ERROR mutable borrows of temporaries
fn write<T: AsRef<[u8]>>(buffer: T) {}

fn main() {
    write(&buf);
    buf[0] = 2; //~ ERROR E0594
}
