use sfml::{graphics::*, system::*, window::*, SfBox};
use std::collections::LinkedList;

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
        (windows_size.1 - remu_size.y as u32) as i32,
    ));

    let remu_texture: SfBox<Texture> = Texture::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/resources/",
        "remu.png"
    ))
    .unwrap();
    let mut remu = Remu::new(windows_size);
    let mut bullets_vec: LinkedList<Bullet> = LinkedList::new();
    let mut put_bullet = false;

    let mut clock = Clock::start();
    let mut bc = Clock::start();
    let mut bullet_time = Time::default();

    while window.is_open() {
        bullet_time = bc.elapsed_time();
        let fmp = window.mouse_position();
        let mp: Vector2f = Vector2f::new(fmp.x as f32, fmp.y as f32);
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button, x, y } => {
                    put_bullet = true;
                }
                Event::MouseButtonReleased { button, .. } => {
                    put_bullet = false;
                }
                _ => {}
            }
        }
        if put_bullet && bullet_time.as_seconds() >= 0.05 {
            bullets_vec.push_back(Bullet::new(Vector2f::new(
                window.mouse_position().x as f32,
                window.mouse_position().y as f32,
            )));
            bc.restart();
        }
        remu.bbody.set_texture(&remu_texture, false);
        window.clear(Color::BLACK);
        let c_npos = Vector2f::from((mp.x - 10.0, mp.y - 10.0));
        let b_npos = Vector2f::from((
            mp.x - remu_size.x as f32 / 2.,
            mp.y - remu_size.y as f32 / 2.,
        ));
        remu.collider.set_position(c_npos);
        remu.bbody.set_position(b_npos);

        let mut rem = false;
        for x in bullets_vec.iter_mut() {
            x.body.move_((0., -0.5));
            if x.body.position().y < 0. - 10.0 {
                rem = true;
            }
            window.draw(&x.body);
        }
        if rem {
            bullets_vec.pop_front();

        }
        window.draw(&remu.bbody);
        window.draw(&remu.collider);
        window.set_active(true);
        window.display();
    }
}

struct Remu<'a> {
    collider: CircleShape<'a>, // Lifetime = Final do Programa;
    bbody: RectangleShape<'a>,
    position: Vector2f,
}

impl Remu<'static> {
    fn new(windwos_size: (u32, u32) /* Windows Size*/) -> Remu<'static> {
        // Var
        let body_size: Vector2f = Vector2f::new(56., 84.);
        let pos_body: Vector2f = Vector2f::new(
            windwos_size.0 as f32 / 2.0 - body_size.x / 2.0,
            windwos_size.1 as f32 - body_size.y,
        );
        let collider_size: f32 = 10.0;
        let pos_collider: Vector2f = Vector2f::new(
            pos_body.x + body_size.x / 2.0 - 10.0,
            pos_body.y + body_size.y / 2.0 - 10.0,
        );

        // Sets
        let mut body = RectangleShape::new();
        body.set_size(body_size);
        body.set_position(pos_body);

        let mut collider = CircleShape::new(collider_size, 30);
        collider.set_fill_color(Color::BLUE);
        collider.set_position(pos_collider);

        Remu {
            collider: collider,
            bbody: body,
            position: pos_collider,
        }
    }
}

struct Bullet<'a> {
    damage: u32,
    body: CircleShape<'a>,
}

impl Bullet<'static> {
    fn new(pos: Vector2f) -> Bullet<'static> {
        let mut body = CircleShape::new(10.0, 30);
        body.set_fill_color(Color::BLUE);
        body.set_position(pos);
        Bullet {
            damage: 1,
            body: body,
        }
    }
}
