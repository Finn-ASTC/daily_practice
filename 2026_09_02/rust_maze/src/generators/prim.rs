use crate::maze::{Direction, MapPos, Maze, ROAD, RoadPos};
use strum::IntoEnumIterator;

pub fn prim(maze: &mut Maze) {
    let mut frontier: Vec<(MapPos, RoadPos)> = Vec::new();

    let mut current = maze.get_random_road_pos();
    maze.set_road(current, ROAD);

    loop {
        for direction in Direction::iter() {
            let Some(neighbor) = maze.neighbor_road_pos(current, direction) else {
                continue;
            };

            if maze.get_road(neighbor) == Some(ROAD) {
                if let Some(wall) = maze.get_wall_by_the_road(current, direction)
                    && let Some(idx) = frontier.iter().position(|p| p.0 == wall)
                {
                    frontier.remove(idx);
                }
            } else {
                if let Some(wall) = maze.get_wall_by_the_road(current, direction) {
                    frontier.push((wall, neighbor));
                }
            }
        }

        if frontier.is_empty() {
            break;
        }

        let (wall, next) = frontier[rand::random_range(..frontier.len())];
        maze.set_pos(wall, ROAD);
        maze.set_road(next, ROAD);
        current = next;
    }

    let start = maze.get_random_road_pos();
    maze.set_start(start);
    let mut end = start;
    while end == start {
        end = maze.get_random_road_pos();
    }
    maze.set_end(end);
}
