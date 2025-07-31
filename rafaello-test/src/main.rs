use rafaello::{handler, Component, render};

#[derive(Default, Component)]
struct Greeter {
    x: i32,
}

impl Greeter {
    #[handler(Plus)]
    fn plus(&mut self) {
        self.x += 1
    }
    #[handler(Minus)]
    fn minus(&mut self) {
        self.x -= 1
    }
    #[render]
    fn render(&self) -> /* насчёт типа не придумал. может быть, что будет Draw */ {
        draw! {
            block [border {color = tailwind::EMERALD, align = Border::ALL, }, title = "Привет"] {
                p { format!("Сейчас на счету: {}", self.x) }
                layout [horizontal] {
                    button [click = Plus] { "ДЕНЬГА СЮДА!" }
                    button [click = || { self.x -= 1; }] { "СОСАЛ?" }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    rafaello::render!(Greeter::new());
    Ok(())
}
