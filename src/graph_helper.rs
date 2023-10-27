use std::fs::File;
use std::io::{BufRead, BufReader};
use gfa_reader::NCGfa;
use hashbrown::HashMap;

pub fn ident_pairs(pairs: &mut Vec<(usize, usize)>, reference: &str, graph: &NCGfa<()>) {
    let mut path_index2 = Vec::new();
    for (path_index, path) in graph.paths.iter().enumerate(){
        if path.name.starts_with(reference) {
            path_index2.push(path_index);
        }
    }
    let mut it = 0;
    while it < pairs.len(){
        let pair = pairs[it];
        if path_index2.contains(&pair.0) || path_index2.contains(&pair.1) {
            it += 1;
        } else {
            pairs.remove(it);
        }
    }

}

pub fn split_string(st: &str) -> (String, String){
    let a = (st.split(",").nth(0).unwrap().to_string(), st.split(",").nth(1).unwrap().to_string());
    a
}


pub fn load_data2(filen: &str) -> Vec<(String, String)> {
    let mut mm = Vec::new();
    let file = File::open(filen).unwrap();
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line_result in reader.lines(){
        let line = line_result.unwrap();
        mm.push(split_string(&line));

    }
    mm
}

pub fn ident_pairs2(pairs: &mut Vec<(usize, usize)>, reference: &Vec<(String, String)>, graph: &NCGfa<()>) {
    let mut path_index2 : HashMap<String, usize> = HashMap::new();
    for (path_index, path) in graph.paths.iter().enumerate(){
        path_index2.insert(path.name.clone(), path_index);
    }
    pairs.clear();
    for x in reference.iter(){
        if x.1 == x.0{
            panic!("Twice the same name")
        } else {
            let a1 = path_index2.get(&x.0).unwrap().clone();
            let a2 = path_index2.get(&x.1).unwrap().clone();
            pairs.push((a1, a2));
        }
    }

}


