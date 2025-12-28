# 🗣 airhorn 

The airhorn soundclip you know and love, in your terminal. 


# Preview

<audio controls>
  <source src="https://github.com/brendanjryan/airhorn/raw/main/assets/airhorn.wav" type="audio/wav">
  Your browser does not support the audio element. <a href="https://github.com/brendanjryan/airhorn/raw/main/assets/airhorn.wav">Download the audio file</a>.
</audio>

Or [click here to preview the airhorn sound](https://github.com/brendanjryan/airhorn/raw/main/assets/airhorn.wav).

## Installation

### From source (local)

```bash
cargo install --path .
```

### From crates.io

```bash
cargo install airhorn
```

## Usage

```bash
airhorn

ls && airhorn

make && airhorn

git push && airhorn

# etc...
```

## Building from source

```bash
# Check formatting, linting, tests, and build
make check

# Build release binary
make build

# Run the airhorn
make run
```

## License

Apache-2.0
