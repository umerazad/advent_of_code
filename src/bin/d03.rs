use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn distance(&self, p: &Point) -> i64 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Line {
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Line {
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Line {
        // Lines are axis-aligned so just making sure that
        // all inputs are either horizontal or vertical.
        if x1 != x2 && y1 != y2 {
            panic!(
                "Only horizontal or vertical lines are allowed. ({},{}),({},{})",
                x1, y1, x2, y2
            );
        }

        if x1 <= x2 {
            Line { x1, y1, x2, y2 }
        } else {
            Line { x2, y2, x1, y1 }
        }
    }

    // Checks whether the line segment contains
    // the point.
    fn contains(&self, p: &Point) -> bool {
        let x1 = self.x1.min(self.x2);
        let x2 = self.x1.max(self.x2);
        let y1 = self.y1.min(self.y2);
        let y2 = self.y1.max(self.y2);

        // Since all lines are either horizontal or vertical
        // we we'll just use this simple heuristic.
        if p.x < x1 || p.x > x2 || p.y < y1 || p.y > y2 {
            false
        } else {
            true
        }
    }

    fn intersects(&self, other: &Line) -> Option<Point> {
        let a1 = self.y2 - self.y1;
        let b1 = self.x1 - self.x2;
        let c1 = a1 * self.x1 + b1 * self.y1;

        let a2 = other.y2 - other.y1;
        let b2 = other.x1 - other.x2;
        let c2 = a2 * other.x1 + b2 * other.y1;

        let determinant = a1 * b2 - a2 * b1;

        if determinant == 0 {
            return None;
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;

            // In this problem, origin doesn't count as
            // a valid intersection point.
            if x == 0 && y == 0 {
                return None;
            }

            // We need to make sure that the point actually
            // lies on both of the segments.
            let p = Point::new(x, y);
            if self.contains(&p) && other.contains(&p) {
                return Some(p);
            } else {
                return None;
            }
        }
    }
}

// Our priority queue depends on ordering. This trait implementation
// ensures that it behaves likes a min-heap for our line segments.
impl Ord for Line {
    fn cmp(&self, o: &Line) -> Ordering {
        // We compare only on y coordinates and break any ties
        // with other coordinates.
        match self.y1.cmp(&o.y1) {
            Ordering::Equal => match self.y2.cmp(&o.y2) {
                Ordering::Equal => match self.x1.cmp(&o.x1) {
                    Ordering::Equal => return self.x2.cmp(&o.x2),
                    ordering => return ordering,
                },
                ordering => return ordering,
            },
            ordering => return ordering,
        }
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, o: &Line) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

#[derive(Debug)]
struct Panel {
    lines: Vec<Line>,
    cursor: Point,
}

impl Panel {
    fn new() -> Panel {
        Panel {
            lines: Vec::new(),
            cursor: Point::new(0, 0),
        }
    }

    fn get_next_line(&self, path: &str) -> Line {
        let direction = path.chars().next().unwrap();
        let distance = path[1..].parse::<i64>().unwrap();
        let mut to = self.cursor;
        match direction {
            'R' => to.x += distance,
            'L' => to.x -= distance,
            'U' => to.y += distance,
            'D' => to.y -= distance,
            x => panic!("Unexpected direction: {}", x),
        }

        Line::new(self.cursor.x, self.cursor.y, to.x, to.y)
    }

    fn insert(&mut self, path: &str) {
        let line = self.get_next_line(path);
        self.lines.push(line);

        // Update cursor location.
        self.cursor = Point::new(line.x2, line.y2);
    }

    fn find_intersection(&self, input: &Line) -> Option<i64> {
        let mut min = i64::max_value();
        let origin = Point::new(0, 0);

        for l in self.lines.iter() {
            match l.intersects(input) {
                Some(p) => {
                    let distance = p.distance(&origin);
                    if distance < min {
                        min = distance;
                    }
                }
                None => (),
            }
        }

        if min != i64::max_value() {
            return Some(min);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let l1 = Line::new(6, 3, 6, 7);
        let l2 = Line::new(3, 5, 8, 5);
        assert_eq!(l1.intersects(&l2), Some(Point::new(6, 5)));

        let l1 = Line::new(0, 0, 8, 0);
        let l2 = Line::new(0, 1, 0, 7);
        assert_eq!(l1.intersects(&l2), None);

        let l1 = Line::new(66, 62, 66, 117);
        let l2 = Line::new(0, 0, 75, 0);
        assert_eq!(l1.intersects(&l2), None);
        assert_eq!(l2.intersects(&l1), None);
    }

    #[test]
    fn test_line_ordering() {
        // l1 is smaller
        let l1 = Line::new(0, 0, 10, 0);
        let l2 = Line::new(1, 10, 11, 10);
        assert_eq!(l1 < l2, true);

        // identical
        let l1 = Line::new(0, 0, 10, 0);
        let l2 = Line::new(0, 0, 10, 0);
        assert_eq!(l1 < l2, false);
        assert_eq!(l1 == l2, true);

        // y1 is same. y2 should break the tie
        let l1 = Line::new(10, 10, 20, 10);
        let l2 = Line::new(20, 10, 20, 11);
        assert_eq!(l1 < l2, true);

        // y1, y2 are identical. x1 should break the tie
        let l1 = Line::new(10, 10, 20, 10);
        let l2 = Line::new(11, 10, 20, 10);
        assert_eq!(l1 < l2, true);

        // x1, y1, y2 are identical. x2 should break the tie
        let l1 = Line::new(10, 10, 20, 10);
        let l2 = Line::new(10, 10, 21, 10);
        assert_eq!(l1 < l2, true);
    }

    #[test]
    fn test_find_closest_intersection() {
        let wire1 = vec!["R8", "U5", "L5", "D3"];
        let wire2 = vec!["U7", "R6", "D4", "L4"];
        assert_eq!(find_closest_intersection(wire1, wire2), Some(6));

        let wire1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let wire2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(find_closest_intersection(wire1, wire2), Some(159));

        let wire1 = vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let wire2 = vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        assert_eq!(find_closest_intersection(wire1, wire2), Some(135));
    }
}

fn find_closest_intersection(wire1: Vec<&str>, wire2: Vec<&str>) -> Option<i64> {
    let mut panel = Panel::new();
    // Layout wires of first panel.
    for x in wire1.iter() {
        panel.insert(x);
    }

    let mut min = i64::max_value();

    let mut other = Panel::new();
    for w in wire2.iter() {
        let l = other.get_next_line(w);
        other.insert(w);
        match panel.find_intersection(&l) {
            Some(v) => {
                if v < min {
                    min = v;
                }
            }
            None => (),
        }
    }

    if min == i64::max_value() {
        return None;
    } else {
        return Some(min);
    }
}

fn main() {
    let contents = fs::read_to_string("assets/day3_input").unwrap();
    let lines: Vec<&str> = contents.lines().filter(|l| l.len() > 1).collect();
    assert_eq!(lines.len(), 2);

    let wire1: Vec<&str> = lines[0].split(",").collect();
    let wire2: Vec<&str> = lines[1].split(",").collect();

    println!(
        "Distance : {}",
        find_closest_intersection(wire1, wire2).unwrap()
    );
}
