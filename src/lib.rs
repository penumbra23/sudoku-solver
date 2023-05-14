use std::{str::FromStr, fmt::Display, num::TryFromIntError};

/// Error thrown on invalid sudoku parsing
#[derive(Debug)]
pub struct SudokuError {
    pub details: String,
}

impl From<TryFromIntError> for SudokuError {
    fn from(value: TryFromIntError) -> Self {
        Self {
            details: value.to_string(),
        }
    }
}

/// Sudoku structure holding the 81 values of the sudoku puzzle
pub struct Sudoku {
    table: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn valid(&self) -> bool {
        // validate rows
        for row in 0..9 {
            let mut counts = [0; 9];
            for col in 0..9 {
                let val = self.table[row][col];
                counts[(val - 1) as usize] += 1;
            }
            if counts.iter().any(|&count| count != 1) {
                return false;
            }
        }
        // validate columns
        for col in 0..9 {
            let mut counts = [0; 9];
            for row in 0..9 {
                let val = self.table[row][col];
                counts[(val - 1) as usize] += 1;
            }
            if counts.iter().any(|&count| count != 1) {
                return false;
            }
        }

        // check 3x3 blocks
        for block_row in 0..3 {
            for block_col in 0..3 {
                let mut counts = [0; 9];
                for row in (block_row * 3)..((block_row + 1) * 3) {
                    for col in (block_col * 3)..((block_col + 1) * 3) {
                        let val = self.table[row][col];
                        counts[(val - 1) as usize] += 1;
                    }
                }
                if counts.iter().any(|&count| count != 1) {
                    return false;
                }
            }
        }
        true
    }
}

impl FromStr for Sudoku {
    type Err = SudokuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.replace("\n", "");
        if input.len() != 81 {
            return Err(SudokuError{details: "Table needs to have 81 cells.".to_string()});
        }
        let mut table = [[0u8; 9]; 9];
        for (i, c) in input.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            let val = match c.to_digit(10) {
                Some(c) => c,
                None => return Err(SudokuError{details: format!("Invalid character {} at {},{}.", c, row, col)}),
            };
            table[row][col] = val.try_into()?;
        }
        Ok(Sudoku { table })
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.table.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                write!(f, "------+-------+------\n")?;
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_sudoku() {
        let sudoku: Sudoku = "534678912\n\
             672195348\n\
             198342567\n\
             859761423\n\
             426853791\n\
             713924856\n\
             961537284\n\
             287419635\n\
             345286179"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(sudoku.valid());
    }

    #[test]
    fn test_invalid_sudoku() {
        let sudoku: Sudoku = "534678912\n\
             672195348\n\
             198342567\n\
             859761423\n\
             426853791\n\
             713924856\n\
             961537284\n\
             287419635\n\
             345286177"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }

    #[test]
    fn test_invlid_blocks() {
        let sudoku: Sudoku = "123456789\n\
            234567891\n\
            345678912\n\
            456789123\n\
            567891234\n\
            678912345\n\
            789123456\n\
            891234567\n\
            912345678"
            .parse()
            .unwrap();
        println!("{}", sudoku);
        assert!(!sudoku.valid());
    }
}
