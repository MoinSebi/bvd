use std::collections::BTreeMap;
use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Criterion};
use gfa_reader::{NCGfa, NCPath};
use hashbrown::HashMap;

pub fn path2combi(path: &NCPath) -> Vec<(&u32, &bool)>{
    //let mut ff = Vec::with_capacity(path.nodes.len());
    let mut ff: Vec<(&u32, &bool)> = path.nodes.iter().zip(path.dir.iter()).collect();
    ff

}

pub fn path2combi3(path: &NCPath) -> Vec<(u32)>{
    //let mut ff = Vec::with_capacity(path.nodes.len());
    let mut ff: Vec<u32> = path.nodes.iter().zip(path.dir.iter()).map(|x| *x.0*2 + *x.1 as u32).collect();
    ff

}

pub fn path2combi2(path: &NCPath) -> Vec<(u32, bool)>{
    //let mut ff = Vec::with_capacity(path.nodes.len());
    let mut ff: Vec<(u32, bool)> = path.nodes.iter().cloned().zip(path.dir.iter().cloned()).collect();
    ff

}




/// Benchmark function
fn criterion_benchmark(c: &mut Criterion) {
    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_direct("data/example_data/chr1.sort.small.gfa", false);
    let data = &graph.paths[0];

    // 900 µs
    c.bench_function("Path2Comb - Reference", |b| b.iter(|| path2combi(&data)));
    // 2.4 ms
    c.bench_function("Path2Comb - New Vec", |b| b.iter(|| path2combi2(&data)));

    c.bench_function("Path2Comb - New Vec", |b| b.iter(|| path2combi3(&data)));



}

criterion_group!(aa, criterion_benchmark);
criterion_main!(aa);