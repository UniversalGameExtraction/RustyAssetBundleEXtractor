# UnityRustEXtractor [![Build Status]][actions] [![Latest Version]][crates.io]
[Build Status]: https://img.shields.io/github/actions/workflow/status/UnityRustEXtractor/urex/ci.yml?branch=main
[actions]: https://github.com/UnityRustEXtractor/urex/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/urex.svg
[crates.io]: https://crates.io/crates/urex


A work-in-progress extractor and patcher for Unity Engine asset files.
Currently it can do about nothing, so please check back later.

### Dependencies

This projects uses following dependencies:

- Compression & Decompression:
  - BundleFiles blocks:
    - [lzma-rs](https://crates.io/crates/lzma-rs)
    - [lz4_flex](lz4_flex) 
  - WebFile:
    - [libflate](https://crates.io/crates/libflate)
    - [brotli](https://crates.io/crates/brotli)

- Other:
  - [bitflags](https://crates.io/crates/bitflags)
  - [num_enum](https://crates.io/crates/num_enum)

## Notes

### TODO
- Parsers:
  - WebFile
  - SerializedFile
- Object Classes:
  - Generator
  - Parser & Writer Generator
  - Export Functions
- Tests:
  - Normal Tests
  - Artificing Test Files
  - 100% Coverage

## Getting Help

TODO:
- Docs
- GitHub Issues and Discussion
- Discord server

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

UnityRustEXtractor is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
