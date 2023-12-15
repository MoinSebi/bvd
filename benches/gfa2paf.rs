use std::collections::BTreeMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use gfa_reader::{NCGfa, NCPath};
use hashbrown::{HashMap};
use nohash_hasher::NoHashHasher;
use bvd::bifurcation_algo::index_wrapper;
use bvd::bifurcation_helper::{IndexMetadata, intersection_two_pointer_u32};
use bvd::writer::gfapos_wrapper;
use bvd::gfa2paf::{gfa2paf_h, gfa2paf_h2, gfa2paf_og, iter_dict};
use bvd::helper::get_all_pairs;
use std::collections::HashSet;



/// Benchmark function
fn criterion_benchmark(c: &mut Criterion) {
    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    println!("aa");
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_direct("data/example_data/chr1.sort.small.gfa", false);
    println!("bbb");
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    println!("{}", graph.nodes.len());
    println!("{}", graph.paths.len());
    let mut pairs_index = get_all_pairs(&f);
    // let index2pos = gfapos_wrapper(&graph, &1);
    // let (path_merges, mut index) = index_wrapper(&graph);
    // let pair = pairs_index[0];
    // println!("{} {}", pair.0, pair.1);
    // let shared_index = path_merges[pair.1].iter().cloned().collect::<HashSet<u32>>(); ;
    // let mut ii2 = index[pair.1].clone();
    // let ii = &mut ii2;
    //let dfdf = gfa2paf_h(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index);
    //let dfdf2 = gfa2paf_h2(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index);
    //println!("{}", dfdf.len());
    //println!("{}", dfdf2.len());
    //let dfdf = gfa2paf_h(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index);
    //let dfdf2 = gfa2paf_h2(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index);
    //println!("{}", dfdf.len());
    println!("{}", pairs_index.len());



    //c.bench_function("Path2Comb - Reference", |b| b.iter(|| gfa2paf_h(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index)));
    //c.bench_function("Path2Comb - Reference", |b| b.iter(|| gfa2paf_h2(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index)));


    // 900 µs
    c.bench_function("Path2Comb - Reference", |b| b.iter(|| black_box({println!("{}", iter_dict(&graph, &1, &pairs_index).len())})));



}

criterion_group!(aa, criterion_benchmark);
criterion_main!(aa);