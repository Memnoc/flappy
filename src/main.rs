use bracket_lib::prelude::*;

struct State {}

// IMmplemeting the trait for the struct
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Bracket Terminal");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;
    main_loop(context, State {})
}
