#[derive(Debug)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug)]
struct WordSpot {
    row: usize,
    col: usize,
    dir: Direction,
    len: usize,

    filled: bool,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,

    word_spots: Vec<WordSpot>,

    solution_lines: Vec<String>,
}

impl Grid {
    fn parse(lines: &str) -> Self {
        let grid: Vec<Vec<char>> = lines
            .trim()
            .split("\n")
            .map(|line| line.chars().collect())
            .collect();
        let mut word_spots = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            for col in 0..cols {
                let c = grid[row][col];
                if c == '.' {
                    continue;
                }
                if c != '#' && (c < 'A' || c > 'Z') {
                    panic!("invalid char: {} at {},{}", c, row, col);
                }
                // horizontal word
                if col == 0 || grid[row][col - 1] == '.' {
                    let mut len = 0;
                    while (col + len < cols) && grid[row][col + len] != '.' {
                        len += 1;
                    }
                    if len != 1 {
                        word_spots.push(WordSpot {
                            row,
                            col,
                            dir: Direction::HORIZONTAL,
                            len,
                            filled: false,
                        });
                    }
                }
                // vertical word
                if row == 0 || grid[row - 1][col] == '.' {
                    let mut len = 0;
                    while (row + len < rows) && grid[row + len][col] != '.' {
                        len += 1;
                    }
                    if len != 1 {
                        word_spots.push(WordSpot {
                            row,
                            col,
                            dir: Direction::VERTICAL,
                            len,
                            filled: false,
                        });
                    }
                }
            }
        }
        Self {
            grid,
            rows,
            cols,
            word_spots,
            solution_lines: Vec::new(),
        }
    }

    fn print(&mut self) {
        for line in self.grid.iter() {
            let l: String = line.iter().collect();
            println!("{}", &l);
            self.solution_lines.push(l);
        }
    }

    fn find_word_spot_to_fill(&mut self) -> Option<usize> {
        // TODO: choose a better word spot to fill in?
        for (i, ws) in self.word_spots.iter().enumerate() {
            if !ws.filled {
                return Some(i);
            }
        }
        // No more words to find!
        println!();
        self.solution_lines.push("".to_string());
        self.print();
        None
    }

    fn solve(&mut self, words: &Vec<Vec<char>>, words_used: &mut Vec<bool>) {
        let ws_idx = match self.find_word_spot_to_fill() {
            Some(x) => x,
            None => {
                return;
            }
        };
        // dbg!(&ws_idx);
        // dbg!(&self.word_spots[ws_idx]);
        for word_idx in 0..words.len() {
            if words_used[word_idx] {
                continue;
            }
            if words[word_idx].len() != self.word_spots[ws_idx].len {
                continue;
            }
            // dbg!(&word_idx);
            words_used[word_idx] = true;
            self.word_spots[ws_idx].filled = true;
            self.add_word(words, words_used, ws_idx, word_idx, 0);
            self.word_spots[ws_idx].filled = false;
            words_used[word_idx] = false;
        }
    }

    fn add_word(
        &mut self,
        words: &Vec<Vec<char>>,
        words_used: &mut Vec<bool>,
        ws_idx: usize,
        word_idx: usize,
        i: usize,
    ) {
        // println!("add_word: {} {} {}", ws_idx, word_idx, i);
        if i >= words[word_idx].len() {
            self.solve(words, words_used);
            return;
        }
        let (new_row, new_col) = {
            let ws = &self.word_spots[ws_idx];
            match ws.dir {
                Direction::HORIZONTAL => {
                    if ws.col + i >= self.cols {
                        return;
                    }
                    (ws.row, ws.col + i)
                }
                Direction::VERTICAL => {
                    if ws.row + i >= self.rows {
                        return;
                    }
                    (ws.row + i, ws.col)
                }
            }
        };
        let c = self.grid[new_row][new_col];
        if c == '#' {
            self.grid[new_row][new_col] = words[word_idx][i];
            self.add_word(words, words_used, ws_idx, word_idx, i + 1);
            self.grid[new_row][new_col] = '#';
        } else if c == words[word_idx][i] {
            self.add_word(words, words_used, ws_idx, word_idx, i + 1);
        }
    }
}

fn read_file(path: &str) -> (Grid, Vec<Vec<char>>) {
    let contents = std::fs::read_to_string(path).expect("read failed");
    let (grid_str, words_str) = contents.split_once("\n\n").expect("no empty line");
    let grid = Grid::parse(grid_str);
    let words = words_str
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    (grid, words)
}

pub fn load_and_solve(path: &str) -> Vec<String> {
    let (mut grid, words) = read_file(path);
    let mut words_used: Vec<bool> = Vec::new();
    words_used.resize(words.len(), false);
    // dbg!(&grid);
    // dbg!(&words_by_len);
    // dbg!(&grid.word_spots.len());
    grid.print();
    grid.solve(&words, &mut words_used);
    grid.solution_lines
}
