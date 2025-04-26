use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Zero: Sized + Add<Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One: Sized + Mul<Output = Self> {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

pub trait Group: One + Div<Output = Self> {}

pub trait Abelian: Zero + Neg<Output = Self> + Sub<Output = Self> {}

pub trait Ring: Abelian + One {}

pub trait Field: Abelian + Group {}
