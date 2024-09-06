# landscaper_wiki

**Idea:** creator and viewer of plants

## Usage
```bash
landscaper_wiki % cargo run
```

## Visuals
You can create a new plant by entering its name, a description and a price per pounds (or lbs).
![screenshot-1](/visuals/add-plant.png)

You can see the details of a newly created plant (red). You can minimize the pop-up for creating a new plant (orange). You can delete a plant, for example I have deleted the potato (yellow).
![screenshot-2](/visuals/description-reduce-and-remove.png)

You can export the current list of plants, which will create an `export.txt` file. You can import a list of plants from a file named `import.txt` located in `landscaper_wiki`.
![screenshot-3](/visuals/export-and-import.png)

## Stack
Written in [Rust](https://www.rust-lang.org/) using [egui](https://www.egui.rs/)

## Requirements
```bash
# Install Rust (rustup 1.27.1)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Dependencies
- `eframe`: the `egui` framework crate
- `egui`: an easy-to-use GUI in pure Rust
