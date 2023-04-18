use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;
const MARGIN: i32 = SCREEN_WIDTH / 25;
const PADDLE_HEIGHT: i32 = SCREEN_HEIGHT / 10 - 1;
const PADDLE_SPEED: i32 = SCREEN_HEIGHT / 10;

enum GameMode {
    Paused,
    Playing,
}

struct Ball {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: SCREEN_WIDTH / 2,
            y: SCREEN_HEIGHT / 2,
            x_velocity: 0,
            y_velocity: 0,
        }
    }
    fn start_move(&mut self) {
        let mut random = RandomNumberGenerator::new();
        self.x_velocity = loop {
            let x = random.range(-2, 2);
            if x != 0 {
                break x;
            }
        };
        self.y_velocity = loop {
            let y = random.range(-1, 1);
            if y != 0 {
                break y;
            }
        };
    }
    fn move_and_bounce(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        if self.x < 0 || self.x > SCREEN_WIDTH - 1 {
            self.x_velocity *= -1;
        }
        if self.y < 0 || self.y > SCREEN_HEIGHT - 1 {
            self.y_velocity *= -1;
        }
    }
    fn draw(&self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, WHITE, BLACK, to_cp437('@'));
    }
    fn check_and_bounce_on_paddle(&mut self, player: &Player) {
        let x_position_range = player.x - 1..=player.x + 1;
        if x_position_range.contains(&self.x) {
            if self.y >= player.y - PADDLE_HEIGHT && self.y <= player.y + PADDLE_HEIGHT {
                self.x_velocity *= -1;
                self.y_velocity = (self.y - player.y) / 2;
            }
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    player_move: fn(&mut BTerm) -> i32,
}

impl Player {
    fn new(x: i32, player_move: fn(&mut BTerm) -> i32) -> Self {
        Self {
            x,
            y: SCREEN_HEIGHT / 2,
            player_move,
        }
    }
    fn draw(&self, ctx: &mut BTerm) {
        for i in -PADDLE_HEIGHT..=PADDLE_HEIGHT {
            ctx.set(self.x, self.y + i, WHITE, BLACK, to_cp437('#'));
        }
    }
    fn move_player(&mut self, ctx: &mut BTerm) {
        let y = (self.player_move)(ctx);
        self.y += y;
    }
}

struct Game {
    mode: GameMode,
    ball: Ball,
    frame_rate: f32,
    player1: Player,
    player2: Player,
}

fn move_player_1(ctx: &mut BTerm) -> i32 {
    if ctx.key == Some(VirtualKeyCode::W) {
        -PADDLE_SPEED
    } else if ctx.key == Some(VirtualKeyCode::S) {
        PADDLE_SPEED
    } else {
        0
    }
}

fn move_player_2(ctx: &mut BTerm) -> i32 {
    if ctx.key == Some(VirtualKeyCode::Up) {
        -PADDLE_SPEED
    } else if ctx.key == Some(VirtualKeyCode::Down) {
        PADDLE_SPEED
    } else {
        0
    }
}

impl Game {
    fn new() -> Self {
        Self {
            mode: GameMode::Paused,
            ball: Ball::new(),
            frame_rate: 0.0,
            player1: Player::new(MARGIN, move_player_1),
            player2: Player::new(SCREEN_WIDTH - MARGIN, move_player_2),
        }
    }
    fn wait_start(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(SCREEN_HEIGHT / 2 - 1, "Press Space to start");
        if ctx.key == Some(VirtualKeyCode::Space) {
            self.mode = GameMode::Playing;
            self.ball.start_move();
        }
    }
    fn render_middle_line(&mut self, ctx: &mut BTerm) {
        let mut i = 0;
        loop {
            if i > SCREEN_HEIGHT {
                break;
            }
            ctx.set(SCREEN_WIDTH / 2, i, WHITE, BLACK, to_cp437('|'));
            i += 2;
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.frame_rate += ctx.frame_time_ms;
        if self.frame_rate > FRAME_DURATION {
            self.frame_rate = 0.0;
            self.ball.move_and_bounce();
            if self.ball.x <= SCREEN_WIDTH / 2 {
                self.ball.check_and_bounce_on_paddle(&self.player1);
            } else {
                self.ball.check_and_bounce_on_paddle(&self.player2);
            }
        }
        self.player1.move_player(ctx);
        self.player2.move_player(ctx);

        self.render_middle_line(ctx);
        self.ball.draw(ctx);
        self.player1.draw(ctx);
        self.player2.draw(ctx);
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Paused => self.wait_start(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Bracket Pong")
        .build()?;
    main_loop(context, Game::new())
}
