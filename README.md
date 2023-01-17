# Cryptography from Scratch

An educational implementation of cryptographic primitives in Rust. 


The structure of the project is based on [Arkworks](https://github.com/arkworks-rs/).

* `cryp_alg` - A library for handeling algebraic structures and finite field arithmetic.
* `cryp_ec` - A library for handeling elliptic curves.
* `cryp_commit` - A library for commitment scheme priitives and applications.
* `cry_std` - A wrapper around the standard library to handle dependencies and provide common infrastructure.

## Basic usage

### Finite Field Arithmetic 
The cryp_alg crate provides a way to construct fields. 

For example, we can construct the prime field with size `2^255 - 19` and using Montgomery representation as follows. 

First, to include all the necessary traits and functions, we need to import the following:
```rust
use cryp_alg::ff::*;
```
We can encode the necessary parameters for Montgomery representation in a struct that implements the `MontParameters` trait.
```rust
/// Parameters for the prime field Fp25519
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp25519Params;

impl MontParameters<4usize> for Fp25519Params {
    type Limb = u64;

    // p=  2^255-19 =  57896044618658097711785492504343953926634992332820282019728792003956564819949
    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    // 2^256 mod p
    const R: [Self::Limb; 4] = [38, 0, 0, 0];

    // R^2 mod p
    const R2: [Self::Limb; 4] = [1444, 0, 0, 0];

    // -p^-1 mod 2^64
    const MP: Self::Limb = 9708812670373448219;
}

```
Then, we can construct the field as follows:
```rust
pub type Fp25519Mont = F<MontgomeryOperations<4, Fp25519Params>>;
```

For the prime `2^255 - 19`, there is a more efficient reduction algorithm by noting that `2^256 = 38 mod p`.  We can use this directly using the `Solinas` reduction algorithm.  We can encode the necessary parameters for Solinas representation by implementing the `SolinasParameters` trait.

```rust
impl SolinasParameters<4usize> for Fp25519Params {
    type Limb = u64;

    // 2^255-19 =  57896044618658097711785492504343953926634992332820282019728792003956564819949
    const MODULUS: [Self::Limb; 4] = [
        18446744073709551597,
        18446744073709551615,
        18446744073709551615,
        9223372036854775807,
    ];

    // 2^256 mod (2^255-19)
    const C: [u64; 4] = [38, 0, 0, 0];
}
```rust

Then the field can be constructed in a similar way:
```rust
pub type Fp25519Sol = F<GeneralReductionOperations<4, SolinasReduction<4, Fp25519Params>>>;
```

### Elliptic curves
Currently we have an implementation of the [EdSDA Edwards cuve](https://www.rfc-editor.org/rfc/rfc8032#page-16) over the prime field modulus `2^255 - 19`.

We can get a random generator as follows:
```rust
use cryp_ec::curves::edwards25519::*;
use cryp_std::rand::thread_rng;

let mut rng = thread_rng();
let g = GroupEd25519::generator(&mut rng);
```
This method generates a random scalar and multiplies the specified generator by the specified EdSDA base point.

The generator is given in affine coordinates.If we want to perform group operations, we can convert the point to the internal coordinate system being used, in this case, the extended twisted Edwards coordinates. 

```rust
    let g : GroupEd25519 = g.into();
    let identity = GroupEd25519::identity();
    
    assert_eq!(identity, identity + identity);
    assert_eq!(g, g + identity);
    assert_ne!(g, identity);
```

We can check that the given element is on the curve:
```rust
    let x = g.point.x;
    let y = g.point.y;
 
    // Get the parameters:   
    let one = Fp25519::one();
    let d = Ed25519Parameters::D;
    
    // check -x^2 + y^2 = 1 + d*x^2*y^2
    assert_eq!(-x.square() + y.square(), one + d * x.square() * y.square()); 
```
We can also check that the group element has the correct order.

```rust
    let order = GroupEd25519::ScalarFied::MODULUS;
    let one = GroupEd25519::ScalarFied::one();
 
    assert_eq!(identity, g *&order);
    assert_ne!(identity, g*&(order-one));
```
