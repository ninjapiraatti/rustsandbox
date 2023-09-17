// Hands-On Data Structures and Algorithms in Rust
// Video 7: Mutability, variables, copying and cloning

#[derive(Debug, Clone)] // String doesn't implement copy so you can't use it here. Clone instead
pub struct Aninmal {
    name: String,
    isfloof: bool,
    noms: NomLocation,
}

#[derive(Debug, Clone, Copy)]
pub struct NomLocation {
    x: i32,
    y: i32,
}

impl NomLocation {
    pub fn new(x: i32, y: i32) -> Self {
        NomLocation { x, y }
    }
}

pub fn run() {
    let mut treefloof = Aninmal {
        name: "Treefloof".to_string(),
        isfloof: true,
        noms: NomLocation::new(2, 4),
    };

    let tigger = treefloof.clone();
    treefloof.name.push_str(" Tabernackle");
    println!("{:?}, {:?}", treefloof, tigger);
    println!("{:?}, {:?}, {:?}", treefloof.isfloof, treefloof.noms.x, treefloof.noms.y);
}
