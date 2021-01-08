//From hands on rust book
use bracket_lib::prelude::*;

struct State {
    i: usize,
    j: usize,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(self.i, self.j, "Hello bracket terminal!");
        self.i += 1;
        self.j += 1;
        if self.i >= 80 {self.i = 0;}
        if self.j >= 50 {self.j = 0;}
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Erez")
        .build()?;

    main_loop(context, State { i: 0, j: 0})
}
