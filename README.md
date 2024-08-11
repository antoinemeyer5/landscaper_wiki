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
- Frontend (`client`): written in Rust using Yew
- Backend (`server`): written in Rust using Actix-Web

## Requirements
### Commun
```bash
# Install Rust (rustup 1.27.1)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Frontend
```bash
# Install WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (trunk 0.20.2)
cargo install --locked trunk
```

## Dependencies
### Frontend
- `yew`: Rust / Wasm (WebAssembly) framework for creating reliable and
    efficient web applications.
- `yew-router`: A routing library for the Yew frontend framework.
### Backend
- `actix-web`: Actix Web is a powerful, pragmatic, and extremely fast web
    framework for Rust.

## Usage
### Frontend (http://127.0.0.1:8080/)
```bash
landscaper % cd client
client % trunk serve
```
### Backend (http://127.0.0.1:8090/)
```bash
landscaper % cd server
server % cargo run
```
