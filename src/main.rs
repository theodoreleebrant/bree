use ghost_cell::{GhostToken};
use std::{time::Instant, env}; 


mod tree;

fn main() {
    println!("Hello, world!");
    
    GhostToken::new(|mut token| {
        let tree = tree::Hook::new( 0);
        // tree.borrow(&token).print(&token);
        let leaf = tree::Hook::tree_extremum(&token, &tree, 0);
        leaf.borrow(&token).print(&token);
    })
    // tree.print();
}
