use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use rand::SeedableRng;
use rand_pcg;

use rtwasm::cast;
use rtwasm::scenes;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("custom_scene/10x10x5", |b| {
        // Benchmark `custom_scene` on 10x10 image at 5 samples per pixel
        // Scene contains about 10 spheres
        const NX: u32 = 10;
        const NY: u32 = 10;
        const NS: u32 = 5;
        const SEED: u64 = 0;

        // initialize rng
        let mut rng = rand_pcg::Pcg64::seed_from_u64(SEED);

        // initialize world and camera
        let (world, cam) = scenes::custom_scene(NX, NY);

        // initalize params
        let params = scenes::Params::new(NX, NY, NS, SEED, String::from("benchmark.png"));

        b.iter_batched(
            || (),
            |_| cast(&params, &world, &cam, &mut rng, false, false),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("rtiow_scene/10x10x5", |b| {
        // Benchmark `rtiow_scene` on 100x100 image at 10 samples per pixel
        // Scene contains about 500 spheres
        const NX: u32 = 10;
        const NY: u32 = 10;
        const NS: u32 = 5;
        const SEED: u64 = 0;

        // initialize rng
        let mut rng = rand_pcg::Pcg64::seed_from_u64(SEED);

        // initialize world and camera
        let (world, cam) = scenes::rtiow_scene(NX, NY, &mut rng);

        // initalize params
        let params = scenes::Params::new(NX, NY, NS, SEED, String::from("benchmark.png"));

        b.iter_batched(
            || (),
            |_| cast(&params, &world, &cam, &mut rng, false, false),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
