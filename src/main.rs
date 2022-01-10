//use std::thread;

fn main() {
    let mut vector = vec![2i32; 100];
    let mut vector2 = vec![2i32; 100];
    for i in 1..101 {
        vector[100-(i as usize)]=i;
        vector2[100-(i as usize)]=i;
    }
    //println!("inital {:?}",vector);
    let sorted =merge_sort(&vector, 0);
    println!("Merge");
    for val in sorted {
        print!("{},",val);
    }
    let sorted2 =quick_sort(&vector2, 0);
    println!("\nquick");
    for val in sorted2 {
        print!("{},",val);
    }
}
//passes immutable reference vector to sort alg then returns a sorted vector
fn merge_sort(vector: &Vec<i32>, num_additional_threads: i8)->Vec<i32> {
    //if length is 1 then vector is sorted
    if vector.len() == 1{
        return vec![vector[0]];
    }

    //set up left side vector
    let mut left_vect =vec![0i32; vector.len()/2];
    for i in 0..vector.len()/2 {
        left_vect[i] = vector[i];
    }

    //set up right side vector
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

    //sort both sides
    
    //determines how many threads each half should create
    let right_additional_threads = num_additional_threads /2;
    let left_additional_threads;
    //makes sure left additional threads is never less than zero
    if num_additional_threads <=right_additional_threads +1 {
        left_additional_threads =0;
    } else {
        left_additional_threads =num_additional_threads -right_additional_threads -1;
    }

    let left;
    let right;
    let handler;
    //if more threads are asked for then spawn a new thread for one of the halfs
    if num_additional_threads > 0 {
        handler =std::thread::spawn(move || {merge_sort(&left_vect, left_additional_threads)}); //spawn new thread
        right = merge_sort(&right_vect, right_additional_threads); //do right side
        left = handler.join().unwrap(); //join the left side
        
    } else{
        left = merge_sort(&left_vect, left_additional_threads); //do the left side
        right = merge_sort(&right_vect, right_additional_threads); //do the right side
    }
    
    

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

    //puts any left over unsorted stuff into the sorted
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

//passes immutable vector to sort alg then retruns a sorted vector
fn quick_sort(vector: &Vec<i32>, num_additional_threads: i8)->Vec<i32> {
    //if length is 1 then vector is sorted
    if vector.len() == 1{
        return vec![vector[0]];
    } else if vector.len() ==0 {
        return vec![];
    }
    
    let pivot = vector[vector.len() -1];

    //set up both sides of the vector by pushing all values less than pivot to the left vector and all greater than to the right
    let mut left_vect =vec![0i32; 0];
    let mut right_vect =vec![-1i32; 0];
    for num in vector {
        if *num <=pivot {
            left_vect.push(*num);
        } else {
            right_vect.push(*num);
        }
    }
    left_vect.pop(); //removes pivot from the vector
    
    //sort both sides
    //determines how many threads each half should create
    let right_additional_threads = num_additional_threads /2;
    let left_additional_threads;
    //makes sure left additional threads is never less than zero
    if num_additional_threads <=right_additional_threads +1 {
        left_additional_threads =0;
    } else {
        left_additional_threads =num_additional_threads -right_additional_threads -1;
    }

    let left;
    let right;
    let handler;
    //if more threads are asked for then spawn a new thread for one of the halfs
    if num_additional_threads > 0 {
        handler =std::thread::spawn(move || {quick_sort(&left_vect, left_additional_threads)}); //spawn new thread
        right = quick_sort(&right_vect, right_additional_threads); //do right side
        left = handler.join().unwrap(); //join the left side
        
    } else{
        left = quick_sort(&left_vect, left_additional_threads); //do the left side
        right = quick_sort(&right_vect, right_additional_threads); //do the right side
    }
    
    

    //form this into a new list
    let mut sorted = vec![0i32;0];
    for num in &left {
        sorted.push(*num);
    }
    sorted.push(pivot);
    for num in &right {
        sorted.push(*num);
    }

    return sorted;
}