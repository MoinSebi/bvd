use std::collections::HashSet;
use std::sync::{Arc};
use std::thread;
use crossbeam_channel::unbounded;
use gfaR_wrapper::{NGfa, NPath};
use hashbrown::{HashMap};
use crate::bifurcation_helper::{path2index_hashmap};
use crate::helper::chunk_inplace;

/// Convert index in the graph to positional information
/// Index based - not node based
/// [10,30,31,32,45]
pub fn graph2pos(graph: & Arc<NGfa>) -> HashMap<String, Vec<usize>>{
    let mut result_hm: HashMap<String, Vec<usize>> = HashMap::new();
    for x in graph.paths.iter(){
        let mut vec_pos: Vec<usize> = Vec::new();
        let mut position: usize = 0;
        for y in x.nodes.iter(){
            position += graph.nodes.get(y).unwrap().len;
            vec_pos.push(position);
        }
        result_hm.insert(x.name.clone(), vec_pos);
    }
    result_hm
}

/// Make index
pub fn index_faster(paths: &Vec<NPath>, threads: &usize) -> Vec<(Vec<u32>, Vec<(u32, u32)>)>{

    let f: Vec<usize> = (0..paths.len()).collect();
    let chunks = chunk_inplace(f, *threads as usize);
    let ps = Arc::new(paths.clone());

    let (s, r) = unbounded();

    for x in chunks{
        let s1 = s.clone();
        let ps2 = ps.clone();
        let _handle = thread::spawn(move || {
            for y in x.iter(){
                let gg = ps2.get(*y).unwrap();
                let f = path2index_hashmap(gg);
                s1.send((y.clone(), f)).expect("Help123");

            }


        });
    }

    let mut res = Vec::new();
    for _x in 0..paths.len(){
        let data = r.recv().unwrap();
        res.push((data));
    }
    res.sort_by_key(|a| a.0);
    let res1 = res.into_iter().map(|a| a.1).collect();

    return res1
}