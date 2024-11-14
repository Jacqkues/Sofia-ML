mod dfs;
mod dfs_rec;
use dfs::solve_dfs;
use dfs_rec::solve_dfs_rec;
use dfs_rec::solve_dfs_iter;

fn main() {
    //solve_dfs(20);
    //solve_dfs_rec(20);
    solve_dfs_iter(21);
}