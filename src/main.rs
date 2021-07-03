use perm::{IntoOptimizedChunks, Permutations};
use std::convert::TryInto;
use std::io::{self, BufRead, Write};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let v = 1;
    let v = v.clone();
    let reader = io::stdin();

    let input = reader
        .lock()
        .lines()
        .next()
        .expect("Empty input")
        .expect("Error reading input");
    let c = input.as_str();

    let p: Permutations<String> = input.try_into().unwrap();

    let r = match p.try_into_optimized_chunks(10) {
        Ok(optimized_iterator) => optimized_iterator
            .map(|chunk| {
                thread::spawn(move || {
                    io::stdout()
                        .write_all(chunk.to_string().as_ref())
                        .expect("Error writing data")
                })
            })
            .collect::<Vec<JoinHandle<()>>>(),
        Err(e) => {
            todo!()
            //eprint!("Cannot use optimized permutations: {}\n. Using normal permutations",e);
            //p.into_chunks(10).map(|c|output_chunk_new_thread(io::stdout(),c)).collect::<Vec<JoinHandle<()>>>()
        }
    };
    r.into_iter().for_each(|h| {
        h.join();
    })
}

fn output_chunk_new_thread<
    W: 'static + Write + Send + Sync,
    T: 'static + ToString + Send + Sync,
>(
    mut writer: W,
    chunk: T,
) -> JoinHandle<()> {
    thread::spawn(move || {
        writer
            .write_all(chunk.to_string().as_ref())
            .expect("Error writing data")
    })
}
