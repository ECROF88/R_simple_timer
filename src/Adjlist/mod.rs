const MAX_VERTEX_NUM: usize = 100;
pub struct ArcNode {
    adjvex: i32,
    nextarc: Box<ArcNode>,
}
pub enum VertexData {
    INT(i32),
    STR(String),
}
pub struct VNode {
    data: VertexData,
    firstarc: Box<ArcNode>,
}

pub struct ALGraph {
    vertices: AdjList,
    vexnum: usize,
    arcnum: usize,
}

pub struct AdjList {
    vexs: Vec<VNode>,
}
pub mod test;

impl ALGraph {
    pub fn add_node(&mut self, node: VNode) {
        if self.vertices.vexs.len() < MAX_VERTEX_NUM {
            self.vertices.vexs.push(node);
        }
    }

    pub fn make_edge_between_node(&mut self) {}
}
