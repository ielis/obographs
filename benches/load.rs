use criterion::{black_box, criterion_group, criterion_main, Criterion};

use obographs::model::GraphDocument;

// The HPO to use for benchmarks.
const HPO_URL: &'static str = "https://github.com/obophenotype/human-phenotype-ontology/releases/download/v2023-10-09/hp.json";

fn benchmark_hpo_loading(c: &mut Criterion) {
    let hpo_path = util::download_hpo_if_necessary(HPO_URL);
    c.bench_function("GraphDocument::from_path", |b| {
        b.iter(|| {
            let gd = GraphDocument::from_path(&hpo_path).unwrap();
            black_box(gd);
        })
    });
}

criterion_group!(benches, benchmark_hpo_loading);
criterion_main!(benches);

/// Benchmarking utilities for benchmarking
mod util {
    use std::{
        error::Error,
        fs::{self, File},
        io::{self, BufWriter},
        path::{Path, PathBuf},
    };

    // A temporary folder for benchmark files.
    const DATA_DIR: &'static str = "data";

    pub fn download_hpo_if_necessary<T>(url: T) -> PathBuf
    where
        T: AsRef<str>,
    {
        let data_dir = Path::new(DATA_DIR);
        fs::create_dir_all(data_dir)
            .expect(format!("Unable to create data folder at {data_dir:?}").as_str());

        let out_path = data_dir.join("hp.json");
        if !out_path.is_file() {
            download_a_file(url, &out_path).expect("Cannot download a file");
        }

        out_path
    }

    fn download_a_file<T, U>(url: T, destination: U) -> Result<(), Box<dyn Error>>
    where
        T: AsRef<str>,
        U: AsRef<Path>,
    {
        let file = File::create(destination.as_ref())?;
        let mut writer = BufWriter::new(file);
        let mut resp = reqwest::blocking::get(url.as_ref())?;

        io::copy(&mut resp, &mut writer)?;

        Ok(())
    }
}
