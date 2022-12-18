use std::{
    fs::File,
    io::{BufRead, BufReader},
};

trait Index2D {
    type Output;

    fn index2d(&self, y: usize, x: usize) -> Option<&Self::Output>;
}

impl<T> Index2D for Vec<Vec<T>> {
    type Output = T;

    fn index2d(&self, y: usize, x: usize) -> Option<&T> {
        self.get(y)?.get(x)
    }
}

#[derive(Debug)]
struct Tree {
    height: u8,
    visible: bool,
}

impl Tree {
    fn from_char(input: char) -> Option<Self> {
        if !(input.is_ascii_alphanumeric() & !input.is_ascii_alphabetic()) {
            return None;
        }

        //input is ASCII numeric

        let height = (input as u8) - ('0' as u8);

        Some(Tree {
            height,
            visible: false,
        })
    }
}

fn mark_visible(trees: &mut Vec<Vec<Tree>>, i: usize, j: usize, num_visible: &mut usize) {
    *num_visible += 1;
    trees.get_mut(i).unwrap().get_mut(j).unwrap().visible = true;
}

fn check_visibilty_x(
    trees: &Vec<Vec<Tree>>,
    height: u8,
    start: usize,
    end: usize,
    row_index: usize,
) -> bool {
    for i in start..end {
        if trees.index2d(row_index, i).unwrap().height >= height {
            return false;
        }
    }
    true
}

fn check_visibilty_y(
    trees: &Vec<Vec<Tree>>,
    height: u8,
    start: usize,
    end: usize,
    column_index: usize,
) -> bool {
    for i in start..end {
        if trees.index2d(i, column_index).unwrap().height >= height {
            return false;
        }
    }
    true
}

fn is_visible(trees: &mut Vec<Vec<Tree>>, row_index: usize, column_index: usize) -> bool {
    let height = trees.index2d(row_index, column_index).unwrap().height;

    let visible_from_left = check_visibilty_x(trees, height, 0, column_index, row_index);
    let visible_from_right =
        check_visibilty_x(trees, height, column_index + 1, trees.len(), row_index);

    let visible_from_top = check_visibilty_y(trees, height, 0, row_index, column_index);
    let visible_from_bottom =
        check_visibilty_y(trees, height, row_index + 1, trees[0].len(), column_index);

    visible_from_left | visible_from_right | visible_from_top | visible_from_bottom
}

fn count_visible_x<R: IntoIterator<Item = usize>>(
    trees: &Vec<Vec<Tree>>,
    height: u8,
    range: R,
    row_index: usize,
) -> usize {
    let mut count = 0;

    for i in range {
        count += 1;

        if trees.index2d(row_index, i).unwrap().height >= height {
            break;
        }
    }
    count
}

fn count_visible_y<R: IntoIterator<Item = usize>>(
    trees: &Vec<Vec<Tree>>,
    height: u8,
    range: R,
    column_index: usize,
) -> usize {
    let mut count = 0;

    for i in range {
        count += 1;
        if trees.index2d(i, column_index).unwrap().height >= height {
            break;
        }
    }

    count
}

fn get_scenic_score(trees: &Vec<Vec<Tree>>, row_index: usize, column_index: usize) -> usize {
    let height = trees.index2d(row_index, column_index).unwrap().height;

    let left_score = count_visible_x(trees, height, (0..column_index).rev(), row_index);
    let right_score = count_visible_x(trees, height, (column_index + 1)..trees.len(), row_index);

    let top_score = count_visible_y(trees, height, (0..row_index).rev(), column_index);
    let bottom_score =
        count_visible_y(trees, height, (row_index + 1)..trees[0].len(), column_index);

    left_score * right_score * top_score * bottom_score
}

fn main() {
    let file = BufReader::new(File::open("./task.txt").expect("Can open input file"));

    let lines = file.lines().flatten().collect::<Vec<_>>();

    let mut trees = lines
        .iter()
        .map(|line| {
            line.chars()
                .flat_map(|c| Tree::from_char(c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let num_rows = trees.len();
    let num_columns = trees[0].len();

    let mut num_visible = 0;
    let mut max_score = 0;
    for row_index in 0..num_rows {
        for column_index in 0..num_columns {
            let score = get_scenic_score(&trees, row_index, column_index);
            max_score = usize::max(max_score, score);

            if ((row_index == 0) | (row_index == num_rows - 1))
                | ((column_index == 0) | (column_index == num_columns - 1))
            {
                mark_visible(&mut trees, row_index, column_index, &mut num_visible);
                continue;
            }

            if is_visible(&mut trees, row_index, column_index) {
                mark_visible(&mut trees, row_index, column_index, &mut num_visible);
            }
        }
    }

    for line in &trees {
        for tree in line {
            let color = if tree.visible { "32" } else { "31" };
            print!("\x1b[{}m{}\x1b[0m ", color, tree.height);
        }
        println!("");
    }
    println!(
        "{} trees are visible. The max scenic score is {}",
        num_visible, max_score
    );
}
