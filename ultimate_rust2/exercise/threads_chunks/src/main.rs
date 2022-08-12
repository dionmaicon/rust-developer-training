// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: &[u32]) -> u32 {
    // Pretend our fancy little filter-map-sum is expensive and takes 500ms
    sleep_ms(500);
    println!("Child thread: just about finished {:#?}", v);
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn split<T>(arr: &[T], n: usize ) -> Vec<&[T]> where T: Debug {
    let chunks: Vec<&[T]> = arr.chunks(n).collect();
    chunks
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let subsets: Vec<&[u32]> = split(&my_vector, 2);

    let mut subts= Vec::new();

    for set in subsets {
        let cloned =  set.to_vec();
        let th = thread::spawn( move || {
           return expensive_sum(&cloned);
        });

        subts.push(th);
    }

    for sub in subts {
        let result = sub.join().expect("Error");
        println!("{:#?}", result );
    }

    // While the child thread is running, the main thread will also do some work
    for letter in &["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    let hash: HashMap<String, i32> = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("ten", 10)
    ].iter().map(|&(k, v)| (k.into(), v)).collect();
    
    let list: Vec<_> = hash.into_iter().collect();

    let chunk_len = (list.len() / 8) as usize + 1;
    let chunks: Vec<HashMap<_, _>> = list.chunks(chunk_len)
        .map(|c| c.iter().cloned().collect())
        .collect();

    println!("{:#?}", chunks );

    println!("Main thread: Exiting.")
}
