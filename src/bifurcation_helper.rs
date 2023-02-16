use std::cmp::{max, min};
use gfaR_wrapper::{NPath};
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
    return index
}



/// This is an alternative approach to node2index with lower memory and faster lookup (in theory)
///
/// This replaces a Hashmap(value -> Vec[index]);
///
/// Workflow
/// 1. Create a vector which in the order of the nodes in the path which returns a (node_id, index)
///     + Check the highest "value" in the vector
/// 2. Sort the new vector by nodes
/// 3. Create a vector which holds a (0,0) from 0 to max value found in the original value
/// 4. Iterate over the (node, index) vector
///     1. If the node changes, report insert the (index, and length of the numbers) in the initialized list
///
/// Result:
///     - Vector of all index of path in the order of nodes
///     - Vector of position and length for each value. Some entries stay (0,0)
///
///
/// Comment: In theory this is possible with every "key" data, when it is possible to convert it into a usize. For now this is only for u32.
///
/// Scales with the amount of nodes in the graph. The more collinear the genomes are, the better is the compression.
///
pub fn node2index_low_mem(vec: &Vec<u32>) -> (Vec<u32>, Vec<(u32, u32)>){
    let mut f = Vec::with_capacity(vec.len());
    let mut m: &u32 = &0;
    for (i, x) in vec.iter().enumerate(){
        f.push((x,i));
        if x > m{
            m = x;
        }
    }
    f.sort();
    let mut old = f[0].0;
    let mut ff = vec![(0,0); *m as usize + 1];
    let test: Vec<u32> = f.iter().map(|a| a.1 as u32).collect();
    let mut last = 0;
    for (i,x) in f.iter().enumerate(){
        if x.0 != old {
            ff[*old as usize] = (last as u32, (i - last) as u32);
            last = i;
            old = x.0;
        }

    }
    ff[*old as usize] = (last as u32, (f.len()- last) as u32);

    (test, ff)
}




/// Get all the index pairs of all shared nodes
///
///
/// Input:
/// - path1_nodes: Nodes of the first path
/// - path2_nodes: Nodes of the second path
/// - path1_index: Index of the first path
/// - path2_index: Index of the second path
///
/// Output:
/// - (index_from, index_to, node_id)
///
/// Comment: This is input for the bifurcation algorithm. Not sure if unsafe code makes it faster...
pub fn get_shared_index(path1_nodes: &Vec<u32>, path2_nodes: &Vec<u32>, path1_index: &(Vec<u32>, Vec<(u32, u32)>), path2_index: &(Vec<u32>, Vec<(u32, u32)>)) -> Vec<[u32; 3]> {

    // Make intersection of the two node sets
    let shared_nodes: Vec<u32> = vec_intersection(path1_nodes, path2_nodes);

    let mut result = Vec::with_capacity(shared_nodes.len()*2);

    for shared_node in shared_nodes.iter(){
        let path1_i = path1_index.1.get(*shared_node as usize).unwrap();
        let path2_i = path2_index.1.get(*shared_node as usize).unwrap();

        let path1_islice = &path1_index.0[path1_i.0 as usize ..(path1_i.0 + path1_i.1) as usize];
        let path2_islice = &path2_index.0[path2_i.0 as usize..(path2_i.0 + path2_i.1) as usize];

        //println!("{:?} {:?} {:?}", k, k2, x);
        if (path1_i.1 > 1) || (path2_i.1 > 1){
            result.extend(all_combinations3(path1_islice, path2_islice, &(*shared_node as u32)))
        } else {
            result.push([path1_islice[0], path2_islice[0], *shared_node as u32])
        }
    }
    //Sort it afterwards
    // This sorting is very slow (same as intersection and index creation together)
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

pub fn sort_nodes(paths: &Vec<NPath>) -> Vec<Vec<u32>>{
    let mut sort_nodes = Vec::with_capacity(paths.len());
    for path in paths.iter(){
        let mut f = path.nodes.clone();
        f.sort();
        sort_nodes.push(f);
    }
    sort_nodes
}


/// **Get all combinations of two vectors**
///
pub fn all_combinations2<T>(a: & Vec<T>, b: & Vec<T>, node_id: &T) -> Vec<[T; 3]>
    where T: Clone{
    {
        let mut p = Vec::with_capacity(a.len() * b.len());
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
    where T: Clone + Ord + Copy{
    {
        let mut p = Vec::new();
        for (i, x) in a.iter().enumerate(){
            for y in i+1..a.len(){
                p.push((*path as usize, *min(x, &a[y]), *max(x, &a[y]), *bubble_id2));
            }
        }
        p
    }
}

/// **Get all combinations of two vectors**
/// Generic version
pub fn all_combinations<T>(a: & Vec<T>, b: & Vec<T>, path: &u32, bubble_id2: &u32) -> Vec<(usize, T, T, u32)>
    where T: Clone + Ord + Copy{
    {
        let mut p = Vec::new();
        for  x in a.iter(){
            for y in b.iter(){
                p.push((*path as usize, *min(x, y), *max(x,y), *bubble_id2));
            }
        }
        p
    }
}

/// Intersection of two vectors
///
/// This is a fast implementation if the two vectors are already sorted (or close)
///
/// Comment:
/// In our examples, sorting takes 1/3 of the total time. Since this function is run multiple times, it might efficient to sort beforehand and hand over the sorted vectors (nodes). This adds some additional memory.
///
/// # Example
///
/// ``` rust
/// let v1 = vec![1,2,3,4];
/// let v2 = vec![1,5,6,4];
/// let v_intersection = vec_intersection(v1, v2);
/// assert_eq(v_intersection, vec![1,4])
/// ```
///
pub fn vec_intersection(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut a2 = a.clone();
    let mut b2 = b.clone();
    a2.sort();
    b2.sort();
    let mut result = Vec::with_capacity(a.len()+b.len());
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
    result.shrink_to_fit();
    result
}





#[cfg(test)]
/// Tests for some functions in this file.
mod tests {
    use crate::bifurcation_helper::vec_intersection;

    #[test]
    fn vec_intersection_test() {
        let vec1 = vec![1,2,3,4,5,1];
        let vec2 = vec![1,6,7,8,98,1];
        let vec_intersection = vec_intersection(&vec1, &vec2);
        assert_eq!(vec_intersection.len(), 1);
    }
}




