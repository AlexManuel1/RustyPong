use std::fmt::Write;
use std::io::{self, stdout};
use crossterm::event::{self, Event, KeyCode, KeyEvent, MediaKeyCode};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand
};
use ratatui::layout::{Layout, self, Constraint, Rect};
use ratatui::style::Style;
use ratatui::{
    prelude::{CrosstermBackend, Terminal, *},
    widgets::*,
    Frame
}; 

struct Player {
    score: u16,
    x: u16,
    y: u16,
}

impl Player {
    fn new(x: u16, y: u16) -> Self {
        Self {
            score: 0, x, y
        }
    }

    fn get_position(&self) -> (u16, u16) {
        return (self.x, self.y);
    }
}

struct PongBall {
    dimensions: Rect,
    velocity: (i16, i16),
}

impl PongBall {
    fn new(dimensions: Rect) -> Self {
        Self {
            dimensions,
            velocity: (1, 1),
        }
    }

    fn switch_x_direction(&mut self) {
        self.velocity.0 *= -1;
    }

}

struct GameState {
    player: Player,
    opponent: Player,
    pong_ball: PongBall,
    dimensions: Rect,
    paddle_size: (u16, u16),
}

impl GameState {
    fn new(dimensions: Rect) -> Self {
        let Rect { x, y, width, height } = dimensions;
        let paddle_size = (2, 4);
        let paddle_y_start_pos= ((y + height) / 2 - (paddle_size.1 / 2)) as u16;

        Self {
            player: Player::new(
                x + 1, 
                paddle_y_start_pos,
            ),
            opponent: Player::new(
                (x + width) - (paddle_size.0 + 1), 
                paddle_y_start_pos,
            ),
            pong_ball: PongBall::new(Rect::new(
                (x + width) / 2 as u16, 
                (y + height) / 2 as u16, 
                1, 
                1,
            )),
            dimensions: dimensions, 
            paddle_size: (2, 4),
        }
    }

    fn get_player_paddle_pos(&self) -> (u16, u16) {
        self.player.get_position()
    }

    fn get_opponent_paddle_pos(&self) -> (u16, u16) {
        self.opponent.get_position()
    }

    fn get_game_borders(&self) -> (u16, u16, u16, u16) {
        let Rect { x, y, width, height } = self.dimensions;
        (y, y + height, x, x + width)
    }

    fn reset_pong_position(&mut self) {
        let Rect { x, y, width, height}  = self.dimensions;
        self.pong_ball.dimensions = Rect::new((x + width) / 2 as u16, (y + height) / 2 as u16, 1, 1);
    }

    fn move_pong_ball(&mut self) {
        // update ball position
        let Rect { x, y, .. } = self.pong_ball.dimensions;

        let (
            borderTop, 
            borderBottom, 
            borderLeft, 
            borderRight
        ) = self.get_game_borders(); 


        if y <= borderTop + 1 || y >= borderBottom - 1 {
            self.pong_ball.velocity.1 *= -1;
        } 

        let player_paddle = self.get_player_paddle_pos();
        let opponent_paddle = self.get_opponent_paddle_pos();

        if x == player_paddle.0 + self.paddle_size.0 && 
            y >= player_paddle.1 && 
            y <= (player_paddle.1 + self.paddle_size.1) {
                self.pong_ball.switch_x_direction();
        }
         
        if x == opponent_paddle.0 - 1 && 
            y >= opponent_paddle.1 && 
            y <= (opponent_paddle.1 + self.paddle_size.1) {
                self.pong_ball.switch_x_direction();
        }

        if x <= self.dimensions.x {
            self.opponent.score += 1;
            self.pong_ball.switch_x_direction();
            self.reset_pong_position();
        }

        if x >= self.dimensions.x + self.dimensions.width - 1 {
            self.player.score += 1;
            self.pong_ball.switch_x_direction();
            self.reset_pong_position();
        }

        if self.pong_ball.velocity.1 < 0 {
            self.pong_ball.dimensions.y -= 1;
        } else {
            self.pong_ball.dimensions.y += 1;
        }
        if self.pong_ball.velocity.0 < 0 {
            self.pong_ball.dimensions.x -= 1;
        } else {
            self.pong_ball.dimensions.x += 1;
        }
    }

