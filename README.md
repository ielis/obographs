# obographs-dev

Crate with definitions of Obographs types and I/O functions.

Typically, the library should be used with `serde` feature enabled
to support parsing Obographs JSON files.

Therefore, the following should be added into `Cargo.toml` file:

```toml
obographs-dev = {version = '0.2.1', features = ["serde"]}
```

> Note: Check `crates.io` for the latest version and update `0.2.1` accordingly.

## Examples

### Create an Obographs type

The Obographs elements can be created programatically.
For instance, the [`obographs::model::Edge`] can be created as:

```rust
use obographs_dev::model::*;

let edge = Edge {
  sub: String::from("http://purl.obolibrary.org/obo/HP_0001250"),
  pred: String::from("is_a"),
  obj: String::from("http://purl.obolibrary.org/obo/HP_0012638"),
  meta: None,
};
```

### Read an Obographs JSON file

With `serde` feature enabled, we can read a JSON file 
into a [`obographs::model::GraphDocument`] by running:

```rust
use obographs_dev::model::GraphDocument;

let toy_hpo_json = "tests/test_data/hp.mini.json";
let graph_document = GraphDocument::from_path(toy_hpo_json).expect("Read graph document from file path");

assert_eq!(graph_document.graphs.len(), 1);
```

## Tests and benches

Run tests with:

```shell
cargo test --features serde
```

The benchmarks can be run with: 

```shell
cargo bench --features serde
```

## Disclaimer

The implementation of the Obographs model is **INCOMPLETE**.
Attributes and components may be missing.
