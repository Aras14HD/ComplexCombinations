use memoize::memoize;
use std::env;
use std::time::Instant;

#[memoize]
fn solve(arr: Vec<u128>) -> u128 {
    /*print!("Array:");
    for i in &arr {
        print!(" {i}");
    }
    println!("");*/
    if arr == vec![1] {
        /*for i in &arr {
            print!("{i} ");
        }
        println!("Got: 1");*/
        return 1;
    }
    //println!("Calculating!");
    let mut out: u128 = 0;
    let mut i = arr.len();
    while i > 0 {
        i -= 1;
        if arr[i] != 0 {
            let mut copy = arr.clone();
            let diff: u128;
            copy[i] -= 1;
            while copy[copy.len() - 1] == 0 {
                copy.truncate(copy.len() - 1);
            }
            if i == arr.len() - 1 {
                diff = arr[i];
            } else {
                diff = arr[i] - arr[i + 1]
            }
            if diff != 0 {
                let res = solve(copy);
                out += diff * res;
            }
        } else {
            println!("Wrong Format!");
            return 0;
        }
    }
    /*for i in &arr {
        print!("{i} ");
    }
    println!("Got: {out}");*/
    return out;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arr: Vec<u128> = Vec::new();
    for arg in args {
        if let Ok(n) = u128::from_str_radix(arg.as_str(), 10) {
            arr.push(n);
        }
    }
    let before = Instant::now();
    let res = solve(arr);
    println!("Took {:.2?}", before.elapsed());
    println!("Result: {res}");
}
