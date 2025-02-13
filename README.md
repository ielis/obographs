# obographs-dev

Crate with definitions of Obographs types and I/O functions.


## Examples

### How to use

Obographs-dev is deployed on `crates.io`, so including in a project is really easy.
Just add the following into your `Cargo.toml` file:

```toml
obographs-dev = "0.2.1"
```

> Note: Check `crates.io` for the latest version and update `0.2.1` accordingly.


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

The `serde` feature is enabled by default,
to support reading a JSON file into a [`obographs::model::GraphDocument`]:

```rust
use obographs_dev::model::GraphDocument;

let toy_hpo_json = "tests/test_data/hp.mini.json";
let graph_document = GraphDocument::from_path(toy_hpo_json).expect("Read graph document from file path");

assert_eq!(graph_document.graphs.len(), 1);
```

## Tests and benches

Run the unit, integration, and documentation tests with:

```shell
cargo test
```

The benchmarks can be run with: 

```shell
cargo bench
```

## Disclaimer

The implementation of the Obographs model is **INCOMPLETE**.
Attributes and components may be missing.
