use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
// use std::sync::Arc;
// use std::thread;
// use bifurcation::helper::get_all_pairs;
// use crossbeam_channel::unbounded;
// use gfa_reader::NCGfa;
// use log::info;
// use crate::helper::chunk_inplace;
// use crate::node2index::node2index_low_mem2;
//
// // use std::collections::HashSet;
// // use std::iter::FromIterator;
// // use std::sync::Arc;
// // use std::{thread};
// // use std::cmp::{max, min};
// // use std::time::Instant;
use bifurcation::bifurcation_analysis_meta;
// // use crossbeam_channel::unbounded;
// // use log::{debug, info};
// // use crate::bifurcation_helper::{all_combinations, all_combinations_self, get_all_pairs, get_shared_index, get_shared_index_low_mem, node2index, sort_nodes};
// // use crate::helper::chunk_inplace;
// // use rayon::prelude::*;
// //
// //


use std::sync::Arc;
use std::thread;
use bifurcation::helper::get_all_pairs;
use crossbeam_channel::unbounded;
use gfa_reader::NCGfa;
use log::info;
use crate::bifurcation_helper::{all_combinations, all_combinations_self, index_meta, intersection_two_pointer, path2combi};
use crate::helper::chunk_inplace;

pub fn biiii(graph: &NCGfa<()>, threads: &usize) {
    let mut result;

    result = bvd_low_memory(graph, threads);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();




}



/// Wrapper function using NGFA input and the precomnputed Hashsets and Node2pos hashmaps
///
/// Returns
/// - Vector of all bubbles (u32, u32) - (start, end)
/// - VEctor of all intervals (usize, u32, u32, u32) - (pindex, index1, index2, bubble_id)
pub fn bifurcation_low(graph: &NCGfa<()>, threads: &usize) -> (Vec<(usize, u32, u32, u32)>, Vec<(u32, u32)>){
    info!("Running bifurcation analysis");

    // Initialize the result vector
    let mut result;

    // Returns all bubbles
    result = bvd_low_memory(graph, threads);

    result.sort_by_key(|a|a.0);
    println!("{:?}", result);


    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let chunks = chunk_inplace(f, *threads as usize);

    let (s, r) = unbounded();
    let resss = result.clone();

    let bubbles = Arc::new(result);
    let arc1 = Arc::new(graph.paths.clone());

    info!("BVG: Find intervals");
    for x in chunks{
        let arc_result = bubbles.clone();
        let arc_path = arc1.clone();
        let arc_sender = s.clone();
        let _handle = thread::spawn(move || {
            for y in x.iter(){
                let paa = arc_path.get(*y).unwrap();
                let paa2 = path2combi(paa);
                let path2index = index_meta(&paa2);
                let mut test = Vec::new();
                for (i, (start, end)) in arc_result.iter().enumerate(){
                    let i1 = path2index.1[*start as usize];
                    let i2 = path2index.1[*end as usize];
                    println!("{:?}", path2index);
                    println!("{:?}", i1);
                    println!("{:?}\n", i2);
                    println!("{:?} {:?}", start, end);


                    if i1 != (0,0)  && i2 != (0,0){
                        let a1 = &path2index.0[i1.0 as usize..(i1.0 + i1.1) as usize];
                        if start != end{
                            let a2 = &path2index.0[i1.0 as usize..(i1.0 + i1.1) as usize];

                            test.extend(all_combinations(a1, a2, &(*y as u32), &(i as u32)));


                        } else {
                            test.extend(all_combinations_self(a1, &(*y as u32), &(i as u32)));

                        }
                    }
                }
                println!("{:?}", test);
                arc_sender.send(test).expect("Help");

            }


        });
    }

    let mut res = Vec::new();
    for _x in 0..graph.paths.len(){
        let data = r.recv().unwrap();
        res.extend(data);
    }
    info!("BVD: Find intervals done");

    //let ff2 = Vec::new();
    return (res, resss);

}
use rayon::prelude::*;

