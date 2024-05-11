#[cfg(all(test, feature = "serde"))]
mod tests {
    use std::{error::Error, path::Path};

    use obographs::model::GraphDocument;

    #[test]
    fn test_from_path() -> Result<(), Box<dyn Error>> {
        let path = Path::new("tests/test_data/hp.mini.json");

        let graph_document = GraphDocument::from_path(path)?;

        assert_eq!(graph_document.graphs.len(), 1);

        let graph = &graph_document.graphs[0];
        assert_eq!(graph.nodes.len(), 11);
        assert_eq!(graph.edges.len(), 5);
        Ok(())
    }
}
