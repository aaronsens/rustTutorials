use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

// Constants
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SPEED: f32 = 500.0;
const BALL_RADIUS: f32 = 15.0;
const BALL_SPEED: f32 = 400.0;
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

struct Paddle {
    rect: Rect,
    vel: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Paddle {
            rect: Rect::new(x, y, PADDLE_WIDTH, PADDLE_HEIGHT),
            vel: 0.0,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = event::delta(ctx);
        let dy = self.vel * dt.as_secs_f32();
        self.rect.y += dy;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.rect,
            Color::from_rgb(255, 255, 255),
        )?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}

struct Ball {
    circle: na::Point2<f32>,
    vel: na::Vector2<f32>,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Ball {
            circle: na::Point2::new(x, y),
            vel: na::Vector2::new(BALL_SPEED, BALL_SPEED),
        }
    }

    fn update(&mut self, dt: f32) {
        self.circle += self.vel * dt;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.circle,
            BALL_RADIUS,
            2.0,
            Color::from_rgb(255, 255, 255),
        )?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        Ok(())
    }
}

struct GameState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    score1: u32,
    score2: u32,
    last_hit: Option<na::Point2<f32>>,
    started: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player1: Paddle::new(10.0, HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            player2: Paddle::new(WIDTH - 30.0, HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            ball: Ball::new(WIDTH / 2.0, HEIGHT / 2.0),
            score1: 0,
            score2: 0,
            last_hit: None,
            started: false,
        }
