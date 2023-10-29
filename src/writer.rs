use std::io::{Write, BufWriter};



use std::fs::File;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use gfa_reader::{NCGfa, NCNode, NCPath};
use itertools::enumerate;
use crate::helper::{chunk_by_index2, mean};

pub fn write_bubbles(data: &Vec<(u32, u32)>, string: &str) {
    let output_prefix = string.to_owned() + ".bubble.txt";
    let file = File::create(output_prefix).unwrap();
    let buf_writer = Mutex::new(BufWriter::new(file));

    // Use Rayon to parallelize the writing process with chunks of size 5
    data.par_chunks(1).for_each(|chunk| {
        let mut buf_writer = buf_writer.lock().unwrap();

        for i in chunk {
            buf_writer.write_all(format!("{}{}\t{}{}\n", i.0/2, if((i.0%2) == 1) {"+"} else {"-"} , i.1/2, if(i.0%2 == 1) {"+"} else {"-"}).as_bytes()).unwrap();
        }

        // Ensure the buffer is flushed for this chunk
        buf_writer.flush().unwrap();
    });
}


pub fn write_index_intervals(data: &Vec<(String, Vec<[u32;3]>)>, output: &str){
    let output_prefix = output.to_owned() + ".index.txt";
    let file = File::create(output_prefix).unwrap();
    let buf_writer = Mutex::new(BufWriter::new(file));

    data.par_chunks(1).for_each(|chunk| {
        let mut buf_writer = buf_writer.lock().unwrap();

        for (path_name, indices) in chunk {
            for i1  in indices.iter(){
                buf_writer.write_all(format!("{}\t{}\t{}\t{}\n", path_name, i1[0], i1[1], i1[2]).as_bytes()).unwrap();

            }
        }

        // Ensure the buffer is flushed for this chunk
        buf_writer.flush().unwrap();
    });

}


pub fn write_index_intervals2(data: &Vec<[u32;3]>, output: &str){
    let output_prefix = output.to_owned() + ".index.txt";
    let file = File::create(output_prefix).unwrap();
    let buf_writer = Mutex::new(BufWriter::new(file));

    data.par_chunks(1).for_each(|chunk| {
        let mut buf_writer = buf_writer.lock().unwrap();

        for i1 in chunk {
            buf_writer.write_all(format!("{}\t{}\t{}\t{}\n", "dasda", i1[0], i1[1], i1[2]).as_bytes()).unwrap();


        }

        // Ensure the buffer is flushed for this chunk
        buf_writer.flush().unwrap();
    });

}



pub fn stats<'a>(aa: &Vec<(String, Vec<[u32;3]>)>, nc: &'a NCGfa<()>) -> Vec<(usize, usize, &'a [u32], &'a [bool], u32)> {

    let mut gg = Vec::new();
    for (i, x) in enumerate(aa.iter()){
        let gfa2pos = gfa2pos(nc.paths[i].clone(), nc.nodes.clone());
        let mut a23 = Vec::new();
        for y in x.1.iter(){
            let a = gfa2pos[y[0] as usize];
            let b = gfa2pos[y[1] as usize];
            let c = &nc.paths[i].nodes[y[0] as usize..y[1] as usize];
            let c2 = &nc.paths[i].dir[y[0] as usize..y[1] as usize];

            a23.push((a,b, c, c2, y[2]));
        }
        gg.extend(a23)

    }

    gg
}

pub fn gfa2pos(np: NCPath, nodes: Vec<NCNode<()>>) -> Vec<usize>{
    let mut vv = Vec::new();
    let mut old = 0;
    for x in np.nodes.iter(){
        vv.push(old + nodes.get(*x as usize - 1).unwrap().seq.len());
        old += nodes.get(*x as usize - 1).unwrap().seq.len();
    }
    vv
}

