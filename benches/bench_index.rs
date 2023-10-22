use std::collections::BTreeMap;
use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Criterion};
use gfa_reader::{NCGfa, NCPath};
use hashbrown::HashMap;


pub fn test() -> Vec<u32> {
     (0..100000).into_iter().collect()
}





/// Approach1 - Hashmap
/// (u32, bool) -> Vec<usize>
pub fn index_hashmap<'a, 'b>(path: &'b Vec<(&'a u32, &'a bool)>) -> HashMap<&'b (&'a u32, &'a bool), Vec<usize>> {
    let mut index: HashMap<&(&u32, &bool),  Vec<usize>> = HashMap::new();
    for (i, x) in path.iter().enumerate(){
        if index.contains_key(x){
            index.get_mut(x).unwrap().push(i);
        }else{
            index.insert(&x, vec![i]);
        }
    }
    index
}

/// Approach2 - BTreeMap
/// (u32, bool) -> Vec<usize>
pub fn index_btree<'a, 'b>(path: &'b Vec<(&'a u32, &'a bool)>) -> BTreeMap<&'b (&'a u32, &'a bool), Vec<usize>> {
    let mut index: BTreeMap<&(&u32, &bool), Vec<usize>> = BTreeMap::new();
    for (i, x) in path.iter().enumerate(){
        if index.contains_key(x){
            index.get_mut(x).unwrap().push(i);
        }else{
            index.insert(&x, vec![i]);
        }
    }
    index
}


/// Index with meta information
/// (u32, u32) -> From this index to this index
/// (u32) -> Index of the node
///
/// Indexing: node_id*2 + direction (0 or 1)
///
/// Size:
/// 32 bit + 64 bit = 12 byte
pub fn index_meta<'a, 'b>(path: &'b Vec<(&'a u32, &'a bool)>) -> (Vec<(u32)>, Vec<(u32, u32)>){

    /// Create a vector of the nodes
    let mut f = Vec::with_capacity(path.len());


    let mut m = path.iter().max().unwrap().clone();
    let mut m: &(&u32, &bool) = &(&0, &true);
    for (i, x) in path.iter().enumerate(){
        // Push node and index
        f.push((x,i));
        if x > m{
            m = x;
        }
    }
    // Sort by node
    f.sort();

    // Only return the index (sorted by node)
    let index_list: Vec<_> = f.iter().map(|a| a.1 as u32).collect();

    let mut prev_val = f[0].0;
    let mut prev_index = 0;

    let mut from_to = vec![(0, 0); (*m.0 as usize + 1) * 2];
    // These are all the nodes
    for (i,x) in f.iter().enumerate(){
        // if it is not the same than the last value, add it to the list
        if x.0 != prev_val {

            from_to[*prev_val.0 as usize + *prev_val.1 as usize] = (prev_index as u32, (i - prev_index) as u32);
            prev_index = i;
            prev_val = x.0;
        }

    }
    // Add again at the end
    from_to[*prev_val.0 as usize + *prev_val.1 as usize] = (prev_index as u32, (f.len()- prev_index) as u32);

    (index_list, from_to)

}

/// Approach4 - Index of index
/// Similar as above -
/// Sorted list of the node name, then the index
///
/// Size:
/// 64 bit + 64 bit = 16 byte
///
///
pub fn index_index<'a, 'b>(nodes: &'b Vec<(&'a u32, &'a bool)>) -> Vec<(&'b (&'a u32, &'a bool), usize)>{
    let mut aa = Vec::new();
    for (i, y) in nodes.iter().enumerate(){
        aa.push((y,i));
    }
    aa.sort_by_key(|a|a.0);
    return aa
}



/// Helper function
///
/// Concatenate the node and the direction of the path
pub fn path2combi(path: &NCPath) -> Vec<(&u32, &bool)>{
    //let mut ff = Vec::with_capacity(path.nodes.len());
    let mut ff: Vec<(&u32, &bool)> = path.nodes.iter().zip(path.dir.iter()).collect();
    ff

}




/// Benchmark function
fn criterion_benchmark(c: &mut Criterion) {
    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_direct("data/example_data/size5.gfa", false);
    let data = path2combi(&graph.paths[0]);

    // 900 µs
    c.bench_function("Index - HashMap", |b| b.iter(|| index_hashmap(&data)));
    // 2.4 ms
    c.bench_function("Index - BtreeMap", |b| b.iter(|| index_btree(&data)));

    // 85 µs
    c.bench_function("Index - Meta", |b| b.iter(|| index_meta(&data)));

    // 35 µs
    c.bench_function("Index - Double", |b| b.iter(|| index_index(&data)));



}

criterion_group!(aa, criterion_benchmark);
criterion_main!(aa);