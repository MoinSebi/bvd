use criterion::{criterion_group, criterion_main, Criterion};
use gfa_reader::{NCGfa, NCPath};
use hashbrown::{HashMap, HashSet};
use std::collections::HashSet as HashSet2;
use crate::bench_index::path2combi;
extern crate rayon;
use rayon::{ThreadPoolBuilder, prelude::*};


use rayon::prelude::*;


pub mod bench_index;

/// Intersection of two vectors using HashSets
///
/// Intersection
pub fn intersection_hashset_intersection<'a >(data1: &'a Vec<(&'a u32, &'a bool)>, data2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)>{
    let set: HashSet<_> = data1.iter().cloned().collect();
    let set2: HashSet<_> = data2.iter().cloned().collect();
    //set2.iter().filter(|&&x| set.contains(&x)).cloned().collect()
    set2.intersection(&set).cloned().collect()


}

pub fn intersection_hashset_intersection23<'a >(data1: & Vec<u32>, data2: & Vec<u32>) -> Vec<u32>{
    let set: HashSet<_> = data1.iter().cloned().collect();
    let set2: HashSet<_> = data2.iter().cloned().collect();
    //set2.iter().filter(|&&x| set.contains(&x)).cloned().collect()
    set2.intersection(&set).cloned().collect()


}

pub fn itert<'a >(data1: & Vec<u32>, data2: & Vec<u32>) -> Vec<u32>{
    data2.iter().cloned().filter(|&x| data2.contains(&x)).collect()



}


/// Intersection of two vectors using HashSets
///
/// Filter
pub fn intersection_hashset_filter<'a >(data1: &'a Vec<(&'a u32, &'a bool)>, data2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)>{
    let set: HashSet<_> = data1.iter().cloned().collect();
    let set2: HashSet<_> = data2.iter().cloned().collect();
    set2.iter().filter(|&&x| set.contains(&x)).cloned().collect()


}




/// Intersection of two vectors using HashSets
///
/// Implementation with rayon and filter
fn intersection_hashset_filter_rayon<'a>(v1: &'a Vec<(&'a u32, &'a bool)>, v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let (small_vec, large_vec) = if v1.len() <= v2.len() { (v1, v2) } else { (v2, v1) };

    // Parallelized HashSet building
    let set: HashSet2<_> = small_vec.par_iter().cloned().collect();
    let set2: HashSet2<_> = large_vec.par_iter().cloned().collect();

    // Parallelized intersection checking
    let res: Vec<_> = set2.par_iter().filter(|&&x| set.contains(&x)).cloned().collect();

    res
}


/// Intersection of two vectors using Itertools
///
/// Using itertools and Hashsets
fn intersection_hashset_itertools<'a>(v1: &'a Vec<(&'a u32, &'a bool)>, v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        match v1[i].cmp(&v2[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                result.push(v1[i]);

                // Move the pointers past duplicates in both vectors
                let current = v1[i];
                while i < v1.len() && v1[i] == current { i += 1; }
                while j < v2.len() && v2[j] == current { j += 1; }
            }
        }
    }

    result
}

/// Intersection of two vectors using two-pointer approach
///
/// Comment: Since multiple vectors can be possible, add these if needed
pub fn intersection_two_pointer<'a>(v1: &'a Vec<(&'a u32, &'a bool)>, v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let mut res = Vec::with_capacity(v1.len().min(v2.len()));
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] == v2[j] {
            if res.len() == 0{
                res.push(v1[i]);
            } else if res.last().unwrap() != &v1[i] {
                res.push(v1[i]);
            }
            i += 1;
            j += 1;
        } else if v1[i] < v2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}


/// Intersection of two vectors using two-pointer approach
///
/// Comment: Since multiple vectors can be possible, add these if needed
pub fn intersection_two_pointer_u32(v1: & Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut res = Vec::with_capacity(v1.len().min(v2.len()));
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] == v2[j] {
            if res.len() == 0{
                res.push(v1[i]);
            } else if res.last().unwrap() != &v1[i] {
                res.push(v1[i]);
            }
            i += 1;
            j += 1;
        } else if v1[i] < v2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}

pub fn intersection_two_pointer2<'a>(v1: &'a Vec<(&'a u32, &'a bool)>, v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let mut res = Vec::with_capacity(v1.len().min(v2.len()));
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] == v2[j] {
            if res.len() == 0{
                res.push(v1[i]);
            } else if res.last().unwrap() != &v1[i] {
                res.push(v1[i]);
            }
            i += 1;
            j += 1;
        } else if v1[i] < v2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}

