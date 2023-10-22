use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use related_intervals::make_nested;


pub fn get_relations(input: &Vec<(String, Vec<[u32;3]>)>, threads: &usize) -> HashSet<(u32, u32)> {

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let result: HashSet<(u32, u32)> = thread_pool.install(|| {
        input.par_chunks(threads*3).flat_map(|chunk| process1(&chunk).into_par_iter())
            .collect()
    });
    result
}

pub fn process1(input: &[(String, Vec<[u32;3]>)]) -> HashSet<(u32, u32)>{
    let mut result = HashSet::new();
    for mut x in input.iter(){
        println!("DONE");
        result.extend(make_nested(&x.1));
    }
    return result
}

pub fn test1(val: &HashSet<(u32, u32)>, bubbles_len: usize){
    let mut g = vec![0; bubbles_len];
    let mut f: Vec<_> = val.iter().collect();
    f.sort_by_key(|a| (a.1));
    for x in f.iter(){
        g[x.1 as usize] += 1;
    }
    let mut a = HashMap::new();
    for x in val.into_iter(){
        a.entry(x.0).or_insert_with(Vec::new).push(x.1);
    }

}
