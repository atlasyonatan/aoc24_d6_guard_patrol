pub mod patrol;

use array2d::Array2D;
use enumflags2::BitFlags;
use patrol::{Direction, PatrolOutcome, PatrolStep, VisitDirections};
use std::{
    io::{self, Read},
    iter::repeat,
};

const IN_START_CHAR: char = '^';
const IN_OBSTACLE_CHAR: char = '#';
const OUT_VISITED_CHAR: char = 'X';
const OUT_NEW_OBSTACLE: char = 'O';
const OUT_MOVEMENT_HORIZONTAL: char = '-';
const OUT_MOVEMENT_VERTICAL: char = '|';
const OUT_MOVEMENT_HORIZONTAL_AND_VERTICAL: char = '+';

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let chars: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let arr = Array2D::from_rows(&chars).unwrap();

    println!("input array:\n{}", arr_to_str(&arr, |c| *c));

    let start_position = arr
        .indices_row_major()
        .find_map(|(row, column)| {
            (*arr.get(row, column).unwrap() == IN_START_CHAR).then(|| (row, column))
        })
        .unwrap();
    let start_direction = Direction::Up;

    let step_fn = |next_char: &char, current_direction| match *next_char {
        IN_OBSTACLE_CHAR => {
            //turn right 90 degrees
            PatrolStep::Face(match current_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            })
        }
        _ => PatrolStep::StepForward,
    };

    let (outcome, visited_arr) = patrol::patrol(&arr, start_position, start_direction, step_fn);
    assert!(matches!(outcome, PatrolOutcome::LeftBounds));

    //visualize visited positions
    let mut display_arr = arr.clone();

    for index in visited_arr.indices_row_major() {
        let directions = visited_arr.get(index.0, index.1).unwrap();
        if directions.is_empty() {
            continue;
        }

        display_arr.set(index.0, index.1, OUT_VISITED_CHAR).unwrap();
    }

    println!("prediction:\n{}", arr_to_str(&display_arr, |c| *c));

    let result = visited_arr
        .elements_row_major_iter()
        .filter(|&visited| !visited.is_empty())
        .count();

    println!("part 1: {}", result);

    //-------------------part 2------------------------
    println!("{}", repeat('-').take(arr.row_len()).collect::<String>());

    //try placing obstacle in each visited position, and check if it creates a loop
    let result = visited_arr
        .indices_row_major()
        //no need to try put new obstacle at an unreached positions from initial partol
        .filter(|&(row, column)| !visited_arr.get(row, column).unwrap().is_empty())
        .filter_map(|new_obstacle_position| {
            //place new obstacle in cloned arr
            let mut arr_with_new_obstacle = arr.clone();
            arr_with_new_obstacle
                .set(
                    new_obstacle_position.0,
                    new_obstacle_position.1,
                    IN_OBSTACLE_CHAR,
                )
                .unwrap();

            //check if causes loop
            let patrol_result = patrol::patrol(
                &mut arr_with_new_obstacle,
                start_position,
                start_direction,
                step_fn,
            );

            match patrol_result.0 {
                //filter only for obstacles which caused a loop
                PatrolOutcome::Loop => Some((new_obstacle_position, patrol_result.1)),
                _ => None,
            }
        })
        .inspect(|(new_obstacle_position, new_visited_directions)| {
            //create display arr
            let mut display_arr = arr.clone();

            display_movement(&mut display_arr, &new_visited_directions);

            //mark new obstacle position
            display_arr
                .set(
                    new_obstacle_position.0,
                    new_obstacle_position.1,
                    OUT_NEW_OBSTACLE,
                )
                .unwrap();

            //show start position
            display_arr
                .set(start_position.0, start_position.1, IN_START_CHAR)
                .unwrap();

            println!(
                "possible loop by placing new obstacle at (row {}, column {}):\n{}",
                new_obstacle_position.0,
                new_obstacle_position.1,
                arr_to_str(&display_arr, |c| *c)
            )
        })
        .count();

    println!("part 2: {}", result)
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

fn display_movement(
    display_arr: &mut Array2D<char>,
    visit_arr: &Array2D<BitFlags<VisitDirections, u8>>,
) {
    //mark visited positions with movement indicators
    for index in visit_arr.indices_row_major() {
        let directions = visit_arr.get(index.0, index.1).unwrap();
        if directions.is_empty() {
            continue;
        }
        let visualize_movement_char = {
            let is_vertical = directions.contains(VisitDirections::Up)
                | directions.contains(VisitDirections::Down);
            let is_horizontal = directions.contains(VisitDirections::Left)
                | directions.contains(VisitDirections::Right);
            match (is_vertical, is_horizontal) {
                (true, true) => OUT_MOVEMENT_HORIZONTAL_AND_VERTICAL,
                (true, false) => OUT_MOVEMENT_VERTICAL,
                (false, true) => OUT_MOVEMENT_HORIZONTAL,
                _ => unreachable!(),
            }
        };
        display_arr
            .set(index.0, index.1, visualize_movement_char)
            .unwrap();
    }
}
