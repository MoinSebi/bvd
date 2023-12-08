use std::cmp::{max, min};
use std::collections::{ HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::time::Instant;
use bifurcation::{bifurcation_meta, bifurcation_sort_hold};
use rayon::prelude::*;

use gfa_reader::{NCGfa, NCPath};
use log::{info, trace};
use crate::bifurcation_helper::{all_combinations, all_combinations_self, index_meta, index_metadata, intersection_two_pointer, intersection_two_pointer_u32, path2combi, path2nodedir};



//-------------------------------------------------------------------------LOWMEM-------------------------------------------------------------



/// Get all bubbles in the graph
///
pub fn bubble_wrapper(graph: &NCGfa<()>, threads: &usize, pairs_index: &Vec<(usize, usize)>) -> Vec<(u32, u32)>{



    // Number of pairs
    //let pp = pairs_index.len().clone();


    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let mut chunk_size = 0;
    if graph.paths.len() % *threads != 0{
        chunk_size = ((pairs_index.len() / *threads))  + 1

    } else {
        chunk_size = max(1, (pairs_index.len() / *threads))
    }
    chunk_size = ((pairs_index.len() / *threads) /5)  + 1;
    chunk_size = min(500, chunk_size);
    info!("BVD: Chunk size {}", chunk_size);


    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(chunk_size) // Process in chunks of 3 elements (you can adjust the chunk size).
            .map(|chunk| detect_bubbles(chunk, graph)) // Pass external data to the processing function.
            .reduce(
                || HashSet::new(), // Initialize an empty HashSet for each thread.
                |mut accumulator, chunk_results| {
                    // Merge the results from each chunk into the accumulator.
                    accumulator.extend(chunk_results);
                    accumulator
                },
            )

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
        let mut path1_combination = path2nodedir(path1);
        let mut path2_combination = path2nodedir(path2);
        let mut aa = index_metadata::new();
        aa.from_path(&path1_combination);
        let mut aa2 = index_metadata::new();
        aa2.from_path(&path2_combination);
        path1_combination.sort();
        path2_combination.sort();
        let mut shared_index = intersect_index(&mut path1_combination, &mut path2_combination, &aa, &aa2);


        let mut result = bifurcation_sort_hold(&shared_index);
        for arr in &mut shared_index {
            // Check if the array has at least two elements
            // Swaps the first and second elements in the array
            arr.swap(0, 1);
        }
        shared_index.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));


        result.extend(bifurcation_sort_hold(&shared_index));

        //let result = Vec::new();
        let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
        bubbles.extend(f);

    }

    bubbles
}





//-------------------------------------------------------------------------HIGHMEM-------------------------------------------------------------

/// Create all the index needed
pub fn index_wrapper(graph: &NCGfa<()>) -> (Vec<Vec<u32>>, Vec<index_metadata>){
    let mut index_index = Vec::new();
    let mut index_struct = Vec::new();
    for path in graph.paths.iter(){
        let mut path1_combination = path2nodedir(path);
        let mut index = index_metadata::new();
        index.from_path(&path1_combination);
        path1_combination.sort();

        index_index.push(path1_combination);
        index_struct.push(index);
    }
    (index_index, index_struct)
}


/// Get all bubbles in the graph
///
pub fn bubble_wrapper_highmem(graph: &NCGfa<()>, threads: &usize, index_vec: & Vec<Vec<u32>>, index_wrapper: &Vec<index_metadata>, pairs_index: &Vec<(usize, usize)>) -> Vec<(u32, u32)>{



    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    // Chunk size computation
    let mut chunk_size = 0;
    if graph.paths.len() % *threads != 0{
        chunk_size = ((pairs_index.len() / *threads))  + 1

    } else {
        chunk_size = max(1, (pairs_index.len() / *threads))
    }
    chunk_size = ((pairs_index.len() / *threads) /5)  + 1;
    chunk_size = min(500, chunk_size);
    info!("BVD: Chunk size {}", chunk_size);


    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(chunk_size) // Process in chunks of 3 elements (you can adjust the chunk size).
            .map(|chunk| detect_bubbles_hm(chunk, &index_wrapper, &index_vec)) // Pass external data to the processing function.
            .reduce(
                || HashSet::new(), // Initialize an empty HashSet for each thread.
                |mut accumulator, chunk_results| {
                    // Merge the results from each chunk into the accumulator.
                    accumulator.extend(chunk_results);
                    accumulator
                },
            )

    });

    let mut results: Vec<_>= results.into_iter().collect();


    // Sorted bubbles
    results.sort();

    results

}

