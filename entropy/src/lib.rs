use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy)]
pub struct CountTable {
    counters: [usize; 256],
    sum: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct ConditionalCountTable {
    tables: [CountTable; 256],
    sum: usize,
}

pub fn count(content: &[u8]) -> CountTable {
    let mut table = CountTable::zeroed();

    for &byte in content {
        table.counters[byte as usize] += 1;
    }

    table.sum = content.len();

    table
}

pub fn count_iter(content: impl Iterator<Item = u8>) -> CountTable {
    let mut table = CountTable::zeroed();
    let mut len = 0;

    for byte in content {
        table.counters[byte as usize] += 1;
        len += 1;
    }

    table.sum = len;

    table
}

pub fn conditional_count(content: &[u8]) -> ConditionalCountTable {
    let mut table = ConditionalCountTable::zeroed();

    let with_zero = {
        let mut v = vec![0];
        v.extend_from_slice(content);
        v
    };

    for pair in with_zero.windows(2) {
        let &[first, second] = pair else { unreachable!() };

        table.tables[first as usize].counters[second as usize] += 1;
        table.tables[first as usize].sum += 1;
    }

    table.sum = content.len();

    table
}

pub fn entropy(table: &CountTable) -> f64 {
    table
        .counters
        .iter()
        .filter(|&&c| c != 0)
        .fold(0.0, |acc, &c| {
            let p = c as f64 / table.sum as f64;
            acc - p * p.log2()
        })
}

pub fn conditional_entropy(table: &ConditionalCountTable) -> f64 {
    table.tables.iter().fold(0.0, |acc, t| {
        let h = entropy(t);
        let p = t.sum as f64 / table.sum as f64;

        acc + h * p
    })
}

impl CountTable {
    pub fn zeroed() -> Self {
        Self {
            counters: [0; 256],
            sum: 0,
        }
    }
}

impl ConditionalCountTable {
    pub fn zeroed() -> Self {
        Self {
            tables: [CountTable::zeroed(); 256],
            sum: 0,
        }
    }
}

impl Display for CountTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..256 {
            if self.counters[i] != 0 {
                writeln!(f, "{}: {}", i, self.counters[i])?;
            }
        }
        writeln!(f, "sum {}", self.sum)?;
        Ok(())
    }
}

impl Display for ConditionalCountTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..256 {
            for j in 0..256 {
                if self.tables[i].counters[j] != 0 {
                    writeln!(f, "({}, {}): {}", i, j, self.tables[i].counters[j])?;
                }
            }
            if self.tables[i].sum != 0 {
                writeln!(f, "sum {}", self.tables[i].sum)?;
            }
        }
        Ok(())
    }
}

pub fn quick_entropy_of_file(file_path: impl AsRef<std::path::Path>) -> f64 {
    use std::{io::{Read, BufReader}, fs::File};

    let mut reader = BufReader::new(File::open(file_path).unwrap());

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let count_table = count(&buf);
    entropy(&count_table)
}

pub fn quick_entropy(it: impl Iterator<Item = u8>) -> f64 {
    let count_table = count_iter(it);
    entropy(&count_table)
}

pub fn quick_entropy_print(it: impl Iterator<Item = u8>) -> f64 {
    let count_table = count_iter(it);
    println!("{}", count_table);
    entropy(&count_table)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_table_test() {
        let content = [0, 1, 3, 1, 0, 255, 12, 12, 12, 1];

        let count_table = count(&content);

        println!("{}", count_table);
    }

    #[test]
    fn count_entropy_test() {
        let content = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let count_table = count(&content);

        println!("{}", &count_table);
        println!("{}", entropy(&count_table));
    }

    #[test]
    fn conditional_count_table_test() {
        let content = [0, 1, 3, 1, 0, 255, 12, 12, 12, 1];

        let count_table = conditional_count(&content);

        println!("{}", count_table);
    }

    #[test]
    fn conditional_entropy_test() {
        let content = [1, 2, 1, 2, 2, 2];

        let count_table = conditional_count(&content);

        println!("{}", &count_table);
        println!("{}", conditional_entropy(&count_table));
    }
}
