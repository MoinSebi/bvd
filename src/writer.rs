use std::fs::File;
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_channel::unbounded;
use gfaR_wrapper::NPath;
use hashbrown::HashMap;
use itertools::interleave;
use log::info;
use crate::helper::mean;


/// Wrapper function for traversal and bubble output files
///
/// Writing two files at the same time
///
/// Comment:
/// - Sending data is not needed (maybe replace later)
pub fn write_wrapper(data:  Vec<Vec<(usize, u32, u32, u32)>>, index2pos: HashMap<String, Vec<usize>>, paths: &Vec<NPath>, filename_prefix: &str, bubbles: Vec<(u32, u32)>) {
    // Create the two files
    let file1 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".bed").unwrap())));
    let file2 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".stats").unwrap())));


    // Additional references
    let arc_index = Arc::new(index2pos);
    let arc_paths = Arc::new(paths.clone());
    let data_len = data.len().clone();
    let arc_bubbles = Arc::new(bubbles);

    // Do it like this
    let (send, rev) = unbounded();

    for x in data.into_iter(){
        let send_clone = send.clone();
        let arc_index_v2 = arc_index.clone();
        let arc_paths_v2 = arc_paths.clone();
        let arc_file1_v2 = file1.clone();
        let arc_file2_v2 = file2.clone();
        let arc_bubbles = arc_bubbles.clone();


        thread::spawn(move || {
            let ff = x.len().clone();
            traversal_stats(x, arc_index_v2, arc_paths_v2, arc_file1_v2, arc_file2_v2, arc_bubbles);
            send_clone.send(format!("1 {}", ff)).unwrap();
        });
    }

    // Just waiting
    for _x in 0..data_len{
        rev.recv().expect("Nothing");


    }
}

/// The actual writer function
/// TODO
/// - Add bubbles here
///
/// Writes [min, max, mean, interval_count, traversal count] in a bubble stats file (missing bubble start and end)
/// Writes accession_name, start, end, interval and bubble number in a bed file.
pub fn traversal_stats(data:  Vec<(usize, u32, u32, u32)>, index2: Arc<HashMap<String, Vec<usize>>>, paths: Arc<Vec<NPath>>, d: Arc<Mutex<BufWriter<File>>>,  d2: Arc<Mutex<BufWriter<File>>>, bubbles: Arc<Vec<(u32, u32)>>) {
    // Initialize variables
    let mut traversal:  Vec<(&[u32], &[bool])> = Vec::new();
    let mut tmp_data: Vec<(usize, u32, u32, u32)> = Vec::new();
    let mut interval_size: Vec<(usize, u32, usize, usize)> = Vec::new();
    let mut traversal_number = 0;
    let mut sizes: Vec<usize> = Vec::new();
    let mut old_bub = data[0].3.clone();




    for interval in data.into_iter(){
        //println!("{:?}", x);


        if interval.3 != old_bub {
            traversal = Vec::new();
            let bubble = bubbles.get(old_bub as usize).unwrap();


            let _soo = tmp_data.len().clone();
            let mut dd = d.lock().unwrap();
            let mut dd2 = d2.lock().unwrap();
            // What is missing? bubble_id, start, end, #subbubbles, parents, ratio?, type
            write!(dd2, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");

            for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
                write!(dd, "{}\t{}\t{}\tb{}\tt{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3 , _x1.3, _x2.0).expect("help");
            }
            // Reset all variables
            sizes = Vec::new();
            tmp_data = Vec::new();
            interval_size = Vec::new();
            traversal_number = 0;
            old_bub = interval.3;

        }



        let p = &paths[interval.0];

        let k = &p.nodes[(interval.1 + 1) as usize..interval.2 as usize];
        let k2 = &p.dir[(interval.1 + 1) as usize..interval.2 as usize];
        let k10: (&[u32], &[bool]) = (k, k2);
        if traversal.contains(&k10) {
            traversal_number = traversal.iter().position(|r| *r == k10).unwrap();
        } else {
            traversal.push(k10);
            traversal_number += 1;
        }


        // Start end
        let from_id: usize = index2.get(&paths[interval.0].name).unwrap()[interval.1 as usize];
        let mut to_id: usize = index2.get(&paths[interval.0].name).unwrap()[interval.2 as usize - 1];
        if interval.2 == interval.1 + 1 {
            to_id = from_id.clone();
        }
        tmp_data.push(interval);
        sizes.push(to_id-from_id);

        interval_size.push((traversal_number, (to_id-from_id) as u32, from_id, to_id));

    }

    // Do the same again when everything is done.
    let mut dd = d.lock().unwrap();
    let mut dd2 = d2.lock().unwrap();

    let soo = tmp_data.len().clone();
    let g = tmp_data.first().unwrap().3;
    let bubble = bubbles.get(g as usize).unwrap();

    write!(dd2, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", g, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");

    for (x1, x2) in tmp_data.into_iter().zip(interval_size.iter()){
        write!(dd, "{}\t{}\t{}\t{}\t{}\n", paths.get(x1.0).unwrap().name, x2.2, x2.3 , x1.3, x2.0).expect("heda");
    }
}




