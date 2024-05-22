## What is kobolt? 
Kobolt is an open-source game engine backend, meaning that it will just implement the logic and core system/functionality of a game engine. Kobolt will **_not have any implemented graphics rendering_**, it will instead support rendering to a buffer to be used elsewere. [Crates.io](https://crates.io/crates/kobolt)

## Example code (just for testing)
```rust
use kobolt::{
    application::{
        Application,
        RunOption,
    },
    //renderer::Renderer, -- For rendering
};

fn main() {
    Application::new()
        .execute(start, RunOption::Init)
    ;
}

fn start() {
    println!("Kobolt is now initiated!");
}
```
