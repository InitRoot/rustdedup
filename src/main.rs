use std::collections::{HashSet, hash_map::DefaultHasher};
use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn deduplicate_lines(input_path: Option<String>, output_path: Option<String>) -> io::Result<()> {
    let num_threads = num_cpus::get();
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    // Create worker threads
    for _ in 0..num_threads {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            let mut local_seen = HashSet::new();
            let rx = rx.lock().unwrap();
            let mut unique_lines = Vec::new();

            while let Ok(line) = rx.recv() {
                let mut hasher = DefaultHasher::new();
                line.hash(&mut hasher);
                let hash = hasher.finish();

                if local_seen.insert(hash) {
                    unique_lines.push(line);
                }
            }
            unique_lines
        });
        handles.push(handle);
    }

    // Conditionally read from a file or stdin
    let reader: Box<dyn BufRead> = if let Some(path) = input_path {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    for line in reader.lines() {
        let line = line?;
        tx.send(line).unwrap();
    }
    drop(tx); // Close the channel, causing worker threads to finish

    // Conditionally write to a file or stdout
    let writer: Box<dyn Write> = if let Some(path) = output_path {
        Box::new(BufWriter::new(File::create(path)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    let mut output_handle = writer;

    for handle in handles {
        let unique_lines = handle.join().unwrap();
        for line in unique_lines {
            writeln!(output_handle, "{}", line)?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut input_path = None;
    let mut output_path = None;

    // Parse command-line arguments for `-i` input and `-o` output paths
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-i" => input_path = args.get(i + 1).cloned(),
            "-o" => output_path = args.get(i + 1).cloned(),
            _ => {}
        }
    }

    deduplicate_lines(input_path, output_path)
}
