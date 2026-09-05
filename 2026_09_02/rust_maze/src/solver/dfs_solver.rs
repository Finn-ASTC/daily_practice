use crate::maze::{Direction, Maze, ROAD, RoadPos};
use strum::IntoEnumIterator;

fn road_id(pos: RoadPos, max_x: usize) -> usize {
    (pos.1 - 1) * max_x + (pos.0 - 1)
}

fn id_to_road(id: usize, max_x: usize) -> RoadPos {
    RoadPos(id % max_x + 1, id / max_x + 1)
}

fn open_neighbors(maze: &Maze, pos: RoadPos) -> Vec<RoadPos> {
    Direction::iter()
        .filter_map(|dir| {
            let neighbor = maze.neighbor_road_pos(pos, dir)?;
            let wall = maze.get_wall_by_the_road(pos, dir)?;
            (maze.get_pos(wall) == Some(ROAD)).then_some(neighbor)
        })
        .collect()
}

pub fn dfs(maze: &mut Maze) {
    let max_x = (maze.width() - 1) / 2;
    let max_y = (maze.height() - 1) / 2;
    let n = max_x * max_y;

    let start = maze.start();
    let end = maze.end();
    let start_id = road_id(start, max_x);
    let end_id = road_id(end, max_x);

    let mut parent: Vec<Option<usize>> = vec![None; n];
    parent[start_id] = Some(start_id);
    let mut stack: Vec<usize> = Vec::new();
    stack.push(start_id);

    while let Some(cur_id) = stack.pop() {
        if cur_id == end_id {
            break;
        }
        let cur = id_to_road(cur_id, max_x);
        for neighbor in open_neighbors(maze, cur) {
            let nid = road_id(neighbor, max_x);
            if parent[nid].is_none() {
                parent[nid] = Some(cur_id);
                stack.push(nid);
            }
        }
    }

    if parent[end_id].is_none() {
        return;
    }

    let visited: Vec<RoadPos> = (0..n)
        .filter(|&id| parent[id].is_some())
        .map(|id| id_to_road(id, max_x))
        .collect();
    maze.mark_visited(&visited);

    let mut path = Vec::new();
    let mut id = end_id;
    while id != start_id {
        path.push(id_to_road(id, max_x));
        id = parent[id].expect("路径上的格子必有 parent");
    }
    path.push(start);
    path.reverse();

    maze.mark_path(&path);
}
