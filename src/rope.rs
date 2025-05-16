#[derive(Debug)]
enum RopeNode<Line> {
    Leaf(Vec<Line>), // stores lines of text
    Internal {
        left: Box<RopeNode<Line>>,
        right: Box<RopeNode<Line>>,
        line_count: usize, // total lines in left subtree
    },
}

impl<Line> RopeNode<Line> {
    pub fn from_lines(lines: Vec<Line>) -> Self {
        RopeNode::Leaf(lines)
    }

    pub fn total_lines(&self) -> usize {
        match self {
            RopeNode::Leaf(lines) => lines.len(),
            RopeNode::Internal { left, right, .. } => left.total_lines() + right.total_lines(),
        }
    }

    fn get_line_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut Line> {
        match self {
            RopeNode::Leaf(lines) => lines.get_mut(index),
            RopeNode::Internal { left, right, line_count } => {
                if index < *line_count {
                    left.get_line_mut(index)
                } else {
                    right.get_line_mut(index - *line_count)
                }
            }
        }
    }

    fn get_line(&self, index: usize) -> Option<&Line> {
        match self {
            RopeNode::Leaf(lines) => lines.get(index),
            RopeNode::Internal { left, right, line_count } => {
                if index < *line_count {
                    left.get_line(index)
                } else {
                    right.get_line(index - *line_count)
                }
            }
        }
    }

    pub fn insert_line(&mut self, index: usize, line: Line) {
        match self {
            RopeNode::Leaf(lines) => {
                lines.insert(index, line);
            }
            RopeNode::Internal { left, right, line_count } => {
                if index <= *line_count {
                    left.insert_line(index, line);
                    *line_count += 1;
                } else {
                    right.insert_line(index - *line_count, line);
                }
            }
        }
    }

    pub fn delete_line(&mut self, index: usize) {
        match self {
            RopeNode::Leaf(lines) => {
                lines.remove(index);
            }
            RopeNode::Internal { left, right, line_count } => {
                if index < *line_count {
                    left.delete_line(index);
                    *line_count -= 1;
                } else {
                    right.delete_line(index - *line_count);
                }
            }
        }
    }
}


pub struct RopeLinesIterator<'a, Line> {
    stack: Vec<&'a RopeNode<Line>>,
    current_leaf: Option<std::slice::Iter<'a, Line>>,
}

impl<'a, Line> RopeLinesIterator<'a, Line> {
    pub fn from_index(root: &'a RopeNode<Line>, mut index: usize) -> Self {
        let mut stack = Vec::new();
        let mut current_leaf_iter = None;

        let mut node = root;

        // Traverse to the correct leaf
        loop {
            match node {
                RopeNode::Leaf(lines) => {
                    if index <= lines.len() {
                        current_leaf_iter = Some(lines[index..].iter());
                    } else {
                        // If index is out of bounds, empty iterator
                        current_leaf_iter = Some([].iter());
                    }
                    break;
                }
                RopeNode::Internal { left, right, line_count } => {
                    stack.push(node);
                    if index < *line_count {
                        node = left;
                    } else {
                        index -= *line_count;
                        node = right;
                    }
                }
            }
        }

        Self {
            stack,
            current_leaf: current_leaf_iter,
        }
    }

    fn descend_leftmost(&mut self, mut node: &'a RopeNode<Line>) {
        while let RopeNode::Internal { left, .. } = node {
            self.stack.push(node);
            node = left;
        }

        if let RopeNode::Leaf(lines) = node {
            self.current_leaf = Some(lines.iter());
        }
    }
}

impl<'a, Line> Iterator for RopeLinesIterator<'a, Line> {
    type Item = &'a Line;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut leaf_iter) = self.current_leaf {
            if let Some(line) = leaf_iter.next() {
                return Some(line);
            }
        }

        // Walk up the stack and explore right siblings
        while let Some(parent) = self.stack.pop() {
            if let RopeNode::Internal { right, .. } = parent {
                self.descend_leftmost(&right);
                return self.next();
            }
        }

        None
    }
}
