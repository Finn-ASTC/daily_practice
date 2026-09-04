use image::{Rgb, RgbImage};

pub const ROAD: usize = 0;
pub const WALL: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MapPos(pub usize, pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RoadPos(pub usize, pub usize);

pub struct Maze {
    maze_map: Vec<Vec<usize>>,
    start: RoadPos,
    end: RoadPos,
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, Debug, strum::EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_offset(self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        assert!(height >= 9 && width >= 9, "maze size must be at least 9");

        let height = if height.is_multiple_of(2) {
            height + 1
        } else {
            height
        };
        let width = if width.is_multiple_of(2) {
            width + 1
        } else {
            width
        };

        Self {
            maze_map: vec![vec![WALL; width]; height],
            start: RoadPos(1, 1),
            end: RoadPos((width - 1) / 2, (height - 1) / 2),
            height,
            width,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn start(&self) -> RoadPos {
        self.start
    }

    pub fn set_start(&mut self, pos: RoadPos) {
        self.start = pos;
    }

    pub fn end(&self) -> RoadPos {
        self.end
    }

    pub fn set_end(&mut self, pos: RoadPos) {
        self.end = pos;
    }

    pub fn check_pos(&self, pos: MapPos) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    pub fn set_pos(&mut self, pos: MapPos, value: usize) {
        if self.check_pos(pos) && (value == ROAD || value == WALL) {
            self.maze_map[pos.1][pos.0] = value;
        }
    }

    pub fn get_pos(&self, pos: MapPos) -> Option<usize> {
        self.check_pos(pos).then(|| self.maze_map[pos.1][pos.0])
    }

    pub fn road_to_map(&self, pos: RoadPos) -> Option<MapPos> {
        if pos.0 == 0 || pos.1 == 0 {
            return None;
        }

        let map = MapPos(
            pos.0.checked_mul(2)?.checked_sub(1)?,
            pos.1.checked_mul(2)?.checked_sub(1)?,
        );
        self.check_pos(map).then_some(map)
    }

    pub fn check_road(&self, pos: RoadPos) -> bool {
        self.road_to_map(pos).is_some()
    }

    pub fn get_road(&self, pos: RoadPos) -> Option<usize> {
        self.road_to_map(pos).map(|map| self.maze_map[map.1][map.0])
    }

    pub fn set_road(&mut self, pos: RoadPos, value: usize) {
        if let Some(map) = self.road_to_map(pos)
            && (value == ROAD || value == WALL)
        {
            self.maze_map[map.1][map.0] = value;
        }
    }

    pub fn connect_road(&mut self, a: RoadPos, b: RoadPos) {
        if !self.check_road(a) || !self.check_road(b) || a.0.abs_diff(b.0) + a.1.abs_diff(b.1) != 1
        {
            return;
        }

        self.set_road(a, ROAD);
        self.set_road(b, ROAD);
        self.set_pos(MapPos(a.0 + b.0 - 1, a.1 + b.1 - 1), ROAD);
    }

    pub fn get_random_road_pos(&self) -> RoadPos {
        let max_x = (self.width - 1) / 2;
        let max_y = (self.height - 1) / 2;
        RoadPos(rand::random_range(1..=max_x), rand::random_range(1..=max_y))
    }

    pub fn neighbor_road_pos(&self, pos: RoadPos, direction: Direction) -> Option<RoadPos> {
        let (dx, dy) = direction.as_offset();
        let neighbor = RoadPos(pos.0.checked_add_signed(dx)?, pos.1.checked_add_signed(dy)?);
        self.check_road(neighbor).then_some(neighbor)
    }

    pub fn get_wall_by_the_road(&self, pos: RoadPos, direction: Direction) -> Option<MapPos> {
        let neighbor = self.neighbor_road_pos(pos, direction)?;
        let current_map = self.road_to_map(pos)?;
        let neighbor_map = self.road_to_map(neighbor)?;
        Some(MapPos(
            (current_map.0 + neighbor_map.0) / 2,
            (current_map.1 + neighbor_map.1) / 2,
        ))
    }

    pub fn render(&self, path: &str, cell_size: u32) -> image::ImageResult<()> {
        assert!(cell_size > 0, "cell_size must be positive");

        let mut image = RgbImage::new(
            self.width as u32 * cell_size,
            self.height as u32 * cell_size,
        );

        let start_pos = self.road_to_map(self.start).expect("start must be valid");
        let end_pos = self.road_to_map(self.end).expect("end must be valid");

        for y in 0..self.height {
            for x in 0..self.width {
                let color = if MapPos(x, y) == start_pos {
                    Rgb([0, 220, 0])
                } else if MapPos(x, y) == end_pos {
                    Rgb([220, 0, 0])
                } else if self.get_pos(MapPos(x, y)) == Some(ROAD) {
                    Rgb([255, 255, 255])
                } else {
                    Rgb([0, 0, 0])
                };

                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        image.put_pixel(
                            x as u32 * cell_size + dx,
                            y as u32 * cell_size + dy,
                            color,
                        );
                    }
                }
            }
        }

        image.save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, MapPos, Maze, RoadPos};

    #[test]
    fn maze_uses_usize_dimensions_and_positions() {
        let maze = Maze::new(9usize, 9usize);

        let _: usize = maze.width;
        let _: usize = maze.height;
        let _: RoadPos = maze.start();
        let _: RoadPos = maze.end();
    }

    #[test]
    fn direction_offsets_use_isize() {
        let _: (isize, isize) = Direction::Up.as_offset();
        assert_eq!(Direction::Up.as_offset(), (0, -1));
        assert_eq!(Direction::Down.as_offset(), (0, 1));
        assert_eq!(Direction::Left.as_offset(), (-1, 0));
        assert_eq!(Direction::Right.as_offset(), (1, 0));
    }

    #[test]
    #[should_panic(expected = "maze size must be at least 9")]
    fn maze_rejects_height_below_nine() {
        Maze::new(8, 9);
    }

    #[test]
    #[should_panic(expected = "maze size must be at least 9")]
    fn maze_rejects_width_below_nine() {
        Maze::new(9, 8);
    }

    #[test]
    fn maze_rounds_even_dimensions_up_to_odd() {
        let maze = Maze::new(10, 12);

        assert_eq!(maze.height, 11);
        assert_eq!(maze.width, 13);
    }

    #[test]
    fn direction_at_boundary_returns_no_neighbor() {
        let maze = Maze::new(9, 9);

        assert_eq!(maze.neighbor_road_pos(RoadPos(1, 1), Direction::Up), None);
        assert_eq!(maze.neighbor_road_pos(RoadPos(1, 1), Direction::Left), None);
    }

    #[test]
    fn get_wall_by_road_returns_wall_position() {
        let maze = Maze::new(9, 9);

        assert_eq!(
            maze.get_wall_by_the_road(RoadPos(1, 1), Direction::Right),
            Some(MapPos(2, 1))
        );
    }

    #[test]
    fn get_wall_by_road_returns_none_at_boundary() {
        let maze = Maze::new(9, 9);

        assert_eq!(
            maze.get_wall_by_the_road(RoadPos(1, 1), Direction::Up),
            None
        );
        assert_eq!(
            maze.get_wall_by_the_road(RoadPos(1, 1), Direction::Left),
            None
        );
    }
}
