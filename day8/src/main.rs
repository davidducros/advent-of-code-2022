use std::{
    fs::File,
    io::{BufRead, BufReader}, fmt::Display,
};

#[derive(PartialEq, Debug)]
enum TreeState {
    Unknown,
    Visible,
    NotVisible,
}

struct Tree {
    height: u32,
    state: TreeState,
    scenic_scores: Vec<u32>
}

struct Grid {
    rows: Vec<Vec<Tree>>,
}

impl Grid {

    fn update_visibility(&mut self) {
        for row in self.rows.iter_mut() {
            visibility_impl(row.iter_mut());
            visibility_impl(row.iter_mut().rev());
        }

        for i in 0..self.rows.get(0).unwrap().len() {
            visibility_impl(self.rows.iter_mut().map(|row| row.get_mut(i).unwrap()));
            visibility_impl(self.rows.iter_mut().rev().map(|row| row.get_mut(i).unwrap()));
        }
    }

    fn update_scenic_scores(&mut self) {
        for row in self.rows.iter_mut() {
            score_impl(row.iter_mut());
            score_impl(row.iter_mut().rev());
        }

        for i in 0..self.rows.get(0).unwrap().len() {
            score_impl(self.rows.iter_mut().map(|row| row.get_mut(i).unwrap()));
            score_impl(self.rows.iter_mut().rev().map(|row| row.get_mut(i).unwrap()));
        }
    }

    fn count_visible(&self) -> u32 {
        let mut count: u32 = 0;

        for row in self.rows.iter() {
            for tree in row.iter() {
                if tree.state == TreeState::Visible {
                    count += 1;
                }
            }
        }

        count
    }

}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for tree in row.iter() {
                write!(f, "{} ({:?}) ", tree.height, tree.state)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn score_impl<'a>(iter: impl Iterator<Item = &'a mut Tree>) {
    let history: Vec<&&Tree> = vec![];

    for tree in iter {
        for (score, t) in history.iter().rev().enumerate() {
            if t.height > tree.height {
                tree.scenic_scores.push(score as u32);
            }
        }
    }
}

fn visibility_impl<'a>(iter: impl Iterator<Item = &'a mut Tree>) {
    let mut highest: Option<u32> = None;
    for tree in iter {
        match highest {
            Some(height) => {
                if tree.height > height {
                    tree.state = TreeState::Visible;
                    highest = Some(tree.height);
                } else if tree.state == TreeState::Unknown {
                    tree.state = TreeState::NotVisible;
                }
            },
            None => {
                tree.state = TreeState::Visible;
                highest = Some(tree.height);
            }
        }
    }
}

fn grid(file: &str) -> Grid {

    let mut rows: Vec<Vec<Tree>> = vec![];

    let file = File::open(file).unwrap();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut row: Vec<Tree> = vec![]; 

        for c in line.chars() {
            let h = c.to_string().parse::<u32>().unwrap();
            row.push(Tree { height: h, state: TreeState::Unknown, scenic_scores: vec![] })
        }

        rows.push(row);
    }

    let mut g = Grid { rows: rows };
    g.update_visibility();

    g
}

fn calculate(file: &str) -> u32 {
    let grid = grid(file);

    grid.count_visible()
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(21, calculate("input/example.txt"));
}