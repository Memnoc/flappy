use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}
struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }
}

// Implemeting the trait for the struct
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket Terminal");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;
    main_loop(context, State { mode: todo!() })
}
