const INPUT: i32 = 312051;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    let mut step = 1;
    let mut remaining = INPUT;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = Direction::Right;
    while remaining - step > 0 {
        remaining -= step;
        match dir {
            Direction::Up => {
                y += step;
                dir = Direction::Left;
                step += 1;
            }
            Direction::Left => {
                x -= step;
                dir = Direction::Down;
            }
            Direction::Down => {
                y -= step;
                dir = Direction::Right;
                step += 1;
            }
            Direction::Right => {
                x += step;
                dir = Direction::Up;
            }
        }
    }
    match dir {
        Direction::Up => y += remaining - 1,
        Direction::Left => x -= remaining - 1,
        Direction::Down => y -= remaining - 1,
        Direction::Right => x += remaining - 1,
    }
    println!(
        "Square located at x:{} y:{}, shortest path to start is {}",
        x,
        y,
        x.abs() + y.abs()
    );
}