/// Intersection of two vectors using two-pointer approach
///
/// Using chunks (mostly the same than above)
/// Needed for wrapper
fn intersection_two_pointer_chunks<'a>(v1: &'a [(&'a u32, &'a bool)], v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let mut res = Vec::with_capacity(v1.len().min(v2.len()));
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] == v2[j] {
            if res.len() == 0{
                res.push(v1[i]);
            } else if res.last().unwrap() != &v1[i] {
                res.push(v1[i]);
            }
            i += 1;
            j += 1;
        } else if v1[i] < v2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}


/// Intersection of two vectors using two-pointer approach with rayon
///
/// Using rayon and helper function
fn intersection_two_pointer_rayon<'a>(v1: &'a Vec<(&'a u32, &'a bool)>, v2: &'a Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a bool)> {
    let (small_vec, large_vec) = if v1.len() <= v2.len() { (v1, v2) } else { (v2, v1) };
    let chunk_size = large_vec.len() / 5;  // Size based on number of CPUs

    large_vec.par_chunks(chunk_size)
        .flat_map(|chunk| intersection_two_pointer_chunks(chunk, small_vec))
        .collect()
}

pub fn sort_1(a: &mut Vec<(&u32, &bool)>, b: &mut Vec<(&u32, &bool)>) {
    a.sort();
    b.sort();
}

pub fn path2combi3(path: &NCPath) -> Vec<(u32)>{
    //let mut ff = Vec::with_capacity(path.nodes.len());
    let mut ff: Vec<u32> = path.nodes.iter().zip(path.dir.iter()).map(|x| *x.0*2 + *x.1 as u32).collect();
    ff

}



fn criterion_benchmark(c: &mut Criterion) {
    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    let mut graph: NCGfa<()> = NCGfa::new();
    //graph.parse_gfa_file_direct("data/example_data/chr1.sort.small.gfa", false);
    graph.parse_gfa_file_direct("data/example_data/chr1.sort.small.gfa", false);
    // Get the data
    let mut data1 = path2combi(&graph.paths[0]);
    let mut data2 = path2combi(&graph.paths[2]);
    let mut data3 = path2combi3(&graph.paths[2]);
    let mut data4 = path2combi3(&graph.paths[3]);


    // Sort the data
    data1.sort();
    data2.sort();
    data3.sort();
    data4.sort();

    // Intersection - check if they do the same stuff
    let mut a =  intersection_hashset_intersection(&data1, &data2);
    let mut a2 = intersection_hashset_filter(&data1, &data2);
    let mut b = intersection_two_pointer(&data1, &data2);
    let mut c2 = intersection_two_pointer_rayon(&data1, &data2);
    let mut c3 = intersection_hashset_itertools(&data1, &data2);
    let mut c4 = intersection_hashset_filter_rayon(&data1, &data2);
    a.sort();
    a2.sort();
    b.sort();
    c2.sort();
    c3.sort();
    c4.sort();

    assert_eq!(a, a2);
    assert_eq!(a, b);
    assert_eq!(a, c2);
    assert_eq!(a, c3);
    assert_eq!(a, c4);

    let mut data1 = path2combi(&graph.paths[0]);
    let mut data2 = path2combi(&graph.paths[2]);
    data1.sort();
    data2.sort();

    // 1.75 ms
    c.bench_function("Sorting operation", |b| b.iter(|| sort_1(&mut data1, &mut data2)));

    // Intersection of Hashsets
    // 100 ms
    c.bench_function("Intersection hashset intersection", |b| b.iter(|| intersection_hashset_intersection(&data1, &data2)));
    c.bench_function("Intersection hashset intersection", |b| b.iter(|| intersection_hashset_intersection23(&data3, &data4)));
    c.bench_function("intersection_hashset_filter", |b| b.iter(|| intersection_hashset_filter(&data1, &data2)));

    // Intersection two pointer
    // 5.2 ms
    c.bench_function("Intersection_two_pointer", |b| b.iter(|| intersection_two_pointer2(&data1, &data2)));
    c.bench_function("Intersection_two_pointer", |b| b.iter(|| intersection_two_pointer_u32(&data3, &data4)));

    // 6.4 ms
    c.bench_function("Intersection_two_pointer_rayon", |b| b.iter(|| intersection_two_pointer_rayon(&data1, &data2)));

    // Hashset with rayon
    // 97 ms
    c.bench_function("intersection_hashset_filter_rayon", |b| b.iter(|| intersection_hashset_filter_rayon(&data1, &data2)));

    // Hashset with Itertools
    // 8.2 ms
    c.bench_function("intersection_hashset_itertools", |b| b.iter(|| intersection_hashset_itertools(&data1, &data2)));








}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);