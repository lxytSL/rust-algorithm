struct Spreadsheet {
    cells: Vec<[i32; 26]>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            cells: vec![[0; 26]; rows as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let mut iter = cell.chars();
        let col = iter.next().unwrap() as usize - 'A' as usize;
        let row = iter.collect::<String>().parse::<usize>().unwrap() - 1;
        self.cells[row][col] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let mut iter = cell.chars();
        let col = iter.next().unwrap() as usize - 'A' as usize;
        let row = iter.collect::<String>().parse::<usize>().unwrap() - 1;
        self.cells[row][col] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let nums = formula[1..].split('+').collect::<Vec<&str>>();
        let mut ans = 0;
        for num in nums {
            let mut iter = num.chars();
            let first = iter.next().unwrap();
            if first.is_digit(10) {
                ans += num.parse::<i32>().unwrap();
            } else {
                let col = first as usize - 'A' as usize;
                let row = iter.collect::<String>().parse::<usize>().unwrap() - 1;
                dbg!(&row);
                dbg!(&col);
                dbg!(&self.cells);
                if row >= self.cells.len() {
                    ans += 0;
                } else {
                    ans += self.cells[row][col];
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spreadsheet() {
        let mut spreadsheet = Spreadsheet::new(969);
        assert!(spreadsheet.get_value("=16587+22861".into()) == 39448);
        assert!(spreadsheet.get_value("=Q672+T262".into()) == 0);
    }
}
