use gfa_reader::NCPath;

/// Chunks our data set in chunks of similiar size (or close to it) with regards that chunks include complete bubble id
///
/// Input:
/// - input: Vec(path_id, index_from, index_to, bubble_id)
/// - bubble_numb: Number of bubbles
/// - numb: Number of chunks
///
/// Returns:
/// - Vector of vectors in which every block contains complete blocks
///
/// # Example
///
/// ```
/// use bvd::helper::chunk_by_index;
/// let mut f = vec![(1,2,3,4), (2,3,4,5), (4,5,6,7), (10,12,12,11)];
/// let f2 = chunk_by_index(&mut f, 4, 2);
/// ```
pub fn chunk_by_index(mut input:  &mut Vec<(usize, u32, u32, u32)>, bubble_numb: u32, number_chunks: u32) -> Vec<&[(usize, u32, u32, u32)]> {
    // Sort by bubble_id
    input.sort_by_key(|a| (a.3));

    // Initialize the chunk vector
    let mut vec_new: Vec<&[(usize, u32, u32, u32)]> = Vec::with_capacity(number_chunks as usize);

    // Break each bubble by this
    // ceil function: (3.14).ceil() = 4
    let each_size = (bubble_numb as f64 / number_chunks as f64).ceil() as usize;
    let mut vec_hell = Vec::with_capacity(each_size);
    let mut start = each_size.clone();
    for (i, x) in input.iter().enumerate(){
        if x.3 > ((start-1) as u32){
            vec_hell.push(i);
            start += each_size;
        }
    }
    start = 0;
    for x in vec_hell.iter(){
        vec_new.push(&input[start..*x]);
        start = x.clone();

    }
    vec_new.push(&input[start..]);


    vec_new
}

pub fn chunk_by_index2(mut input:  &mut Vec<(usize, u32, u32, u32)>, bubble_numb: u32, number_chunks: u32) -> Vec<&[(usize, u32, u32, u32)]> {
    // Sort by bubble_id
    input.sort_by_key(|a| (a.3));

    // Initialize the chunk vector
    let mut vec_new: Vec<&[(usize, u32, u32, u32)]> = Vec::with_capacity(number_chunks as usize);

    // Break each bubble by this
    // ceil function: (3.14).ceil() = 4
    let each_size = (bubble_numb as f64 / number_chunks as f64).ceil() as usize;
    let mut vec_hell = Vec::with_capacity(each_size);
    let mut start = each_size.clone();
    for (i, x) in input.iter().enumerate(){
        if x.3 > ((start-1) as u32){
            vec_hell.push(i);
            start += each_size;
        }
    }
    start = 0;
    for x in vec_hell.iter(){
        vec_new.push(&input[start..*x]);
        start = x.clone();

    }
    vec_new.push(&input[start..]);


    vec_new
}





/// **Get chunks of a Vector**
///
/// Takes full vector and get new vector
pub fn chunk_inplace<T>(it: Vec<T>, numb: usize) -> Vec<Vec<T>>{
    let mut vec_new: Vec<Vec<T>> = Vec::new();
    for _x in 0..numb{
        vec_new.push(Vec::new());
    }
    let each_size = (it.len() as f64 /numb as f64).ceil() as usize;

    let mut count = 0;
    for x in it{

        vec_new[count/each_size].push(x);
        count += 1;

    }
    vec_new

}


pub fn getSlice_test<'a>(data: &mut Vec<(usize, u32, u32, u32)>, path: &'a NCPath, index2: &Vec<usize>) -> Vec<(&'a [u32], &'a [bool])>{
    let mut slices = Vec::new();
    let mut hellper = Vec::new();
    for interval in data{

        let slice_node = &path.nodes[(interval.1 + 1) as usize..interval.2 as usize];
        let slice_dir = &path.dir[(interval.1 + 1) as usize..interval.2 as usize];
        slices.push((slice_node, slice_dir));
        let from_id: usize = index2[interval.1 as usize];
        let mut to_id: usize = index2[interval.2 as usize - 1];
        if interval.2 == interval.1 + 1 {
            to_id = from_id.clone();
        }
        hellper.push((from_id, to_id));
    }
    return slices
}


/// Mean function for usize verctor
///
/// Returns:
/// - Mean (in f64)
pub fn mean(numbers: &Vec<usize>) -> f64 {
    let sum: usize = numbers.iter().sum();
    let length = numbers.len() as f64;
    sum as f64 / length
}

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

