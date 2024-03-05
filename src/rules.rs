use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum Change {
    Birth(i16, i16),
    Death(i16, i16),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix {
    alive: HashSet<(i16, i16)>,
}
#[allow(dead_code)]
impl Matrix {
    pub fn blank() -> Self {
        Self {
            alive: HashSet::new(),
        }
    }

    pub fn get_alive(&self) -> &HashSet<(i16, i16)> {
        &self.alive
    }

    pub fn is_alive(&self, x: i16, y: i16) -> bool {
        self.alive.contains(&(x, y))
    }

    pub fn insert(&mut self, x: i16, y: i16) {
        self.alive.insert((x, y));
    }

    pub fn remove(&mut self, x: i16, y: i16) {
        self.alive.remove(&(x, y));
    }

    pub fn advance(&mut self) -> Vec<Change> {
        let mut changes = Vec::new();

        let mut dead_cells_checked: HashSet<(i16, i16)> = HashSet::new();

        for (x, y) in &self.alive {
            let n = self.count_alive_neighbors(*x, *y);

            if n < 2 || n > 3 {
                changes.push(Change::Death(*x, *y));
            }

            let neighbors = Self::get_neighbors(*x, *y);
            let dead_neighbors: Vec<&(i16, i16)> = neighbors
                .iter()
                .filter(|p| !self.is_alive(p.0, p.1) && !dead_cells_checked.contains(p))
                .collect();

            for p in dead_neighbors {
                dead_cells_checked.insert(*p);
                if self.count_alive_neighbors(p.0, p.1) == 3 {
                    changes.push(Change::Birth(p.0, p.1));
                }
            }
        }

        self.execute_changes(&changes);

        return changes;
    }

    fn execute_changes(&mut self, changes: &Vec<Change>) {
        for change in changes {
            if let Change::Birth(x, y) = change {
                self.alive.insert((*x, *y));
            }
            if let Change::Death(x, y) = change {
                self.alive.remove(&(*x, *y));
            }
        }
    }

    fn count_alive_neighbors(&self, x: i16, y: i16) -> u8 {
        let mut count = 0;

        for (nx, ny) in Self::get_neighbors(x, y) {
            if self.is_alive(nx, ny) {
                count += 1;
            }
        }

        return count;
    }

    fn get_neighbors(x: i16, y: i16) -> Vec<(i16, i16)> {
        vec![
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
        ]
    }
}

#[cfg(test)]
mod rules_tests {
    use super::*;

    #[test]
    fn glider() {
        let mut glider_matrix = Matrix::blank();
        glider_matrix.insert(-1, -1);
        glider_matrix.insert(-1, 1);
        glider_matrix.insert(0, 0);
        glider_matrix.insert(0, 1);
        glider_matrix.insert(1, 0);

        let _changes = glider_matrix.advance();

        let mut new_state = Matrix::blank();
        new_state.insert(0, -1);
        new_state.insert(0, 1);
        new_state.insert(-1, 1);
        new_state.insert(1, 1);
        new_state.insert(1, 0);

        assert_eq!(glider_matrix, new_state);
    }
}
