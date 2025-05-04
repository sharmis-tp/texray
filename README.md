# TexRay

This repository demonstrates the use of `tracing-texray` for visualizing and debugging performance in Rust applications. It shows how to:

1. Instrument sequential code with spans and events
2. Handle tracing across parallel execution with Rayon
3. Debug parallel code using maybe-rayon
4. Properly propagate tracing context across async boundaries

## Running the texray

```bash
cargo run
```

The texray will:

1. Create test files of varying sizes (1KB to 1MB)
2. Run each example with tracing instrumentation
3. Display timing visualizations using tracing-texray
4. Clean up the test files

## What You'll See

The output shows ASCII timelines of execution, with:

- Overall operation duration
- Individual file read durations
- How different execution models (sequential, parallel, async) affect tracing visibility

This is particularly useful for:

- Identifying performance bottlenecks
- Understanding how tracing behaves in different execution contexts
- Debugging parallel and async code
