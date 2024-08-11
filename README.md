# landscaper

## Def
Their job duties include planting seasonal flowers, trimming hedges,
pruning trees, fertilizing plants, mowing grass and managing pests. Landscapers
may also work with building contractors to construct garden walls, walkways and
steps.

## Idea
Plant management application for your garden. Know when to harvest carrots,
when to water tomatoes, tell us when you've planted tulips, know when to clean
up strawberry cuttings, etc.

## Stack
Frontend in Rust (Yew).

## Requirements
```bash
# Install Rust (rustup 1.27.1)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (trunk 0.20.2)
cargo install --locked trunk

# Clone this repository
git clone git@github.com:antoinemeyer5/landscaper.git
```

## Usage
```bash
# Start the development server
landscaper % trunk serve --open
```
