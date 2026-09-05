mod generators;
mod maze;
mod solver;

use generators::Generator;
use maze::Maze;
use solver::{Solver, solve};

fn main() -> image::ImageResult<()> {
    let mut maze = Maze::new(111, 111);
    generators::generate(Generator::Prim, &mut maze);

    let mut bfs = maze.clone();
    let mut dfs = maze.clone();
    let mut astar = maze.clone();

    solve(Solver::Bfs, &mut bfs);
    solve(Solver::Dfs, &mut dfs);
    solve(Solver::AStar, &mut astar);

    maze.render("maze_plain.png", 30)?;
    bfs.render("maze_bfs.png", 30)?;
    dfs.render("maze_dfs.png", 30)?;
    astar.render("maze_astar.png", 30)?;
    Ok(())
}