pub fn tdsatda(input1: &Vec<(usize, usize, & [u32], &[bool], u32)>, input2: &Vec<(usize, u32, u32, u32)>){
    let mut traversals = Vec::new();
    //let mut sizes = Vec::new();
    let intervals = 0;
    let mut old_bub = 0;




    let output_prefix = "bubble.txt";
    let file = File::create(output_prefix).unwrap();
    let buf_writer = Mutex::new(BufWriter::new(file));

    for (i1,i2) in input1.iter().zip(input2.iter()){
        let a = i1.2;
        if old_bub != i2.3{
            traversals.clear();
            old_bub = i2.3

        }
        if ! traversals.contains(&(i1.2, i1.3)){
            traversals.push((i1.2, i1.3))
        }

    }
}



pub fn all_one(input: &mut Vec<(usize, u32, u32, u32)>,  nc: & NCGfa<()>){
    let max_bubble = input.last().unwrap().3;
    let k = chunk_by_index2(input, max_bubble, 20);



    let output_prefix = "bubble.txt";
    let file = File::create(output_prefix).unwrap();
    let buf_writer = BufWriter::new(file);
    let arc_buf = Arc::new(Mutex::new(buf_writer));


    let output_prefix2 = "interval.txt";
    let file2 = File::create(output_prefix2).unwrap();
    let buf_writer2 = BufWriter::new(file2);
    let arc_buf2 = Arc::new(Mutex::new(buf_writer2));

    let output_prefix3 = "bubbles.stats";
    let file3 = File::create(output_prefix3).unwrap();
    let buf_writer3 = BufWriter::new(file3);
    let arc_buf3 = Arc::new(Mutex::new(buf_writer3));

    let wr = gfapos_wrapper(&nc, &1);

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    thread_pool.install(|| {
        k.par_iter().for_each(|chunk| {
            let mut traversals = Vec::new();
            //let mut sizes = Vec::new();
            let intervals = 0;
            let mut start = true;
            let mut old_bub = 0;
            let mut sizes: Vec<usize> = Vec::new();
            let mut all_trav = "".to_owned();
            let mut all_interval = "".to_owned();
            let mut all_bubble = "".to_owned();
            let mut intervals = 0;
            let mut bub = 0;


            for y in chunk.iter() {
                //println!("yy {:?}", y);
                let gfa2pos = &wr[y.0];

                let start_pos = gfa2pos[y.1 as usize];
                let end_pos = gfa2pos[y.2 as usize - 1];
                let size = end_pos - start_pos;
                let nodes = &nc.paths[y.0 as usize].nodes[y.1 as usize..y.2 as usize -1];
                let bools = &nc.paths[y.0 as usize].dir[y.1 as usize..y.2 as usize -1];
                //let s = hash_vector(nodes, bools);
                let traversal = (nodes, bools);
                let mut traversal_number = 1;

                bub = y.3;
                if old_bub != bub {
                    if !start {
                        // all_trav += &write_traversal(&traversals, old_bub);
                        // all_bubble  += &write_bubbles2(old_bub, intervals, traversals.len() as u32, sizes.iter().min().unwrap().clone() as u32, sizes.iter().max().unwrap().clone() as u32, mean(&sizes) as f64);

                        //let fall_trav = &write_traversal(&traversals, old_bub);
                        //let fall_bubble  = &write_bubbles2(old_bub, intervals, traversals.len() as u32, sizes.iter().min().unwrap().clone() as u32, sizes.iter().max().unwrap().clone() as u32, mean(&sizes) as f64);


                        // let mut dsada2 = arc_buf2.lock().unwrap();
                        // write!(dsada2, "{}", fall_trav).expect("helpa");
                        //
                        // let mut dsada = arc_buf.lock().unwrap();
                        // write!(dsada, "{}", fall_bubble).expect("helpa");



                        intervals = 0;
                        traversal_number = 1;
                        sizes.clear();
                        traversals.clear();
                        old_bub = bub;
                    } else {
                        old_bub = bub;
                    }
                }
                if traversals.contains(&traversal) {
                    traversal_number = traversals.iter().position(|r| *r == traversal).unwrap();
                } else {
                    let mut dsada2 = arc_buf2.lock().unwrap();
                    write!(dsada2, "{}\t{}\t{}", bub, traversal_number, "dasds").expect("helpa");
                    traversals.push(traversal);
                    traversal_number = traversals.len();
                }
                intervals += 1;
                start = false;



                //all_interval += &write_interval(y.0 as usize, start_pos, end_pos, &bub, &traversal_number);
                let fall_interval = &write_interval(y.0 as usize, start_pos, end_pos, &bub, &traversal_number);


                // let mut dsada3 = arc_buf3.lock().unwrap();
                // write!(dsada3, "{}", fall_interval).expect("helpa");

                sizes.push(size);

            }
            // all_trav += &write_traversal(&traversals, bub);
            // all_bubble += &write_bubbles2(old_bub, intervals, traversals.len() as u32, sizes.iter().min().unwrap().clone() as u32, sizes.iter().max().unwrap().clone() as u32, mean(&sizes) as f64);
            //
            //
            //
            //
            // let mut dsada2 = arc_buf2.lock().unwrap();
            // write!(dsada2, "{}", all_interval).expect("helpa");
            //
            // let mut dsada = arc_buf.lock().unwrap();
            // write!(dsada, "{}", all_bubble).expect("helpa");
            //
            //
            // let mut dsada3 = arc_buf3.lock().unwrap();
            // write!(dsada3, "{}", all_interval).expect("helpa");

        });
    });

}

