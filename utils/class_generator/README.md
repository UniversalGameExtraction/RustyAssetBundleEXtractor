# Class Generator

This dir contains scripts to generate `urex::objects::classes`.

`metadata_scraper.py` scraps the [Unity Scripting Reference](https://docs.unity3d.com/ScriptReference/) and generates a `metadata.json`.

`generator.py` uses [TypeTreeDumps/InfoJson](https://github.com/AssetRipper/TypeTreeDumps/tree/main/InfoJson) to generate `classes.rs`.
If a `metadata.json` is present, then it will also try to find relevant information about each class and its field, adding them as documentation to `classes.rs`

## Usage

1. clone [AssetRipper/TypeTreeDumps](https://github.com/AssetRipper/TypeTreeDumps)
2. set ``TYPETREEDUMPS_PATH`` to the clone dir
3. run ``metadata_scraper.py``
4. run ``generator.py``
5. copy generated ``classes.rs`` to ``urex/objects/classes.rs``

## Requirements

- Python 3.7+

  - requests
  - bs4

- local (shallow) clone of [AssetRipper/TypeTreeDumps](https://github.com/AssetRipper/TypeTreeDumps)
