use std::fs::File;
use std::io::{BufRead, BufReader};
use gfa_reader::NCGfa;
use hashbrown::HashMap;


/// Remove all pairs which are not reference
pub fn pairs_reference(pairs: &mut Vec<(usize, usize)>, reference: &str, graph: &NCGfa<()>) {
    let mut path_references = Vec::new();
    for (path_index, path) in graph.paths.iter().enumerate(){
        if path.name.starts_with(reference) {
            path_references.push(path_index);
        }
    }
    // Remove everything which is not reference
    let mut index = 0;
    while index < pairs.len(){
        let pair = pairs[index];
        if path_references.contains(&pair.0) || path_references.contains(&pair.1) {
            index += 1;
        } else {
            pairs.remove(index);
        }
    }

}

/// Split string by comma
/// Take first and second element
pub fn split_string(st: &str) -> (String, String){
    let a = (st.split(",").nth(0).unwrap().to_string(), st.split(",").nth(1).unwrap().to_string());
    a
}


/// Parse pair file
pub fn parse_pair_file(filen: &str) -> Vec<(String, String)> {
    let mut pairs_vec = Vec::new();
    let file = File::open(filen).unwrap();
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line_result in reader.lines(){
        let line = line_result.unwrap();
        pairs_vec.push(split_string(&line));

    }
    pairs_vec
}

/// Only hold the pairs which are in the list
pub fn pair_list_filter(pairs: &mut Vec<(usize, usize)>, reference: &Vec<(String, String)>, graph: &NCGfa<()>) {
    let mut pathname2index: HashMap<String, usize> = HashMap::new();
    for (path_index, path) in graph.paths.iter().enumerate(){
        pathname2index.insert(path.name.clone(), path_index);
    }
    pairs.clear();
    for x in reference.iter(){
        if x.1 == x.0{
            panic!("Twice the same name")
        } else {
            let a1 = pathname2index.get(&x.0).unwrap().clone();
            let a2 = pathname2index.get(&x.1).unwrap().clone();
            pairs.push((a1, a2));
        }
    }

}


