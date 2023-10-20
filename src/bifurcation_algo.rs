use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use bifurcation::bifurcation_analysis_meta;
use rayon::prelude::*;

use bifurcation::helper::get_all_pairs;
use gfa_reader::{NCGfa, NCPath};
use crate::bifurcation_helper::{all_combinations, all_combinations_self, index_meta, intersection_two_pointer, path2combi};


/// Wrapper function for
///     - Bubble detection
///     - Interval generation
pub fn get_interval(graph: &NCGfa<()>, threads: &usize) -> (Vec<(String, Vec<[u32; 3]>)>, Vec<(u32, u32)>){


    // Get bubbles
    let mut bubble = bvd_wrapper(graph, threads);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let interval_groups: Vec<(String, Vec<[u32; 3]>)> = thread_pool.install(|| {
        graph.paths.par_chunks(2) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| interval(chunk, &bubble).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });
    (interval_groups, bubble)


}

pub fn interval(paths: &[NCPath], bubbles: &Vec<(u32, u32)>) -> Vec<(String, Vec<[u32; 3]>)>{
    let mut all_test = Vec::new();
    for y in paths.iter(){
        let mut test = Vec::new();

        let paa2 = path2combi(y);

        let path2index = index_meta(&paa2);
        for (i, (start, end)) in bubbles.iter().enumerate(){

            if start >= &(path2index.1.len() as u32) || end >= &(path2index.1.len() as u32) {
                continue
            }
            let i1 = path2index.1[*start as usize];
            let i2 = path2index.1[*end as usize];


            if i1 != (0,0)  && i2 != (0,0){
                let a1 = &path2index.0[i1.0 as usize..(i1.0 + i1.1) as usize];
                if start != end{
                    let a2 = &path2index.0[i2.0 as usize..(i2.0 + i1.1) as usize];

                    test.extend(all_combinations(a1, a2, &(i as u32)));


                } else {
                    test.extend(all_combinations_self(a1, &(i as u32)));

                }
            }
        }
        test.sort_by(|a, b| (a[0].cmp(&b[0]).then(b[1].cmp(&a[1]))));
        all_test.push((y.name.clone(), test));

    }
    all_test
}


/// Get all bubbles in the graph
///
pub fn bvd_wrapper(graph: &NCGfa<()>, threads: &usize) -> Vec<(u32, u32)>{


    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs_index = get_all_pairs(&f);

    // Number of pairs
    //let pp = pairs_index.len().clone();


    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();


    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(10) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| detect_bubbles(chunk, graph).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });
    let mut results: Vec<_>= results.into_iter().collect();
    results.sort();
    results

}

/// Detect bubbles
fn detect_bubbles(chunk: &[(usize, usize)], graph: &NCGfa<()>) -> HashSet<(u32, u32)>{
    let mut a = HashSet::new();
    for pair2 in chunk.iter(){
        // Get the paths

        let path1 = graph.paths.get(pair2.0).unwrap();
        let path2 = graph.paths.get(pair2.1).unwrap();

        // Get the index
        let mut path1_combination = path2combi(path1);
        let mut path2_combination = path2combi(path2);
        let shared_index = intersect_index(&mut path1_combination, &mut path2_combination);


        let result = bifurcation_analysis_meta(&shared_index);
        //let result = Vec::new();
        let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
        a.extend(f);

    }

    a
}


pub fn intersect_index(vec1: &mut Vec<(&u32, &bool)>, vec2: &mut Vec<(&u32, &bool)>) -> Vec<[u32; 3]>{

    let aa = index_meta(&vec1);
    let aa2 = index_meta(&vec2);
    vec1.sort();
    vec2.sort();
    // Intersection
    let ff = intersection_two_pointer(&vec1, &vec2);
    let mut rr = Vec::with_capacity(ff.len());

    let mut dd1 = &aa.0[0..aa.1[0].0 as usize];
    let mut dd2 = &aa2.0[0..aa2.1[0].0 as usize];
    ff.iter().for_each(|x|{
        let (start, end) = aa.1[*x.0 as usize*2 + *x.1 as usize];
        dd1 = &aa.0[start as usize..(start + end) as usize];
        let (start, end) = aa2.1[*x.0 as usize*2 + *x.1 as usize];
        dd2 = &aa2.0[start as usize..(start + end) as usize];
        rr.extend(all_combinations3(dd1, dd2, &(x.0*2 + *x.1 as u32)));

    });
    rr



}


pub fn all_combinations3<T>(a: &[T], b: &[T], node_id: &T) -> Vec<[T; 3]>
    where T: Clone + Copy{
    {
        let mut p = Vec::with_capacity(a.len() * b.len());
        for x in a.iter(){
            for y in b.iter(){
                p.push([*x,*y, *node_id])
            }
        }
        p
    }
}




