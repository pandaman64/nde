fn main() {
    let source = std::fs::read_to_string("../flake.nix").unwrap();
    let ast = rnix::parse(&source);

    println!("{:#?}", ast.node());
}
