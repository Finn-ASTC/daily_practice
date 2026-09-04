use crate::maze::{Direction, Maze, ROAD, RoadPos, WALL};
use strum::IntoEnumIterator;

pub fn dfs(maze: &mut Maze) {
    let mut stack: Vec<RoadPos> = Vec::new();
    let mut current = maze.get_random_road_pos();

    stack.push(current);

    while !stack.is_empty() {
        let directions: Vec<Direction> = Direction::iter()
            .filter(|&direction| {
                maze.neighbor_road_pos(current, direction)
                    .and_then(|next| maze.get_road(next))
                    == Some(WALL)
            })
            .collect();

        if directions.is_empty() {
            if let Some(pos) = stack.pop() {
                current = pos;
            }
            continue;
        }

        let direction = directions[rand::random_range(..directions.len())];
        let next = maze
            .neighbor_road_pos(current, direction)
            .expect("direction was selected from a valid neighbor");

        maze.set_road(next, ROAD);
        maze.connect_road(current, next);
        current = next;
        stack.push(current);
    }

    let start = maze.get_random_road_pos();
    maze.set_start(start);
    let mut end = start;
    while end == start {
        end = maze.get_random_road_pos();
    }
    maze.set_end(end);
}
