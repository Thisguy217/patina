# patina
A Datalog Interpreter written in Rust, so it is *blazingly* fast.

## Features

- Written in Rust
- Interprets only `.dl` files because I said so.
- ...I don't know what else...it is a binary?

## Installation

### From Source

1. Clone the repository:
```bash
git clone https://github.com/Thisguy217/patina.git
cd patina
```

2. Build the project:
```bash
cargo build --release
```

3. Run the interpreter:
```bash
./target/release/patina <datalog file>
```

### From Binary

Download the latest release from the [Releases](https://github.com/Thisguy217/patina/releases) page.

## Usage

### Basic Example

```bash
[Show a simple usage example. Example:
$ ./interpreter <datalog file>
```

### Command-Line Options

- Maybe this should be a thing...I guess...

## Language Syntax

I should fill this out a little more later, but your program needs to have the following generalized structure to work properly:

```
Schemes:
<structure of data>

Facts:
<dataa>

Rules:
<rules to structure the data>

Queries:
<questions on the data>

```

### Example Program

```
Schemes:
  a(x)
  b(y)

Facts:
  a('1').
  a('2').
  a('4').

Rules:
  b(y) :- a(y).

Queries:
  a(x)?
  b(y)?
```

## Building & Testing

```bash
# Build
cargo build

# Run tests
bash run-tests.sh # Normal people
bat run-tests.bat # Windows CMD
```

## Requirements

- Rust 1.89 or later (I actually don't know what the minimum version is...I should test that)

## Contributing

Any contributors are welcome! Feel free to open issues or submit pull requests. I am not a great Rust developer, but I think it is a fun language to work in.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- C S 236 at BYU follows the general requirements that were set forth in this project, but they require it to be done in `C++`.

## TODO / Future Work

- [ ] Clean up the outputs; right now it has a ton of output, and I think it should be optional.
- [ ] Stronger types. I know it was a predecessor, but maybe we could make it better by introducing types?








