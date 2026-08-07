use crate::traits::Calculate;
use std::ops::{Add, Sub, Mul, Div};

pub enum Calculator<T> {
    Add(T, T),
    Subtract(T, T),
    Multiply(T, T),
    Divide(T, T),
}

impl<T> Calculate<T> for Calculator<T> where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn calculate(&self) -> T {
        match self {
            Calculator::Add(num1, num2) => *num1 + *num2,
            Calculator::Subtract(num1, num2) => *num1 - *num2,
            Calculator::Multiply(num1, num2) => *num1 * *num2,
            Calculator::Divide(num1, num2) => *num1 / *num2,
        }
    }
}