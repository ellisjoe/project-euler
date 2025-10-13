use std::cmp::max;
use std::collections::HashMap;

#[test]
fn problem_18() {
    let lines: Vec<Vec<u32>> = TRIANGLE
        .lines()
        .map(|x| x.split(" ").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();
    let result = Triangle::new(lines).paths();
    println!("{}", result);
}

struct Triangle {
    values: Vec<Vec<u32>>,
    cache: HashMap<(usize, usize), u32>,
}

impl Triangle {
    fn new(values: Vec<Vec<u32>>) -> Self {
        Self {
            values,
            cache: HashMap::new(),
        }
    }

    fn paths(&mut self) -> u32 {
        self.path(0, 0)
    }

    fn path(&mut self, x: usize, y: usize) -> u32 {
        if y + 1 == self.values.len() {
            return self.values[y][x];
        }

        if self.cache.contains_key(&(x, y)) {
            return self.cache[&(x, y)];
        }

        let left = self.path(x, y + 1);
        let right = self.path(x + 1, y + 1);

        let max_path = max(left, right) + self.values[y][x];
        self.cache.insert((x, y), max_path);
        max_path
    }
}

const TRIANGLE: &str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
