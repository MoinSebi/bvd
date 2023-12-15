use std::cmp::{max, min};
use std::usize;
use gfa_reader::{NCGfa};
use std::collections::{HashSet};
use std::time::Instant;
use log::{info, trace};
use crate::writer::gfapos_wrapper;
use rayon::prelude::*;
use crate::bifurcation_algo::index_wrapper;
use crate::bifurcation_helper::{IndexMetadata};



/// GFA2MAF algo
pub fn iter_dict(graph: &NCGfa<()>, threads: &usize, pairs_index: &Vec<(usize, usize)>) -> Vec<(u32, u32)>{
    info!("BVD: gfa2paf");
    let start = Instant::now();

    let pairs_index = pairs_index.clone();
    let index2pos = gfapos_wrapper(&graph, &1);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let mut chunk_size;
    if graph.paths.len() % *threads != 0{
        chunk_size = ((pairs_index.len() / *threads))  + 1

    } else {
        chunk_size = max(1, pairs_index.len() / *threads)
    }
    chunk_size = min(500, chunk_size);

    let (path_merges, index) = index_wrapper(&graph);
    let pp: Vec<_> = path_merges.iter().map(|s| s.iter().cloned().collect::<HashSet<u32>>()).collect();




    let results: HashSet<(u32, u32)> = thread_pool.install(|| {
        pairs_index.par_chunks(chunk_size).map(|chunk| {
            let mut result: HashSet<(u32, u32)> = HashSet::new();
            for pair in chunk.iter() {
                let start = Instant::now();

                let shared_index = &pp[pair.1];

                let ii = &index[pair.1];
                let res = gfa2paf_h(&index2pos[pair.0], &index2pos[pair.1], &path_merges[pair.0], ii, &shared_index);
                let duration = start.elapsed();
                trace!("Time elapsed is: {:?}", duration);
                result.extend(res);

            }

            result

        }).flatten().collect()
    });
    let duration = start.elapsed();
    trace!("Total time elapsed is: {:?}", duration);

    let r_vec: Vec<(u32, u32)> = results.into_iter().collect();
    return r_vec

}


/// Gfa2paf with dict
pub fn gfa2paf_h(distance_p1: &[usize], distance_p2: &[usize], path_merge: &Vec<u32>, index: &IndexMetadata, shared_index_p2: &HashSet<u32>) -> Vec<(u32, u32)> {

    // Last shared bubble index + bubble
    let mut last_shared_bubble_index1 = 0;
    let mut last_shared_bubble_index2 = 0;
    let mut last_shared_bub = 0;

    // Last min distance index + bubble
    let mut last_mindis_index1 = 0;
    let mut last_mindis_index2 = 0;
    let mut last_mindis_bub = 0;

    // Last distance
    let mut last_distance2;
    let mut last_distance1;

    // Min distance
    let mut min_distance = usize::MAX;

    // Path 1 index
    let mut p1_index = 0;

    // Last distance
    // Result vector
    let mut result = Vec::new();


    // Iterate over p1
    while p1_index < path_merge.len() {

        // Calculate the distance on the first path
        last_distance1 = distance_p1[p1_index] - distance_p1[last_shared_bubble_index1];

        // If the distance double the size of the min distance, add the last shared bubble to the result vector
        if last_distance1 / 2 > min_distance{
            min_distance = usize::MAX;
            if (last_shared_bubble_index1 +1) != last_mindis_index1 || (last_shared_bubble_index2 +1) as usize != last_mindis_index2 {
                result.push((last_shared_bub, last_mindis_bub));
            }
            last_shared_bub = last_mindis_bub;
            last_shared_bubble_index1 = last_mindis_index1;
            p1_index = last_mindis_index1;
            last_shared_bubble_index2 = last_mindis_index2 as u32;
        }
        // if the node is contained in the p2 we continue
        if shared_index_p2.contains(&path_merge[p1_index]) {
            // Get all index of this node
            let possible_index = index.get(&path_merge[p1_index]);

            // Iterate over the indices
            for x in possible_index.iter(){
                // If you have a bigger index than the last bubble
                if x > &last_shared_bubble_index2 {

                    // calculate the distance
                    last_distance2 = distance_p2[*x as usize] - distance_p2[last_shared_bubble_index2 as usize];

                    // Then check if the total sequence is smaller than the min distance
                    let distance = last_distance1 + last_distance2;
                    if distance < min_distance {

                        // Temporarily save the index and the other information
                        min_distance = distance;
                        last_mindis_index1 = p1_index;
                        last_mindis_index2 = *x as usize;
                        last_mindis_bub = path_merge[p1_index];
                    }
                    // Everything which is bigger, stop
                    break;
                }
            }
        }
        p1_index += 1;
    }
    result
}

pub fn gfa2paf_h2(d1: &[usize], d2: &[usize], path_merges: &Vec<u32>, inde: &IndexMetadata, shared_index: &HashSet<u32>) -> Vec<(u32, u32)> {
    let mut last_shared_i1 = 0;
    let mut last_shared_i2 = 0;
    let mut last_mindis_index1 = 0;
    let mut last_mindis_index2 = 0;

    let mut last_shared_bub = 0;
    let mut last_mindis_bub = 0;
    let mut result = Vec::new();
    let mut min_distance = usize::MAX;
    let l1 = &path_merges;
    let mut i = 0;
    let mut last_distance2 = 0;
    let mut aa = 0;
    let mut index = inde.clone();
    let inde = &mut index;
    //println!("{}", l1.len());
    //println!("{}", shared_index.len());
    while i < l1.len() {
        let last_distance1 = d1[i] - d1[last_shared_i1];
        if last_distance1 / 2 > min_distance{
            min_distance = usize::MAX;
            if (last_shared_i1+1) != last_mindis_index1 || (last_shared_i2+1) as usize != last_mindis_index2 {
                result.push((last_shared_bub, last_mindis_bub));
            }
            inde.get_index_mut(&l1[i]).0 +=  aa as u32;
            last_shared_bub = last_mindis_bub;
            last_shared_i1 = last_mindis_index1;
            i = last_mindis_index1;
            last_shared_i2 = last_mindis_index2 as u32;
        }
        if shared_index.contains(&l1[i]) {
            let f = inde.get(&l1[i]);
            for (i2, x) in f.iter().enumerate(){
                if x > &last_shared_i2 {

                    last_distance2 = d2[*x as usize] - d2[last_shared_i2 as usize];

                    let distance = last_distance1 + last_distance2;
                    if distance < min_distance {
                        aa = i2;
                        min_distance = distance;
                        last_mindis_index1 = i;
                        last_mindis_index2 = *x as usize;
                        last_mindis_bub = l1[i];
                    }
                    break;
                }
            }
            if last_distance1 / 2 > min_distance || last_distance2 / 2 > min_distance {
                min_distance = usize::MAX;
                if (last_shared_i1+1) != last_mindis_index1 || (last_shared_i2+1) as usize != last_mindis_index2 {
                    result.push((last_shared_bub, last_mindis_bub));
                }
                inde.get_index_mut(&l1[i]).0 +=  aa as u32;
                last_shared_bub = last_mindis_bub;
                last_shared_i1 = last_mindis_index1;
                i = last_mindis_index1;
                last_shared_i2 = last_mindis_index2 as u32;

            }
        }
        i += 1;
    }
    result
}



