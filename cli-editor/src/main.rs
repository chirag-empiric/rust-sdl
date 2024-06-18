mod ds;
// mod editor;
// use editor::Editor;

fn main() {
    let node1 = ds::Rope::new("Hello ").unwrap();
    let node2 = ds::Rope::new("World ").unwrap();
    let node3 = ds::Rope::new("Chirag ").unwrap();
    let node4 = ds::Rope::new("Jani").unwrap();

    let concat1 = ds::Rope::concate(node1, node2).unwrap();

    let concat2 = ds::Rope::concate(node3, node4).unwrap();

    let concat3 = ds::Rope::concate(concat1, concat2).unwrap();

    let _find_at_index = concat3.at_index(3).unwrap();

    let _ = concat3.traverse();
    println!(); // for line break

    let _ = concat3.clone().split_from_idx(3);

    let inseted = concat3.insert_at_index("dhoom", 4);

    let del = ds::Rope::delete_between_index(&inseted.unwrap(), 4, 10);
    let _ = del.unwrap().clone().traverse();

    /* let mut editor = Editor::default();
    editor.run(); */
}
