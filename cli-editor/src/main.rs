mod ds;
// mod editor;
// use editor::Editor;

fn main() {
    let node1 = ds::Rope::new("Hello").unwrap();
    let node2 = ds::Rope::new("World").unwrap();
    let node3 = ds::Rope::new("World").unwrap();

    println!("{node1:?}");
    println!("{node2:?}");

    let old = ds::Rope::concate(node1, node2).unwrap();

    let ans = ds::Rope::concate(old, node3).unwrap();
    println!("{ans:?}");

    let traverse = ans.at_index(3).unwrap();
    println!("Found char: {traverse:?}");

    /* let mut editor = Editor::default();
    editor.run(); */
}
