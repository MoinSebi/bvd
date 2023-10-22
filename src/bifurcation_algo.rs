use std::cmp::max;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use bifurcation::bifurcation_analysis_meta;
use rayon::prelude::*;

use bifurcation::helper::get_all_pairs;
use gfa_reader::{NCGfa, NCPath};
use log::info;
use crate::bifurcation_helper::{all_combinations, all_combinations_self, index_meta, intersection_two_pointer, path2combi, path2combi2};
use crate::writer::{hash_vector, hash_vector2, trav_sum, traversal_to_string};


/// Wrapper function for
///     - Bubble detection
///     - Interval generation
pub fn bvd_total(graph: &NCGfa<()>, threads: &usize, bubble: &Vec<(u32, u32)>) -> Vec<(String, Vec<[u32; 3]>)>{



    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let mut chunk_size = 0;
    if graph.paths.len() % *threads != 0{
        chunk_size = (graph.paths.len() / *threads)  + 1

    } else {
        chunk_size = max(1, (graph.paths.len() / *threads))
    }


    let interval_groups: Vec<(String, Vec<[u32; 3]>)> = thread_pool.install(|| {
        graph.paths.par_chunks(chunk_size) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| interval_identification(chunk, &bubble).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });

    interval_groups


}

pub fn interval_identification(paths: &[NCPath], bubbles: &Vec<(u32, u32)>) -> Vec<(String, Vec<[u32; 3]>)>{
    let mut result_interval = Vec::new();
    for path in paths.iter(){
        let mut index_output = Vec::new();

        let path_num_bool = path2combi(path);
        let path2index = index_meta(&path_num_bool);

        for (index, (start, end)) in bubbles.iter().enumerate(){

            // If it is bigger than then the res t
            if start >= &(path2index.1.len() as u32) || end >= &(path2index.1.len() as u32) {
                continue
            }

            // Speedup by precompute this
            let start_index = path2index.1[*start as usize];
            let end_index = path2index.1[*end as usize];


            if start_index != (0, 0)  && end_index != (0, 0){
                let start_indices = &path2index.0[start_index.0 as usize..(start_index.0 + start_index.1) as usize];
                if start != end{
                    let end_indices = &path2index.0[end_index.0 as usize..(end_index.0 + start_index.1) as usize];
                    index_output.extend(all_combinations(start_indices, end_indices, &(index as u32)));


                } else {
                    index_output.extend(all_combinations_self(start_indices, &(index as u32)));

                }
            }
        }
        // Sort for later
        index_output.sort_by(|a, b| (a[0].cmp(&b[0]).then(b[1].cmp(&a[1]))));
        result_interval.push((path.name.clone(), index_output));

    }
    result_interval
}


/// Get all bubbles in the graph
///
pub fn bubble_wrapper(graph: &NCGfa<()>, threads: &usize) -> Vec<(u32, u32)>{


    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs_index = get_all_pairs(&f);
    // Number of pairs
    //let pp = pairs_index.len().clone();


    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let mut chunk_size = 0;
    if graph.paths.len() % *threads != 0{
        chunk_size = (graph.paths.len() / *threads)  + 1

    } else {
        chunk_size = max(1, (graph.paths.len() / *threads))
    }

    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(chunk_size) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| detect_bubbles(chunk, graph).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });
    let mut results: Vec<_>= results.into_iter().collect();

    // Sorted bubbles
    results.sort();
    results

}

/// Detect bubbles
fn detect_bubbles(chunk: &[(usize, usize)], graph: &NCGfa<()>) -> HashSet<(u32, u32)>{
    let mut bubbles = HashSet::new();
    for pair in chunk.iter(){

        // Get the paths
        let path1 = graph.paths.get(pair.0).unwrap();
        let path2 = graph.paths.get(pair.1).unwrap();

        // Get the index
        let mut path1_combination = path2combi(path1);
        let mut path2_combination = path2combi(path2);
        let mut shared_index = intersect_index(&mut path1_combination, &mut path2_combination);

        let result = bifurcation_analysis_meta(&shared_index);

        //let result = Vec::new();
        let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
        bubbles.extend(f);

    }

    bubbles
}


/// Get all bubbles in the graph
///
pub fn bubble_wrapper2(graph: &NCGfa<()>, threads: &usize, gog: & Vec<Vec<(&u32, &bool)>>, ggg:  &Vec<(Vec<u32>, Vec<(u32, u32)>)>) -> Vec<(u32, u32)>{


    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs_index = get_all_pairs(&f);

    // Number of pairs
    //let pp = pairs_index.len().clone();


    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let mut chunk_size = 0;
    if graph.paths.len() % *threads != 0{
        chunk_size = (graph.paths.len() / *threads)  + 1

    } else {
        chunk_size = max(1, (graph.paths.len() / *threads))
    }

    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(chunk_size) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| detect_bubbles2(chunk, graph, gog, ggg).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });
    let mut results: Vec<_>= results.into_iter().collect();

    // Sorted bubbles
    results.sort();
    results

}

/// Detect bubbles
fn detect_bubbles2(chunk: &[(usize, usize)], graph: &NCGfa<()>, gog: & Vec<Vec<(&u32, &bool)>>, ggg:  &Vec<(Vec<u32>, Vec<(u32, u32)>)>) -> HashSet<(u32, u32)>{
    let mut bubbles = HashSet::new();
    for pair in chunk.iter(){


        // Get the index
        let path1_combination = & gog[pair.0];
        let path2_combination = & gog[pair.1];
        let mut shared_index = intersect_index2(& path1_combination, & path2_combination, &ggg[pair.0], &ggg[pair.1]);
        let result = bifurcation_analysis_meta(&shared_index);
        //let result = Vec::new();
        let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
        bubbles.extend(f);

    }

    bubbles
}



pub fn jooo(graph: &NCGfa<()>)  -> (Vec<Vec<(&u32, &bool)>>, Vec<(Vec<u32>, Vec<(u32, u32)>)>){
    let mut a = Vec::new();
    let mut b = Vec::new();
    for path1 in graph.paths.iter(){
        let mut path1_combination = path2combi(path1);

        let mut index1 = index_meta(&path1_combination);
        path1_combination.sort();

        a.push(path1_combination);

        b.push(index1);
    }
    (a,b)
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



pub fn intersect_index2(vec1: & Vec<(&u32, &bool)>, vec2: & Vec<(&u32, &bool)>, aa: &(Vec<u32>, Vec<(u32, u32)>), aa2: &(Vec<u32>, Vec<(u32, u32)>)) -> Vec<[u32; 3]>{



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




