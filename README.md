DEWQ QR Code Generation Library
===============================

[<img alt="github" src="https://img.shields.io/badge/github-RubenMovsesyan/DEWQ-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/RubenMovsesyan/DEWQ)
[<img alt="crates.io" src="https://img.shields.io/crates/v/DEWQ.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/DEWQ)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-DEWQ-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/DEWQ)

DEWQ (Denso Wave QR Code Generator) is a QR Code generation library for rust designed to parse an input string with a given error correction level
and generate a bitmap of the qr code.

## Getting Started

Adding DEWQ to your project is as simple as
```bash
cargo add DEWQ
```

## Usage Examples
```rust
use qr_code::*;

let qr_code = create_qr_code("HELLO, WORLD!", ErrorCorrectionLevel::Q);
println!("{}", qr_code);
```

or you can save the bitmap to a file:

```rust
use qr_code::*;

create_qr_code("HELLO, WORLD!", ErrorCorrectionLevel::Q).save_to_file("./qrcode.bmp");
```

## Features

- [x] Numeric Encoding
- [x] Alphanumeric Encoding
- [x] Byte Encoding
- [ ] Kanji (Double Byte) Encoding
