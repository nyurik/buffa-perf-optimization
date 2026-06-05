# buffa packed repeated decode perf repro

See https://github.com/anthropics/buffa/issues/171

Minimal repro for allocation-heavy `buffa` view decoding of many small packed
`repeated uint32` fields.  I see a drop from 9.3s to 7.6s -- a 22% improvement.

The proto encodes a `Tile` with 65,536 `Feature` messages, each containing 16
packed values. Decoding this creates many small vectors, matching the expensive
shape seen in real MVT tiles.

```bash
just generate      # writes packed.bin
just time          # runs generated and fixed decoders
just flamegraphs   # writes flamegraph-generated.svg and flamegraph-fixed.svg
```

`src/generated/` is unmodified `buffa-build` output. `src/fixed/` is the same
generated code with one manual change in `perf.__view.rs`: packed values are
decoded into a preallocated temporary vector, then assigned to the view.

### Generated

```rust
while !pcur.is_empty() {
    view.values.push(::buffa::types::decode_uint32(&mut pcur)?);
}
```

### Fixed

Note that this loop can be further optimized if `view.values` would support `.reserve()` method. 

```rust
let mut values = ::buffa::alloc::vec::Vec::with_capacity(payload.len());
while !pcur.is_empty() {
    values.push(::buffa::types::decode_uint32(&mut pcur)?);
}
if view.values.is_empty() {
    view.values = values.into();
} else {
    for value in values {
        view.values.push(value);
    }
}
```

# LICENSE
This project is licensed under the CC0-1.0 License (public domain).
