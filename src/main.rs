// two messages can be emited by the client
//
// move: Moves
// shot: 0-360 which resembles the direction in which the player shoots

// the server emits the world state to the cliets

// the intervall should be 60 times a second

enum Moves {
  W,
  S,
  A,
  D,
  WA,
  WD,
  SA,
  SD
}

struct Position {
    x: i32,
    y: i32,
}

struct Player {
    uuid:     u32,
    position: Position,
    deaths:   u32,
    kills:    u32,
}

fn main() {
  // init four players
  let mut world_state: Vec<Player> = Vec::new();
  let mut i = 0u32;

  loop {
    i += 1;
    if i == 5 {
      break
    }
    let position   = Position { x: 0, y: 0 };
    let player     = Player { uuid: i, position: position, deaths: 0, kills: 0};
    world_state.push(player);
  }

  println!("Hello, world!");
}
