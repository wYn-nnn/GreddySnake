mod snake;
mod food;

use ggez::timer;
use ggez::{event::EventHandler, graphics, Context, GameResult};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
// use ggez::input::mouse::{MouseButton, Mouse};
use snake::{Snake, Snake2};
use food::Food;

//const SCALE_FACTOR:f32 = 2.0;

#[derive(PartialEq)]
enum GameState {
    Title,
    ModeSelect,
    DifficultySelect,
    Playing,
    GameOver,
}

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(PartialEq)]
enum GameMode {
    SinglePlayer,
    TwoPlayer,
}

pub struct Game {
    snake: Snake,
    snake2: Option<Snake2>, //player2
    food: Food,
    game_state: GameState,
    last_update: f32,
    update_interval: f32,
    difficulty:Difficulty,
    game_mode: GameMode,
    score: u32,
    score2: u32,//player2 score
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let snake = Snake::new(ctx, (100.0, 100.0), KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right)?;
        let food = Food::new(ctx)?;
        Ok(Game { 
            snake, 
            snake2: None,
            food,
            game_state: GameState::Title,
            last_update: 0.0,
            update_interval: 0.2,
            difficulty: Difficulty::Easy,
            game_mode: GameMode::SinglePlayer,
            score: 0,
            score2: 0,
        })
    }

    fn reset(&mut self, ctx: &mut Context) -> GameResult {
        self.snake = Snake::new(ctx, (100.0, 100.0), KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right)?;
        
        if self.game_mode == GameMode::TwoPlayer {
            self.snake2 = Some(Snake2::new(ctx, (500.0, 100.0), KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D)?);
        } else {
            self.snake2 = None;
        }
        self.food = Food::new(ctx)?;
        self.game_state = GameState::Playing;
        self.last_update = 0.0;
        self.score = 0;
        self.score2 = 0;
        //difficulty select
        self.update_interval = match self.difficulty {
            Difficulty::Easy => 0.2,    // 最慢
            Difficulty::Normal => 0.1, // 中等
            Difficulty::Hard => 0.05,    // 最快
        };
        Ok(())
    }

    fn check_food_collision(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(head) = self.snake.body.first() {
            if head.0 == self.food.position.0 && head.1 == self.food.position.1 {
                self.snake.grow = true;
                self.score +=1;
                self.food = Food::new(ctx)?;
                
                while self.snake.check_collision(self.food.position) || 
                      self.snake2.as_ref().map_or(false, |s| s.check_collision(self.food.position)) {
                    self.food = Food::new(ctx)?;
                    
                }
            }
        }
        if let Some(snake2) = &mut self.snake2 {
            if let Some(head) = snake2.body.first() {
                if head.0 == self.food.position.0 && head.1 == self.food.position.1 {
                    snake2.grow = true;
                    self.score2 += 1;
                    self.food = Food::new(ctx)?;
                    
                    while self.snake.check_collision(self.food.position) || 
                          snake2.check_collision(self.food.position) {
                        self.food = Food::new(ctx)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn check_boundary_collision(&self) -> bool {
        if let Some(head) = self.snake.body.first() {
            head.0 < 0.0 || head.0 >= 640.0 || head.1 < 0.0 || head.1 >= 480.0
        } else {
            false
        }
    }
    // 添加检查第二条蛇边界碰撞的方法
    fn check_boundary_collision_snake2(&self, snake2: &Snake2) -> bool {
        if let Some(head) = snake2.body.first() {
            head.0 < 0.0 || head.0 >= 640.0 || head.1 < 0.0 || head.1 >= 480.0
        } else {
            false
        }
    }

    fn draw_title_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        
        let title = graphics::Text::new("SNAKE GAME");
        //let scale = [SCALE_FACTOR, SCALE_FACTOR];
        
        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([270.0 , 180.0 ])
                //.scale(scale)
                .color(graphics::Color::WHITE),
        )?;

        let start_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(270.0 , 250.0 , 
                100.0 , 40.0 ),
            graphics::Color::GREEN,
        )?;
        graphics::draw(ctx, &start_button, graphics::DrawParam::default())?;

        let start_text = graphics::Text::new("START");
        graphics::draw(
            ctx,
            &start_text,
            graphics::DrawParam::default()
                .dest([290.0 , 260.0])
                //.scale(scale)
                .color(graphics::Color::BLACK),
        )?;

        graphics::present(ctx)
    }

    fn draw_mode_select_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        
        let title = graphics::Text::new("SELECT GAME MODE");
        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([200.0, 150.0])
                .color(graphics::Color::WHITE),
        )?;

        // 单人模式按钮
        let single_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 200.0, 200.0, 40.0),
            graphics::Color::GREEN,
        )?;
        graphics::draw(ctx, &single_button, graphics::DrawParam::default())?;

        let single_text = graphics::Text::new("SINGLE PLAYER");
        graphics::draw(
            ctx,
            &single_text,
            graphics::DrawParam::default()
                .dest([240.0, 210.0])
                .color(graphics::Color::BLACK),
        )?;

        // 双人模式按钮
        let two_player_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 260.0, 200.0, 40.0),
            graphics::Color::BLUE,
        )?;
        graphics::draw(ctx, &two_player_button, graphics::DrawParam::default())?;

        let two_player_text = graphics::Text::new("TWO PLAYERS");
        graphics::draw(
            ctx,
            &two_player_text,
            graphics::DrawParam::default()
                .dest([250.0, 270.0])
                .color(graphics::Color::BLACK),
        )?;

        graphics::present(ctx)
    }

    fn draw_difficulty_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        
        let title = graphics::Text::new("SELECT DIFFICULTY");
        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([200.0, 150.0])
                .color(graphics::Color::WHITE),
        )?;

        // Easy 按钮
        let easy_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 200.0, 200.0, 40.0),
            graphics::Color::GREEN,
        )?;
        graphics::draw(ctx, &easy_button, graphics::DrawParam::default())?;

        let easy_text = graphics::Text::new("EASY");
        graphics::draw(
            ctx,
            &easy_text,
            graphics::DrawParam::default()
                .dest([300.0, 210.0])
                .color(graphics::Color::BLACK),
        )?;


        // Normal 按钮
        let normal_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 260.0, 200.0, 40.0),
            graphics::Color::YELLOW,
        )?;
        graphics::draw(ctx, &normal_button, graphics::DrawParam::default())?;

        let normal_text = graphics::Text::new("NORMAL");
        graphics::draw(
            ctx,
            &normal_text,
            graphics::DrawParam::default()
                .dest([290.0, 270.0])
                .color(graphics::Color::BLACK),
        )?;

        // Hard 按钮
        let hard_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 320.0, 200.0, 40.0),
            graphics::Color::RED,
        )?;
        graphics::draw(ctx, &hard_button, graphics::DrawParam::default())?;

        let hard_text = graphics::Text::new("HARD");
        graphics::draw(
            ctx,
            &hard_text,
            graphics::DrawParam::default()
                .dest([300.0, 330.0])
                .color(graphics::Color::BLACK),
        )?;

        graphics::present(ctx)
    }

    fn draw_playing_screen(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        
        // 绘制得分
        let score_text = graphics::Text::new(format!("Player 1: {}", self.score));
        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::default()
                .dest([10.0, 10.0])
                .color(graphics::Color::WHITE),
        )?;

        if let Some(_) = &self.snake2 {
            let score2_text = graphics::Text::new(format!("Player 2: {}", self.score2));
            graphics::draw(
                ctx,
                &score2_text,
                graphics::DrawParam::default()
                    .dest([500.0, 10.0])
                    .color(graphics::Color::WHITE),
            )?;
        }

        // 绘制蛇和食物
        self.snake.draw(ctx)?;
        if let Some(snake2) = &self.snake2 {
            snake2.draw(ctx)?;
        }
        self.food.draw(ctx)?;
        
        graphics::present(ctx)
    }

    fn draw_game_over_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        
        self.snake.draw(ctx)?;
        if let Some(snake2) = &self.snake2 {
            snake2.draw(ctx)?;
        }
        self.food.draw(ctx)?;

        let game_over_text = graphics::Text::new("Game Over!");
        graphics::draw(
            ctx,
            &game_over_text,
            graphics::DrawParam::default()
                .dest([250.0, 180.0])
                .color(graphics::Color::WHITE),
        )?;

        let score_text = if self.game_mode == GameMode::SinglePlayer {
            graphics::Text::new(format!("Final Score: {}", self.score))
        } else {
            graphics::Text::new(format!("Player 1: {} | Player 2: {}", self.score, self.score2))
        };
        
        graphics::draw(
            ctx,
            &score_text,
            graphics::DrawParam::default()
                .dest([200.0, 220.0])
                .color(graphics::Color::WHITE),
        )?;


        //back to title
        let back_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 250.0, 200.0, 40.0),
            graphics::Color::GREEN,
        )?;
        graphics::draw(ctx, &back_button, graphics::DrawParam::default())?;

        let back_text = graphics::Text::new("Back To Title");
        graphics::draw(
            ctx,
            &back_text,
            graphics::DrawParam::default()
                .dest([250.0, 260.0])
                .color(graphics::Color::BLACK),
        )?;

        //restart
        let restart_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 290.0, 200.0, 40.0),
            graphics::Color::BLUE,
        )?;
        graphics::draw(ctx, &restart_button, graphics::DrawParam::default())?;

        let restart_text = graphics::Text::new("Restart");
        graphics::draw(
            ctx,
            &restart_text,
            graphics::DrawParam::default()
                .dest([290.0, 300.0])
                .color(graphics::Color::WHITE),
        )?;

        graphics::present(ctx)
    }

    fn check_button_click(&mut self, ctx: &mut Context, x: f32, y: f32) {
        match self.game_state {
            GameState::Title => {
                if x >= 270.0 && x <= 370.0 && y >= 250.0 && y <= 290.0 {
                    self.score = 0;
                    self.score2 = 0;
                    self.game_state = GameState::ModeSelect;
                }
            }
            GameState::ModeSelect => {
                if x >= 220.0 && x <= 420.0 {
                    if y >= 200.0 && y <= 240.0 {
                        self.game_mode = GameMode::SinglePlayer;
                        self.score = 0;
                        self.score2 = 0;
                        self.game_state = GameState::DifficultySelect;
                    } else if y >= 260.0 && y <= 300.0 {
                        self.game_mode = GameMode::TwoPlayer;
                        self.score = 0;
                        self.score2 = 0;
                        self.game_state = GameState::DifficultySelect;
                    }
                }
            }
            GameState::DifficultySelect => {
                if x >= 220.0 && x <= 420.0 {
                    if y >= 200.0 && y <= 240.0 {
                        self.difficulty = Difficulty::Easy;
                        let _ = self.reset(ctx);
                    } else if y >= 260.0 && y <= 300.0 {
                        self.difficulty = Difficulty::Normal;
                        let _ = self.reset(ctx);
                    } else if y >= 320.0 && y <= 360.0 {
                        self.difficulty = Difficulty::Hard;
                        let _ = self.reset(ctx);
                    }
                }
            }
            GameState::GameOver => {
                if x >= 220.0 && x <= 420.0 {
                    if y >= 230.0 && y <= 270.0 {
                        self.game_state = GameState::Title;
                    } else if y >= 290.0 && y <= 330.0 {
                        let _ = self.reset(ctx);
                    }
                }
            }
            _ => {}
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_state != GameState::Playing {
            return Ok(());
        }

        let current_time = timer::time_since_start(ctx).as_secs_f32();
        let delta = current_time - self.last_update;

        if delta >= self.update_interval {
            self.last_update = current_time;
            
            self.snake.update();
            if let Some(snake2) = &mut self.snake2 {
                snake2.update();
            }

        // 检查游戏结束条件
        let snake1_dead = self.check_boundary_collision() || 
            self.snake.check_self_collision() ||
            (self.snake2.is_some() && 
            self.snake.check_collision_with_other(&self.snake2.as_ref().unwrap().body));

        let snake2_dead = if let Some(snake2) = &self.snake2 {
            self.check_boundary_collision_snake2(snake2) ||
            snake2.check_self_collision() ||
            snake2.check_collision_with_other(&self.snake.body)
        } else {
            false
        };

        // 双人模式下，只有两条蛇都死亡才结束游戏
        if self.game_mode == GameMode::TwoPlayer {
            if snake1_dead && snake2_dead {
                self.game_state = GameState::GameOver;
            } else {
                // 如果一条蛇死亡，就停止更新它
                if snake1_dead {
                    self.snake.direction = (0.0, 0.0);
                    self.snake.next_direction = (0.0, 0.0);
                }
                if snake2_dead {
                    if let Some(snake2) = &mut self.snake2 {
                        snake2.direction = (0.0, 0.0);
                        snake2.next_direction = (0.0, 0.0);
                    }
                }
            }
        } 
        // 单人模式下，蛇死亡就结束游戏
        else if snake1_dead {
            self.game_state = GameState::GameOver;
        }

        self.check_food_collision(ctx)?;
    }

    Ok(())
}

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.game_state {
            GameState::Title => self.draw_title_screen(ctx),
            GameState::ModeSelect => self.draw_mode_select_screen(ctx),
            GameState::DifficultySelect => self.draw_difficulty_screen(ctx),
            GameState::Playing => self.draw_playing_screen(ctx),
            GameState::GameOver => self.draw_game_over_screen(ctx),
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _mods: KeyMods,
        _repeat: bool,
    ) {
        match self.game_state {
            GameState::Playing => {
                self.snake.change_direction(keycode);
                if let Some(snake2) = &mut self.snake2 {
                    snake2.change_direction(keycode);
                }
            }
            GameState::GameOver if keycode == KeyCode::R => {
                let _ = self.reset(ctx);
            }
            _ => {}
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.check_button_click(ctx, x, y);
    }
} 
            // GameState::Playing => {
            //     graphics::clear(ctx, graphics::Color::BLACK);
            //     self.snake.draw(ctx)?;
            //     self.food.draw(ctx)?;
            //     graphics::present(ctx)
            // }