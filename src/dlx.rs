#[derive(Debug)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    column: usize,
    row_id: usize,
}

#[derive(Debug)]
struct Column {
    primary: bool,
    size: usize,
}

pub struct DLX {
    nodes: Vec<Node>,
    columns: Vec<Column>,
    head: usize,
    answer: Vec<usize>,
}

impl DLX {
    pub fn new(
        primary_columns: Vec<bool>,
        rows: Vec<Vec<u8>>,
        clues: Vec<usize>,
    ) -> Self {
        let n_cols = rows[0].len();

        assert_eq!(primary_columns.len(), n_cols);

        let mut columns = vec![
            Column {
                primary: true,
                size: 0
            }
        ];

        for primary in primary_columns {
            columns.push(Column {
                primary,
                size: 0,
            });
        }

        let mut nodes = vec![];

        for i in 0..=n_cols {
            nodes.push(Node {
                left: if i == 0 { n_cols } else { i - 1 },
                right: if i == n_cols { 0 } else { i + 1 },
                up: i,
                down: i,
                column: i,
                row_id: usize::MAX,
            });
        }

        let mut row_nodes = vec![vec![]; rows.len()];

        for (r, row) in rows.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                if v == 1 {
                    let idx = nodes.len();

                    let up = nodes[c + 1].up;

                    nodes.push(Node {
                        left: idx,
                        right: idx,
                        up,
                        down: c + 1,
                        column: c + 1,
                        row_id: r,
                    });

                    nodes[up].down = idx;
                    nodes[c + 1].up = idx;

                    columns[c + 1].size += 1;

                    row_nodes[r].push(idx);
                }
            }
        }

        for row in row_nodes {
            let len = row.len();

            for i in 0..len {
                let cur = row[i];

                nodes[cur].left =
                    row[(i + len - 1) % len];

                nodes[cur].right =
                    row[(i + 1) % len];
            }
        }

        let mut dlx = Self {
            nodes,
            columns,
            head: 0,
            answer: vec![],
        };

        for clue in clues {
            dlx.apply_clue(clue);
        }

        dlx
    }

    fn apply_clue(&mut self, row_id: usize) {
        let row = self
            .nodes
            .iter()
            .position(|n| n.row_id == row_id)
            .expect("invalid clue");

        self.answer.push(row_id);

        self.cover(self.nodes[row].column);

        let mut j = self.nodes[row].right;

        while j != row {
            self.cover(self.nodes[j].column);
            j = self.nodes[j].right;
        }
    }

    fn cover(&mut self, col: usize) {
        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = right;
        self.nodes[right].left = left;

        let mut i = self.nodes[col].down;

        while i != col {
            let mut j = self.nodes[i].right;

            while j != i {
                let up = self.nodes[j].up;
                let down = self.nodes[j].down;

                self.nodes[up].down = down;
                self.nodes[down].up = up;

                if self.columns[self.nodes[j].column].primary {
                    self.columns[self.nodes[j].column].size -= 1;
                }

                j = self.nodes[j].right;
            }

            i = self.nodes[i].down;
        }
    }

    fn uncover(&mut self, col: usize) {
        let mut i = self.nodes[col].up;

        while i != col {
            let mut j = self.nodes[i].left;

            while j != i {
                if self.columns[self.nodes[j].column].primary {
                    self.columns[self.nodes[j].column].size += 1;
                }

                let up = self.nodes[j].up;
                let down = self.nodes[j].down;

                self.nodes[up].down = j;
                self.nodes[down].up = j;

                j = self.nodes[j].left;
            }

            i = self.nodes[i].up;
        }

        let left = self.nodes[col].left;
        let right = self.nodes[col].right;

        self.nodes[left].right = col;
        self.nodes[right].left = col;
    }

    fn search(&mut self) -> bool {
        let mut selected = None;
        let mut best = usize::MAX;

        let mut c = self.nodes[self.head].right;

        while c != self.head {
            if self.columns[c].primary {
                if self.columns[c].size < best {
                    best = self.columns[c].size;
                    selected = Some(c);
                }
            }

            c = self.nodes[c].right;
        }

        if selected.is_none() {
            return true;
        }

        let col = selected.unwrap();

        if self.columns[col].size == 0 {
            return false;
        }

        self.cover(col);

        let mut r = self.nodes[col].down;

        while r != col {
            self.answer.push(self.nodes[r].row_id);

            let mut j = self.nodes[r].right;

            while j != r {
                self.cover(self.nodes[j].column);
                j = self.nodes[j].right;
            }

            if self.search() {
                return true;
            }

            self.answer.pop();

            let mut j = self.nodes[r].left;

            while j != r {
                self.uncover(self.nodes[j].column);
                j = self.nodes[j].left;
            }

            r = self.nodes[r].down;
        }

        self.uncover(col);

        false
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        if self.search() {
            Some(self.answer.clone())
        } else {
            None
        }
    }
}