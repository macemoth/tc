# tc

CLI utility to compute the number of `cl100k_base` tokens similar to `wc`.

## Examples

```bash
# Compute the number of tokens of a string
> echo "Never gonna give you up" | tc
6

# ...or the number of tokens within a file
> cat file.txt | tc
39

# ...or of a git repository
> git ls-files | xargs tc
213
```

## Installation

### Precompiled

1. Download from releases
2. Put the `tc` executable in your path

### Build from source

```bash
git clone https://github.com/macemoth/tc
cd tc
cargo build --release
```

Put the `tc` executable in your path