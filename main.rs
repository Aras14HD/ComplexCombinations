use std::collections::HashMap;
use std::time::Instant;

fn solve(arr: Vec<u128>, map: &mut HashMap<Vec<u128>, u128>) -> (u128, HashMap<Vec<u128>, u128>) {
    print!("Array:");
    for i in &arr {
        print!(" {i}");
    }
    println!("");
    if let Some(&cached) = map.get(&arr) {
        println!("Cached!");
        for i in &arr {
            print!("{i} ")
        }
        println!("Got: {cached}");
        return (cached, map.clone());
    }
    if arr == vec![1] {
        for i in &arr {
            print!("{i} ");
        }
        println!("Got: 1");
        return (1, map.clone());
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
            if i == copy.len() - 1 && copy[i] == 0 {
                copy.truncate(copy.len() - 1);
            }
            if i == arr.len() - 1 {
                diff = arr[i];
            } else {
                diff = arr[i] - arr[i + 1]
            }
            if diff != 0 {
                let (res, nmap) = solve(copy, map);
                map.extend(nmap);
                out += diff * res;
            }
        }
    }
    for i in &arr {
        print!("{i} ");
    }
    map.insert(arr, out);
    println!("Got: {out}");
    return (out, map.clone());
}

fn main() {
    let before = Instant::now();
    let map: &mut HashMap<Vec<u128>, u128> = &mut HashMap::new();
    let (res, nmap) = solve(vec![9, 9, 9, 9, 9, 9], map);
    println!("Took {:.2?}", before.elapsed());
    println!("Result: {res}");
    println!("Cache Size: {}", nmap.len());
}