/// Get all bubbles in the graph
///
pub fn aaaa(graph: &NCGfa<()>, threads: &usize) -> Vec<(u32, u32)>{


    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs_index = get_all_pairs(&f);

    // Number of pairs
    //let pp = pairs_index.len().clone();


    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    // Chunk the pairs
    let c: Vec<_> = pairs_index.chunks(3).collect();
    //let chunks = chunk_inplace(pairs_index, *threads);
    // Shared references
    let arc1 = Arc::new(graph.paths.clone());

    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(10) // Process in chunks of 3 elements (you can adjust the chunk size).
            .flat_map(|chunk| process_chunk(chunk, graph).into_par_iter()) // Pass external data to the processing function.
            .collect()

    });
    let mut results: Vec<_>= results.into_iter().collect();
    results.sort();
    results

}

fn process_chunk(chunk: &[(usize, usize)], graph: &NCGfa<()>) -> HashSet<(u32, u32)>{
    let mut a = HashSet::new();
    for pair2 in chunk.iter(){
        // Get the paths

        let path1 = graph.paths.get(pair2.0).unwrap();
        let path2 = graph.paths.get(pair2.1).unwrap();

        // Get the index
        let mut path1_combi = path2combi(path1);
        let mut path2_combi = path2combi(path2);

        let shared_index = intersect_index(&mut path1_combi, &mut path2_combi);


        let result = bifurcation_analysis_meta(&shared_index);
        //let result = Vec::new();
        let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
        a.extend(f);

    }

    a
}