    fn move_paddle(&mut self, key: KeyCode) {
        let Rect { x, y, width, height } = self.dimensions;
        let (
            borderTop, 
            borderBottom, 
            borderLeft, 
            borderRight
        ) = self.get_game_borders(); 

        match key {
            KeyCode::Char('w') => {
                if self.player.y - 1 > borderTop {
                    self.player.y -= 1;
                }
            },
            KeyCode::Char('s') => {
                if (self.player.y + self.paddle_size.1) + 1 < borderBottom {
                    self.player.y += 1;
                }
            },
            KeyCode::Up => {
                if self.opponent.y - 1 > borderTop {
                    self.opponent.y -= 1;
                }
            },
            KeyCode::Down => {
                if (self.opponent.y + self.paddle_size.1) + 1 < borderBottom {
                    self.opponent.y += 1;
                }
            },
            _ => {}
        }
    }
}

pub struct TerminalOutput {
    game_state: GameState
}


impl TerminalOutput {
    pub fn new(width: u16, height: u16) -> Self {
        let dimensions = Rect::new(0, 0, width, height);
        Self {
            game_state: GameState::new(dimensions),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            if event::poll(std::time::Duration::from_millis(50))? {
                if !self.read_key()? { break }
            }
            self.draw(&mut terminal)?;
            // draw UI
            self.game_state.move_pong_ball();
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn read_key(&mut self) -> io::Result<bool> {
        if let Event::Key(key) = event::read()? {
            match key {
                KeyEvent { 
                    code: KeyCode::Char('c'), 
                    modifiers: event::KeyModifiers::CONTROL, 
                    ..
                } => return Ok(false),
                KeyEvent {
                    code: KeyCode::Char('w') | KeyCode::Char('s') | KeyCode::Up | KeyCode::Down,
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => self.game_state.move_paddle(key.code),
                _ => {},
            }
        }
        Ok(true)
    }

    fn draw<W: io::Write>(&self, terminal: &mut Terminal<CrosstermBackend<W>>) -> io::Result<()> {
        terminal.draw(|frame| {

            let player1Score = self.game_state.player.score;
            let player2Score = self.game_state.opponent.score;

            // draw game
            let game_area = Block::new()
                .borders(Borders::all())
                .border_type(BorderType::Rounded)
                .title(
                    format!(
                        "RustyPong ({},{}) | P1: {} P2: {} ", 
                        self.game_state.dimensions.width, 
                        self.game_state.dimensions.height, 
                        player1Score, 
                        player2Score,
                    )
                )
                .title_alignment(Alignment::Center);
            frame.render_widget(game_area, self.game_state.dimensions);
            
            // draw player
            let (player_paddle_x, player_paddle_y) = self.game_state.get_player_paddle_pos();
            let paddle_size = self.game_state.paddle_size;
            let player_paddle= Block::new()
                .borders(Borders::all())
                .border_style(Style::new().light_green());
            frame.render_widget(
                player_paddle, 
                Rect::new(
                    player_paddle_x, 
                    player_paddle_y,
                    paddle_size.0, 
                    paddle_size.1
                )
            );

            // draw opponent
            let (opponent_paddle_x, opponent_paddle_y) = self.game_state.get_opponent_paddle_pos();
            let opponent_paddle = Block::new()
                .borders(Borders::all())
                .border_style(Style::new().light_green());
            frame.render_widget(
                opponent_paddle, 
                Rect::new(
                    opponent_paddle_x, 
                    opponent_paddle_y, 
                    paddle_size.0, 
                    paddle_size.1
                )
            );

            // draw pong ball
            let pong_ball = Paragraph::new("o");
            frame.render_widget(
                pong_ball, 
                self.game_state.pong_ball.dimensions
            );
        })?;
        Ok(())
    }
}