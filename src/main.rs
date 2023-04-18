use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;
const MARGIN: i32 = SCREEN_WIDTH / 25;
const PADDLE_HEIGHT: i32 = SCREEN_HEIGHT / 10 - 1;
const PADDLE_SPEED: i32 = 3;

enum GameMode {
    Menu,
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
    fn reset_position(&mut self) {
        self.x = SCREEN_WIDTH / 2;
        self.y = SCREEN_HEIGHT / 2;
        self.x_velocity = 0;
        self.y_velocity = 0;
    }
    fn start_move(&mut self) {
        let mut random = RandomNumberGenerator::new();
        self.x_velocity = loop {
            let x = random.range(-1, 2);
            if x != 0 {
                break x * 2;
            }
        };
        self.y_velocity = loop {
            let y = random.range(-1, 2);
            if y != 0 {
                break y;
            }
        };
    }
    fn move_and_bounce(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        if self.y < 0 || self.y > SCREEN_HEIGHT - 1 {
            self.y_velocity *= -1;
        }
    }
    fn draw(&self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, WHITE, BLACK, to_cp437('@'));
    }
    fn bounce_and_score(&mut self, players: &[Player; 2]) -> Option<(i32, i32)> {
        if self.x <= 0 {
            return Some((0, 1));
        } else if self.x >= SCREEN_WIDTH - 1 {
            return Some((1, 0));
        }

        for player in players {
            let x_position_range = player.x - 1..=player.x + 1;
            if x_position_range.contains(&self.x) {
                if self.y >= player.y - PADDLE_HEIGHT && self.y <= player.y + PADDLE_HEIGHT {
                    self.x_velocity *= -1;
                    self.y_velocity = (self.y - player.y) / 2;
                }
            }
        }
        None
    }
}

struct Player {
    x: i32,
    y: i32,
    score: i32,
    player_move: fn(&mut BTerm) -> i32,
}

impl Player {
    fn new(x: i32, player_move: fn(&mut BTerm) -> i32) -> Self {
        Self {
            x,
            y: SCREEN_HEIGHT / 2,
            score: 0,
            player_move,
        }
    }
    fn reset_position(&mut self) {
        self.y = SCREEN_HEIGHT / 2;
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
    players: [Player; 2],
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
            mode: GameMode::Menu,
            ball: Ball::new(),
            frame_rate: 0.0,
            players: [
                Player::new(MARGIN, move_player_1),
                Player::new(SCREEN_WIDTH - MARGIN, move_player_2),
            ],
        }
    }
    fn wait_start(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(SCREEN_HEIGHT / 2 - 1, "Press Space to start");
        self.reset_round(ctx);
    }
    fn reset_round(&mut self, ctx: &mut BTerm) {
        self.ball.reset_position();
        self.players[0].reset_position();
        self.players[1].reset_position();
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
    fn render_scores(&mut self, ctx: &mut BTerm, player1_score: i32, player2_score: i32) {
        ctx.print(MARGIN, 1, player1_score.to_string());
        ctx.print(SCREEN_WIDTH - MARGIN, 1, player2_score.to_string());
    }
    fn paused(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(SCREEN_HEIGHT / 2 - 1, "Press Space to start");
        self.ball.draw(ctx);
        self.render_middle_line(ctx);
        self.render_scores(ctx, self.players[0].score, self.players[1].score);
        self.players[0].draw(ctx);
        self.players[1].draw(ctx);

        if ctx.key == Some(VirtualKeyCode::Space) {
            self.mode = GameMode::Playing;
            self.ball.start_move();
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.frame_rate += ctx.frame_time_ms;
        if self.frame_rate > FRAME_DURATION {
            self.frame_rate = 0.0;
            self.ball.move_and_bounce();
            if let Some((player1_score, player2_score)) = self.ball.bounce_and_score(&self.players)
            {
                self.players[0].score += player1_score;
                self.players[1].score += player2_score;
                self.mode = GameMode::Paused;
                self.reset_round(ctx);
            }
        }
        self.players[0].move_player(ctx);
        self.players[1].move_player(ctx);

        self.ball.draw(ctx);
        self.render_middle_line(ctx);
        self.render_scores(ctx, self.players[0].score, self.players[1].score);
        self.players[0].draw(ctx);
        self.players[1].draw(ctx);
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.wait_start(ctx),
            GameMode::Paused => self.paused(ctx),
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
