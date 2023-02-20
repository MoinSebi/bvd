use std::fs::File;
use std::io::{Write, BufWriter};
use std::ptr::write;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_channel::unbounded;
use gfaR_wrapper::NPath;
use hashbrown::HashMap;
use crate::helper::mean;


/// Wrapper function for traversal and bubble output files
///
/// Writing two files at the same time
///
/// Comment:
/// - Sending data is not needed (maybe replace later)
pub fn write_wrapper(data:  Vec<Vec<(usize, u32, u32, u32)>>, index2pos: Vec<Vec<usize>>, paths: Vec<NPath>, filename_prefix: &str, bubbles: Vec<(u32, u32)>) {
    // Create the two files
    let file1 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".bed").unwrap())));
    let file2 = Arc::new(Mutex::new(BufWriter::new(File::create(filename_prefix.to_owned() + ".stats").unwrap())));


    // Additional references
    let arc_index = Arc::new(index2pos);
    let arc_paths = Arc::new(paths);
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
        //let ff = Arc::new(*x);


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

pub fn solo_stats(data:  &[(usize, u32, u32, u32)], index2: Vec<Vec<usize>>, paths: Vec<NPath>, bubbles: Vec<(u32, u32)>){
    let mut file1 =BufWriter::new(File::create("test1".to_owned() + ".bed").unwrap());
    let mut file2 = BufWriter::new(File::create("test2".to_owned() + ".stats").unwrap());
    let mut traversal:  Vec<(&[u32], &[bool])> = Vec::new();
    let mut tmp_data: Vec<&(usize, u32, u32, u32)> = Vec::new();
    let mut interval_size: Vec<(usize, u32, usize, usize)> = Vec::new();
    let mut traversal_number = 0;
    let mut sizes: Vec<usize> = Vec::new();
    let mut old_bub = data[0].3.clone();
    let mut ff1: Vec<u8> = Vec::new();


    // Iterate over the data
    for interval in data.into_iter(){
        // Check if the interval contains a new bubble_id
        if interval.3 != old_bub {
            let bubble = bubbles.get(old_bub as usize).unwrap();
            let _soo = tmp_data.len().clone();

            // What is missing? bubble_id, start, end, #subbubbles, parents, ratio?, type
            write!(file1, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
            //let dd = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone());
            //ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());
            for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
                write!(file2, "{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).expect("help");
            }

            // Reset all variables
            traversal = Vec::new();
            sizes = Vec::new();
            tmp_data = Vec::new();
            interval_size = Vec::new();
            traversal_number = 0;
            old_bub = interval.3;

        }



        let path = &paths[interval.0];

        let slice_node = &path.nodes[(interval.1 + 1) as usize..interval.2 as usize];
        let slice_dir = &path.dir[(interval.1 + 1) as usize..interval.2 as usize];
        let slice_merge: (&[u32], &[bool]) = (slice_node, slice_dir);
        if traversal.contains(&slice_merge) {
            traversal_number = traversal.iter().position(|r| *r == slice_merge).unwrap();
        } else {
            traversal.push(slice_merge);
            traversal_number += 1;
        }


        // Start end
        let from_id: usize = index2[interval.0][interval.1 as usize];
        let mut to_id: usize = index2[interval.0][interval.2 as usize - 1];
        if interval.2 == interval.1 + 1 {
            to_id = from_id.clone();
        }
        tmp_data.push(interval);
        sizes.push(to_id-from_id);
        interval_size.push((traversal_number, (to_id-from_id) as u32, from_id, to_id));

    }

    // Do the same again when everything is done.
    let bubble_id = tmp_data.first().unwrap().3;
    let bubble = bubbles.get(bubble_id as usize).unwrap();

    write!(file1, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", bubble_id, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");

    for (x1, x2) in tmp_data.into_iter().zip(interval_size.iter()){
        write!(file2, "{}\t{}\t{}\t{}\t{}\n", paths.get(x1.0).unwrap().name, x2.2, x2.3, x1.3, x2.0).expect("heda");
    }
}

/// The actual writer function
/// TODO
/// - Add bubbles here
///
/// Writes [min, max, mean, interval_count, traversal count] in a bubble stats file (missing bubble start and end)
/// Writes accession_name, start, end, interval and bubble number in a bed file.
pub fn traversal_stats(data:  Vec<(usize, u32, u32, u32)>, index2: Arc<Vec<Vec<usize>>>, paths: Arc<Vec<NPath>>, file_bed: Arc<Mutex<BufWriter<File>>>, file_bubble: Arc<Mutex<BufWriter<File>>>, bubbles: Arc<Vec<(u32, u32)>>) {
    // Initialize variables
    let mut traversal:  Vec<(&[u32], &[bool])> = Vec::new();
    let mut tmp_data: Vec<(usize, u32, u32, u32)> = Vec::new();
    let mut interval_size: Vec<(usize, u32, usize, usize)> = Vec::new();
    let mut traversal_number = 0;
    let mut sizes: Vec<usize> = Vec::new();
    let mut old_bub = data[0].3.clone();
    let mut ff1: Vec<u8> = Vec::new();
    let mut ff2: Vec<u8> = Vec::new();



    // Iterate over the data
    for interval in data.into_iter(){
        // Check if the interval contains a new bubble_id
        if interval.3 != old_bub {
            let bubble = bubbles.get(old_bub as usize).unwrap();
            let _soo = tmp_data.len().clone();

            ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());

            // let mut bed_arc = file_bed.lock().unwrap();
            // let mut bubble_arc = file_bubble.lock().unwrap();
            // What is missing? bubble_id, start, end, #subbubbles, parents, ratio?, type

            // write!(bubble_arc, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
            // //let dd = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone());
            // //ff1.extend(format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", old_bub, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).as_bytes().to_vec());
            // for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
            //     write!(bed_arc, "{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).expect("help");
            // }
            for (_x1, _x2) in tmp_data.into_iter().zip(interval_size.iter()){
                ff2.extend(format!("{}\t{}\t{}\t{}\t{}\n", paths.get(_x1.0).unwrap().name, _x2.2, _x2.3, _x1.3, _x2.0).as_bytes().to_vec());
            }


            // Reset all variables
            traversal = Vec::new();
            sizes = Vec::new();
            tmp_data = Vec::new();
            interval_size = Vec::new();
            traversal_number = 0;
            old_bub = interval.3;

        }



        let path = &paths[interval.0];

        let slice_node = &path.nodes[(interval.1 + 1) as usize..interval.2 as usize];
        let slice_dir = &path.dir[(interval.1 + 1) as usize..interval.2 as usize];
        let slice_merge: (&[u32], &[bool]) = (slice_node, slice_dir);
        if traversal.contains(&slice_merge) {
            traversal_number = traversal.iter().position(|r| *r == slice_merge).unwrap();
        } else {
            traversal.push(slice_merge);
            traversal_number += 1;
        }


        // Start end
        let from_id: usize = index2[interval.0][interval.1 as usize];
        let mut to_id: usize = index2[interval.0][interval.2 as usize - 1];
        if interval.2 == interval.1 + 1 {
            to_id = from_id.clone();
        }
        tmp_data.push(interval);
        sizes.push(to_id-from_id);
        interval_size.push((traversal_number, (to_id-from_id) as u32, from_id, to_id));

    }

    // Do the same again when everything is done.

    let mut bed_arc = file_bed.lock().unwrap();
    let mut stats_arc = file_bubble.lock().unwrap();

    let bubble_id = tmp_data.first().unwrap().3;
    let bubble = bubbles.get(bubble_id as usize).unwrap();

    stats_arc.write_all(&ff1[..]);
    bed_arc.write_all(&ff2[..]);
    // write!(stats_arc, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", bubble_id, bubble.0, bubble.1, sizes.iter().min().unwrap().clone(), sizes.iter().max().unwrap().clone(), mean(&sizes), traversal_number as usize, tmp_data.len().clone()).expect("helpa");
    //
    // for (x1, x2) in tmp_data.into_iter().zip(interval_size.iter()){
    //     write!(bed_arc, "{}\t{}\t{}\t{}\t{}\n", paths.get(x1.0).unwrap().name, x2.2, x2.3, x1.3, x2.0).expect("heda");
    // }
}




