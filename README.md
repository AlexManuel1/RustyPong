# Multiplayer pong

## Setup

### Spin-up Server
- run the command `cargo run --bin pong_server 127.0.0.1` to start server on IP 127.0.0.1
- port is hard coded to 3737

### Connect to Server
- open two terminal windows representing player 1 and player 2
- for each terminal run the command `cargo run --bin pong_game 127.0.0.1 3737`
- If an error occurs on a terminal that says "Trying to access position outside the buffer..." this could mean that the terminal window is too small and you need to increase either width or height. Width and height are hardcoded to be 80, 40 respectively