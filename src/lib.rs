//! This package provides utilities for designing and analyzing truss structures

use structural_shapes::StructuralShape;

struct Joint {
    position: [f64; 3],
    reaction: [bool; 3],
    load: [f64; 3],
    deflection: [f64; 3],
}

struct Member {
    cross_section: StructuralShape,
    elastic_modulus: f64,
    yield_strength: f64,
    force: f64,
    stress: f64,
}

/// This is the truss object that contains all of the necessary information about trusses
pub struct Truss {
    graph: petgraph::Graph<Joint, Member>,
    results: bool,
}

impl Default for Truss {
    fn default() -> Truss {
        Truss::new()
    }
}

impl Truss {
    /// This function instantiates an empty truss
    pub fn new() -> Truss {
        Truss {
            graph: petgraph::Graph::new(),
            results: false,
        }
    }

    /// This function creates a new joint
    pub fn add_joint(&mut self, position: [f64; 3]) -> petgraph::graph::NodeIndex {
        self.clear();
        self.graph.add_node(Joint {
            position,
            reaction: [false, false, false],
            load: [0.0; 3],
            deflection: [0.0; 3],
        })
    }

    /// This function creates a new member to connect two joints
    pub fn add_edge(
        &mut self,
        a: petgraph::graph::NodeIndex,
        b: petgraph::graph::NodeIndex,
    ) -> petgraph::graph::EdgeIndex {
        self.clear();
        self.graph.add_edge(
            a,
            b,
            Member {
                cross_section: StructuralShape::Pipe {
                    outer_radius: 0.0,
                    thickness: 0.0,
                },
                elastic_modulus: 0.0,
                yield_strength: 0.0,
                force: 0.0,
                stress: 0.0,
            },
        )
    }

    /// This function moves a joint
    pub fn move_joint(&mut self, a: petgraph::graph::NodeIndex, position: [f64; 3]) {
        self.clear();
        let joint = self.graph.node_weight_mut(a);
        match joint {
            None => {
                panic!("This joint does not exist");
            }
            Some(joint) => {
                joint.position = position;
            }
        }
    }

    /// This function deletes a joint
    pub fn delete_joint(&mut self, a: petgraph::graph::NodeIndex) {
        self.clear();
        self.graph.remove_node(a);
    }

    /// This function deletes a member
    pub fn delete_member(&mut self, ab: petgraph::graph::EdgeIndex) {
        self.clear();
        self.graph.remove_edge(ab);
    }

    /// Set reaction forces available at each joint
    pub fn set_reactions(&mut self, a: petgraph::graph::NodeIndex, reaction: [bool; 3]) {
        self.clear();
        let joint = self.graph.node_weight_mut(a);
        match joint {
            None => {
                panic!("This joint does not exist");
            }
            Some(joint) => {
                joint.reaction = reaction;
            }
        }
    }

    /// Set material for a member
    pub fn set_material(
        &mut self,
        ab: petgraph::graph::EdgeIndex,
        elastic_modulus: f64,
        yield_strength: f64,
    ) {
        self.clear();
        let member = self.graph.edge_weight_mut(ab);
        match member {
            None => {
                panic!("This joint does not exist");
            }
            Some(member) => {
                member.elastic_modulus = elastic_modulus;
                member.yield_strength = yield_strength;
            }
        }
    }

    /// Set shape for a member
    pub fn set_shape(&mut self, ab: petgraph::graph::EdgeIndex, shape: StructuralShape) {
        self.clear();
        let member = self.graph.edge_weight_mut(ab);
        match member {
            None => {
                panic!("This joint does not exist");
            }
            Some(member) => {
                member.cross_section = shape;
            }
        }
    }

    /// Set material for all members
    pub fn set_material_for_all(&mut self, elastic_modulus: f64, yield_strength: f64) {
        self.clear();
        for mut member in self.graph.edge_weights_mut() {
            member.elastic_modulus = elastic_modulus;
            member.yield_strength = yield_strength;
        }
    }

    /// Set material for all members
    pub fn set_shape_for_all(&mut self, shape: StructuralShape) {
        self.clear();
        for mut member in self.graph.edge_weights_mut() {
            member.cross_section = shape;
        }
    }

    /// Clear results after a change
    fn clear(&mut self) {
        if self.results {
            self.results = false;
            for mut member in self.graph.edge_weights_mut() {
                member.force = 0.0;
                member.stress = 0.0;
            }

            for mut joint in self.graph.node_weights_mut() {
                joint.deflection = [0.0; 3];
            }
        }
    }

    fn calculate_member_forces(&mut self) {
        let n = self.graph.node_count();
        let stiffness_matrix = vec![vec![0.0; n * 3]; n * 3];
        let deflections = vec![vec![0.0; 3]; n];
        let loads = vec![vec![0.0; 3]; n];
        unimplemented!();
    }

    fn calculate_member_stress(&mut self) {
        self.clear();
        unimplemented!()
    }

    /// This function calculates the forces in each member and outputs a report
    pub fn evaluate(&mut self) {
        self.results = true;
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let elastic_modulus = 2000000.0;
        let yield_strength = 2000000.0;

        let mut x = Truss::new();
        let a = x.add_joint([0.0, 0.0, 0.0]);
        let b = x.add_joint([3.0, 0.0, 0.0]);
        let c = x.add_joint([1.5, 1.5, 0.0]);
        let _ab = x.add_edge(a, b);
        let _bc = x.add_edge(b, c);
        let _ac = x.add_edge(a, c);
        x.set_material_for_all(elastic_modulus, yield_strength);
        x.set_shape_for_all(StructuralShape::Pipe {
            outer_radius: 1.0,
            thickness: 0.0,
        })
    }
}
