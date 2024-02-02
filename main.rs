use memoize::memoize;
use std::{env, time::Instant};

#[memoize]
fn solve(mut arr: Box<[u128]>) -> Result<u128, String> {
    // Safety: We now have arr as mutable, even though it shouldn't be, any change to it needs to be reverted!
    // print!("Array:");
    // for i in arr {
    //     print!(" {i}");
    // }
    // println!("");
    if *arr == [1] {
        return Ok(1);
    }
    // println!("Calculating!");
    let mut out: u128 = 0;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            let diff = if i == arr.len() - 1 {
                arr[i]
            } else {
                if arr[i + 1] > arr[i] {
                    return Err(format!(
                        "Invalid format! Following can't be bigger than pervious. Input: {:?}",
                        arr
                    ));
                }
                if arr[i + 1] == arr[i] {
                    continue;
                }
                arr[i] - arr[i + 1]
            };
            arr[i] -= 1;
            let (len, _) = arr
                .iter()
                .enumerate()
                .find(|x| x.1 == &0)
                .unwrap_or((arr.len(), &0));
            // for i in arr {
            //     print!("{i} ");
            // }
            // println!();
            // println!("Length: {len}");
            let res = solve(arr[..len].into())?;
            println!("{arr:?}: {diff} * {res}");
            out += diff * res;
            arr[i] += 1;
        } else {
            return Err(format!("Invalid format! Can't contain 0. Input: {:?}", arr));
        }
    }
    // for i in arr {
    //     print!("{i} ");
    // }
    // println!("Got: {out}");
    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arr: Vec<u128> = Vec::new();
    for arg in args {
        if let Ok(n) = str::parse(arg.as_str()) {
            arr.push(n);
        }
    }
    let before = Instant::now();
    let res = solve(arr.into()).unwrap();
    println!("Took: {:.2?}", before.elapsed());
    println!("Result: {res}");
}
