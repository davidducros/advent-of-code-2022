use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

enum Node {
    File((String, usize)),
    Directory((String, Vec<Node>)),
}

impl Node {
    fn fmt_impl(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let indent = " ".repeat(depth);
        match self {
            Node::File((name, size)) => writeln!(f, "{}{}: {}", indent, name, size)?,
            Node::Directory((name, children)) => {
                writeln!(f, "{}{}/", indent, name)?;
                for c in children {
                    c.fmt_impl(f, depth + 2)?;
                }
            }
        }

        Ok(())
    }

    fn directories(&self) -> (Vec<(String, usize)>, usize) {

        if let Node::Directory((name, children)) = self {

            let mut my_size: usize = 0;
            let mut v: Vec<(String, usize)> = vec![];

            for c in children {
                match c {
                    Node::Directory(_) => {
                        let (children, total) = c.directories();

                        v.extend(children);
                        my_size += total;
                    },
                    Node::File((_name, size)) => {
                        my_size += size;
                    }
                }
            }

            v.push((name.clone(), my_size));

            return (v, my_size);
        }

        unreachable!();
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_impl(f, 0)
    }
}

fn parse(file: &str) -> Node {
    let file = File::open(file).unwrap();
    let mut iter = BufReader::new(file).lines().map(|l| l.unwrap()).peekable();

    // consume a line, we know it start "cd /"
    iter.next();

    parse_directory(&mut iter, "/")
}

fn parse_directory<T: Iterator<Item = String>>(lines: &mut Peekable<T>, name: &str) -> Node {
    let mut children: Vec<Node> = vec![];

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            break;
        } else if line.starts_with("$ cd") {
            let tokens = line.split(" ").collect::<Vec<&str>>();
            let d = tokens.get(2).unwrap();
            children.push(parse_directory(lines, d));
        } else if line.starts_with("$ ls") {
            while lines.peek().is_some() {
                if lines.peek().unwrap().starts_with("$") {
                    break;
                }

                let line = lines.next().unwrap();

                if line.starts_with("dir") {
                    // do nothing
                } else {
                    let tokens = line.split(" ").collect::<Vec<&str>>();
                    let size = tokens.get(0).unwrap().parse::<usize>().unwrap();
                    let name = tokens.get(1).unwrap();

                    children.push(Node::File((name.to_string(), size)));
                }
            }
        }
    }

    Node::Directory((name.to_string(), children))
}

fn calculate(file: &str, max: usize) -> usize {
    let tree = parse(file);

    let d = tree.directories();

    d.0.iter().filter(|d| d.1 <= max).map(|d| d.1).sum()
}

fn calculate_part2(file: &str, required: usize, disk_size: usize) -> usize {
    let tree = parse(file);

    println!("tree: {}", tree);

    let d = tree.directories();

    let used = d.1;
    println!("used: {}, required: {}, disk_size {}", used, required, disk_size);
    let to_free = required - (disk_size - used);

    let mut to_delete = disk_size;

    for dir in d.0 {
        if dir.1 >= to_free && dir.1 < to_delete {
            to_delete = dir.1;
        }
    }

    to_delete
}

fn main() {
    println!("result: {}", calculate("input/problem.txt", 100000));
    println!("result part 2: {}", calculate_part2("input/problem.txt", 30000000, 70000000));
}

#[test]
fn test_example() {
    assert_eq!(95437, calculate("input/example.txt", 100000));
}

#[test]
fn test_example_part2() {
    assert_eq!(24933642, calculate_part2("input/example.txt", 30000000, 70000000));
}
