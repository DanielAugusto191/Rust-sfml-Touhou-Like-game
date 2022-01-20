use ::sfml::{graphics::*, system::Vector2f};

pub struct Remu<'a> {
    pub collider: CircleShape<'a>, // Lifetime = Final do Programa;
    pub bbody: RectangleShape<'a>,
    pub position: Vector2f,
}

impl Remu<'static> {
    pub fn new(windwos_size: (u32, u32)/* Windows Size*/) -> Remu<'static> {
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
