use std::fmt;

use crate::Point;

#[derive(Clone, Debug)]
pub struct CostList {
    /// data ist ein 1D Array der Größe n_points * n_points
    /// hier werden die Abstände zwischen den Punkten gespeichert
    /// Wird wie ein 2D Array verwendet Spaltennummer ist ein Punkt 
    /// und Zeilennummer der Anderere
    data: Vec<Option<f64>>,
    pub size: usize,
}

impl CostList {
    // Speichert die Abstände zwischen den Punkten
    pub fn new(points: &Vec<Point>) -> Self {
        let size = points.len();
        let mut data = vec![None; size * size];
        let mut i = 0;
        for y in 0..size {
            for x in 0..size {
                if x != y {
                    let dist = points[x].dist_to(&points[y]);
                    data[i] = Some(dist);
                }
                i += 1;
            }
        }
        Self { data, size }
    }
    // Gibt den Index im 1D Array zurück
    fn get_idx(&self, x: usize, y: usize) -> usize {
        self.size * y + x
    }
    pub fn get(&self, x: usize, y: usize) -> &Option<f64> {
        &self.data[self.get_idx(x, y)]
    }
    // Berechnet die Länge eines Pfades
    pub fn calc_len(&self, path: &Vec<usize>) -> f64 {
        let mut len = 0.0;
        for i in 0..path.len() - 1 {
            let x = path[i];
            let y = path[i + 1];
            len += self.get(x, y).unwrap();
        }
        len
    }
}
impl fmt::Display for CostList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        writeln!(f)?;
        for _y in 0..self.size {
            for _x in 0..self.size {
                write!(f, "{:.0}\t", self.data[i].unwrap_or(f64::NAN))?;
                i += 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}