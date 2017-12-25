// https://adventofcode.com/2017/day/25

use std::collections::VecDeque;

fn main() {
    // Initialize tape as a deque to get fast pushes to front and back
    let mut tape = VecDeque::new();
    tape.push_back(false);
    tape.push_back(false);
    tape.push_back(false);

    // Run the machine
    let mut position = 1;
    let mut state = State::A;
    for _ in 0..12172063 {
        let (new_position, new_state) = step(position, state, &mut tape);
        position = new_position;
        state = new_state;
    }

    // Checksum is simply adding the on bits together
    println!(
        "The diagnostic checksum is {} after 12172063",
        tape.iter()
            .fold(0, |acc, &b| if b == true { acc + 1 } else { acc })
    );
}

fn step(mut position: usize, mut state: State, tape: &mut VecDeque<bool>) -> (usize, State) {
    // Step accorging to state and current value
    match state {
        State::A => match tape[position] {
            false => {
                tape[position] = true;
                position += 1;
                state = State::B;
            }
            true => {
                tape[position] = false;
                position -= 1;
                state = State::C;
            }
        },
        State::B => match tape[position] {
            false => {
                tape[position] = true;
                position -= 1;
                state = State::A;
            }
            true => {
                position -= 1;
                state = State::D;
            }
        },
        State::C => match tape[position] {
            false => {
                tape[position] = true;
                position += 1;
                state = State::D;
            }
            true => {
                tape[position] = false;
                position += 1;
                state = State::C;
            }
        },
        State::D => match tape[position] {
            false => {
                position -= 1;
                state = State::B;
            }
            true => {
                tape[position] = false;
                position += 1;
                state = State::E;
            }
        },
        State::E => match tape[position] {
            false => {
                tape[position] = true;
                position += 1;
                state = State::C;
            }
            true => {
                position -= 1;
                state = State::F;
            }
        },
        State::F => match tape[position] {
            false => {
                tape[position] = true;
                position -= 1;
                state = State::E;
            }
            true => {
                position += 1;
                state = State::A;
            }
        },
    }

    // Extend tape if necessary
    if position == 0 {
        tape.push_front(false);
        position += 1;
    } else if position == tape.len() - 1 {
        tape.push_back(false);
    }


    (position, state)
}

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}
