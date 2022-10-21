enum State {
    Start,
    Running {hp: u32},
    GameOver(Animation)
}

enum Animation {
    Running,
    Stopped
}

fn main() {
    let mut state = State::Start ;
    loop {
        match state {
            State::Start => {
                println!("Game has Started");
                state = State::Running{hp:100};
            },
            State::Running{hp:0} => {
                state = State::GameOver(Animation::Running);
                println!("Low Dead end");
            },
            State::Running{ ref mut hp} => {
                *hp -= 25;
                println!("Player got hit->{hp}");
            },
            State::GameOver(Animation::Running) => {
                println!("Transition");
                state = State::GameOver(Animation::Stopped);
            },
            State::GameOver(Animation::Stopped) => break

        }
    }
}
