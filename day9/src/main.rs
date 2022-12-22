use std::collections::HashSet;
use std::fs;

use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::PointMarker;
use plotlib::style::PointStyle;
use plotlib::view::ContinuousView;
use plotlib::view::View;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(direction_char: char) -> Direction {
        if direction_char == 'U' {
            return Direction::Up;
        } else if direction_char == 'D' {
            return Direction::Down;
        } else if direction_char == 'L' {
            return Direction::Left;
        } else if direction_char == 'R' {
            return Direction::Right;
        } else {
            panic!("invalid direction char specified");
        }
    }
}

struct Motion {
    direction: Direction,
    steps: u32,
}

impl Motion {
    fn new(motion_vec: &Vec<&str>) -> Motion {
        let direction_char = motion_vec[0].parse::<char>().unwrap();
        let steps = motion_vec[1].parse::<u32>().unwrap();
        return Motion {
            direction: Direction::new(direction_char),
            steps,
        };
    }
}

#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32,
}

impl Pair {
    fn new() -> Pair {
        return Pair { x: 0, y: 0 };
    }
}

fn tail_touching_head(head: &Pair, tail: &Pair) -> bool {
    // touching if abs diff hx tx <= 1 and abs diff hy ty <= 1
    let delta_x = (head.x - tail.x).abs();
    let delta_y = (head.y - tail.y).abs();
    if delta_x < 2 && delta_y < 2 {
        return true;
    }
    false
}

fn move_head(head: &Pair, direction: &Direction) -> Pair {
    let mut new_head = Pair {
        x: head.x,
        y: head.y,
    };
    if *direction == Direction::Left {
        new_head.x -= 1;
    } else if *direction == Direction::Right {
        new_head.x += 1;
    } else if *direction == Direction::Down {
        new_head.y -= 1;
    } else {
        new_head.y += 1;
    }
    new_head
}

fn take_step(head: &Pair, tail: &Pair, direction: &Direction) -> (Pair, Pair) {
    let mut new_tail = Pair {
        x: tail.x,
        y: tail.y,
    };
    let new_head = move_head(head, direction);
    // if tail no longer touching head then update tail
    if !tail_touching_head(&new_head, &new_tail) {
        // println!("Head not touching Tail - Head at: {:?}, Tail at: {:?}", head, tail);
        new_tail.x = head.x;
        new_tail.y = head.y;
    }
    return (new_head, new_tail);
}

fn get_delta(tail: &Pair, head: &Pair) -> (i32, i32) {
    let delta_x = (head.x - tail.x).abs();
    let delta_y = (head.y - tail.y).abs();
    (delta_x, delta_y)
}

fn get_difference(tail: &Pair, head: &Pair) -> (i32, i32) {
    let difference_x = head.x - tail.x;
    let difference_y = head.y - tail.y;
    (difference_x, difference_y)
}

fn move_tail(tail: &Pair, head: &Pair) -> Pair {
    let mut new_tail = Pair {
        x: tail.x,
        y: tail.y,
    };
    let (delta_x, delta_y) = get_delta(tail, head);
    let (diff_x, diff_y) = get_difference(tail, head);
    if delta_x > 0 && delta_y > 0 {
        // diaganol move
        if diff_x.is_negative() {
            new_tail.x -= 1;
        } else {
            new_tail.x += 1;
        }

        if diff_y.is_negative() {
            new_tail.y -= 1;
        } else {
            new_tail.y += 1;
        }
    } else {
        // horizontal move
        if delta_x > 0 {
            if diff_x.is_negative() {
                new_tail.x -= 1;
            } else {
                new_tail.x += 1;
            }
        } else {
            if diff_y.is_negative() {
                new_tail.y -= 1;
            } else {
                new_tail.y += 1;
            }
        }
    }
    new_tail
}

fn take_step_v2(rope: &mut Vec<Pair>, direction: &Direction) {
    for idx in (0..rope.len()).rev() {
        if idx == rope.len() - 1 {
            rope[idx] = move_head(&rope[idx], direction);
        } else {
            if !tail_touching_head(&rope[idx + 1], &rope[idx]) {
                rope[idx] = move_tail(&rope[idx], &rope[idx + 1]);
            }
        }
    }
}

fn part_one(motions: &Vec<Motion>) {
    // need an x/y plane where we start at (0, 0) so indices can be negative & positive
    // hashset where each entry is an x/y coordinate pair
    // on each movement the head and tail coordinates get updated

    // add the new tail coordinate to the locations hashset
    // answer is the size of the hashset

    // how do we move?
    // x = 0, y = 0 for both head and tail
    // add x,y to hashset
    // function which takes in hx,hy,tx,ty,direction and returns new hx,hy,tx,ty
    // add new tx,ty pair to hashset
    let mut head = Pair::new();
    let mut tail = Pair::new();
    let mut unique_tail_locations: HashSet<(i32, i32)> = HashSet::new();
    for motion in motions {
        for _ in 0..motion.steps {
            (head, tail) = take_step(&head, &tail, &motion.direction);
            unique_tail_locations.insert((tail.x, tail.y));
        }
    }
    println!("Part One Ans: {}", unique_tail_locations.len());
}

fn generate_tail_plot(unique_tail_locations: &HashSet<(i32, i32)>) {
    let x_min: f64 = -300.0;
    let x_max: f64 = 200.0;
    let y_min: f64 = -300.0;
    let y_max: f64 = 100.0;

    let mut coord_vec: Vec<(f64, f64)> = vec![];
    for tail_coord in unique_tail_locations {
        let coord = (tail_coord.0 as f64, tail_coord.1 as f64);
        coord_vec.push(coord);
    }
    let my_plot = Plot::new(coord_vec).point_style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );
    let mut v = ContinuousView::new()
        .add(my_plot)
        .x_range(x_min, x_max)
        .y_range(y_min, y_max);

    v.add_grid(Grid::new(10, 10));
    Page::single(&v).save("scatter.svg").unwrap();
}

fn part_two(motions: &Vec<Motion>) {
    let mut rope: Vec<Pair> = vec![];
    let mut unique_tail_locations: HashSet<(i32, i32)> = HashSet::new();
    for _ in 0..10 {
        rope.push(Pair::new());
    }
    for motion in motions {
        for _ in 0..motion.steps {
            take_step_v2(&mut rope, &motion.direction);
            // println!("Rope: {:?}", rope);
            unique_tail_locations.insert((rope[0].x, rope[0].y));
        }
    }
    generate_tail_plot(&unique_tail_locations);
    println!("Part Two Ans: {}", unique_tail_locations.len());
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt").unwrap();
    let motions_input: Vec<&str> = input_str.split("\n").collect();
    let mut motions: Vec<Motion> = vec![];
    for motion in motions_input {
        let tmp_motions: Vec<&str> = motion.split_whitespace().collect();
        motions.push(Motion::new(&tmp_motions));
    }
    part_one(&motions);
    part_two(&motions);
}
