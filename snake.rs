use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, MeshBuilder};
use ggez::input::keyboard::KeyCode;

pub struct Snake {
    pub body: Vec<(f32, f32)>,
    pub direction: (f32, f32),
    pub next_direction: (f32, f32),
    pub grow: bool,
    up_key: KeyCode,
    down_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
}

pub struct Snake2 {
    pub body: Vec<(f32, f32)>,
    pub direction: (f32, f32),
    pub next_direction: (f32, f32),
    pub grow: bool,
    up_key: KeyCode,
    down_key: KeyCode,
    left_key: KeyCode,
    right_key: KeyCode,
}

// 为 Snake 实现的方法
impl Snake {
    pub fn new(ctx: &mut Context, start_pos: (f32, f32), up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) -> GameResult<Snake> {
        let body = vec![
            start_pos,
            (start_pos.0 - 20.0, start_pos.1),
            (start_pos.0 - 40.0, start_pos.1)
        ];
        let direction = (20.0, 0.0);
        let next_direction = direction;
        Ok(Snake { 
            body, 
            direction,
            next_direction,
            grow: false,
            up_key: up,
            down_key: down,
            left_key: left,
            right_key: right,
        })
    }

    pub fn update(&mut self) {
        if self.direction == (0.0, 0.0) {
            return;
        }
        self.direction = self.next_direction;
        
        let head = self.body[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.body.insert(0, new_head);
        
        if !self.grow {
            self.body.pop();
        } else {
            self.grow = false;
        }
    }

    pub fn change_direction(&mut self, key: KeyCode) {
        match key {
            k if k == self.up_key && self.direction.1 == 0.0 => self.next_direction = (0.0, -20.0),
            k if k == self.down_key && self.direction.1 == 0.0 => self.next_direction = (0.0, 20.0),
            k if k == self.left_key && self.direction.0 == 0.0 => self.next_direction = (-20.0, 0.0),
            k if k == self.right_key && self.direction.0 == 0.0 => self.next_direction = (20.0, 0.0),
            _ => (),
        }
    }

    pub fn check_collision(&self, position: (f32, f32)) -> bool {
        self.body.iter().any(|&segment| segment == position)
    }

    pub fn check_self_collision(&self) -> bool {
        if let Some(head) = self.body.first() {
            self.body.iter().skip(1).any(|segment| segment == head)
        } else {
            false
        }
    }

    pub fn check_collision_with_other(&self, other_body: &[(f32, f32)]) -> bool {
        if let Some(head) = self.body.first() {
            other_body.iter().any(|segment| segment == head)
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = MeshBuilder::new();
        let color = if self.direction == (0.0, 0.0) {  // 如果蛇已经死亡
            Color::from_rgb(100, 100, 100)  // 灰色表示死亡
        } else {
            Color::GREEN
        };
        
        for (x, y) in &self.body {
            mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(*x, *y, 20.0, 20.0),
                color,
            )?;
        }
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }
}

// 为 Snake2 实现的方法 (与 Snake 基本相同，只是颜色不同)
impl Snake2 {
    pub fn new(ctx: &mut Context, start_pos: (f32, f32), up: KeyCode, down: KeyCode, left: KeyCode, right: KeyCode) -> GameResult<Snake2> {
        let body = vec![
            start_pos,
            (start_pos.0 - 20.0, start_pos.1),
            (start_pos.0 - 40.0, start_pos.1)
        ];
        let direction = (20.0, 0.0);
        let next_direction = direction;
        Ok(Snake2 { 
            body, 
            direction,
            next_direction,
            grow: false,
            up_key: up,
            down_key: down,
            left_key: left,
            right_key: right,
        })
    }

    pub fn update(&mut self) {
        if self.direction == (0.0, 0.0) {
            return;
        }
        self.direction = self.next_direction;
        
        let head = self.body[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);
        self.body.insert(0, new_head);
        
        if !self.grow {
            self.body.pop();
        } else {
            self.grow = false;
        }
    }

    pub fn change_direction(&mut self, key: KeyCode) {
        match key {
            k if k == self.up_key && self.direction.1 == 0.0 => self.next_direction = (0.0, -20.0),
            k if k == self.down_key && self.direction.1 == 0.0 => self.next_direction = (0.0, 20.0),
            k if k == self.left_key && self.direction.0 == 0.0 => self.next_direction = (-20.0, 0.0),
            k if k == self.right_key && self.direction.0 == 0.0 => self.next_direction = (20.0, 0.0),
            _ => (),
        }
    }

    pub fn check_collision(&self, position: (f32, f32)) -> bool {
        self.body.iter().any(|&segment| segment == position)
    }

    pub fn check_self_collision(&self) -> bool {
        if let Some(head) = self.body.first() {
            self.body.iter().skip(1).any(|segment| segment == head)
        } else {
            false
        }
    }

    pub fn check_collision_with_other(&self, other_body: &[(f32, f32)]) -> bool {
        if let Some(head) = self.body.first() {
            other_body.iter().any(|segment| segment == head)
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = MeshBuilder::new();
        let color = if self.direction == (0.0, 0.0) {  // 如果蛇已经死亡
            Color::from_rgb(100, 100, 100)  // 灰色表示死亡
        } else {
            Color::BLUE
        };
        
        for (x, y) in &self.body {
            mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(*x, *y, 20.0, 20.0),
                color,
            )?;
        }
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
    }
}