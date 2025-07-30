use rafaello::{handler, Component, render};

#[derive(Component)]
struct Greeter {
    x: i32,
}

impl Greeter {
    // #[handler(Plus)]
    fn plus(&mut self) {
        self.x += 1
    }
    // #[handler(Minus)]
    fn minus(&mut self) {
        self.x -= 1
    }
    // fn render()
}

fn main() {
    loop {
        println!("{}", render! {});
    }
}
