
pub fn chunk_by_index(mut input:  Vec<(usize, u32, u32, u32)>, bubble_numb: u32, numb: u32) -> Vec<Vec<(usize, u32, u32, u32)>> {
    input.sort_by_key(|a| (a.3));

    let mut vec_new: Vec<Vec<(usize, u32, u32, u32)>> = Vec::new();

    let each_size = (bubble_numb as f64 /numb as f64).ceil() as usize;
    let mut vec_hell = Vec::new();
    let mut start = each_size.clone();
    for (i, x) in input.iter().enumerate(){
        if x.3 > ((start-1) as u32){
            vec_hell.push(i);
            start += each_size;
        }
    }
    start = 0;
    for x in vec_hell.iter(){
        vec_new.push(input[start..*x].to_vec());
        start = x.clone();

    }
    vec_new.push(input[start..].to_vec());


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


/// Mean function for usize verctor
///
/// Returns:
/// - Mean (in f64)
pub fn mean(numbers: &Vec<usize>) -> f64 {
    let sum: usize = numbers.iter().sum();
    let length = numbers.len() as f64;
    sum as f64 / length
}