/// Detect bubbles
fn detect_bubbles_hm(chunk: &[(usize, usize)], gog: &Vec<index_metadata>, ggg: &Vec<Vec<u32>>) -> HashSet<(u32, u32)>{
    let mut bubbles = HashSet::new();
    for pair in chunk.iter(){


        let start = Instant::now();
        // Get the index
        let path1_combination = & ggg[pair.0];
        let path2_combination = & ggg[pair.1];
        let elapsed = start.elapsed();
        trace!("Time to get lookup: {:?}", elapsed);



        let mut shared_index = intersect_index(path1_combination, path2_combination, &gog[pair.0], &gog[pair.1]);
        if shared_index.len() > 1{
            let elapsed = start.elapsed();
            trace!("Time to intersect: {:?}", elapsed);
            trace!("Shared index len {:?}", shared_index.len());






            let mut result = bifurcation_sort_hold(&shared_index);
            let elapsed = start.elapsed();
            trace!("Time to bifurcation1: {:?}", elapsed);

            for arr in &mut shared_index {
                // Check if the array has at least two elements
                // Swaps the first and second elements in the array
                arr.swap(0, 1);
            }
            let elapsed = start.elapsed();
            trace!("Time to switch: {:?}", elapsed);
            shared_index.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));
            let elapsed = start.elapsed();
            trace!("Time to sort: {:?}", elapsed);

            result.extend(bifurcation_sort_hold(&shared_index));
            let elapsed = start.elapsed();
            trace!("Time to bifurcation2: {:?}", elapsed);


            let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());

            //let result = Vec::new();

            bubbles.extend(f);
            let elapsed = start.elapsed();
            trace!("Time to hashset: {:?}", elapsed);
        }
    }

    bubbles
}








pub fn intersect_index(vec1: & Vec<u32>, vec2: & Vec<u32>, aa: &index_metadata, aa2: &index_metadata) -> Vec<[u32; 3]>{



    // Intersection
    let mut shared_nodes = intersection_two_pointer_u32(&vec1, &vec2);





    let mut result2 = Vec::with_capacity(shared_nodes.len());

    shared_nodes.iter().for_each(|x|{
        let g1 = aa.get(x);
        let g2 = aa2.get(x);
        if g1.len() > 10 && g2.len() > 10{
            return
        } else {
            if g1.len() == 1 && g2.len() == 1 {
                result2.push([g1[0], g2[0], *x]);
            } else {
                result2.extend(combinations2D(g1, g2, x));
            }
        }

    });

    result2.sort_by(|a, b| (a[0].cmp(&b[0]).then(a[1].cmp(&b[1]))));

    result2



}



pub fn combinations2D<T>(vec1: &[T], vec2: &[T], node_id: &T) -> Vec<[T; 3]>
    where T: Copy{
    {
        // let all_combinations: Vec<[T; 3]> = a.iter().flat_map(| x| {
        //     b.iter().map(move | y| [*x, *y, *node_id])
        // }).collect();
        // return all_combinations;

        let mut new_vec = Vec::with_capacity(vec1.len() * vec2.len());
        for &val1 in vec1.iter(){
            for &val2 in vec2.iter(){
                new_vec.push([val1, val2, *node_id])
            }
        }
        new_vec
    }
}

//------------------------------------------------------------------------------Intervals-------------------------------------------------------------

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
    info!("BVD: Chunk size {}", chunk_size);


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



