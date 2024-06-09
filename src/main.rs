mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    Editor::run(&editor);
}