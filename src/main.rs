use tree_sitter::Node;
use tree_sitter::{Parser, Tree};

fn main() -> std::io::Result<()> {
    // Load the Lua parser
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_lua::LANGUAGE.into())
        .expect("Error loading Lua grammar");

    // Read Lua source file
    let source_code = std::fs::read_to_string("example.lua")?;

    // Parse the source
    let tree: Tree = parser
        .parse(&source_code, None)
        .expect("Failed to parse Lua code");

    let root_node = tree.root_node();
    print_node(root_node, &source_code, 0);

    Ok(())
}

fn print_node(node: Node, source: &str, indent: usize) {
    let padding = "  ".repeat(indent);
    println!(
        "{}{}: '{}'",
        padding,
        node.kind(),
        &source[node.start_byte()..node.end_byte()]
    );

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_node(child, source, indent + 1);
    }
}
