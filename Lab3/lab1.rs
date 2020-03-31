use std::io;
use std::convert::TryInto;

/* -------------------------------------------------------------------------- */
/*                                Main Function                               */
/* -------------------------------------------------------------------------- */

fn main() {
    println!("Please enter the number of values in the array: ")
    let mut n = String::new();                  //initialize and handle array length
    io::stdin().read_line(&mut n)
                .expect("Failed to read line!");
    let n: u32 = n.trim().parse()
                            .expect("Please enter a valid number");
    
    let mut arr: Vec<f64> = array_input(n.try_into().unwrap());   //initialize array as vec, vec makes pushing and other operations convenient
    
    let mean: f64 = find_mean(&arr);
    println!("mean: {}",mean);

    let median: f64 = find_median(&mut arr);
    println!("median: {}",median);

    let standard_deviation: f64 = find_sd(&arr, mean);
    println!("standard deviation: {}", standard_deviation);
}

/* ------------------------------- Array Input ------------------------------ */

fn array_input(n: i32) -> Vec<f64>{
    let mut arr: Vec<f64> = vec![];
    for x in 0..n{
        println!("Please enter the [{}] item of the array",x);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
                        .expect("Failed to read line!");
        let input: f64 = input.trim().parse()
                            .expect("Please enter a valid number!");
        arr.push(input.try_into().unwrap());
    }
    arr
}

/* -------------------------------- Find Mean ------------------------------- */

fn find_mean(arr: &[f64]) -> f64{
    let mut sum: f64 = 0.0;
    let len: f64 = arr.len() as f64;
    for x in 0..arr.len(){
        sum = sum + arr[x] as f64
    }
    sum/len
}
/* ------------------------------- Find Median ------------------------------ */

fn find_median(arr: &mut Vec<f64>) -> f64{
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = arr.len()/2;
    if arr.len() % 2 == 0 {
        (((arr[mid - 1] as f64/2.0) + (arr[mid] as f64/2.0)))
    }
    else{
        arr[mid] as f64
    }
}

/* ------------------------- Find Standard Deviation ------------------------ */

fn find_sd(arr: &[f64], mean: f64) -> f64{
    let len: f64 = arr.len() as f64;
    let mut sum: f64 = 0.0;
    for x in 0..arr.len(){
        sum = sum + (((arr[x] as f64) - (mean)).powf(2.0))/len;
    }
    sum = sum.sqrt();
    sum
}
