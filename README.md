# Rust (readonly) support for the CP437 encoding
cp537 aka DOS-US / OEM-US was widley used ~1980.

this crate supports reading the encoding in rust

# install
cp437 = "*"

# example

```rust

	let mut f = File::open("...").unwrap();
	let mut bytes = f.bytes();
	let mut r = Reader::new(&mut bytes);
	
	println!("Kurzname {}", r.consume(12)); // Kurzname

```

there is also a convert method you can use to convert a char byte by byte
