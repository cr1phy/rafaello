use rafaello::{Component, draw, handler, render, run};

// #[derive(Default)]
// struct Greeter {
//     x: i32,
// }

// #[component(Greeter)]
// fn greeter()

// impl Greeter {
//     #[handler(Plus)]
//     fn plus(&mut self) {
//         self.x += 1;
//     }

//     #[handler(Minus)]
//     fn minus(&mut self) {
//         self.x -= 1;
//     }

//     #[render]
//     fn render(&self) -> _ {
//         draw! {
//             block [ title = "Привет" ] {
//                 format!("Сейчас на счету: {}", &self.x)
//             }
//         }
//     }
// }

struct CounterState {
    x: i32,
    y: i32,
}

#[component(Counter)]
fn counter() -> Draw {
    let state = state!(CounterState); // сюда можно будет засовывать всякие штучки
    // ещё пример: `let state = state!([x = 10] as CounterState)` => CounterState{x = 10, y = 0} (default)
    // или... `let state = state!([x, y]) => {x = 0, y = 0} (default anonymous struct/hashmap)`

    // реализация через 
    let set_x = || {
        state.x = 2;
    };

    // реализация через функцию
    fn set_y() {
        state.y = -2;
    };

    draw! {
        block [border={/* кастомный бордер */}, title="Каунтер"] {
            // Вертикально
            vertical + block [title = format!("{}", state.x + state.y)] {
                // Горизонтально
                horizontal {
                    // Кнопка с параграфом и кликом
                    button [click={cb!(set_x)}] {
                        p { format!("x: {}", state.x) }
                    }
                    // Кнопка с кликом
                    button [click={cb!(set_y)}] {
                        format!("y: {}", state.y)
                    }
                }
                // Кнопка со стилями
                button [style={color!(if state.x == 2 "emerald-500" else "emerald-700"), }] {}
                button [style={color!(if state.y == -2 "red" else "purple"), }] {}
                input {}
            }
        }
    }
}

#[component(App)]
fn app() -> Draw {
    draw! {
        // Counter [x = 148, y = 8] // Это когда пропсы есть, а так...
        Counter {}
    }
}

#[tokio::main]
async fn main() {
    run!(App);
}
