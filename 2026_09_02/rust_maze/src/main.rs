mod generators;
mod maze;

use generators::Generator;
use maze::Maze;

fn main() -> image::ImageResult<()> {
    let mut maze = Maze::new(51, 51);
    generators::generate(Generator::Prim, &mut maze);
    maze.render("maze.png", 30)
}
