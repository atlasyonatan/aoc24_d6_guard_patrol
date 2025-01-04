use array2d::Array2D;
use enum_map::{enum_map, Enum};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let chars: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut arr = Array2D::from_rows(&chars).unwrap();

    println!("input array:\n{}", arr_to_str(&arr, |c| *c));

    let start_char = '^';
    let obstacle_char = '#';
    let visited_char = 'X';

    let mut position = arr
        .indices_row_major()
        .find_map(|(row, column)| {
            (*arr.get(row, column).unwrap() == start_char).then(|| (row, column))
        })
        .unwrap();

    let directions = enum_map! {
        Direction::Up => (-1,0),
        Direction::Down => (1,0),
        Direction::Left => (0,-1),
        Direction::Right => (0,1)
    };

    let mut direction = Direction::Up;

    loop {
        //mark current cell as visited
        arr.set(position.0, position.1, visited_char).unwrap();

        let d = directions[direction];
        let next_step_position = match (
            position.0.checked_add_signed(d.0),
            position.1.checked_add_signed(d.1),
        ) {
            (Some(new_row), Some(new_column)) => (new_row, new_column),
            _ => break,
        };

        let next_step_char = match arr.get(next_step_position.0, next_step_position.1) {
            None => break,
            Some(c) => c,
        };

        if *next_step_char == obstacle_char {
            //turn right 90 degrees
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            //step forward
            position = next_step_position;
        }
    }

    println!("prediction:\n{}", arr_to_str(&arr, |c| *c));

    let result = arr
        .elements_row_major_iter()
        .filter(|&c| *c == visited_char)
        .count();

    println!("part 1: {}", result)
}

#[derive(Debug, Enum, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn arr_to_str<T, F>(arr: &Array2D<T>, f: F) -> String
where
    F: Fn(&T) -> char,
{
    let mut s = String::new();

    for row in arr.rows_iter() {
        for cell in row {
            let c = f(cell);
            s.push(c);
        }
        s.push('\n');
    }

    return s;
}
