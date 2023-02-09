use std::cmp::{max, min};
use gfaR_wrapper::{NPath};
use std::collections::{HashSet};
use hashbrown::HashMap;

/// **Get all pairs of a vector**
///
/// - Only upper "triangle"
/// - Clones the items
pub fn get_all_pairs<T>(vector: &Vec<T>) -> Vec<(T,T)>
    where T: Clone{
    let mut pairs: Vec<(T, T)> = Vec::new();
    let mut count = 0;
    for item1 in vector.iter(){
        for item2 in vector[count+1..].iter(){
            pairs.push((item1.clone(), item2.clone()));
        }
        count += 1;
    }
    pairs
}


//------------------INDEX--------------------------------------------------------------
/// Index constructor
///
/// Returns:
/// - Hashset of all nodes in a path
/// - Hashmap of all {nodes -> vec<index>})
pub fn path2index_hashmap(path1: &NPath) -> (HashSet<u32>, HashMap<u32, Vec<u32>>){
    let node_hashset: HashSet<u32> = path2hashset(path1);
    let node2index = node2index(path1);
    return (node_hashset, node2index)
}


/// HashSet of the nodes in a path
///
/// Copy the data (this might not be needed)
pub fn path2hashset(path: &NPath) -> HashSet<u32>{
    let node_hs = path.nodes.iter().cloned().collect();
    return node_hs
}

/// Takes a path and creates a node index
///
///
/// Returns a hashmap:
/// {node1: [index1, index2], node2: [index3, index10]}
pub fn node2index(path: &NPath) -> HashMap<u32, Vec<u32>>{
    let mut index: HashMap<u32, Vec<u32>> = HashMap::new();
    for (path_index, node) in path.nodes.iter().enumerate(){
        // index.entry(*node).or_default().push(path_index as u32);
        if index.contains_key(node){
            index.get_mut(node).unwrap().push(path_index as u32);
        } else {
            index.insert(*node, vec![path_index as u32]);

        }
    }
    // for x in index.iter_mut(){
    //     x.1.shrink_to_fit();
    // }

    return index
}




/// Get all positions [x1, x2] of the same shared nodes
pub fn get_shared_index(jo11: &Vec<u32>, jo12: &Vec<u32>, jo21: &HashMap<u32, Vec<u32>>, jo22: &HashMap<u32, Vec<u32>>) -> Vec<[u32; 3]> {

    let shared_nodes: Vec<u32> = vec_intersection(jo11, jo12);
    let mut result = Vec::new();
    for x in shared_nodes.iter(){
        let k = jo21.get(x).unwrap();
        let k2 = jo22.get(x).unwrap();
        if (k.len() > 1) | (k2.len() > 1){
            result.extend(all_combinations2(k, k2, &(*x as u32)))
        } else {
            result.push([k[0], k2[0], *x as u32])
        }
    }
    //Sort it afterwards
    result.sort();
    result
}


/// Get all positions [x1, x2] of the same shared nodes
pub fn get_shared_index_low_mem(path1: &NPath, path2: &NPath) -> Vec<[u32; 3]> {
    //let node_hashset: HashSet<u32> = path2hashset(path1);
    //let node_hashset2: HashSet<u32> = path2hashset(path2);

    //let shared_nodes: HashSet<u32> = node_hashset.intersection(&node_hashset2).cloned().collect();

    let f = vec_intersection(&path1.nodes, &path2.nodes);
    let node2i = node2index(path1);
    let node2i2 = node2index(path2);

    let mut result = Vec::new();
    for x in f.iter(){
        let k = node2i.get(x).unwrap();
        let k2 = node2i2.get(x).unwrap();
        if (k.len() > 1) | (k2.len() > 1){
            result.extend(all_combinations2(k, k2, &(*x as u32)))
        } else {
            result.push([k[0], k2[0], *x as u32])
        }
    }
    //Sort it afterwards
    result.sort();
    result
}


/// **Get all combinations of two vectors**
///
pub fn all_combinations2<T>(a: & Vec<T>, b: & Vec<T>, node_id: &T) -> Vec<[T; 3]>
    where T: Clone{
    {
        let mut p = Vec::new();
        for x in a.iter(){
            for y in b.iter(){
                p.push([x.clone(),y.clone(), node_id.clone()])
            }
        }
        p
    }
}


/// **Get all non-self combinations of a 2D vector
///
pub fn all_combinations_self<T>(a: & Vec<T>, path: &u32, bubble_id2: &u32) -> Vec<(usize, T, T, u32)>
    where T: Clone + Ord{
    {
        let mut p = Vec::new();
        for (i, x) in a.iter().enumerate(){
            for y in i..a.len(){
                p.push((path.clone() as usize, min(x.clone(), a[y].clone()), max(x.clone(), a[y].clone()).clone(), bubble_id2.clone()));
            }
        }
        p
    }
}

/// **Get all combinations of two vectors**
/// Generic version
pub fn all_combinations<T>(a: & Vec<T>, b: & Vec<T>, path: &u32, bubble_id2: &u32) -> Vec<(usize, T, T, u32)>
    where T: Clone + Ord{
    {
        let mut p = Vec::new();
        for  x in a.iter(){
            for y in b.iter(){
                p.push((path.clone() as usize, min(x.clone(),y.clone()), max(x.clone(),y.clone()), bubble_id2.clone()));
            }
        }
        p
    }
}


/// Intersection of two vectors
pub fn vec_intersection(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut a2 = a.clone();
    let mut b2 = b.clone();
    a2.sort();
    b2.sort();
    let mut result = vec![];
    let mut i = 0;
    let mut j = 0;
    let mut old = 0;
    while i < a2.len() && j < b2.len() {
        if a2[i] < b2[j] {
            i += 1;
        } else if a2[i] > b2[j] {
            j += 1;
        } else {
            if a2[i] != old {
                result.push(a2[i]);
                old = a2[i]
            }
            i += 1;
            j += 1;
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use log::info;
    use crate::bifurcation_helper::vec_intersection;

    #[test]
    fn vec_intersection_test() {
        let vec1 = vec![1,2,3,4,5,1];
        let vec2 = vec![1,6,7,8,98,1];
        let vec_intersection = vec_intersection(&vec1, &vec2);
        //eprint!("dasdad {:?}", vec_intersection);
        assert_eq!(vec_intersection.len(), 1);
    }
}




