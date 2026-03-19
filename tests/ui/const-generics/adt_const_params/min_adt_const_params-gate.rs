// gate-test-min_adt_const_params
//@run-pass
//! Without the #![feature(min_adt_const_params)]
//! this shouldn't fail.
#![feature(const_param_ty_trait)]
#![allow(incomplete_features)]
use std::marker::ConstParamTy_;

#[derive(PartialEq, Eq)]
pub struct Meowl {
    pub public: i32,
    private: i32
}

impl ConstParamTy_ for Meowl {}

fn main() {}
