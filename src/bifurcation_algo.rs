use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;
use std::{thread};
use std::cmp::{max, min};
use std::time::Instant;
use bifurcation::bifurcation_analysis_meta;
use crossbeam_channel::unbounded;
use gfaR_wrapper::NGfa;
use log::{info};
use crate::bifurcation_helper::{all_combinations, all_combinations_self, get_all_pairs, get_shared_index, get_shared_index_low_mem, node2index, sort_nodes};
use crate::helper::chunk_inplace;


/// Wrapper function using NGFA input and the precomnputed Hashsets and Node2pos hashmaps
///
/// Returns
/// - Vector of all bubbles (u32, u32) - (start, end)
/// - VEctor of all intervals (usize, u32, u32, u32) - (pindex, index1, index2, bubble_id)
pub fn bifurcation_bubble(graph: &Arc<NGfa>,  threads: &usize, jo2: Vec<(Vec<u32>, Vec<(u32, u32)>)>) -> (Vec<(usize, u32, u32, u32)>, Vec<(u32, u32)>){
    info!("Running bifurcation analysis");
    let mut result;
    // This returns all bubbles
    result = bvd2(&graph, threads.clone(), jo2);



    result.sort_by_key(|a|a.0);


    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let chunks = chunk_inplace(f, *threads as usize);

    let (s, r) = unbounded();
    let resss = result.clone();

    let result_arc = Arc::new(result);

    info!("Merge");
    for x in chunks{
        let ff1 = result_arc.clone();
        let aa1 = graph.clone();
        let s1 = s.clone();
        let _handle = thread::spawn(move || {
            for y in x.iter(){
                let paa = aa1.paths.get(*y).unwrap();
                let path2index = node2index(paa);
                let mut test = Vec::new();
                for (i, (start, end)) in ff1.iter().enumerate(){
                    if path2index.contains_key(&(*start as u32)) && path2index.contains_key(&(*end as u32)) {
                        if start != end {
                            let i1 = path2index.get(&(*start as u32)).unwrap();
                            let i2 = path2index.get(&(*end as u32)).unwrap();
                            if i1.len() == 1 && i2.len() == 1{
                                test.push((*y, min(i1[0], i2[0]), max(i1[0], i2[0]), i as u32));

                            } else {
                                test.extend(all_combinations(path2index.get(&(*start as u32)).unwrap(), path2index.get(&(*end as u32)).unwrap(), &(*y as u32), &(i as u32)));
                            }
                        } else {
                            test.extend(all_combinations_self(path2index.get(&(*start as u32)).unwrap(), &(*y as u32), &(i as u32)));
                        }
                    }
                }
                s1.send(test).expect("Help");

            }


        });
    }


    let mut res = Vec::new();
    for _x in 0..graph.paths.len(){
        let data = r.recv().unwrap();
        res.extend(data);
    }
//    drop(graph);
    info!("Merge done");
    //let f = result_arc.as_ref().clone();


    //let ff2 = Vec::new();
    return (res, resss);

}

/// Wrapper function using NGFA input and the precomnputed Hashsets and Node2pos hashmaps
///
/// Returns
/// - Vector of all bubbles (u32, u32) - (start, end)
/// - VEctor of all intervals (usize, u32, u32, u32) - (pindex, index1, index2, bubble_id)
pub fn bifurcation_bubble_lowmem(graph: &NGfa, threads: &usize) -> (Vec<(usize, u32, u32, u32)>, Vec<(u32, u32)>){
    info!("Running bifurcation analysis");
    let mut result;
    // This returns all bubbles
    result = bvd_low_memory(graph, threads.clone());

    result.sort_by_key(|a|a.0);


    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let chunks = chunk_inplace(f, *threads as usize);

    let (s, r) = unbounded();
    let resss = result.clone();

    let result_arc = Arc::new(result);
    let arc1 = Arc::new(graph.paths.clone());

    info!("Merge");
    for x in chunks{
        let ff1 = result_arc.clone();
        let aa1 = arc1.clone();
        let s1 = s.clone();
        let _handle = thread::spawn(move || {
            for y in x.iter(){
                let paa = aa1.get(*y).unwrap();
                let path2index = node2index(paa);
                let mut test = Vec::new();
                for (i, (start, end)) in ff1.iter().enumerate(){
                    if path2index.contains_key(&(*start as u32)) && path2index.contains_key(&(*end as u32)){
                        if start != end{
                            test.extend(all_combinations(path2index.get(&(*start as u32)).unwrap(), path2index.get(&(*end as u32)).unwrap(), &(*y as u32), &(i as u32)));


                        } else {
                            test.extend(all_combinations_self(path2index.get(&(*start as u32)).unwrap(), &(*y as u32), &(i as u32)));

                        }
                    }
                }
                s1.send(test).expect("Help");

            }


        });
    }

    let mut res = Vec::new();
    for _x in 0..graph.paths.len(){
        let data = r.recv().unwrap();
        res.extend(data);
    }
    info!("Merge done");

    //let ff2 = Vec::new();
    return (res, resss);

}


