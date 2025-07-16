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

// 添加颜色常量
const BACKGROUND_COLOR: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);
const PRIMARY_COLOR: graphics::Color = graphics::Color::new(0.2, 0.6, 0.8, 1.0);
const SECONDARY_COLOR: graphics::Color = graphics::Color::new(0.8, 0.4, 0.2, 1.0);
const ACCENT_COLOR: graphics::Color = graphics::Color::new(0.9, 0.9, 0.9, 1.0);
const BUTTON_HOVER_COLOR: graphics::Color = graphics::Color::new(0.3, 0.7, 0.9, 1.0);
const BUTTON_TEXT_COLOR: graphics::Color = graphics::Color::new(0.1, 0.1, 0.1, 1.0);

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
    TimeAttack,  // 新增限时模式
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
    game_time: f32,        // 游戏剩余时间（秒）
    game_start_time: f32,  // 游戏开始时间
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
            game_time: 180.0,      // 3分钟 = 180秒
            game_start_time: 0.0,
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
        self.game_time = 180.0;  // 重置游戏时间
        self.game_start_time = timer::time_since_start(ctx).as_secs_f32();  // 记录开始时间
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
        graphics::clear(ctx, BACKGROUND_COLOR);
        
        // 绘制标题
        let title = graphics::Text::new(graphics::TextFragment {
            text: "SNAKE GAME".to_string(),
            color: Some(PRIMARY_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(48.0)),
        });

        // 获取窗口尺寸（假设你的窗口是800x600）
        let (window_width, window_height) = {
            let size = graphics::size(ctx);
            (size.0 as f32, size.1 as f32)
        };

        // 计算标题位置（水平居中，垂直方向2/3处）
        let title_width = title.width(ctx) as f32;
        let title_x = (window_width - title_width) / 2.0;  // 水平居中
        let title_y = window_height * 0.33;          

        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([title_x, title_y])
        )?;

        // 绘制开始按钮
        let start_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(270.0, 250.0, 100.0, 40.0),
            5.0,
            PRIMARY_COLOR,
        )?;
        graphics::draw(ctx, &start_button, graphics::DrawParam::default())?;

        let start_text = graphics::Text::new(graphics::TextFragment {
            text: "START".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        // 计算文本宽度
        let text_width = start_text.width(ctx) as f32;
        let button_width = 100.0; // 按钮宽度
        let button_x = 270.0;     // 按钮x位置
        let button_y = 250.0;     // 按钮y位置

        // 计算居中位置
        let text_x = button_x + (button_width - text_width) / 2.0;
        let text_y = button_y + 10.0; // 垂直居中近似值(40px按钮高度-24px字体≈8px上下，所以+10)
        
        graphics::draw(
            ctx,
    &start_text,
    graphics::DrawParam::default()
                .dest([text_x, text_y])
            )?;

        graphics::present(ctx)
    }

    fn draw_mode_select_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BACKGROUND_COLOR);
        
        let title = graphics::Text::new(graphics::TextFragment {
            text: "SELECT GAME MODE".to_string(),
            color: Some(PRIMARY_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(36.0)),
        });
        
        // 获取窗口尺寸（假设你的窗口是800x600）
        let (window_width, window_height) = {
            let size = graphics::size(ctx);
            (size.0 as f32, size.1 as f32)
        };

        // 计算标题位置（水平居中，垂直方向2/3处）
        let title_width = title.width(ctx) as f32;
        let title_x = (window_width - title_width) / 2.0;  // 水平居中
        let title_y = window_height * 0.20;          

        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([title_x, title_y])
        )?;

        // 单人模式按钮
        let single_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 180.0, 200.0, 50.0),
            8.0,
            PRIMARY_COLOR,
        )?;
        graphics::draw(ctx, &single_button, graphics::DrawParam::default())?;

        let single_text = graphics::Text::new(graphics::TextFragment {
            text: "SINGLE PLAYER".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        let single_text_width = single_text.width(ctx) as f32;
        let single_text_x = 220.0 + (200.0 - single_text_width) / 2.0;
        let single_text_y = 180.0 + (50.0 - 24.0) / 2.0 + 5.0;

        graphics::draw(
            ctx,
            &single_text,
            graphics::DrawParam::default()
                .dest([single_text_x, single_text_y])
        )?;

        // 双人模式按钮
        let two_player_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 250.0, 200.0, 50.0),
            8.0,
            SECONDARY_COLOR,
        )?;
        graphics::draw(ctx, &two_player_button, graphics::DrawParam::default())?;

        let two_player_text = graphics::Text::new(graphics::TextFragment {
            text: "TWO PLAYERS".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        let two_player_text_width = two_player_text.width(ctx) as f32;
        let two_player_text_x = 220.0 + (200.0 - two_player_text_width) / 2.0;
        let two_player_text_y = 250.0 + (50.0 - 24.0) / 2.0 + 5.0;

        graphics::draw(
            ctx,
            &two_player_text,
            graphics::DrawParam::default()
                .dest([two_player_text_x, two_player_text_y])
        )?;

        // 限时模式按钮
        let time_attack_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 320.0, 200.0, 50.0),
            8.0,
            graphics::Color::new(0.8, 0.2, 0.8, 1.0),  // 紫色
        )?;
        graphics::draw(ctx, &time_attack_button, graphics::DrawParam::default())?;

        let time_attack_text = graphics::Text::new(graphics::TextFragment {
            text: "TIME ATTACK".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        let time_attack_text_width = time_attack_text.width(ctx) as f32;
        let time_attack_text_x = 220.0 + (200.0 - time_attack_text_width) / 2.0;
        let time_attack_text_y = 320.0 + (50.0 - 24.0) / 2.0 + 5.0;

        graphics::draw(
            ctx,
            &time_attack_text,
            graphics::DrawParam::default()
                .dest([time_attack_text_x, time_attack_text_y])
        )?;

        graphics::present(ctx)
    }

    fn draw_difficulty_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BACKGROUND_COLOR);
        
        let title = graphics::Text::new(graphics::TextFragment {
            text: "SELECT DIFFICULTY".to_string(),
            color: Some(PRIMARY_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(36.0)),
        });
        
        // 获取窗口尺寸（假设你的窗口是800x600）
        let (window_width, window_height) = {
            let size = graphics::size(ctx);
            (size.0 as f32, size.1 as f32)
        };

        // 计算标题位置
        let title_width = title.width(ctx) as f32;
        let title_x = (window_width - title_width) / 2.0;  // 水平居中
        let title_y = window_height * 0.25;          

        graphics::draw(
            ctx,
            &title,
            graphics::DrawParam::default()
                .dest([title_x, title_y])
        )?;

        // 简单难度按钮
        let easy_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 200.0, 200.0, 50.0),
            8.0,
            graphics::Color::new(0.2, 0.8, 0.2, 1.0),
        )?;
        graphics::draw(ctx, &easy_button, graphics::DrawParam::default())?;

        let easy_text = graphics::Text::new(graphics::TextFragment {
            text: "EASY".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        // 计算简单难度文本居中位置
        let easy_text_width = easy_text.width(ctx) as f32;
        let easy_text_x = 220.0 + (200.0 - easy_text_width) / 2.0;
        let easy_text_y = 200.0 + (50.0 - 24.0) / 2.0 + 5.0; // 垂直居中调整

        graphics::draw(
            ctx,
            &easy_text,
            graphics::DrawParam::default()
                .dest([easy_text_x, easy_text_y])
        )?;

        // 普通难度按钮
        let normal_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 270.0, 200.0, 50.0),
            8.0,
            graphics::Color::new(0.8, 0.8, 0.2, 1.0),
        )?;
        graphics::draw(ctx, &normal_button, graphics::DrawParam::default())?;

        let normal_text = graphics::Text::new(graphics::TextFragment {
            text: "NORMAL".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        // 计算普通难度文本居中位置
        let normal_text_width = normal_text.width(ctx) as f32;
        let normal_text_x = 220.0 + (200.0 - normal_text_width) / 2.0;
        let normal_text_y = 270.0 + (50.0 - 24.0) / 2.0 + 5.0; // 垂直居中调整

        graphics::draw(
            ctx,
            &normal_text,
            graphics::DrawParam::default()
                .dest([normal_text_x, normal_text_y])
        )?;

        // 困难难度按钮
        let hard_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 340.0, 200.0, 50.0),
            8.0,
            graphics::Color::new(0.8, 0.2, 0.2, 1.0),
        )?;
        graphics::draw(ctx, &hard_button, graphics::DrawParam::default())?;

        let hard_text = graphics::Text::new(graphics::TextFragment {
            text: "HARD".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        // 计算困难难度文本居中位置
        let hard_text_width = hard_text.width(ctx) as f32;
        let hard_text_x = 220.0 + (200.0 - hard_text_width) / 2.0;
        let hard_text_y = 340.0 + (50.0 - 24.0) / 2.0 + 5.0; // 垂直居中调整

        graphics::draw(
            ctx,
            &hard_text,
            graphics::DrawParam::default()
                .dest([hard_text_x, hard_text_y])
        )?;

        graphics::present(ctx)
    }

    fn draw_playing_screen(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BACKGROUND_COLOR);
        
        // 绘制得分
        let score_text = match self.game_mode {
            GameMode::TimeAttack => {
                format!("Score: {}", self.score)
            },
            _ => {
                format!("Player 1: {}", self.score)
            }
        };
        
        let score_display = graphics::Text::new(graphics::TextFragment {
            text: score_text,
            color: Some(PRIMARY_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(20.0)),
        });
        
        graphics::draw(
            ctx,
            &score_display,
            graphics::DrawParam::default()
                .dest([20.0, 20.0])
        )?;

        // 在限时模式下显示倒计时和难度
        if self.game_mode == GameMode::TimeAttack {
            let remaining_time = self.game_time - (timer::time_since_start(ctx).as_secs_f32() - self.game_start_time);
            let minutes = (remaining_time as i32) / 60;
            let seconds = (remaining_time as i32) % 60;
            let time_text = graphics::Text::new(graphics::TextFragment {
                text: format!("Time: {:02}:{:02}", minutes, seconds),
                color: Some(if remaining_time <= 30.0 { SECONDARY_COLOR } else { ACCENT_COLOR }),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(20.0)),
            });
            
            graphics::draw(
                ctx,
                &time_text,
                graphics::DrawParam::default()
                    .dest([445.0, 20.0])
            )?;

            // 显示当前难度
            let difficulty_text = match self.difficulty {
                Difficulty::Easy => "Easy",
                Difficulty::Normal => "Normal",
                Difficulty::Hard => "Hard",
            };
            let difficulty_display = graphics::Text::new(graphics::TextFragment {
                text: format!("Difficulty: {}", difficulty_text),
                color: Some(ACCENT_COLOR),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(20.0)),
            });
            
            graphics::draw(
                ctx,
                &difficulty_display,
                graphics::DrawParam::default()
                    .dest([445.0, 50.0])
            )?;
        }

        if let Some(_) = &self.snake2 {
            let score2_text = graphics::Text::new(graphics::TextFragment {
                text: format!("Player 2: {}", self.score2),
                color: Some(SECONDARY_COLOR),
                font: Some(graphics::Font::default()),
                scale: Some(graphics::PxScale::from(20.0)),
            });
            
            graphics::draw(
                ctx,
                &score2_text,
                graphics::DrawParam::default()
                    .dest([480.0, 50.0])
            )?;
        }

        // 绘制游戏区域边框
        let border = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            graphics::Rect::new(0.0, 0.0, 640.0, 480.0),
            ACCENT_COLOR,
        )?;
        graphics::draw(ctx, &border, graphics::DrawParam::default())?;

        // 绘制蛇和食物
        self.snake.draw(ctx)?;
        if let Some(snake2) = &self.snake2 {
            snake2.draw(ctx)?;
        }
        self.food.draw(ctx)?;
        
        graphics::present(ctx)
    }

    fn draw_game_over_screen(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 0.9));
        
        // 绘制半透明背景
        let overlay = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 640.0, 480.0),
            graphics::Color::new(0.0, 0.0, 0.0, 0.7),
        )?;
        graphics::draw(ctx, &overlay, graphics::DrawParam::default())?;

        // 绘制游戏结束文字
        let game_over_text = graphics::Text::new(graphics::TextFragment {
            text: "GAME OVER!".to_string(),
            color: Some(PRIMARY_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(48.0)),
        });
        
        // 获取窗口尺寸
        let (window_width, window_height) = {
            let size = graphics::size(ctx);
            (size.0 as f32, size.1 as f32)
        };

        // 计算标题位置
        let title_width = game_over_text.width(ctx) as f32;
        let title_x = (window_width - title_width) / 2.0;
        let title_y = window_height * 0.25;    

        // 绘制标题
        graphics::draw(
            ctx,
            &game_over_text,
            graphics::DrawParam::default()
                .dest([title_x, title_y])
        )?;

        // 根据游戏模式显示不同的分数信息
        let score_text = match self.game_mode {
            GameMode::SinglePlayer => {
                format!("Final Score: {}", self.score)
            },
            GameMode::TwoPlayer => {
                format!("Player 1: {} | Player 2: {}", self.score, self.score2)
            },
            GameMode::TimeAttack => {
                let difficulty_text = match self.difficulty {
                    Difficulty::Easy => "Easy",
                    Difficulty::Normal => "Normal",
                    Difficulty::Hard => "Hard",
                };
                format!("Time Attack Mode ({})\n    Final Score: {}", difficulty_text, self.score)
            }
        };
        
        let score_display = graphics::Text::new(graphics::TextFragment {
            text: score_text,
            color: Some(ACCENT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });

        // 计算分数位置
        let score_width = score_display.width(ctx) as f32;
        let score_x = (window_width - score_width) / 2.0 - 5.0;
        let score_y = title_y + 60.0;

        graphics::draw(
            ctx,
            &score_display,
            graphics::DrawParam::default()
                .dest([score_x, score_y])
        )?;
        let back_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 280.0, 200.0, 50.0),
            8.0,
            PRIMARY_COLOR,
        )?;
        graphics::draw(ctx, &back_button, graphics::DrawParam::default())?;
        
        let back_text = graphics::Text::new(graphics::TextFragment {
            text: "BACK TO MENU".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });
        
        // 计算返回按钮文本居中位置
        let back_text_width = back_text.width(ctx) as f32;
        let back_text_x = 220.0 + (200.0 - back_text_width) / 2.0;
        let back_text_y = 280.0 + (50.0 - 24.0) / 2.0 + 5.0; // 垂直居中调整
        
        graphics::draw(
            ctx,
            &back_text,
            graphics::DrawParam::default()
                .dest([back_text_x, back_text_y])
        )?;
        
        // 重新开始按钮
        let restart_button = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(220.0, 340.0, 200.0, 50.0),
            8.0,
            SECONDARY_COLOR,
        )?;
        graphics::draw(ctx, &restart_button, graphics::DrawParam::default())?;
        
        let restart_text = graphics::Text::new(graphics::TextFragment {
            text: "RESTART".to_string(),
            color: Some(BUTTON_TEXT_COLOR),
            font: Some(graphics::Font::default()),
            scale: Some(graphics::PxScale::from(24.0)),
        });
        
        // 计算重新开始按钮文本居中位置
        let restart_text_width = restart_text.width(ctx) as f32;
        let restart_text_x = 220.0 + (200.0 - restart_text_width) / 2.0;
        let restart_text_y = 340.0 + (50.0 - 24.0) / 2.0 + 5.0; // 垂直居中调整
        
        graphics::draw(
            ctx,
            &restart_text,
            graphics::DrawParam::default()
                .dest([restart_text_x, restart_text_y])
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
                    if y >= 180.0 && y <= 230.0 {
                        self.game_mode = GameMode::SinglePlayer;
                        self.score = 0;
                        self.score2 = 0;
                        self.game_state = GameState::DifficultySelect;
                    } else if y >= 250.0 && y <= 300.0 {
                        self.game_mode = GameMode::TwoPlayer;
                        self.score = 0;
                        self.score2 = 0;
                        self.game_state = GameState::DifficultySelect;
                    } else if y >= 320.0 && y <= 370.0 {
                        self.game_mode = GameMode::TimeAttack;
                        self.score = 0;
                        self.score2 = 0;
                        self.game_state = GameState::DifficultySelect;  // 改为进入难度选择
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
                    if y >= 280.0 && y <= 330.0 {
                        self.game_state = GameState::Title;
                    } else if y >= 340.0 && y <= 390.0 {
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

        // 在限时模式下检查游戏时间
        if self.game_mode == GameMode::TimeAttack {
            let current_time = timer::time_since_start(ctx).as_secs_f32();
            let elapsed_time = current_time - self.game_start_time;
            if elapsed_time >= self.game_time {
                self.game_state = GameState::GameOver;
                return Ok(());
            }
        }

        let delta = timer::time_since_start(ctx).as_secs_f32() - self.last_update;

        if delta >= self.update_interval {
            self.last_update = timer::time_since_start(ctx).as_secs_f32();
            
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