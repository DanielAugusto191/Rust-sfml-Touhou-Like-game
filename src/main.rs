use sfml::{graphics::*, system::*, window::*, SfBox};
use std::collections::LinkedList;
use rand::prelude::*;

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
    // Textures
    let remu_texture: SfBox<Texture> = Texture::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/resources/",
        "remu.png"
    )).unwrap();

    let remu_bullets_texture: SfBox<Texture> = Texture::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/resources/",
        "bullet.png"
    )).unwrap();

    let enemy1_texture: SfBox<Texture> = Texture::from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/resources/",
        "Enemy1.png"
    )).unwrap();
    
    // Rand
    let mut rng = rand::thread_rng();

    // Clocks and Timers
    let mut clock = Clock::start();
    let mut bc = Clock::start();
    let mut bullet_time = Time::default();
    let mut ec = Clock::start();
    let mut enemy_spawn_time = Time::default();
    
    // Init
    let mut remu = Remu::new(windows_size);
    let mut bullets_vec: Vec<Bullet> = Vec::new();
    let mut enemy_vec: Vec<Enemy> = Vec::new();
    let mut put_bullet = false;

    while window.is_open() {
        bullet_time = bc.elapsed_time();
        enemy_spawn_time = ec.elapsed_time();
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
        if put_bullet && bullet_time.as_seconds() >= 0.02 {
            let mut bb = Bullet::new(Vector2f::new(
                window.mouse_position().x as f32 - 8.,
                window.mouse_position().y as f32 - 8.,
            ));
            bb.body.set_texture(&remu_bullets_texture, false);
            bullets_vec.push(bb);
            bc.restart();
        }
        if enemy_spawn_time.as_seconds() >= 0.8 {
            let mut n: f64 = rng.gen();
            let x;
            let t;
            if n >= 0.5 {
                n = 0.;
                t = -1;
            } else{
                n = windows_size.0 as f64;
                t = 1;
            }
            x = n-30.; // enemy sprites size;
            let mut ne = Enemy::new(x as f32, 0.);
            ne.body.set_texture(&enemy1_texture, false);
            ne.tipo = t;
            enemy_vec.push(ne);
            ec.restart();
        }
        window.clear(Color::BLACK);
        remu.bbody.set_texture(&remu_texture, false);
        let c_npos = Vector2f::from((mp.x - 10.0, mp.y - 10.0));
        let b_npos = Vector2f::from((
            mp.x - remu_size.x as f32 / 2.,
            mp.y - remu_size.y as f32 / 2.,
        ));
        remu.collider.set_position(c_npos);
        remu.bbody.set_position(b_npos);
        let mut remove_enemy: Vec<u32> = Vec::new();
        for i in 0..enemy_vec.len(){
            if enemy_vec[i].tipo == -1{
                enemy_vec[i].body.move_((0.1, 0.));
                if enemy_vec[i].body.position().x > windows_size.0 as f32 + 5.0{
                    remove_enemy.push(i as u32);
                }
            }else{
                enemy_vec[i].body.move_((-0.1, 0.));
                if enemy_vec[i].body.position().x < 0. - 5.0{
                    remove_enemy.push(i as u32);
                }
            }
            window.draw(&enemy_vec[i].body);
        }
        println!("{}", enemy_vec.len());
        let mut remove_bullet: Vec<u32> = Vec::new();
        for i in 0..bullets_vec.len() {
            bullets_vec[i].body.move_((0., -0.5));
            if bullets_vec[i].body.position().y < 0. - 10.0 {
                remove_bullet.push(i as u32);
            }
            window.draw(&bullets_vec[i].body);
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
        body.set_position(pos);
        Bullet {
            damage: 1,
            body: body,
        }
    }
}

struct Enemy<'a> {
    life: u32,
    body: RectangleShape<'a>,
    tipo: i32, // if its was spawned of left or right, -1 Left, 1 Right
}

impl Enemy<'static> {
    fn new(x: f32, y: f32) -> Enemy<'static>{
        let l = 10;
        let mut bbody = RectangleShape::new();
        bbody.set_size(Vector2f::new(60.,60.));
        bbody.set_position(Vector2f::new(x,y));
        Enemy{
            life: l,
            body:  bbody,
            tipo: 0,
        }
    }

}