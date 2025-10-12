use std::collections::HashMap;

#[test]
fn problem_015() {
    let result = LatticePaths::new(20).paths();
    println!("{}", result);
}

struct LatticePaths {
    size: i32,
    cache: HashMap<(i32, i32), u64>,
}

impl LatticePaths {
    fn new(grid_size: i32) -> Self {
        Self {
            size: grid_size,
            cache: HashMap::new(),
        }
    }

    fn paths(&mut self) -> u64 {
        self.path(0, 0)
    }

    fn path(&mut self, x: i32, y: i32) -> u64 {
        if self.cache.contains_key(&(x, y)) {
            return self.cache.get(&(x, y)).unwrap().clone();
        }

        if x == self.size && y == self.size {
            return 1;
        }

        let route_1 = if x < self.size {
            self.path(x + 1, y)
        } else {
            0
        };

        let route_2 = if y < self.size {
            self.path(x, y + 1)
        } else {
            0
        };

        let total = route_1 + route_2;
        self.cache.insert((x, y), total);
        total
    }
}
