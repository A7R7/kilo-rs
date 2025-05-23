#[derive(Debug)]
pub enum RopeNode<Line> {
    Leaf(Vec<Line>), // stores lines of text
    Internal {
        left: Box<RopeNode<Line>>,
        right: Box<RopeNode<Line>>,
        left_count: usize,
        count: usize, // total lines in left subtree
    },
}

const MAX_LEAF_SIZE: usize = 64;

impl<Line> Default for RopeNode<Line> {
    fn default() -> Self {
        RopeNode::Leaf(Vec::new())
    }
}

impl<Line> RopeNode<Line> {
    pub fn from_lines(mut lines: Vec<Line>) -> Self {
        if lines.len() <= MAX_LEAF_SIZE {
            RopeNode::Leaf(lines)
        } else {
            let mid = lines.len() / 2;
            let right_lines: Vec<Line> = lines.drain(mid..).collect();
            let left = Box::new(RopeNode::from_lines(lines));
            let right = Box::new(RopeNode::from_lines(right_lines));
            let lc = left.count();
            let rc = right.count();
            RopeNode::Internal {
                left,
                right,
                left_count: lc,
                count: lc + rc,
            }
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        match self {
            RopeNode::Leaf(lines) => lines.len(),
            RopeNode::Internal { count, .. } => *count,
        }
    }

    pub fn get_line_mut<'a>(&'a mut self, index: usize) -> &'a mut Line {
        match self {
            RopeNode::Leaf(lines) => lines.get_mut(index).unwrap(),
            RopeNode::Internal {
                left,
                right,
                left_count,
                ..
            } => {
                if index < *left_count {
                    left.get_line_mut(index)
                } else {
                    right.get_line_mut(index - *left_count)
                }
            }
        }
    }

    pub fn get_line(&self, index: usize) -> &Line {
        match self {
            RopeNode::Leaf(lines) => lines.get(index).unwrap(),
            RopeNode::Internal {
                left,
                right,
                left_count,
                ..
            } => {
                if index < *left_count {
                    left.get_line(index)
                } else {
                    right.get_line(index - *left_count)
                }
            }
        }
    }

    pub fn insert_line(&mut self, index: usize, line: Line) {
        match self {
            RopeNode::Leaf(lines) => {
                lines.insert(index, line);
                if lines.len() > MAX_LEAF_SIZE {
                    *self = self.split_leaf();
                }
            }
            RopeNode::Internal {
                left,
                right,
                left_count,
                count,
            } => {
                if index <= *left_count {
                    left.insert_line(index, line);
                    *left_count += 1;
                } else {
                    right.insert_line(index - *left_count, line);
                }
                *count += 1;
                self.rebalance();
            }
        }
    }

    pub fn delete_line(&mut self, index: usize) -> Line {
        match self {
            RopeNode::Leaf(lines) => {
                lines.remove(index)
            }
            RopeNode::Internal {
                left,
                right,
                left_count,
                count,
            } => {
                let line = if index < *left_count {
                    *left_count -= 1;
                    left.delete_line(index)
                } else {
                    right.delete_line(index - *left_count)
                };
                *count -= 1;
                self.try_merge_children();
                self.rebalance();
                line
            }
        }
    }

    fn split_leaf(&mut self) -> RopeNode<Line> {
        if let RopeNode::Leaf(lines) = std::mem::take(self) {
            Self::from_lines(lines)
        } else {
            panic!("Called split_leaf on non-leaf node");
        }
    }

    fn try_merge_children(&mut self) {
        if let RopeNode::Internal {
            left,
            right,
            ..
        } = self
        {
            // Try to merge if both children are leaves and combined size is acceptable
            match (&mut **left, &mut **right) {
                (RopeNode::Leaf(l_lines), RopeNode::Leaf(r_lines)) => {
                    if l_lines.len() + r_lines.len() <= MAX_LEAF_SIZE {
                        let mut merged = Vec::with_capacity(l_lines.len() + r_lines.len());
                        merged.append(l_lines);
                        merged.append(r_lines);
                        *self = RopeNode::Leaf(merged);
                    }
                }
                _ => {}
            }
        }
    }

    fn rebalance(&mut self) {
        if let RopeNode::Internal { left, right, .. } = self {
            let lc = left.count();
            let rc = right.count();
            if lc > 2 * rc + 1 {
                self.rotate_right();
            } else if rc > 2 * lc + 1 {
                self.rotate_left();
            }
        }
    }

    fn rotate_left(&mut self) {
        if let RopeNode::Internal {
            left,
            right,
            left_count,
            count,
        } = std::mem::take(self)
        {
            if let RopeNode::Internal {
                left: r_left,
                right: r_right,
                ..
            } = *right
            {
                let l_c = left_count;
                let r_lc = r_left.count();
                *self = RopeNode::Internal {
                    left: Box::new(RopeNode::Internal {
                        left,
                        right: r_left,
                        left_count: l_c,
                        count: l_c + r_lc,
                    }),
                    right: r_right,
                    left_count: l_c + r_lc,
                    count,
                };
            } else {
                *self = RopeNode::Internal {
                    left,
                    right,
                    left_count,
                    count,
                };
            }
        }
    }

    fn rotate_right(&mut self) {
        if let RopeNode::Internal {
            left,
            right,
            left_count,
            count,
            ..
        } = std::mem::take(self)
        {
            if let RopeNode::Internal {
                left: l_left,
                right: l_right,
                left_count: l_lc,
                ..
            } = *left
            {
                let l_rc = l_right.count();
                let r_c = right.count();
                *self = RopeNode::Internal {
                    left: l_left,
                    right: Box::new(RopeNode::Internal {
                        left: l_right,
                        right,
                        left_count: l_rc,
                        count: l_rc + r_c,
                    }),
                    left_count: l_lc,
                    count,
                };
            } else {
                *self = RopeNode::Internal {
                    left,
                    right,
                    left_count,
                    count,
                };
            }
        }
    }

    // line iterator
    pub fn from_index(&self, mut index: usize) -> RopeLinesIterator<Line> {
        let mut stack = Vec::new();
        let mut current_leaf_iter = None;

        let mut node = self;

        loop {
            match node {
                RopeNode::Leaf(lines) => {
                    if index <= lines.len() {
                        current_leaf_iter = Some(lines[index..].iter());
                    } else {
                        current_leaf_iter = Some([].iter());
                    }
                    break;
                }
                RopeNode::Internal {
                    left,
                    right,
                    left_count,
                    ..
                } => {
                    if index < *left_count {
                        stack.push(node);
                        node = left;
                    } else {
                        index -= *left_count;
                        node = right;
                    }
                }
            }
        }

        RopeLinesIterator {
            stack,
            current_leaf: current_leaf_iter,
        }
    }

    pub fn lines(&self) -> RopeLinesIterator<Line> {
        self.from_index(0)
    }

}

