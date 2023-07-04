# UnityRustEXtractor [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs]][docs.rs] [![License_MIT]][license_mit] [![License_APACHE]][license_apache] 

[Build Status]: https://img.shields.io/github/actions/workflow/status/UnityRustEXtractor/urex/ci.yml?branch=main
[actions]: https://github.com/UnityRustEXtractor/urex/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/urex.svg
[crates.io]: https://crates.io/crates/urex
[Docs]: https://docs.rs/urex/badge.svg
[docs.rs]: https://docs.rs/crate/urex/
[License_MIT]: https://img.shields.io/badge/License-MIT-yellow.svg
[license_mit]: https://raw.githubusercontent.com/UnityRustEXtractor/urex/main/LICENSE-MIT
[License_APACHE]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[license_apache]: https://raw.githubusercontent.com/UnityRustEXtractor/urex/main/LICENSE-APACHE


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
use std::{
    fs::{DirBuilder, File},
    io::{Seek, Write},
    path::Path,
};

use urex::files::{BundleFile, SerializedFile};
use urex::config::ExtractionConfig;

let mut reader = File::open(fp).unwrap();
let export_dir = Path::new("dump");

// parse the bundle file
let config = ExtractionConfig::new();
let mut bundle = BundleFile::from_reader(&mut reader, &config).unwrap();

// iterate over the files in the bundle
for directory in &bundle.m_DirectoryInfo {
    // generate export dir for cab
    let export_cab_dir = export_dir.join(&directory.path);
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

                // try to get the name
                let name = match handler.peak_name() {
                    Ok(name) => format!("{}_{}", object.m_PathID, name),
                    Err(_) => format!("{}", object.m_PathID),
                };

                // ensure that the parent directory exists
                let dst_path = export_cab_dir.join(name);
                DirBuilder::new()
                    .recursive(true)
                    .create(dst_path.parent().unwrap())
                    .unwrap_or_else(|_| panic!("Failed to create {:?}", dst_path.parent()));

                // parse the object as json
                let json = handler.parse_as_json().unwrap();
                // println!("{:?}", json);
                File::create(format!("{}.json", dst_path.to_string_lossy()))
                    .unwrap()
                    .write_all(json.to_string().as_bytes())
                    .unwrap();

                // parse the object as yaml
                let yaml = handler.parse_as_yaml().unwrap().unwrap();
                // println!("{:?}", yaml);
                File::create(format!("{}.yaml", dst_path.to_string_lossy()))
                    .unwrap()
                    .write_all(serde_yaml::to_string(&yaml).unwrap().as_bytes())
                    .unwrap();

                // parse the object as msgpack
                let msgpack = handler.parse_as_msgpack().unwrap();
                File::create(format!("{}.msgpack", dst_path.to_string_lossy()))
                    .unwrap()
                    .write_all(&msgpack)
                    .unwrap();

                // serialize as actual class
                // note: a small part of the object classes isn't implemented yet
                if object.m_ClassID == urex::objects::map::AssetBundle {
                    let ab = handler
                        .parse::<urex::objects::classes::AssetBundle>()
                        .unwrap();
                    println!("{:?}", ab);
                }
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

  - [ ] WebFile
  - [x] SerializedFile

- Object Classes:

  - [x] Generator
  - [x] Parser
  - [ ] Writer
  - [ ] Export Functions

- Tests:

  - [ ] Normal Tests
  - [ ] Artificing Test Files
  - [ ] 100% Coverage

- Other:
  - [ ] feature config

## Getting Help

TODO:

- [ ] Docs
- [ ] GitHub Issues and Discussion
- [ ] Discord server

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

UnityRustEXtractor is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
