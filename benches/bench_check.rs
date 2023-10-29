use std::collections::BTreeMap;
use criterion::{criterion_group, criterion_main, Criterion};
use gfa_reader::{NCGfa, OptFields};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub mod bench_index;
use crate::bench_index::{index_btree, index_hashmap, index_index, index_meta, path2combi};
use rand::{seq::IteratorRandom, thread_rng}; // 0.6.1


/// Make random order graph path
pub fn random_sample<'a, 'b>(vec: &'b Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a  bool)>{
    let mut rng = thread_rng();
    let aa = (vec.len()/3)*2;
    let mut sample = vec.iter().choose_multiple(&mut rng, aa);
    let mut f: Vec<_> =  sample.iter().map(|x| **x).collect();
    f.sort();
    f
}


pub fn random_sample_nosort<'a, 'b>(vec: &'b Vec<(&'a u32, &'a bool)>) -> Vec<(&'a u32, &'a  bool)>{
    let mut rng = thread_rng();
    let aa = (vec.len()/3)*2;
    let mut sample = vec.iter().choose_multiple(&mut rng, aa);
    let mut f: Vec<_> =  sample.iter().map(|x| **x).collect();
    f
}

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


pub fn test_hashmap<'a, 'b>(sample: &Vec<&(&'a u32, &'a  bool)>, data:  &HashMap<&'b (&'a u32, &'a bool), Vec<usize>>,  data2:  &HashMap<&'b (&'a u32, &'a bool), Vec<usize>>) -> Vec<[usize; 3]>{

    let mut rr = Vec::new();
    sample.iter().for_each(|x|{
        rr.extend(all_combinations3(data.get(x).unwrap(), data2.get(x).unwrap(), &1));
        // data.get(x).unwrap();
        // data2.get(x).unwrap();
    });
    rr.sort();
    rr
}


pub fn test_btree<'a, 'b>(sample: &Vec<&(&'a u32, &'a  bool)>, data:  &BTreeMap<&'b (&'a u32, &'a bool), Vec<usize>>, data2: &BTreeMap<&'b (&'a u32, &'a bool), Vec<usize>>) -> Vec<[usize; 3]>{

    let mut rr = Vec::new();

    sample.iter().for_each(|x|{
        rr.extend(all_combinations3(data.get(x).unwrap(), data2.get(x).unwrap(), &1));
        //
        // data.get(x).unwrap();
        // data2.get(x).unwrap();
    });
    rr.sort();
    rr
}


pub fn test_range<'a, 'b>(ff: &Vec<&(&'a u32, &'a  bool)>, aa: &(Vec<(u32)>, Vec<(u32, u32)>), aa2: &(Vec<(u32)>, Vec<(u32, u32)>)) -> Vec<[u32; 3]>{

    let mut rr = Vec::new();

    let mut dd1: &[u32] = &aa.0;
    let mut dd2: &[u32] = &aa.0;
    for x in ff.iter(){
        let (start, end) = aa.1[*x.0 as usize + *x.1 as usize];
        dd1 = &aa.0[start as usize..(start + end) as usize];
        let (start, end) = aa2.1[*x.0 as usize + *x.1 as usize];
        dd2 = &aa2.0[start as usize..(start + end) as usize];
        rr.extend(all_combinations3(dd1, dd2, &1));

    }
    rr.sort();
    rr
}


fn find_indices(A: &[(& (&u32, &bool), usize)], B: &[(& (&u32, &bool), usize)], S: &Vec<&(&u32, &bool)>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut i = 0;  // Pointer for vector A
    let mut j = 0;  // Pointer for vector B
    let mut k = 0;  // Pointer for samples vector S

    let mut indices_a = Vec::with_capacity(S.len());
    let mut indices_b = Vec::with_capacity(S.len());


    let mut indices_a2 = Vec::new();
    let mut indices_b2 = Vec::new();
    let mut old = 0;
    let mut froma = 0;
    let mut fromb = 0;
    while i < A.len() && j < B.len() && k < S.len() {
        if A[i].0 == S[k] && B[j].0 == S[k] {

            if k != old {
                indices_a.push(indices_a2.clone());
                indices_b.push(indices_b2.clone());
                indices_a2.clear();
                indices_b2.clear();
                old = k;
            }
            indices_a2.push(A[i].1);
            indices_b2.push(B[j].1);
            i += 1;
            j += 1;
        } else if A[i].0 < &S[k] {
            i += 1;
        } else if B[j].0 < &S[k] {
            j += 1;
        } else {
            k += 1;
        }
    }

    (indices_a, indices_b)
}






fn criterion_benchmark(c: &mut Criterion) {
    // Read the graph - using this function if faster because if ignores all edges (not needed here)
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file_direct("data/example_data/size5.gfa", false);
    let data = path2combi(&graph.paths[0]);
    let data2 = path2combi(&graph.paths[1]);

    let dhs1: HashSet<(&u32, &bool)> = data.iter().cloned().collect();
    let dhs2: HashSet<(&u32, &bool)> = data2.iter().cloned().collect();

    let mut a: Vec<&(&u32, &bool)> = dhs1.intersection(&dhs2).collect::<HashSet<_>>().iter().cloned().collect();




    let hashmap_index = index_hashmap(&data);
    let hashmap_index2 = index_hashmap(&data2);


    let btree_index = index_btree(&data);
    let btree_index2 = index_btree(&data2);

    let meta_index = index_meta(&data);
    let meta_index2 = index_meta(&data2);

    let index_index2 = index_index(&data);
    let index_index3 = index_index(&data2);


    println!("dsajkdhasj");
    a.sort();
    let t1 = test_hashmap(&a, &hashmap_index, &hashmap_index2);
    let t2 = test_btree(&a, &btree_index, &btree_index2);
    let t3 = test_range(&a, &meta_index, &meta_index2);
    let t33 = t3.iter().map(|x| [x[0] as usize, x[1] as usize, x[2] as usize]).collect::<Vec<_>>();
    assert_eq!(t1, t2);
    assert_eq!(t1, t33);
    println!("dsajkdhasj");

    let mut a: Vec<&(&u32, &bool)> = dhs1.intersection(&dhs2).collect::<HashSet<_>>().iter().cloned().collect();


    // let t4 = find_indices(&index_index2, &index_index3, &get_sample1);
    //assert_eq!(t1, t2);
    //assert_eq!(t1.len(), t4.0.len());


    c.bench_function("Lookup: HashMap", |b| b.iter(|| test_hashmap(&a, &hashmap_index, &hashmap_index2)));
    c.bench_function("Lookup: BTreeMap", |b| b.iter(|| test_btree(&a, &btree_index, &btree_index2)));
    c.bench_function("Lookup: Range", |b| b.iter(|| test_range(&a, &meta_index, &meta_index2)));

    a.sort();
    c.bench_function("Lookup: HashMap", |b| b.iter(|| test_hashmap(&a, &hashmap_index, &hashmap_index2)));
    c.bench_function("Lookup: BTreeMap", |b| b.iter(|| test_btree(&a, &btree_index, &btree_index2)));
    c.bench_function("Lookup: Range", |b| b.iter(|| test_range(&a, &meta_index, &meta_index2)));
    c.bench_function("Index", |b| b.iter(|| find_indices(&index_index2, &index_index3, &a)));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);