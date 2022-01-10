use trussx::Truss;
use uom::si::length::meter;

fn main() {
    let mut x = Truss::new();
    let a = x.add_joint::<meter>(0.0, 0.0, 0.0);
    let b = x.add_joint::<meter>(3.0, 0.0, 0.0);
    let _ab = x.add_edge(a, b);
}
