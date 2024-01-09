use std::io::{self, stdout, Write, Read};
use std::net::TcpStream;
use std::process;
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
use std::str;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};


#[derive(Deserialize, Serialize, Debug)]
pub struct GameDataJSON {
    player_one_pos: [u16; 2],
    player_two_pos: [u16; 2],
    pong_pos: [u16; 2],
    player_one_score: u16,
    player_two_score: u16,
}

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

    fn get_position(&self) -> [u16; 2] {
        [self.x, self.y]
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

    pub fn get_pong_pos(&self) -> [u16; 2] {
        [self.dimensions.x, self.dimensions.y]
    }

    pub fn set_pong_pos(&mut self, pong_pos: [u16; 2]) {
        self.dimensions.x = pong_pos[0];
        self.dimensions.y = pong_pos[1];
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

    fn get_player_paddle_pos(&self) -> [u16; 2] {
        self.player.get_position()
    }

    fn set_player_paddle_pos(&mut self, pos: [u16; 2]) {
        self.player.x = pos[0];
        self.player.y = pos[1];
    }

    fn get_opponent_paddle_pos(&self) -> [u16; 2] {
        self.opponent.get_position()
    }

    fn set_opponent_paddle_pos(&mut self, pos: [u16; 2]) {
        self.opponent.x = pos[0];
        self.opponent.y = pos[1];
    }

    fn set_player_score(&mut self, score: u16) {
        self.player.score = score;
    }

    fn set_opponent_score(&mut self, score: u16) {
        self.opponent.score = score;
    }

    fn get_game_borders(&self) -> (u16, u16, u16, u16) {
        let Rect { x, y, width, height } = self.dimensions;
        (y, y + height, x, x + width)
    }

    fn get_pong_pos(&mut self) -> [u16; 2] {
        [self.pong_ball.dimensions.x, self.pong_ball.dimensions.y]
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

        if x == player_paddle[0] + self.paddle_size.0 && 
            y >= player_paddle[1] && 
            y <= (player_paddle[1] + self.paddle_size.1) {
                self.pong_ball.switch_x_direction();
        }
         
        if x == opponent_paddle[0] - 1 && 
            y >= opponent_paddle[1] && 
            y <= (opponent_paddle[1] + self.paddle_size.1) {
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

    pub fn get_game_data(&self) -> GameDataJSON {
        let player_one_pos = self.game_state.get_player_paddle_pos();
        let player_two_pos = self.game_state.get_opponent_paddle_pos();
        let pong_pos = self.game_state.pong_ball.get_pong_pos();
        let game_data = GameDataJSON {
            player_one_pos,
            player_two_pos,
            pong_pos,
            player_one_score: self.game_state.player.score,
            player_two_score: self.game_state.opponent.score
        };
        game_data
    }

    pub fn set_game_data(&mut self, game_data: &GameDataJSON) {
        self.game_state.set_player_paddle_pos(game_data.player_one_pos);
        self.game_state.set_opponent_paddle_pos(game_data.player_two_pos);
        self.game_state.pong_ball.set_pong_pos(game_data.pong_pos);
        self.game_state.set_player_score(game_data.player_one_score);
        self.game_state.set_opponent_score(game_data.player_two_score);
    }

    pub fn run_client(&mut self, stream: &mut TcpStream) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        loop {
	        let mut client_read_buffer = [0u8; 1024];
            if !self.read_key_client(stream)? {
                break;
            }
            //println!("Waiting for server data");
            match stream.read(&mut client_read_buffer) {
                Ok(n) => {
                    if n == 0 {
                        process::exit(0);
                    }
                    else
                    {
                        let buffer_as_string = str::from_utf8(&client_read_buffer).unwrap().trim_matches(char::from(0));
                        //println!("json data as string: {:?}", buffer_as_string);
                        let json_data: GameDataJSON = serde_json::from_str(&buffer_as_string).unwrap();
                        self.set_game_data(&json_data);
                        //println!("json data: {:?}", json_data);
                        //println!("{:?}", self.game_state.get_pong_pos());
                        //io::stdout().flush().unwrap();
                    }
                },
                Err(error) => {
                    println!("{}", error.to_string());
                    process::exit(-1);
                },
            }
            self.draw(&mut terminal)?; // draw UI
            self.game_state.move_pong_ball();
        }
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn run_server(&mut self, player_one: &mut TcpStream, player_two: &mut TcpStream) -> io::Result<()> {
        loop {
            if event::poll(std::time::Duration::from_millis(50))? {
                if !self.read_key()? { break }
            }

            self.read_key_server(player_one, player_two);
            println!("Moving ball");
            self.game_state.move_pong_ball();
            
            let game_data = serde_json::to_string(&self.get_game_data()).unwrap();
            println!("{:?}", game_data);

            player_one.write_all(game_data.as_bytes()).unwrap();
            player_two.write_all(game_data.as_bytes()).unwrap();
        }
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            if event::poll(std::time::Duration::from_millis(50))? {
                if !self.read_key()? { break }
            }
            self.draw(&mut terminal)?; // draw UI
            self.game_state.move_pong_ball();
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn read_key_server(&mut self, player_one: &mut TcpStream, player_two: &mut TcpStream) {
	    let mut player_one_read_buffer: [u8; 1] = [0; 1];
	    let mut player_two_read_buffer: [u8; 1] = [0; 1];
        println!("Reading key on server side!");
        match player_one.read(&mut player_one_read_buffer) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                else {
                    let key = str::from_utf8(&player_one_read_buffer).unwrap();
                    if key == "w" {
                        self.game_state.move_paddle(KeyCode::Char('w'));
                    } else if key == "s" {
                        self.game_state.move_paddle(KeyCode::Char('s'));
                    }
                    println!("{}", key);
                }
            },
            Err(error) => {
                println!("{}", error.to_string());
                process::exit(-1);
            },
        }

        match player_two.read(&mut player_two_read_buffer) {
            Ok(n) => {
                if n == 0 {
                    return;
                }
                else {
                    let key = str::from_utf8(&player_two_read_buffer).unwrap();
                    if key == "w" {
                        self.game_state.move_paddle(KeyCode::Up);
                    } else if key == "s" {
                        self.game_state.move_paddle(KeyCode::Down);
                    }
                    println!("{}", key);
                }
            },
            Err(error) => {
                println!("{}", error.to_string());
                process::exit(-1);
            },
        }
    }

    fn read_key_client(&mut self, stream: &mut TcpStream) -> io::Result<bool> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key {
                    KeyEvent { 
                        code: KeyCode::Char('c'), 
                        modifiers: event::KeyModifiers::CONTROL, 
                        ..
                    } => return Ok(false),
                    KeyEvent {
                        code: KeyCode::Char('w'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        let key_pressed_buffer: [u8; 1] = [b'w'];
                        let bytes_written = stream.write(&key_pressed_buffer).unwrap();
                        if bytes_written <= 0 {
                            return Ok(false);
                        } 
                        //println!("Sent key press 'w' to server");
                    },
                    KeyEvent {
                        code: KeyCode::Char('s'),
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        let key_pressed_buffer: [u8; 1] = [b's'];
                        let bytes_written = stream.write(&key_pressed_buffer).unwrap();
                        if bytes_written <= 0 {
                            return Ok(false);
                        } 
                        //println!("Sent key press 's' to server");
                    },
                    _ => {
                        let key_pressed_buffer: [u8; 1] = [b'0'];
                        let bytes_written = stream.write(&key_pressed_buffer).unwrap();
                        if bytes_written <= 0 {
                            return Ok(false);
                        } 
                        //println!("Sent nothing to server!");
                    },
                }
                //println!("finished reading key!");
            }
        } else {
            //println!("No key pressed");
            let key_pressed_buffer: [u8; 1] = [b'0'];
            let bytes_written = stream.write(&key_pressed_buffer).unwrap();
            if bytes_written <= 0 {
                return Ok(false);
            } 
        }
        Ok(true)
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
            let [player_paddle_x, player_paddle_y] = self.game_state.get_player_paddle_pos();
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
            let [opponent_paddle_x, opponent_paddle_y] = self.game_state.get_opponent_paddle_pos();
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