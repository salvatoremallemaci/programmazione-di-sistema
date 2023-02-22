pub mod graph {

    use std::collections::HashMap;
    use maplit::hashmap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    #[derive(PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph { nodes: vec![], edges: vec![], attrs: hashmap![] }
        }

        pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
            self.nodes = nodes;
            self
        }

        pub fn with_edges(mut self, edges: Vec<Edge>) -> Self {
            self.edges = edges;
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self  {
            for attr in attrs.iter(){
                self.attrs.insert(String::from(attr.0), String::from(attr.1));
            }
            self
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            for n in self.nodes.iter() {
                if n.get_name() == node_name {
                    return Some(n);
                }
            }
            None
        }

    }


    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(PartialEq, Debug)]
            pub struct Edge {
                node1: String,
                node2: String,
                pub attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge { node1: String::from(node1), node2: String::from(node2), attrs: hashmap![] }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self  {
                    for attr in attrs.iter(){
                        self.attrs.insert(String::from(attr.0), String::from(attr.1));
                    }
                    self
                }

                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    let value = self.attrs.get(attr_name);
                    if value != None {
                        return Some(value.unwrap().as_str());
                    }

                    None
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(PartialEq, Debug)]
            pub struct Node {
                name: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node { name: String::from(name), attrs: hashmap![] }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self  {
                    for attr in attrs.iter(){
                        self.attrs.insert(String::from(attr.0), String::from(attr.1));
                    }
                    self
                }

                pub fn get_name(&self) -> &str {
                    &self.name
                }

                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    let value = self.attrs.get(attr_name);
                    if value != None {
                        return Some(value.unwrap().as_str());
                    }

                    None
                }

            }
        }
    }
}
