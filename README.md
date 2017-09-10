# huff-rs

Huffman encoding library.

This library has a `HuffTree` used for writing with `HuffWriter`
and reading with `HuffReader`.

## Example

Below is an example of encoding `"hello world"` with the string as
the input data to the Huffman tree.

```rust
let tree = HuffBuilder::new()
    .add('h', 1)
    .add('e', 1)
    .add('l', 2)
    .add('o', 2)
    .add(' ', 1)
    .add('w', 1)
    .add('r', 1)
    .add('d', 1)
    .build().unwrap();

let stream = vec![];
let writer = HuffWriter(tree, &mut stream);

for c in "hello world".to_owned() {
    writer.write(c);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
