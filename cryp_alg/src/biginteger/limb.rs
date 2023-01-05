pub trait Limb: Sized {
    /// The type used to represent a carry bit.
    type Carry;

    ///  Calculates `self + rhs + carry` without the ability to overflow.
    ///
    /// Performs "ternary addition" which takes in an extra bit to add, and may return an
    /// additional bit of overflow. This allows for chaining together multiple additions
    /// to create "big integers" which represent larger values.
    fn carrying_add(&self, rhs: Self, carry: Self::Carry) -> (Self, Self::Carry);
}
