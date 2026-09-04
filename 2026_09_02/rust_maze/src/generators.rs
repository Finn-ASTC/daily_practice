mod dfs;
mod kruskal;
mod prim;

pub enum Generator {
    Dfs,
    Kruskal,
    Prim,
}

pub fn generate(generator: Generator, maze: &mut super::maze::Maze) {
    match generator {
        Generator::Dfs => dfs::dfs(maze),
        Generator::Kruskal => kruskal::kruskal(maze),
        Generator::Prim => prim::prim(maze),
    }
}
