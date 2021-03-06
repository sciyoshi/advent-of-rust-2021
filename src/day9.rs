use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn flood(grid: &Vec<Vec<u32>>, pt: (usize, usize)) -> usize {
    let mut stack = vec![pt];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    while let Some(p) = stack.pop() {
        seen.insert(p);

        if p.0 > 0
            && !seen.contains(&(p.0 - 1, p.1))
            && grid[p.0][p.1] < grid[p.0 - 1][p.1]
            && grid[p.0 - 1][p.1] < 9
        {
            stack.push((p.0 - 1, p.1));
        }

        if p.1 > 0
            && !seen.contains(&(p.0, p.1 - 1))
            && grid[p.0][p.1] < grid[p.0][p.1 - 1]
            && grid[p.0][p.1 - 1] < 9
        {
            stack.push((p.0, p.1 - 1));
        }

        if p.0 < grid.len() - 1
            && !seen.contains(&(p.0 + 1, p.1))
            && grid[p.0][p.1] < grid[p.0 + 1][p.1]
            && grid[p.0 + 1][p.1] < 9
        {
            stack.push((p.0 + 1, p.1));
        }

        if p.1 < grid[0].len() - 1
            && !seen.contains(&(p.0, p.1 + 1))
            && grid[p.0][p.1] < grid[p.0][p.1 + 1]
            && grid[p.0][p.1 + 1] < 9
        {
            stack.push((p.0, p.1 + 1));
        }
    }

    seen.len()
}

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let grid: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;
    let mut basins = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if i > 0 && grid[i - 1][j] <= col
                || j > 0 && grid[i][j - 1] <= col
                || i < grid.len() - 1 && grid[i + 1][j] <= col
                || j < row.len() - 1 && grid[i][j + 1] <= col
            {
                continue;
            }

            part1 += 1 + col;

            basins.push(flood(&grid, (i, j)));
        }
    }

    basins.sort();

    println!("[Part 1] {:?}", part1);
    println!(
        "[Part 2] {:?}",
        basins[basins.len() - 3..].iter().product::<usize>()
    );
}
