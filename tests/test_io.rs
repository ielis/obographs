#[cfg(all(test, feature = "serde"))]
mod tests {
    use std::error::Error;

    use obographs_dev::model::GraphDocument;

    #[test]
    fn test_from_path() -> Result<(), Box<dyn Error>> {
        let graph_document = GraphDocument::from_path("tests/test_data/hp.mini.json")?;

        assert_eq!(graph_document.graphs.len(), 1);

        let graph = &graph_document.graphs[0];
        assert_eq!(graph.nodes.len(), 11);
        assert_eq!(graph.edges.len(), 5);
        Ok(())
    }
}
