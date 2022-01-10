//use std::thread;

fn main() {
    let mut vector = vec![2i32; 100];
    for i in 1..101 {
        vector[100-(i as usize)]=i;
    }
    println!("arr{:?}",vector);
    let sorted =qsort(vector, 0);
    for val in sorted {
        print!("{},",val);
    }
}

fn qsort(vector: Vec<i32>, num_depth: i8)->Vec<i32> {
    //if length is 1 then vector is sorted
    if vector.len() == 1{
        return vector;
    }

    let mut left_vect =vec![0i32; vector.len()/2];
    for i in 0..vector.len()/2 {
        left_vect[i] = vector[i];
    }
    //sort both sides
    
    let left;
    if num_depth < 2 {
        println!("created thread");
        left = qsort(left_vect, num_depth +1);
    } else{
        left = qsort(left_vect, num_depth +1);
    }
    let size;
    if vector.len()%2 ==1 {
        size =vector.len()/2 +1;
    } else {
        size =vector.len()/2;
    }
    let mut right_vect =vec![-1i32; size];
    for i in 0..size  {
        right_vect[i] = vector[i+vector.len()/2];
    }
    let right = qsort(right_vect, num_depth +1);

    //form this into a new list
    let mut sorted = vec![0i32;vector.len()];

    let mut i =0;
    let mut j=0;
    let mut k=0;
    while i <left.len() && j<right.len() {
        if left[i] < right[j] {
            sorted[k] =left[i];
            k +=1;
            i+=1;
        } else {
            sorted[k] =right[j];
            k +=1;
            j+=1;
        }
    }

    if i ==left.len() {
        for l in j..right.len() {
            sorted[k]=right[l];
            k +=1;
        }
    } else {
        for l in i..left.len() {
            sorted[k]=left[l];
            k +=1;
        }
    }


    return sorted;
}