/// **Wrapper function for genome graphs**
///
/// - Multithreaded with Crossbeam
/// - Return (Name1, Name2) -> Vec<[[<start, stop>] (name1), [start, stop] (name2)]
/// TODO:
/// - Make outcome clear
pub fn bvd2(graph: &Arc<NGfa>, threads: usize, jo2: Vec<(Vec<u32>, Vec<(u32, u32)>)>) -> Vec<(u32, u32)>{
    let (s, r) = unbounded();
    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();

    let new_vec = sort_nodes(&graph.paths);

    let pairs2 = get_all_pairs(&f);
    let pp = pairs2.len().clone();
    // Chunk the pairs
    let chunks = chunk_inplace(pairs2, threads);

    // Shared references
    let arc3 = Arc::new(jo2);

    let arc5 = Arc::new(new_vec);

    // Handles
    //let mut handles = Vec::new();

    // Iterate over chunks
    for chunk in chunks{

        let s1 = s.clone();
        let test2 = arc3.clone();
        let apath = arc5.clone();
        let _handle = thread::spawn(move || {



            for pair2 in chunk.iter(){
                let start = Instant::now();

                info!("This pair:  {:?}", &pair2);
                let p3 = test2.get(pair2.0).unwrap();
                let p4 = test2.get(pair2.1).unwrap();
                // In my example this was 300 ms    let elapsed = start.elapsed();


                let elapsed = start.elapsed();

                // Debug format
                println!("Debug: {:?}", elapsed);
                //
                //     // Debug format
                //     println!("Debug: {:?}", elapsed);
                let mut shared_index = get_shared_index(&apath.get(pair2.0).unwrap(), &apath.get(pair2.1).unwrap(), p3, p4);
                let elapsed = start.elapsed();
                shared_index.sort();

                // Debug format
                println!("Debug: {:?} {}", elapsed, shared_index.len());

                let result = bifurcation_analysis_meta(&shared_index);

                //let result = Vec::new();
                let elapsed = start.elapsed();
                //
                //     // Debug format
                     println!("Debug231: {:?}", elapsed);
                let f: HashSet<(u32, u32)> = HashSet::from_iter(result);
                s1.send(f).expect("Help");    let elapsed = start.elapsed();

                // Debug format
                println!("Debug: {:?}", elapsed);



            }
        });
    }

    let mut res: HashSet<(u32, u32)> = HashSet::new();
    //let mut res = Vec::new();
    for _x in 0..pp{
        let data = r.recv().unwrap();
        res.extend(data.into_iter());
    }
    let mut res: Vec<(u32, u32)> = res.into_iter().collect();
    res.sort();
    res

}


pub fn bvd_low_memory(graph: &NGfa, threads: usize) -> Vec<(u32, u32)>{
    let (s, r) = unbounded();

    // Get all pairs of paths - (n*n-1)/2
    let f: Vec<usize> = (0..graph.paths.len()).collect();
    let pairs2 = get_all_pairs(&f);
    let pp = pairs2.len().clone();
    // Chunk the pairs
    let chunks = chunk_inplace(pairs2, threads);

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
                // Get the shared index
                let p1 = arc11.get(pair2.0).unwrap();
                let p2 = arc11.get(pair2.1).unwrap();
                let shared_index = get_shared_index_low_mem(&p1, &p2);
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

