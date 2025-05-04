use rand::Rng;
use std::fs;
use std::path::Path;
use tokio::task;
use tracing::{info, info_span, Instrument};
use tracing_texray::examine;

// Helper function to create test files with random content
fn create_test_files() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let files = vec!["small.txt", "medium.txt", "large.txt", "huge.txt"];
    let sizes = vec![1024, 1024 * 10, 1024 * 100, 1024 * 1000]; // 1KB, 10KB, 100KB, 1MB

    for (file, size) in files.iter().zip(sizes) {
        let content: String = (0..size)
            .map(|_| rng.gen_range(32..127) as u8 as char)
            .collect();
        fs::write(file, content).expect("Failed to write test file");
    }

    files.into_iter().map(String::from).collect()
}

// Example 1: Basic sequential file reading
fn example_sequential() {
    println!("\n=== Example 1: Sequential File Reading ===");
    let files = create_test_files();

    examine(info_span!("read_files")).in_scope(|| {
        for file in &files {
            info_span!("read_file", file = %file).in_scope(|| {
                let _data = fs::read_to_string(file).expect("Failed to read file");
                info!(%file, "Finished reading");
            });
        }
    });
}

// Example 2: Rayon parallel file reading
fn example_rayon() {
    println!("\n=== Example 2: Rayon Parallel File Reading ===");
    let files = create_test_files();

    examine(info_span!("read_files")).in_scope(|| {
        use rayon::prelude::*;
        files.par_iter().for_each(|file| {
            info_span!("read_file", file = %file).in_scope(|| {
                let _data = fs::read_to_string(file).expect("Failed to read file");
                info!(%file, "Finished reading");
            });
        });
    });
}

// Example 3: Maybe-rayon for debugging
fn example_maybe_rayon() {
    println!("\n=== Example 3: Maybe-rayon for Debugging ===");
    let files = create_test_files();

    examine(info_span!("read_files")).in_scope(|| {
        use p3_maybe_rayon::prelude::*;
        files.par_iter().for_each(|file| {
            info_span!("read_file", file = %file).in_scope(|| {
                let _data = fs::read_to_string(file).expect("Failed to read file");
                info!(%file, "Finished reading");
            });
        });
    });
}

// Example 4: Async file reading with tokio
async fn example_async() {
    println!("\n=== Example 4: Async File Reading ===");
    let files = create_test_files();

    examine(info_span!("read_files"))
        .in_scope(|| async {
            let mut handles = Vec::new();

            for file in &files {
                let file = file.clone();
                let span = info_span!("read_file", file = %file).or_current();
                let handle = task::spawn(
                    async move {
                        let _data = tokio::fs::read_to_string(&file)
                            .await
                            .expect("Failed to read file");
                        info!(%file, "Finished reading");
                    }
                    .instrument(span),
                );
                handles.push(handle);
            }

            for handle in handles {
                handle.await.expect("Task failed");
            }
        })
        .await;
}

#[tokio::main]
async fn main() {
    // Initialize tracing-texray
    tracing_texray::init();

    // Run all examples
    example_sequential();
    example_rayon();
    example_maybe_rayon();
    example_async().await;

    // Clean up test files
    for file in ["small.txt", "medium.txt", "large.txt", "huge.txt"].iter() {
        if Path::new(file).exists() {
            fs::remove_file(file).expect("Failed to remove test file");
        }
    }
}
