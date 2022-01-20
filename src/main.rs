use sfml::graphics::*;
use sfml::system::{Clock, Vector2f, Vector2i};
use sfml::window::{mouse, Event, Key, Style, VideoMode};
use sfml::SfBox;
mod remu;
use remu::Remu;

fn main() {
    // Settings
    let windows_size = (1024, 768); // Width Height;
    let remu_size: Vector2i = Vector2i::new(56, 84);
    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        VideoMode::new(windows_size.0, windows_size.1, desktop.bits_per_pixel),
        "New",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_mouse_position(Vector2i::new(
        (windows_size.0 / 2) as i32,
        (windows_size.1 - remu_size.y as u32) as i32));

    let remu_texture: SfBox<Texture> = Texture::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/resources/",
        "remu.png"
    ))
    .unwrap();
    let mut remu = Remu::new(windows_size);
    while window.is_open() {
        let fmp = window.mouse_position();
        let mp: Vector2f = Vector2f::new(fmp.x as f32, fmp.y as f32);

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }
        remu.bbody.set_texture(&remu_texture, false);
        window.clear(Color::BLACK);
        let c_npos = Vector2f::from((mp.x - 10.0, mp.y - 10.0));
        let b_npos = Vector2f::from((mp.x - remu_size.x as f32 / 2., mp.y - remu_size.y as f32 / 2.));
        remu.collider.set_position(c_npos);
        remu.bbody.set_position(b_npos);
        window.draw(&remu.bbody);
        window.draw(&remu.collider);
        window.set_active(true);
        window.display();
    }
}
