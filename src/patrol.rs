use array2d::Array2D;
use enum_map::{enum_map, Enum};
use enumflags2::BitFlags;

pub enum PatrolStep {
    Face(Direction),
    StepForward,
}
pub enum PatrolOutcome {
    LeftBounds,
    Loop,
}

#[derive(Debug, Enum, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VisitDirections {
    Up = 0b0001,
    Right = 0b0010,
    Down = 0b0100,
    Left = 0b1000,
}

pub fn patrol<T, F>(
    arr: &Array2D<T>,
    mut position: (usize, usize),
    mut direction: Direction,
    step: F,
) -> (PatrolOutcome, Array2D<BitFlags<VisitDirections, u8>>)
where
    F: Fn(&T, Direction) -> PatrolStep,
{
    let mut visited_arr: Array2D<BitFlags<VisitDirections, _>> =
        Array2D::filled_with(BitFlags::default(), arr.num_rows(), arr.num_columns());

    let directions = enum_map! {
        Direction::Up => (-1,0),
        Direction::Down => (1,0),
        Direction::Left => (0,-1),
        Direction::Right => (0,1)
    };

    let visit_directions_map = enum_map! {
        Direction::Up => VisitDirections::Up,
        Direction::Down => VisitDirections::Down,
        Direction::Left => VisitDirections::Left,
        Direction::Right => VisitDirections::Right
    };

    loop {
        //if reached the same position with the same direction as previously visited -> loop found
        let previously_visited_directions = visited_arr.get_mut(position.0, position.1).unwrap();

        let visit_direction = visit_directions_map[direction];
        if previously_visited_directions.contains(visit_direction) {
            return (PatrolOutcome::Loop, visited_arr);
        }

        //mark current position as visited with current direction
        previously_visited_directions.insert(visit_direction);

        let d = directions[direction];
        let next_step_position = match (
            position.0.checked_add_signed(d.0),
            position.1.checked_add_signed(d.1),
        ) {
            (Some(new_row), Some(new_column)) => (new_row, new_column),
            _ => return (PatrolOutcome::LeftBounds, visited_arr),
        };

        let next_step_cell = match arr.get(next_step_position.0, next_step_position.1) {
            None => return (PatrolOutcome::LeftBounds, visited_arr),
            Some(c) => c,
        };

        match step(next_step_cell, direction) {
            PatrolStep::Face(d) => direction = d,
            PatrolStep::StepForward => position = next_step_position,
        }
    }
}
