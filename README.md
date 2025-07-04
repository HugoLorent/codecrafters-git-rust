# Git Implementation in Rust

A Git implementation written in Rust, built as part of the [CodeCrafters](https://codecrafters.io) "Build Your Own Git" challenge.

## Features

This implementation supports the core Git object model and basic operations:

### Git Objects
- **Blob objects** - Store file contents
- **Tree objects** - Store directory structures
- **Commit objects** - Store snapshots with metadata

### Commands
- `init` - Initialize a new Git repository
- `cat-file -p <hash>` - Display object contents
- `hash-object [-w] <file>` - Compute object hash and optionally store it
- `ls-tree [--name-only] <hash>` - List tree contents
- `write-tree` - Create a tree object from the current directory
- `commit-tree <tree> [-p <parent>] -m <message>` - Create a commit object

## Architecture

The project follows a clean modular architecture:

```
src/
├── main.rs              # CLI entry point and argument parsing
├── commands.rs          # Commands module declarations
├── commands/            # Command implementations
│   ├── init.rs
│   ├── cat_file.rs
│   ├── hash_object.rs
│   ├── ls_tree.rs
│   ├── write_tree.rs
│   └── commit_tree.rs
├── git_objects.rs       # Git objects module and re-exports
└── git_objects/         # Git object handling
    ├── blob.rs         # Blob object operations
    ├── tree.rs         # Tree object operations
    ├── commit.rs       # Commit object operations
    ├── hash.rs         # SHA-1 hashing utilities
    └── path.rs         # File path utilities
```

## Installation

### Prerequisites
- Rust 1.80 or later
- Git (for testing compatibility)

### Build
```bash
git clone <repository-url>
cd codecrafters-git-rust
cargo build --release
```

## Usage

### Initialize a repository
```bash
cargo run init
```

### Create and examine objects
```bash
# Create a blob from a file
cargo run hash-object -w myfile.txt

# Examine the blob
cargo run cat-file -p <blob-hash>

# Create a tree from current directory
cargo run write-tree

# Examine the tree
cargo run ls-tree <tree-hash>

# Create a commit
cargo run commit-tree <tree-hash> -m "Initial commit"

# Examine the commit
cargo run cat-file -p <commit-hash>
```

## Dependencies

- **anyhow** - Error handling
- **clap** - Command line argument parsing
- **flate2** - Zlib compression/decompression
- **sha1** - SHA-1 hashing
