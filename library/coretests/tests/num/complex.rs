use core::num::Complex;

#[test]
fn add_works() {
    assert_eq!(Complex::new(1.0, 2.0) + Complex::new(3.0, 4.0), Complex::new(4.0, 6.0));
    assert_eq!(Complex::new(0.0, 2.0) + Complex::new(0.0, 4.0), Complex::new(0.0, 6.0));
    assert_eq!(Complex::new(1.0, 0.0) + Complex::new(3.0, 0.0), Complex::new(4.0, 0.0));
    assert_eq!(Complex::new(1.0, 0.0) + 1.0, Complex::new(2.0, 0.0));
}

#[test]
fn sub_works() {
    assert_eq!(Complex::new(3.0, 4.0) - Complex::new(1.0, 2.0), Complex::new(2.0, 2.0));
    assert_eq!(Complex::new(3.0, 4.0) - Complex::new(0.0, 2.0), Complex::new(3.0, 2.0));
    assert_eq!(Complex::new(3.0, 4.0) - Complex::new(1.0, 0.0), Complex::new(2.0, 4.0));
    assert_eq!(Complex::new(1.0, 0.0) - 1.0, Complex::new(0.0, 0.0));
}
