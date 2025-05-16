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

    pub fn break_line(&mut self, index: usize, char_idx: usize) {
        if let Some(line) = self.get_line_mut(index) {
            let new_line = line.split_off(char_idx);
            self.insert_line(index + 1, new_line);
        }
    }

    pub fn concat_lines(&mut self, first_index: usize) {
        if let (Some(line1), Some(line2)) = (
            self.get_line_mut(first_index),
            self.get_line(first_index + 1).cloned(),
        ) {
            line1.push_str(&line2);
            self.delete_line(first_index + 1);
        }
    }

    pub fn insert_char(&mut self, row: usize, col: usize, c: char) {
        if let Some(line) = self.get_line_mut(row) {
            if col <= line.len() {
                line.insert(col, c);
            }
        }
    }

    pub fn delete_char(&mut self, row: usize, col: usize) {
        if let Some(line) = self.get_line_mut(row) {
            if col < line.len() {
                line.remove(col);
            }
        }
    }
}