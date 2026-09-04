use crate::maze::{MapPos, Maze, RoadPos};
use rand::seq::SliceRandom;

fn get_all_walls(maze: &Maze) -> Vec<MapPos> {
    let mut walls: Vec<MapPos> = Vec::new();

    for x in 1..maze.width() - 1 {
        for y in 1..maze.height() - 1 {
            if (x + y) % 2 == 1 {
                walls.push(MapPos(x, y));
            }
        }
    }

    walls
}

fn get_roads_by_wall(wall: MapPos) -> (RoadPos, RoadPos) {
    if wall.0.is_multiple_of(2) {
        (
            RoadPos(wall.0 / 2, wall.1.div_ceil(2)),
            RoadPos(wall.0 / 2 + 1, wall.1.div_ceil(2)),
        )
    } else {
        (
            RoadPos(wall.0.div_ceil(2), wall.1 / 2),
            RoadPos(wall.0.div_ceil(2), wall.1 / 2 + 1),
        )
    }
}

fn road_id(pos: RoadPos, max_x: usize) -> usize {
    (pos.1 - 1) * max_x + (pos.0 - 1)
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else {
            self.parent[rb] = ra;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
        }
    }
}

pub fn kruskal(maze: &mut Maze) {
    let max_x = (maze.width() - 1) / 2;
    let max_y = (maze.height() - 1) / 2;
    let mut uf = UnionFind::new(max_x * max_y);

    let mut walls = get_all_walls(maze);
    walls.shuffle(&mut rand::rng());

    for wall in walls {
        let (r1, r2) = get_roads_by_wall(wall);
        let (id1, id2) = (road_id(r1, max_x), road_id(r2, max_x));

        if uf.find(id1) != uf.find(id2) {
            maze.connect_road(r1, r2);
            uf.union(id1, id2);
        }
    }

    let start = maze.get_random_road_pos();
    maze.set_start(start);
    let mut end = start;
    while end == start {
        end = maze.get_random_road_pos();
    }
    maze.set_end(end);
}

#[cfg(test)]
mod tests {
    use super::kruskal;
    use crate::maze::{Maze, ROAD, RoadPos};

    #[test]
    fn kruskal_carves_every_road_cell() {
        let mut maze = Maze::new(9, 9);
        kruskal(&mut maze);

        let max_x = (maze.width() - 1) / 2;
        let max_y = (maze.height() - 1) / 2;
        for x in 1..=max_x {
            for y in 1..=max_y {
                assert_eq!(maze.get_road(RoadPos(x, y)), Some(ROAD),);
            }
        }
        assert_ne!(maze.start(), maze.end());
    }
}
