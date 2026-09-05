mod a_star;
mod bfs;
mod dfs_solver;

use crate::maze::Maze;

pub enum Solver {
    Bfs,
    Dfs,
    AStar,
}

pub fn solve(solver: Solver, maze: &mut Maze) {
    match solver {
        Solver::Bfs => bfs::bfs(maze),
        Solver::Dfs => dfs_solver::dfs(maze),
        Solver::AStar => a_star::a_star(maze),
    }
}
