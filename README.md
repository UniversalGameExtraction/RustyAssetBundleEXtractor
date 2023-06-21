# UnityRustEXtractor [![Build Status]][actions] [![Latest Version]][crates.io]
[Build Status]: https://img.shields.io/github/actions/workflow/status/UnityRustEXtractor/urex/ci.yml?branch=main
[actions]: https://github.com/UnityRustEXtractor/urex/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/urex.svg
[crates.io]: https://crates.io/crates/urex


A work-in-progress extractor and patcher for Unity Engine asset files.
Currently it can do about nothing, so please check back later.

## Dependencies

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

## Examples

### parsing a normal BundleFile and dumping its objects

```rust
use urex::files::{BundleFile, SerializedFile};
use urex::config::ExtractionConfig;

let mut reader = File::open(fp).unwrap();

// parse the bundle file
let config = ExtractionConfig::new();
let mut bundle = BundleFile::from_reader(&mut reader, &config).unwrap();

// iterate over the files in the bundle
for directory in &bundle.m_DirectoryInfo {
    // seek to the start of the file in the bundle
    bundle
        .m_BlockReader
        .seek(std::io::SeekFrom::Start(directory.offset as u64))
        .unwrap();
    
    // try to parse the file as a SerializedFile
    match SerializedFile::from_reader(&mut bundle.m_BlockReader, &config) {
        Ok(serialized) => {
            // iterate over objects
            for object in &serialized.m_Objects {
                // get a helper object to parse the object
                let mut handler =
                    serialized.get_object_handler(object, &mut bundle.m_BlockReader);
                // parse the object as json
                println!("{:?}", handler.parse_as_json().unwrap());
                // parse the object as yaml
                println!("{:?}", handler.parse_as_yaml().unwrap().unwrap());
            }
        }
        Err(e) => {
            // TODO - try to filter out resource files
            println!(
                "Failed to parse {} as SerializedFile.",
                &directory.path.to_string()
            );
        }
    }
}
```

### parsing a by UnityCN encrypted BundleFile and handling stripped Unity version
```rust
let mut reader = File::open(fp).unwrap();
let config = ExtractionConfig {
    unitycn_key: Some("Decryption Key".as_bytes().try_into().unwrap()),
    fallback_unity_version: "2020.3.0f1".to_owned(),
};
let bundle = crate::files::BundleFile::from_reader(&mut reader, &config).unwrap();
```

### reading a UnityCN encrypted BundleFile



## Notes

### TODO
- Parsers:
  - [] WebFile
  - [x] SerializedFile

- Object Classes:
  - [] Generator
  - [x] Parser
  - [] Writer
  - [] Export Functions

- Tests:
  - [] Normal Tests
  - [] Artificing Test Files
  - [] 100% Coverage

- Other:
  - [] feature config

## Getting Help

TODO:
- [] Docs
- [] GitHub Issues and Discussion
- [] Discord server

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

UnityRustEXtractor is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
