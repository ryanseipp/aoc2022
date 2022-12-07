#[derive(Debug)]
struct Node {
    idx: usize,
    size: usize,
    path: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(idx: usize, path: String, size: usize) -> Self {
        Self {
            idx,
            size,
            path,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<Node>,
}

impl ArenaTree {
    pub fn insert(&mut self, val: &str, parent: Option<usize>, size: Option<usize>) -> usize {
        let idx = self.arena.len();
        let path = self.construct_path(val, parent);

        self.arena.push(Node::new(idx, path, size.unwrap_or(0)));

        if let Some(parent_node) = parent {
            self.arena[idx].parent = Some(parent_node);
            self.arena[parent_node].children.push(idx);
        }

        if let Some(_) = size {
            self.set_size_for_parents(idx);
        }

        return idx;
    }

    pub fn node(&self, val: &str, parent: usize) -> Option<usize> {
        let path = self.construct_path(val, Some(parent));
        for node in &self.arena {
            if node.path == path {
                return Some(node.idx);
            }
        }

        return None;
    }

    fn set_size_for_parents(&mut self, from: usize) {
        let size = self.arena[from].size;
        let mut trav = &mut self.arena[from];

        while let Some(inner) = trav.parent {
            trav = &mut self.arena[inner];
            trav.size += size;
        }
    }

    fn construct_path(&self, val: &str, parent: Option<usize>) -> String {
        if let Some(parent_node) = parent {
            return self.arena[parent_node].path.clone() + "/" + &val.to_string();
        }

        return val.to_string();
    }
}

fn build_tree(input: &str) -> ArenaTree {
    let mut tree: ArenaTree = ArenaTree::default();
    let mut current_node: usize = tree.insert("/", None, None);

    for line in input.lines().skip(1) {
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd ..") {
            if let Some(parent) = tree.arena[current_node].parent {
                current_node = parent;
            } else {
                unreachable!();
            }
        } else if line.starts_with("$ cd ") {
            let dir_name = line.split_at(5).1;

            if let Some(new_dir) = tree.node(dir_name, current_node) {
                current_node = new_dir;
            } else {
                let new_dir = tree.insert(dir_name, Some(current_node), None);
                current_node = new_dir;
            }
        } else if line.starts_with("dir ") {
            let dir_name = line.split_at(4).1;

            if let None = tree.node(dir_name, current_node) {
                tree.insert(dir_name, Some(current_node), None);
            }
        } else {
            let file_info: Vec<&str> = line.split(' ').collect();
            let file_size = file_info[0].parse().unwrap();
            let file_name = file_info[1];
            if let None = tree.node(file_name, current_node) {
                tree.insert(file_name, Some(current_node), Some(file_size));
            }
        }
    }

    return tree;
}

pub fn part_one(input: &str) -> usize {
    let tree = build_tree(input);

    let mut sum: usize = 0;
    for node in tree.arena {
        if node.children.len() > 0 && node.size < 100000 {
            sum += node.size;
        }
    }

    return sum;
}

pub fn part_two(input: &str) -> Option<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const MIN_SPACE: usize = 30000000;
    let tree = build_tree(input);

    let used_space = tree.arena[0].size;

    return tree
        .arena
        .iter()
        .filter(|n| n.children.len() > 0 && TOTAL_SPACE - used_space + n.size > MIN_SPACE)
        .map(|n| n.size)
        .min();
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    fn get_input() -> String {
        return fs::read_to_string("input.txt").expect("File input.txt should exist");
    }

    #[test]
    fn part_one_computes_correct_result() {
        assert_eq!(1297683, part_one(&get_input()));
    }

    #[test]
    fn part_two_computes_correct_result() {
        assert_eq!(Some(5756764), part_two(&get_input()));
    }
}
