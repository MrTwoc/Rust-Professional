/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    /// 添加一条边到图中
    /// 
    /// 如果边的两个节点都不存在，则先添加节点，然后添加边。
    /// 
    /// # 参数
    /// 
    /// * `edge` - 一个包含边的两个节点和权重的元组
    /// 
    /// # 示例
    /// 
    /// ```
    /// let mut graph = UndirectedGraph::new();
    /// graph.add_edge(("a", "b", 5));
    /// ```
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        // 解构边的元组，获取起始节点、目标节点和权重
        let (from_node, to_node, weight) = edge;
        // 检查起始节点是否存在，如果不存在，则添加节点
        if !self.contains(from_node) {
            self.add_node(from_node);
        }
        // 检查目标节点是否存在，如果不存在，则添加节点
        if !self.contains(to_node) {
            self.add_node(to_node);
        }
        // 获取起始节点的邻接表，并添加目标节点和权重
        self.adjacency_table_mutable().get_mut(from_node).unwrap().push((to_node.to_string(), weight));
        // 获取目标节点的邻接表，并添加起始节点和权重
        self.adjacency_table_mutable().get_mut(to_node).unwrap().push((from_node.to_string(), weight));
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    /// 添加一个节点到图中
    /// 
    /// 如果节点已经存在，则返回false；否则，添加节点并返回true。
    /// 
    /// # 参数
    /// 
    /// * `node` - 要添加的节点的名称
    /// 
    /// # 返回值
    /// 
    /// 如果节点已经存在，则返回false；否则，返回true。
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
		// true
        // 检查节点是否已经存在于邻接表中
        if self.adjacency_table().contains_key(node) {
            // 如果节点已经存在，返回false
            false
        } else {
            // 如果节点不存在，将其添加到邻接表中，并返回true
            self.adjacency_table_mutable().insert(node.to_string(),Vec::new());
            true
        }

    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from_node, to_node, weight) = edge;
        if !self.contains(from_node){
            self.add_node(from_node);
        }
        if !self.contains(to_node){
            self.add_node(to_node);
        }
        self.adjacency_table_mutable().get_mut(from_node).unwrap().push((to_node.to_string(), weight));
        self.adjacency_table_mutable().get_mut(to_node).unwrap().push((from_node.to_string(), weight));
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}