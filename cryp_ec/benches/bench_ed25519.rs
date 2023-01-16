use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cryp_alg::PrimeField;
use cryp_ec::curves::edwards25519::*;
use cryp_std::rand::thread_rng;

pub fn bench_scalar_mul_ed25519(c : &mut Criterion) {
    let mut rng = thread_rng();
    let generator = GroupEd25519::from(GroupEd25519::generator(Some(&mut rng)));
    let scalar = ScalarEd25519::from_int(&[333944u64, 0, 0,0].into()).inverse().unwrap();
    c.bench_with_input(BenchmarkId::new("scalar_mul", "random"),
    &(generator, scalar), |b, &(generator, scalar)| 
    b.iter(|| generator * &scalar));
}


criterion_group!(benches, bench_scalar_mul_ed25519);
criterion_main!(benches);