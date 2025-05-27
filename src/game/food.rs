// food.rs - 无需修改
use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, MeshBuilder};
use rand::Rng;

pub struct Food {
    pub position: (f32, f32),
}

impl Food {
    pub fn new(ctx: &mut Context) -> GameResult<Food> {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0..=31) as f32;
        let b = rng.gen_range(0..=23) as f32;
        let x = a * 20.0;
        let y = b * 20.0;
        Ok(Food { position: (x, y) })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = MeshBuilder::new();
        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.position.0, self.position.1, 20.0, 20.0),
            Color::RED,
        )?;
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }
}