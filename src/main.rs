use std::io::{stdin, stdout, Write};

/// Kind of Entity in area/game.
#[derive(Copy, Clone)]
enum Kind {
   Food,
   Player,
   Empty,
}

/// Position enum to detect which way to go.
enum Position {
   Left,
   Right,
   Up,
   Down,
}

const PLAYER: &str = "😳";
const FOOD: &str = "🍒";
const EMPTY: &str = "⚫";

const ROW: u8 = 11;
const COLUMN: u8 = 11;

type Area = [[Kind; COLUMN as usize]; ROW as usize];

/// A struct for creatures/entities in game which has X and Y coordinates.
struct Entity {
   x: u8,
   y: u8,
}

/// Main game struct which is owner of every function.
struct Game {
   area: Area,
   player: Entity,
   food: Entity,
   score: u16,
}

/// Trait for Game Struct.
trait GameTrait {
   fn init(&mut self) -> ();
   fn draw(&self) -> ();
   fn read_key(&mut self) -> ();
   fn update(&mut self) -> ();
   fn update_player_position(&mut self, position: Position) -> ();
}

impl GameTrait for Game {
   /// Sets everything (player position, food position) and runs the game.
   fn init(&mut self) {
      self.area[self.player.y as usize][self.player.x as usize] = Kind::Player;
      self.area[self.food.y as usize][self.food.x as usize] = Kind::Food;
      self.update();
   }

   /// Draws everything inside area array. Player, Food and Empty kinds.
   fn draw(&self) {
      print!("\x1B[2J\x1B[1;1H");
      for y in 0..self.area.len() {
         for x in 0..self.area[y].len() {
            match self.area[y][x] {
               Kind::Food => print!("{}", FOOD),
               Kind::Player => print!("{}", PLAYER),
               Kind::Empty => print!("{}", EMPTY),
            }
         }
         print!("\n");
      }
      print!("SKOR :{}", self.score);
   }

   /// Read's user key and moves player or exits the app.
   fn read_key(&mut self) -> () {
      print!("\nBir değer bekleniyor (a, s, d, w): ");
      if let Ok(_) = stdout().flush() {};
      let mut input = String::new();
      if let Ok(_) = stdin().read_line(&mut input) {
         input = String::from(input.trim());
         match input.as_str() {
            "a" => {
               self.update_player_position(Position::Left);
               self.update();
            }
            "d" => {
               self.update_player_position(Position::Right);
               self.update();
            }
            "w" => {
               self.update_player_position(Position::Up);
               self.update();
            }
            "s" => {
               self.update_player_position(Position::Down);
               self.update();
            }
            "e" => {}
            _ => self.read_key(),
         }
      }
   }

   /// Updates player position in "area" array by given Position.
   fn update_player_position(&mut self, position: Position) -> () {
      let (cx, cy) = (self.player.x, self.player.y);
      self.area[cy as usize][cx as usize] = Kind::Empty;
      match position {
         Position::Down => {
            let ny = if cy != (ROW - 1) { cy + 1 } else { 0 };
            self.player.y = ny;
            self.area[ny as usize][cx as usize] = Kind::Player;
         }
         Position::Up => {
            let ny = if cy != 0 { cy - 1 } else { ROW - 1 };
            self.player.y = ny;
            self.area[ny as usize][cx as usize] = Kind::Player;
         }
         Position::Left => {
            let nx = if cx != 0 { cx - 1 } else { COLUMN - 1 };
            self.player.x = nx;
            self.area[cy as usize][nx as usize] = Kind::Player;
         }
         Position::Right => {
            let nx = if cx != (COLUMN - 1) { cx + 1 } else { 0 };
            self.player.x = nx;
            self.area[cy as usize][nx as usize] = Kind::Player;
         }
      }
      if self.player.x == self.food.x && self.player.y == self.food.y {
         self.score += 1;
      }
   }

   /// Update function which updates the game and draws.
   fn update(&mut self) -> () {
      self.draw();
      self.read_key();
   }
}

fn main() {
   let mut game = Game {
      area: [[Kind::Empty; COLUMN as usize]; ROW as usize],
      player: Entity { x: 5, y: 5 },
      food: Entity { x: 7, y: 3 },
      score: 0,
   };

   game.init();
}
