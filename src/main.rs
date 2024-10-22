#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_macros;

fn main() {
    println!("{}", rustc_ast::node_id::CRATE_NODE_ID)
}
