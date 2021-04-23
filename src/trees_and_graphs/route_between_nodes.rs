use super::graph::*;

/// Determines whether there is a connection route between two nodes.
/// It assumes the graph is directional
/// 
/// The following example runs on the floowing graph
/// 
/// N0 -> N2
/// 
/// |
/// 
/// v
/// 
/// N1
/// 
/// # Example
/// ```
/// use ctci::trees_and_graphs::route_between_nodes::route_between_nodes;
/// use ctci::trees_and_graphs::graph::Graph;
/// 
/// let mut graph = Graph::new();
///
/// let n0 = graph.add_node();
/// let n1 = graph.add_node();
/// let n2 = graph.add_node();
///
/// graph.add_edge(n0, n1);
/// graph.add_edge(n0, n2);
///
/// assert!(route_between_nodes(&graph, n0, n1));
/// assert!(route_between_nodes(&graph, n0, n2));
/// assert!(!route_between_nodes(&graph, n1, n2));
/// ```
pub fn route_between_nodes(graph: &Graph, node1id: usize, node2id: usize) -> bool {
  for node_id in graph.successors(node1id) {
    if node_id == node2id {
      return true;
    }
    if route_between_nodes(graph, node_id, node2id) {
      return true;
    }
  }
  return false;
}

mod test {
  #[test]
  fn example() {
    use super::*;

    // N0 ---E0---> N1 ---E1---> N2
    // |                         ^
    // E2                        |
    // |                         |
    // v                         |
    // N3 ----------E3-----------+

    let mut graph = Graph::new();

    let n0 = graph.add_node();
    let n1 = graph.add_node();
    let n2 = graph.add_node();
    let n3 = graph.add_node();

    graph.add_edge(n0, n1); // e0
    graph.add_edge(n1, n2); // e1
    graph.add_edge(n0, n3); // e2
    graph.add_edge(n3, n2); // e3

    assert!(route_between_nodes(&graph, n0, n1));
    assert!(route_between_nodes(&graph, n0, n2));
    assert!(route_between_nodes(&graph, n0, n3));

    assert!(!route_between_nodes(&graph, n1, n0));
    assert!(!route_between_nodes(&graph, n2, n0));
    assert!(!route_between_nodes(&graph, n3, n0));

    assert!(route_between_nodes(&graph, n3, n2));
    assert!(!route_between_nodes(&graph, n2, n3));
  }
}
