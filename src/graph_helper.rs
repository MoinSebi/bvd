use std::sync::{Arc};
use std::thread;
use crossbeam_channel::unbounded;
use gfaR_wrapper::{NGfa, NPath};
use hashbrown::{HashMap};
use crate::bifurcation_helper::{node2index_low_mem};
use crate::helper::chunk_inplace;

/// Convert index in the graph to positional information
/// Index based - not node based
/// [10,30,31,32,45]
pub fn graph2pos(graph: & Arc<NGfa>) -> Vec<Vec<usize>>{
    let mut result_hm: Vec<Vec<usize>> = Vec::new();
    for x in graph.paths.iter(){
        let mut vec_pos: Vec<usize> = Vec::new();
        let mut position: usize = 0;
        for y in x.nodes.iter(){
            position += graph.nodes.get(y).unwrap().len;
            vec_pos.push(position);
        }
        result_hm.push(vec_pos);
    }
    result_hm
}

/// Creates a "node to index" index for each path
///
/// This is mainly a wrapper function for multithreading
pub fn node2index_wrapper(paths: &Vec<NPath>, threads: &usize) -> (Vec<Vec<u32>>, Vec<Vec<(u32, u32)>>){

    // Number of path for computation + chunking in number of threads
    let f: Vec<usize> = (0..paths.len()).collect();
    let chunks = chunk_inplace(f, *threads as usize);

    // Create reference for each thread
    let path_arc = Arc::new(paths.clone());

    // Crossbeam setup
    let (s, r) = unbounded();

    // Iterate over chunks
    for paths_chunks in chunks{
        let sender_copy = s.clone();
        let path_arc2 = path_arc.clone();
        let _handle = thread::spawn(move || {
            for path_index in paths_chunks.iter(){
                // Get the path
                let path = path_arc2.get(*path_index).unwrap();

                // Compute the index
                let node2index_data = node2index_low_mem(&path.nodes);

                // Send data
                sender_copy.send((*path_index, node2index_data)).expect("ERROR");
            }
        });
    }
    // Pre-result vector
    let mut pre_result = Vec::new();

    // Iterate over data from threads
    for _path_index in 0..paths.len(){
        let data_received = r.recv().unwrap();
        pre_result.push(data_received);
    }
    // Sort and create new vector which is in order of the path
    pre_result.sort_by_key(|a| a.0);

    // Only copy the real data (but in the order of
    let result2: Vec<Vec<u32>> = pre_result.iter().map(|a| a.1.0.clone()).collect();
    let result3: Vec<Vec<(u32, u32)>> = pre_result.into_iter().map(|a| a.1.1).collect();



    return (result2, result3)
}