pub struct RopeLinesIterator<'a, Line> {
    stack: Vec<&'a RopeNode<Line>>,
    current_leaf: Option<std::slice::Iter<'a, Line>>,
}

impl<'a, Line> RopeLinesIterator<'a, Line> {
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

pub struct RopeBuilder<Line> {
    leaf_buffer: Vec<Line>,
    nodes: Vec<RopeNode<Line>>,
}

impl<Line> RopeBuilder<Line> {
    pub fn new() -> Self {
        Self {
            leaf_buffer: Vec::with_capacity(MAX_LEAF_SIZE),
            nodes: Vec::new(),
        }
    }

    pub fn insert(&mut self, line: Line) {
        self.leaf_buffer.push(line);
        if self.leaf_buffer.len() >= MAX_LEAF_SIZE {
            self.flush_leaf();
        }
    }

    fn flush_leaf(&mut self) {
        if !self.leaf_buffer.is_empty() {
            let leaf = RopeNode::Leaf(std::mem::take(&mut self.leaf_buffer));
            self.nodes.push(leaf);
        }
    }

    pub fn build(mut self) -> Option<RopeNode<Line>> {
        self.flush_leaf();
        if self.nodes.is_empty() {
            return None;
        }
        Some(Self::build_balanced(self.nodes))
    }

    fn build_balanced(mut nodes: Vec<RopeNode<Line>>) -> RopeNode<Line> {
        while nodes.len() > 1 {
            let mut next_level = Vec::with_capacity((nodes.len() + 1) / 2);
            let mut i = 0;
            while i + 1 < nodes.len() {
                let left = Box::new(nodes[i].take());
                let right = Box::new(nodes[i + 1].take());
                let lc = left.count();
                let rc = right.count();
                next_level.push(RopeNode::Internal {
                    left,
                    right,
                    left_count: lc,
                    count: lc + rc,
                });
                i += 2;
            }
            if i < nodes.len() {
                next_level.push(nodes[i].take());
            }
            nodes = next_level;
        }
        nodes.pop().unwrap()
    }
}

trait Take<T> {
    fn take(&mut self) -> T;
}

impl<T> Take<T> for T
where
    T: Default,
{
    fn take(&mut self) -> T {
        std::mem::take(self)
    }
}
