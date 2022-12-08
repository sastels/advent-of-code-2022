pub fn parse_forest(data: &[String]) -> Vec<Vec<u32>> {
    data.iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn is_visible(y0: usize, x0: usize, forest: &[Vec<u32>]) -> bool {
    if x0 == 0 || x0 == forest[0].len() - 1 || y0 == 0 || y0 == forest.len() - 1 {
        return true;
    }
    let tree_height = forest[x0][y0];
    ((0..(x0)).filter(|x| forest[*x][y0] >= tree_height).count() == 0)
        || ((x0 + 1..forest[0].len())
            .filter(|x| forest[*x][y0] >= tree_height)
            .count()
            == 0)
        || ((0..(y0)).filter(|y| forest[x0][*y] >= tree_height).count() == 0)
        || ((y0 + 1..forest.len())
            .filter(|y| forest[x0][*y] >= tree_height)
            .count()
            == 0)
}

pub fn scene_score(y0: usize, x0: usize, forest: &[Vec<u32>]) -> usize {
    let mut score = 1;
    let width = forest[0].len();
    let height = forest.len();
    let tree_height = forest[x0][y0];

    let mut direction_score = 0;
    for x in (0..x0).rev() {
        direction_score += 1;
        if forest[x][y0] >= tree_height {
            break;
        }
    }
    score *= direction_score;

    direction_score = 0;
    for x in (x0 + 1)..width {
        direction_score += 1;
        if forest[x][y0] >= tree_height {
            break;
        }
    }
    score *= direction_score;

    direction_score = 0;
    for y in (0..y0).rev() {
        direction_score += 1;
        if forest[x0][y] >= tree_height {
            break;
        }
    }
    score *= direction_score;

    direction_score = 0;
    for y in (y0 + 1)..height {
        direction_score += 1;
        if forest[x0][y] >= tree_height {
            break;
        }
    }
    score *= direction_score;

    score
}

pub fn solve_a(data: &[String]) -> usize {
    let forest = parse_forest(data);
    let width = forest[0].len();
    let height = forest.len();
    let mut trees_visible = 0;
    for x in 0..width {
        for y in 0..height {
            if is_visible(x, y, &forest) {
                trees_visible += 1;
            }
        }
    }
    trees_visible
}

pub fn solve_b(data: &[String]) -> usize {
    let forest = parse_forest(data);
    let mut max_score = 0;
    let width = forest[0].len();
    let height = forest.len();
    for x in 0..width {
        for y in 0..height {
            let score = scene_score(x, y, &forest);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}
