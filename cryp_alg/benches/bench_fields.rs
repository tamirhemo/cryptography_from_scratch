
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cryp_alg::ff::*;
use cryp_std::rand::thread_rng;

#[path = "../tests/fields/test_fields.rs"]
mod test_fields;

use test_fields::{Fp25519Mont, Fp25519Sol};

#[allow(non_snake_case)]
pub fn bench_Fp25519Sol(c : &mut Criterion) {
    FieldBench::<Fp25519Sol>::run_all(c, "Fp25519Sol");
}


#[allow(non_snake_case)]
pub fn bench_Fp25519Mont(c : &mut Criterion) {
    FieldBench::<Fp25519Mont>::run_all(c, "Fp25519Mont");
}


criterion_group!(benches, bench_Fp25519Sol, bench_Fp25519Mont);
criterion_main!(benches);


struct FieldBench<F: Field> {
    _marker: cryp_std::marker::PhantomData<F>,
}


impl<F: Field> FieldBench<F> {
    fn bench_field_addition(c: &mut Criterion, name: &str) {
        let mut rng = thread_rng();
        let x= F::rand(&mut rng);
        let y= F::rand(&mut rng);
        c.bench_with_input(BenchmarkId::new(name, ""),
        &(x, y), |b, &(x,y)| b.iter(|| x + y));
    }
    fn bench_field_mul(c: &mut Criterion, name: &str) {
        let mut rng = thread_rng();
        let x= F::rand(&mut rng);
        let y= F::rand(&mut rng);
        c.bench_with_input(BenchmarkId::new(name, ""),
        &(x, y), |b, &(x,y)|b.iter(|| x * y));
    }

    fn bench_field_inverse(c: &mut Criterion, name: &str) {
        let mut rng = thread_rng();
        let x= F::rand(&mut rng);
        c.bench_with_input(BenchmarkId::new(name, ""), &x,
          |b, &x| b.iter(|| x.inverse()));
    }

    fn bench_field_square(c: &mut Criterion, name: &str) {
        let mut rng = thread_rng();
        let x= F::rand(&mut rng);
        c.bench_with_input(BenchmarkId::new(name, ""), &x,
          |b, &x| b.iter(|| x.square()));
    }

    fn run_all(c: &mut Criterion, name: &str) {
        Self::bench_field_addition(c, &format!("{} addition", name));
        Self::bench_field_mul(c, &format!("{} multiplication", name));
        Self::bench_field_square(c, &format!("{} square", name));
        Self::bench_field_inverse(c, &format!("{} inverse", name));
    }
}