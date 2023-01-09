# Cryptography Primitives 


* `cryp_alg` - A library for handeling algebraic structures and finite field arithmetic.
* `cryp_ec` - A library for handeling elliptic curves.
* `cryp_commit` - A library for commitment scheme priitives and applications.
* `cry_std` - A wrapper around the standard library to handle dependencies and provide common infrastructure.

## Basic usage

### Finite Field Arithmetic 

```rust
use cryp_alg::ff::*;

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
We can also check that the group element has the correct order.

```rust
    let order = GroupEd25519::ScalarFied::MODULUS;
    let one = GroupEd25519::ScalarFied::one();
 
    assert_eq!(identity, g *&order);
    assert_ne!(identity, g*&(order-one));
```

### Commitment Schemes

Pedersen commitments are agnostic to the underlying group. To define a Pedersen commitment.