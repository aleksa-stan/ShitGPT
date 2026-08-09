# ShitGPT

**The cutting-edge, ultra-scalable AI model leveraging real-time dynamic inference to deliver hyper-relevant, next-generation enterprise conversational solutions.**

## Run it

Requires the [Rust toolchain](https://www.rust-lang.org/tools/install).

```
cargo run -- "how do I fix my boiler"    
echo "can dogs eat chocolate" | cargo run    
\\\# Open the interactive UI    
cargo run
```

Install it as a terminal command:

```
cargo install --git https://github.com/aleksa-stan/shitgpt    
shitgpt "Why is my code throwing a segmentation fault?"    
\\\# Open the interactive UI    
shitgpt
```

For local development before the repository exists, use `cargo install --path .` instead.

In the UI, type `/models` to list models, `/model 1` through `/model 4` to switch, `/clear` to reset the conversation, or `/exit` (or `q`) to quit.

`shitgpt` picks a stable bad reply from its response set based on the question. To print every response instead:

```
shitgpt --all "what time does the grocery store close"
```

## Development

```
cargo fmt --check    
cargo test    
cargo clippy -- -D warnings
```

## License

MIT. See [LICENSE](file:///home/aleksa/Documents/Codex/2026-08-09/mk/outputs/LICENSE).