/// Get all bubbles in the graph
///
pub fn bvd_low_memory(graph: &NCGfa<()>, threads: &usize) -> Vec<(u32, u32)>{

    // Sender and Receiver
    let (s, r) = unbounded();

    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs_index = get_all_pairs(&f);

    // Number of pairs
    let pp = pairs_index.len().clone();


    // Chunk the pairs
    let c: Vec<_> = pairs_index.chunks(3).collect();
    let chunks = chunk_inplace(pairs_index, *threads);
    // Shared references
    let arc1 = Arc::new(graph.paths.clone());




    // Handles
    //let mut handles = Vec::new();

    // Iterate over chunks
    for chunk in chunks{

        let s1 = s.clone();
        let arc11 = arc1.clone();

        let _handle = thread::spawn(move || {
            for pair2 in chunk.iter(){
                // Get the paths
                let path1 = arc11.get(pair2.0).unwrap();
                let path2 = arc11.get(pair2.1).unwrap();

                // Get the index
                let mut path1_combi = path2combi(path1);
                let mut path2_combi = path2combi(path2);

                let shared_index = intersect_index(&mut path1_combi, &mut path2_combi);


                let result = bifurcation_analysis_meta(&shared_index);
                //let result = Vec::new();
                let f: HashSet<(u32, u32)> = HashSet::from_iter(result.iter().cloned());
                s1.send(f).expect("Help");


            }
        });
    }

    let mut res: HashSet<(u32, u32)> = HashSet::new();
    for _x in 0..pp{
        let data = r.recv().unwrap();
        res.extend(data);
    }
    let mut res: Vec<(u32, u32)> = res.into_iter().collect();
    res.sort();
    res
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
        let (start, end) = aa.1[*x.0 as usize + *x.1 as usize];
        dd1 = &aa.0[start as usize..(start + end) as usize];
        let (start, end) = aa2.1[*x.0 as usize + *x.1 as usize];
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







// // /// Wrapper function using NGFA input and the precomnputed Hashsets and Node2pos hashmaps
// // ///
// // /// Returns
// // /// - Vector of all bubbles (u32, u32) - (start, end)
// // /// - VEctor of all intervals (usize, u32, u32, u32) - (pindex, index1, index2, bubble_id)
// // pub fn bifurcation_bubble(graph: &Arc<NGfa>, threads: &usize, index1: Vec<Vec<u32>>, index2: Vec<Vec<(u32, u32)>>) -> (Vec<(usize, u32, u32, u32)>, Vec<(u32, u32)>){
// //     info!("Running bifurcation analysis");
// //     let mut result;
// //     let index1 = Arc::new(index1);
// //     let index2 = Arc::new(index2);
// //     // This returns all bubbles
// //     result = bvd2(&graph, threads.clone(), &index1, &index2);
// //
// //
// //
// //     result.sort_by_key(|a|a.0);
// //
// //
// //     let f: Vec<usize> = (0..graph.paths.len()).collect();
// //     let chunks = chunk_inplace(f, *threads as usize);
// //
// //     let (s, r) = unbounded();
// //     let resss = result.clone();
// //
// //     let result_arc = Arc::new(result);
// //
// //     info!("Merge");
// //     for x in chunks{
// //         let ff1 = result_arc.clone();
// //         let aa1 = graph.clone();
// //         let s1 = s.clone();
// //         let index11 = index1.clone();
// //         let index22 = index2.clone();
// //         let _handle = thread::spawn(move || {
// //             for y in x.iter(){
// //                 let mut test = Vec::new();
// //
// //                 let index1_local = index11.get(*y).unwrap();
// //                 let index2_local = index22.get(*y).unwrap();
// //
// //                 for (i, (start, end)) in ff1.iter().enumerate() {
// //                     let vv = (index2_local.len() > *start as usize) && (index2_local.len() > *end as usize);
// //                     if vv && index2_local[*start as usize] != (0, 0) && index2_local[*end as usize] != (0, 0) {
// //                         let path1_i = index2_local.get(*start as usize).unwrap();
// //                         let path2_i = index2_local.get(*end as usize).unwrap();
// //
// //                         let path1_islice = &index1_local[path1_i.0 as usize..(path1_i.0 + path1_i.1) as usize];
// //                         let path2_islice = &index1_local[path2_i.0 as usize..(path2_i.0 + path2_i.1) as usize];
// //
// //
// //                         if start != end {
// //                             if path1_islice.len() == 1 && path2_islice.len() == 1 {
// //                                 test.push((*y, min(path1_islice[0], path2_islice[0]), max(path1_islice[0], path2_islice[0]), i as u32));
// //                             } else {
// //                                 test.extend(all_combinations(path1_islice, path2_islice, &(*y as u32), &(i as u32)));
// //                             }
// //                         } else {
// //                             test.extend(all_combinations_self(path2_islice, &(*y as u32), &(i as u32)));
// //                         }
// //                     }
// //                 }
// //                 s1.send(test).expect("Help");
// //
// //             }
// //
// //
// //         });
// //     }
// //
// //
// //     let mut res = Vec::new();
// //     for _x in 0..graph.paths.len(){
// //         let data = r.recv().unwrap();
// //         res.extend(data);
// //     }
// // //    drop(graph);
// //     info!("Merge done");
// //     //res.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
// //     return (res, resss);
// //
// // }
// //
//
//
// /// Wrapper function using NGFA input and the precomnputed Hashsets and Node2pos hashmaps
// ///
// /// Returns
// /// - Vector of all bubbles (u32, u32) - (start, end)
// /// - VEctor of all intervals (usize, u32, u32, u32) - (pindex, index1, index2, bubble_id)
// pub fn bifurcation_bubble_lowmem(graph: &NCGfa<()>, threads: &usize) -> (Vec<(usize, u32, u32, u32)>, Vec<(u32, u32)>){
//     info!("Running bifurcation analysis");
//
//     // Initialize the result vector
//     let mut result;
//
//     // Returns all bubbles
//     result = bvd_low_memory(graph, threads.clone());
//
//     result.sort_by_key(|a|a.0);
//
//
//     let f: Vec<usize> = (0..graph.paths.len()).collect();
//     let chunks = chunk_inplace(f, *threads as usize);
//
//     let (s, r) = unbounded();
//     let resss = result.clone();
//
//     let result_arc = Arc::new(result);
//     let arc1 = Arc::new(graph.paths.clone());
//
//     info!("Merge");
//     for x in chunks{
//         let ff1 = result_arc.clone();
//         let aa1 = arc1.clone();
//         let s1 = s.clone();
//         let _handle = thread::spawn(move || {
//             for y in x.iter(){
//                 let paa = aa1.get(*y).unwrap();
//                 let path2index = node2index(paa);
//                 let mut test = Vec::new();
//                 for (i, (start, end)) in ff1.iter().enumerate(){
//                     if path2index.contains_key(&(*start as u32)) && path2index.contains_key(&(*end as u32)){
//                         if start != end{
//                             test.extend(all_combinations(path2index.get(&(*start as u32)).unwrap(), path2index.get(&(*end as u32)).unwrap(), &(*y as u32), &(i as u32)));
//
//
//                         } else {
//                             test.extend(all_combinations_self(path2index.get(&(*start as u32)).unwrap(), &(*y as u32), &(i as u32)));
//
//                         }
//                     }
//                 }
//                 s1.send(test).expect("Help");
//
//             }
//
//
//         });
//     }
//
//     let mut res = Vec::new();
//     for _x in 0..graph.paths.len(){
//         let data = r.recv().unwrap();
//         res.extend(data);
//     }
//     info!("Merge done");
//
//     //let ff2 = Vec::new();
//     return (res, resss);
//
// }
// //
// //
// // /// **Wrapper function for genome graphs**
// // ///
// // /// - Multithreaded with Crossbeam
// // /// - Return (Name1, Name2) -> Vec<[[<start, stop>] (name1), [start, stop] (name2)]
// // /// TODO:
// // /// - Make outcome clear
// // pub fn bvd2(graph: &Arc<NGfa>, threads: usize, arc3: &Arc<Vec<Vec<u32>>>, arc4: &Arc<Vec<Vec<(u32, u32)>>>) -> Vec<(u32, u32)>{
// //     info!("BVD indexing");
// //
// //     let (s, r) = unbounded();
// //     // Get all pairs of paths - (n*n-1)/2
// //     let f: Vec<usize> = (0..graph.paths.len()).collect();
// //
// //     let new_vec = sort_nodes(&graph.paths);
// //
// //     let pairs2 = get_all_pairs(&f);
// //     let pp = pairs2.len().clone();
// //     // Chunk the pairs
// //     let chunks = chunk_inplace(pairs2, threads);
// //
// //
// //     let arc5 = Arc::new(new_vec);
// //
// //     // Handles
// //     //let mut handles = Vec::new();
// //
// //     // Iterate over chunks
// //     for chunk in chunks{
// //
// //         let s1 = s.clone();
// //         let test2 = arc3.clone();
// //         let test3 = arc4.clone();
// //
// //         let apath = arc5.clone();
// //         let _handle = thread::spawn(move || {
// //
// //
// //
// //             for pair2 in chunk.iter(){
// //                 let p3 = test2.get(pair2.0).unwrap();
// //                 let p31 = test3.get(pair2.0).unwrap();
// //
// //                 let p4 = test2.get(pair2.1).unwrap();
// //                 let p41 = test3.get(pair2.1).unwrap();
// //
// //
// //
// //                 let nodes1 = &apath.get(pair2.0).unwrap()[..];
// //                 let nodes2 =&apath.get(pair2.1).unwrap()[..];
// //
// //                 let mut shared_index = get_shared_index(nodes1, nodes2, p3, p31, p4, p41);
// //                 // Debug format
// //
// //                 let mut result = bifurcation_analysis_meta(&shared_index);
// //
// //                 //let result = Vec::new();
// //                 result.par_sort();
// //                 s1.send(result).expect("Help");
// //
// //                 // Debug format
// //
// //
// //
// //             }
// //         });
// //     }
// //
// //     let mut res: HashSet<(u32, u32)> = HashSet::new();
// //     //let mut res = Vec::new();
// //     for _x in 0..pp{
// //
// //         let data = r.recv().unwrap();
// //         res.extend(data.into_iter());
// //     }
// //     let mut res: Vec<(u32, u32)> = res.into_iter().collect();
// //     res.par_sort();
// //     res
// //
// // }
// //
// //
//
