use crate::maze::{Direction, Maze, ROAD, RoadPos};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

fn heuristic(a: RoadPos, b: RoadPos) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn a_star(maze: &mut Maze) {
    let max_x = (maze.width() - 1) / 2;
    let max_y = (maze.height() - 1) / 2;
    let n = max_x * max_y;

    let start = maze.start();
    let end = maze.end();
    let start_id = road_id(start, max_x);
    let end_id = road_id(end, max_x);

    let mut g: Vec<usize> = vec![usize::MAX; n];
    let mut parent: Vec<Option<usize>> = vec![None; n];
    g[start_id] = 0;
    parent[start_id] = Some(start_id);

    let mut open: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    open.push(Reverse((heuristic(start, end), start_id)));

    while let Some(Reverse((f, cur_id))) = open.pop() {
        let cur = id_to_road(cur_id, max_x);
        if f > g[cur_id] + heuristic(cur, end) {
            continue;
        }

        if cur_id == end_id {
            break;
        }

        for neighbor in open_neighbors(maze, cur) {
            let nid = road_id(neighbor, max_x);
            let new_g = g[cur_id] + 1;
            if new_g < g[nid] {
                g[nid] = new_g;
                parent[nid] = Some(cur_id);
                open.push(Reverse((new_g + heuristic(neighbor, end), nid)));
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