pub fn gfapos_wrapper(graph: &NCGfa<()>, threads: &usize) -> Vec<Vec<usize>>{
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(*threads)
        .build()
        .unwrap();

    let result: Vec<Vec<usize>> = thread_pool.install(|| {
        graph.paths.par_iter().map(|x| gfa2pos(x.clone(), graph.nodes.clone())).collect()
    });
    result
}

pub fn trav_sum(i1: &[u32], i2: &[bool]) -> String{
    let mut a = 0;
    if let Some(value) = i1.iter().max() {
        a = *value;
    } else {
        a = 0
    }
    let l = a.to_string();
    l
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_vector(vec: &[u32], vec2: &[bool]) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut  hasher);
    vec2.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_vector2(vec: &[(u32, bool)]) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut  hasher);
    hasher.finish()
}

pub fn traversal_to_string(i1: &[u32], i2: &[bool]) -> String{
    let mut v = Vec::new();
    for (y,z) in i1.iter().zip(i2.iter()){
        v.push(format!("{}{}", y, if *z { "+" } else { "-" }));
    }
    v.join(",")
}


pub fn write_traversal(input: &Vec<(&[u32], &[bool])>, bubble_id: u32) -> String{
    let mut string_vec = Vec::new();
    for (index, traversal) in input.iter().enumerate(){
        let mut v = Vec::new();
        for (y,z) in traversal.0.iter().zip(traversal.1.iter()){
            v.push(format!("{}{}", y, if *z { "+" } else { "-" }));
        }
        string_vec.push(v);
    }
    let mut a = "".to_owned();
    for (index, x) in string_vec.iter().enumerate(){
        a += &format!("{}\t{}\t{}\n", bubble_id, index+1,  x.join(","));

    }
    a
}


pub fn write_bubbles2(bubble_id: u32, number_inter: u32, number_trav: u32, minlen: u32, maxlen: u32, avlen: f64) -> String{
    format!("{}\t{}\t{}\t{}\t{}\t{}\n", bubble_id, number_inter, number_trav, minlen, maxlen, avlen)

}



pub fn write_interval(genome: usize, start: usize, end: usize, bubble_id: &u32, traversal_id: &usize) -> String {
    format!("{}\t{}\t{}\t{}\t{}\n", genome, start, end, bubble_id, traversal_id)
}




