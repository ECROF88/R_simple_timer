#[cfg(test)]
mod tests {
    use crate::Adjlist::{ALGraph, AdjList, VertexData};

    use super::*;

    #[test]
    fn test_adjlist() {
        // Basic creation test
        let adj_list = AdjList { vexs: Vec::new() };
        assert_eq!(adj_list.vexs.len(), 0);
    }

    #[test]
    fn test_vertex_data() {
        let int_data = VertexData::INT(42);
        let str_data = VertexData::STR(String::from("vertex"));

        match int_data {
            VertexData::INT(val) => assert_eq!(val, 42),
            _ => panic!("Expected INT variant"),
        }

        match str_data {
            VertexData::STR(val) => assert_eq!(val, "vertex"),
            _ => panic!("Expected STR variant"),
        }
    }

    #[test]
    fn test_graph_creation() {
        let graph = ALGraph {
            vertices: AdjList { vexs: Vec::new() },
            vexnum: 0,
            arcnum: 0,
        };

        assert_eq!(graph.vexnum, 0);
        assert_eq!(graph.arcnum, 0);
        assert_eq!(graph.vertices.vexs.len(), 0);
    }
}
