use crate::biginteger::Limb;


pub struct Montgomery<L : Limb, const N : usize>{
     element : [L; N],
}

// Implement montgomery multiplication, addition, and modular reduction

