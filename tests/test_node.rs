use profqu_neat::*;

#[test]
fn test_node_type() {
    let mut node = Node::new();
    assert!(node.is_neuron());
    assert!(!node.is_sensor());
    node.set_type(NodeType::Sensor);
    assert!(!node.is_neuron());
    assert!(node.is_sensor());
}

#[test]
fn test_node_history() {
    let mut node = Node::new();
    assert_eq!(node.prev_active_sums.len(), 0);
    node.archive_active_sum(1.0);
    assert_eq!(node.prev_active_sums.len(), 1);
    node.archive_active_sum(17.0);
    assert_eq!(node.prev_active_sums.len(), 2);
    node.archive_active_sum(7.0);
    assert_eq!(node.prev_active_sums.len(), 2);
}