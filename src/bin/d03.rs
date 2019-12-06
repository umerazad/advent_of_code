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
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Line {
        // Lines are axis-aligned so just making sure that
        // all inputs are either horizontal or vertical.
        if p1.x != p2.x && p1.y != p2.y {
            panic!(
                "Only horizontal or vertical lines are allowed. ({:?},{:?})",
                p1, p2
            );
        }

        Line { p1, p2 }
    }

    // Checks whether the line segment contains
    // the point.
    fn contains(&self, p: &Point) -> bool {
        let x1 = self.p1.x.min(self.p2.x);
        let x2 = self.p1.x.max(self.p2.x);
        let y1 = self.p1.y.min(self.p2.y);
        let y2 = self.p1.y.max(self.p2.y);

        // Since all lines are either horizontal or vertical
        // we we'll just use this simple heuristic.
        if p.x < x1 || p.x > x2 || p.y < y1 || p.y > y2 {
            false
        } else {
            true
        }
    }

    fn length(&self) -> i64 {
        self.p1.distance(&self.p2)
    }

    fn intersects(&self, other: &Line) -> Option<Point> {
        let a1 = self.p2.y - self.p1.y;
        let b1 = self.p1.x - self.p2.x;
        let c1 = a1 * self.p1.x + b1 * self.p1.y;

        let a2 = other.p2.y - other.p1.y;
        let b2 = other.p1.x - other.p2.x;
        let c2 = a2 * other.p1.x + b2 * other.p1.y;

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

        Line::new(self.cursor, to)
    }

    fn insert(&mut self, path: &str) {
        let line = self.get_next_line(path);
        self.lines.push(line);

        // Update cursor location.
        self.cursor = Point::new(line.p2.x, line.p2.y);
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

    // Returns the cost of intersection in terms of panel's wiring.
    fn find_intersection_cost(&self, input: &Line) -> Option<Vec<(Point, i64)>> {
        let mut result = vec![];
        let mut distance = 0;
        for l in self.lines.iter() {
            match l.intersects(input) {
                Some(p) => {
                    distance += l.p1.distance(&p);
                    result.push((p, distance));
                }
                None => {
                    distance += l.length();
                }
            }
        }

        if result.is_empty() {
            return None;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let l1 = Line::new(Point::new(6, 3), Point::new(6, 7));
        let l2 = Line::new(Point::new(3, 5), Point::new(8, 5));
        assert_eq!(l1.intersects(&l2), Some(Point::new(6, 5)));

        let l1 = Line::new(Point::new(0, 0), Point::new(8, 0));
        let l2 = Line::new(Point::new(0, 1), Point::new(0, 7));
        assert_eq!(l1.intersects(&l2), None);

        let l1 = Line::new(Point::new(66, 62), Point::new(66, 117));
        let l2 = Line::new(Point::new(0, 0), Point::new(75, 0));
        assert_eq!(l1.intersects(&l2), None);
        assert_eq!(l2.intersects(&l1), None);
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

    #[test]
    fn test_find_cheapest_intersection() {
        let wire1 = vec!["R8", "U5", "L5", "D3"];
        let wire2 = vec!["U7", "R6", "D4", "L4"];
        assert_eq!(find_cheapest_intersection(wire1, wire2), Some(30));

        let wire1 = vec![
            "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        ];
        let wire2 = vec![
            "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
        ];
        assert_eq!(find_cheapest_intersection(wire1, wire2), Some(410));

        let wire1 = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let wire2 = vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(find_cheapest_intersection(wire1, wire2), Some(610));
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

fn find_cheapest_intersection(wire1: Vec<&str>, wire2: Vec<&str>) -> Option<i64> {
    let mut panel = Panel::new();
    // Layout wires of first panel.
    for x in wire1.iter() {
        panel.insert(x);
    }

    let mut cost = 0;
    let mut other = Panel::new();
    let mut absolute_min = i64::max_value();
    for w in wire2.iter() {
        let l = other.get_next_line(w);
        other.insert(w);
        if let Some(v) = panel.find_intersection_cost(&l) {
            let mut min = i64::max_value();
            for (p, wire1_cost) in v.iter() {
                let tmp = cost + wire1_cost + l.p1.distance(&p);
                if tmp < min {
                    min = tmp;
                }
            }

            if min < absolute_min {
                absolute_min = min;
            }
        }
        cost += l.length();
    }

    if absolute_min < i64::max_value() {
        return Some(absolute_min);
    }
    None
}

fn main() {
    let contents = fs::read_to_string("assets/day3_input").unwrap();
    let lines: Vec<&str> = contents.lines().filter(|l| l.len() > 1).collect();
    assert_eq!(lines.len(), 2);

    let wire1: Vec<&str> = lines[0].split(",").collect();
    let wire2: Vec<&str> = lines[1].split(",").collect();

    println!(
        "Closest Distance : {}",
        find_closest_intersection(wire1, wire2).unwrap()
    );

    let wire1: Vec<&str> = lines[0].split(",").collect();
    let wire2: Vec<&str> = lines[1].split(",").collect();

    println!(
        "Closest Cost: {}",
        find_cheapest_intersection(wire1, wire2).unwrap()
    );
}