// /// Wrapper function for traversal and bubble output files
// ///
// /// Writing two files at the same time
// ///
// /// Comment:
// /// - Sending data is not needed (maybe replace later)
// pub fn write_wrapper(data:  Vec<Vec<(usize, u32, u32, u32)>>, index2pos: Vec<Vec<usize>>, paths: Vec<NCPath>, filename_prefix: &str, bubbles: Vec<(u32, u32)>) {
//     // Create the two files
//     let file1 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".bed").unwrap())));
//     let file2 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".stats").unwrap())));
//
//
//     // Additional references
//     let arc_index = Arc::new(index2pos);
//     let arc_paths = Arc::new(paths);
//     let data_len = data.len().clone();
//     let arc_bubbles = Arc::new(bubbles);
//
//     // Do it like this
//     let (send, rev) = unbounded();
//
//     for x in data.into_iter(){
//         let send_clone = send.clone();
//         let arc_index_v2 = arc_index.clone();
//         let arc_paths_v2 = arc_paths.clone();
//         let arc_file1_v2 = file1.clone();
//         let arc_file2_v2 = file2.clone();
//         let arc_bubbles = arc_bubbles.clone();
//         //let ff = Arc::new(*x);
//
//
//         thread::spawn(move || {
//             let ff = x.len().clone();
//             traversal_stats(x, arc_index_v2, arc_paths_v2, arc_file1_v2, arc_file2_v2, arc_bubbles);
//             send_clone.send(format!("1 {}", ff)).unwrap();
//         });
//     }
//
//     // Just waiting
//     for _x in 0..data_len{
//         rev.recv().expect("Nothing");
//
//
//     }
// }
//
// pub fn solo_stats(data:  &[(usize, u32, u32, u32)], index2: Vec<Vec<usize>>, paths: Vec<NCPath>, bubbles: Vec<(u32, u32)>){
//     let mut file1 =BufWriter::new(File::create("test1".to_owned() + ".bed").unwrap());
//     let mut file2 = BufWriter::new(File::create("test2".to_owned() + ".stats").unwrap());
//     let mut traversal:  Vec<(&[u32], &[bool])> = Vec::new();
//     let mut tmp_data: Vec<&(usize, u32, u32, u32)> = Vec::new();
//     let mut interval_size: Vec<(usize, u32, usize, usize)> = Vec::new();
//     let mut traversal_number = 0;
//     let mut sizes: Vec<usize> = Vec::new();
//     let mut old_bub = data[0].3.clone();
//     let mut ff1: Vec<u8> = Vec::new();
//
//
//     // Iterate over the data
//     for interval in data.into_iter(){
//         // Check if the interval contains a new bubble_id
//         if interval.3 != old_bub {
//             let bubble = bubbles.get(old_bub as usize).unwrap();
//             let _soo = tmp_data.len().clone();
//
//             // What is missing? bubble_id, start, end, #subbubbles, parents, ratio?, type
//             write!(file1, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
//             //let dd = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone());
//             //ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());
//             for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
//                 write!(file2, "{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).expect("help");
//             }
//
//             // Reset all variables
//             traversal = Vec::new();
//             sizes = Vec::new();
//             tmp_data = Vec::new();
//             interval_size = Vec::new();
//             traversal_number = 0;
//             old_bub = interval.3;
//
//         }
//
//
//
//         let path = &paths[interval.0];
//
//         let slice_node = &path.nodes[(interval.1 + 1) as usize..interval.2 as usize];
//         let slice_dir = &path.dir[(interval.1 + 1) as usize..interval.2 as usize];
//         let slice_merge: (&[u32], &[bool]) = (slice_node, slice_dir);
//         if traversal.contains(&slice_merge) {
//             traversal_number = traversal.iter().position(|r| *r == slice_merge).unwrap();
//         } else {
//             traversal.push(slice_merge);
//             traversal_number += 1;
//         }
//
//
//         // Start end
//         let from_id: usize = index2[interval.0][interval.1 as usize];
//         let mut to_id: usize = index2[interval.0][interval.2 as usize - 1];
//         if interval.2 == interval.1 + 1 {
//             to_id = from_id.clone();
//         }
//         tmp_data.push(interval);
//         sizes.push(to_id-from_id);
//         interval_size.push((traversal_number, (to_id-from_id) as u32, from_id, to_id));
//
//     }
//
//     // Do the same again when everything is done.
//     let bubble_id = tmp_data.first().unwrap().3;
//     let bubble = bubbles.get(bubble_id as usize).unwrap();
//
//     write!(file1, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", bubble_id, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
//
//     for (x1, x2) in tmp_data.into_iter().zip(interval_size.iter()){
//         write!(file2, "{}\t{}\t{}\t{}\t{}\n", paths.get(x1.0).unwrap().name, x2.2, x2.3, x1.3, x2.0).expect("heda");
//     }
// }
//
// /// The actual writer function
// /// TODO
// /// - Add bubbles here
// ///
// /// Writes [min, max, mean, interval_count, traversal count] in a bubble stats file (missing bubble start and end)
// /// Writes accession_name, start, end, interval and bubble number in a bed file.
// pub fn traversal_stats(data:  Vec<(usize, u32, u32, u32)>, index2: Arc<Vec<Vec<usize>>>, paths: Arc<Vec<NCPath>>, file_bed: Arc<Mutex<BufWriter<File>>>, file_bubble: Arc<Mutex<BufWriter<File>>>, bubbles: Arc<Vec<(u32, u32)>>) {
//     // Initialize variables
//     let mut traversal:  Vec<(&[u32], &[bool])> = Vec::new();
//     let mut tmp_data: Vec<(usize, u32, u32, u32)> = Vec::new();
//     let mut interval_size: Vec<(usize, u32, usize, usize)> = Vec::new();
//     let mut traversal_number = 0;
//     let mut sizes: Vec<usize> = Vec::new();
//     let mut old_bub = data[0].3.clone();
//     let mut ff1: Vec<u8> = Vec::new();
//     let mut ff2: Vec<u8> = Vec::new();
//
//
//
//     // Iterate over the data
//     for interval in data.into_iter(){
//         // Check if the interval contains a new bubble_id
//         if interval.3 != old_bub {
//             let bubble = bubbles.get(old_bub as usize).unwrap();
//             let _soo = tmp_data.len().clone();
//
//             ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());
//
//             // let mut bed_arc = file_bed.lock().unwrap();
//             // let mut bubble_arc = file_bubble.lock().unwrap();
//             // What is missing? bubble_id, start, end, #subbubbles, parents, ratio?, type
//
//             // write!(bubble_arc, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
//             // //let dd = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone());
//             // //ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());
//             // for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
//             //     write!(bed_arc, "{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).expect("help");
//             // }
//             for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
//                 ff2.extend(format!("{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).as_bytes().to_vec());
//             }
//
//
//             // Reset all variables
//             traversal = Vec::new();
//             sizes = Vec::new();
//             tmp_data = Vec::new();
//             interval_size = Vec::new();
//             traversal_number = 0;
//             old_bub = interval.3;
//
//         }
//
//
//
//         let path = &paths[interval.0];
//
//         let slice_node = &path.nodes[(interval.1 + 1) as usize..interval.2 as usize];
//         let slice_dir = &path.dir[(interval.1 + 1) as usize..interval.2 as usize];
//         let slice_merge: (&[u32], &[bool]) = (slice_node, slice_dir);
//         if traversal.contains(&slice_merge) {
//             traversal_number = traversal.iter().position(|r| *r == slice_merge).unwrap();
//         } else {
//             traversal.push(slice_merge);
//             traversal_number += 1;
//         }
//
//
//         // Start end
//         let from_id: usize = index2[interval.0][interval.1 as usize];
//         let mut to_id: usize = index2[interval.0][interval.2 as usize - 1];
//         if interval.2 == interval.1 + 1 {
//             to_id = from_id.clone();
//         }
//         tmp_data.push(interval);
//         sizes.push(to_id-from_id);
//         interval_size.push((traversal_number, (to_id-from_id) as u32, from_id, to_id));
//
//     }
//
//     // Do the same again when everything is done.
//
//     let mut bed_arc = file_bed.lock().unwrap();
//     let mut stats_arc = file_bubble.lock().unwrap();
//
//     let bubble_id = tmp_data.first().unwrap().3;
//     let bubble = bubbles.get(bubble_id as usize).unwrap();
//
//     stats_arc.write_all(&ff1[..]);
//     bed_arc.write_all(&ff2[..]);
//     // write!(stats_arc, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", bubble_id, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
//     //
//     // for (x1, x2) in tmp_data.into_iter().zip(interval_size.iter()){
//     //     write!(bed_arc, "{}\t{}\t{}\t{}\t{}\n", paths.get(x1.0).unwrap().name, x2.2, x2.3, x1.3, x2.0).expect("heda");
//     // }
// }
//



