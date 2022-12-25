use log::{debug, info, Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::collections::{HashSet, VecDeque};
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

#[derive(Debug, PartialEq, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Vertex {
    x: usize,
    y: usize,
    height: u8,
}

fn get_adjacent_vertex(
    map: &Vec<Vec<u8>>,
    vertex: &Vertex,
    direction: Direction,
) -> Option<Vertex> {
    let mut new_vertex: Option<Vertex> = None;
    if direction == Direction::Left && vertex.x > 0 {
        new_vertex = Some(Vertex {
            x: vertex.x - 1,
            y: vertex.y,
            height: map[vertex.y][vertex.x - 1],
        });
    } else if direction == Direction::Right && vertex.x < map[vertex.y].len() - 1 {
        new_vertex = Some(Vertex {
            x: vertex.x + 1,
            y: vertex.y,
            height: map[vertex.y][vertex.x + 1],
        });
    } else if direction == Direction::Up && vertex.y > 0 {
        new_vertex = Some(Vertex {
            x: vertex.x,
            y: vertex.y - 1,
            height: map[vertex.y - 1][vertex.x],
        });
    } else if direction == Direction::Down && vertex.y < map.len() - 1 {
        new_vertex = Some(Vertex {
            x: vertex.x,
            y: vertex.y + 1,
            height: map[vertex.y + 1][vertex.x],
        });
    }

    // too high to be adjacent
    if new_vertex.is_some() && new_vertex.as_ref().unwrap().height > vertex.height + 1 {
        new_vertex = None;
    }
    new_vertex
}

fn find_adjacent_vertices(map: &Vec<Vec<u8>>, vertex: &Vertex) -> Vec<Vertex> {
    let mut adjacent_vertices: Vec<Vertex> = vec![];
    for direction in Direction::iter() {
        // debug!("Looking for vertex {:?}", direction);
        let adjacent_vertex = get_adjacent_vertex(map, &vertex, direction);
        if adjacent_vertex.is_some() {
            // debug!("Found Vertex");
            adjacent_vertices.push(adjacent_vertex.unwrap());
        }
    }
    adjacent_vertices
}

fn bfs(map: &Vec<Vec<u8>>, start_vertex: &Vertex, end_vertex: &Vertex) -> u64 {
    let mut work_queue: VecDeque<(Vertex, u64)> = VecDeque::new();
    let mut visited: HashSet<Vertex> = HashSet::new();
    work_queue.push_front((*start_vertex, 0));
    visited.insert(*start_vertex);
    while let Some((next_vertex, dist)) = work_queue.pop_back() {
        debug!("Visited vertices: {}", visited.len());
        if next_vertex == *end_vertex {
            return dist;
        }
        let adjacent_vertices = find_adjacent_vertices(map, &next_vertex);
        for adjacent_vertex in adjacent_vertices {
            if !visited.contains(&adjacent_vertex) {
                work_queue.push_front((adjacent_vertex, dist + 1));
                visited.insert(adjacent_vertex);
            }
        }
    }
    std::u64::MAX
}

fn part_one_v2(map: &Vec<Vec<u8>>, start_vertex: &Vertex, end_vertex: &Vertex) {
    let ans = bfs(map, start_vertex, end_vertex);
    info!("Part One Ans: {}", ans);
}

fn part_two(map: &Vec<Vec<u8>>, start_vertices: &Vec<Vertex>, end_vertex: &Vertex) {
    let mut dist_vec: Vec<u64> = vec![];
    for start_vertex in start_vertices {
        dist_vec.push(bfs(map, start_vertex, end_vertex));
    }
    dist_vec.sort();
    info!("Part Two Ans: {}", dist_vec[0]);
}

fn main() {
    init().unwrap();
    let mut map: Vec<Vec<u8>> = vec![];
    let input_str = fs::read_to_string("./src/input.txt").unwrap();
    let mut start_vertex: Vertex = Default::default();
    let mut end_vertex: Vertex = Default::default();
    let mut start_vertices: Vec<Vertex> = vec![];

    // build map and grab start and end positions
    for (row, input_line) in input_str.split("\n").enumerate() {
        map.push(vec![]);
        for (col, mut input_char) in input_line.chars().enumerate() {
            if input_char == 'S' || input_char == 'a' {
                start_vertices.push(Vertex {
                    x: col,
                    y: row,
                    height: 'a' as u8,
                });
                if input_char == 'S' {
                    start_vertex = Vertex {
                        x: col,
                        y: row,
                        height: 'a' as u8,
                    };
                    input_char = 'a';
                }
            }
            if input_char == 'E' {
                end_vertex = Vertex {
                    x: col,
                    y: row,
                    height: 'z' as u8,
                };
                input_char = 'z';
            }
            map[row].push(input_char as u8);
        }
    }
    debug!("Total Vertices: {}", map.len() * map[0].len());
    part_one_v2(&map, &start_vertex, &end_vertex);
    part_two(&map, &start_vertices, &end_vertex);
}
