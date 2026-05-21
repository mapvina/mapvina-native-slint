mod mapvina;

slint::include_modules!();

fn main() {
    let ui = MapWindow::new().unwrap();
    let map = mapvina::create_map(ui.get_map_size());

    mapvina::init(&ui, &map);

    ui.run().unwrap();
}
