// intellij-rust won't accept min_const_generics as a substitute for const_generics
// until this issue is closed: https://github.com/intellij-rust/intellij-rust/issues/6361
// TODO: replace this with min_const_generics, or remove when Rust 1.51 hits.
#![feature(const_generics)]

pub mod hash;
