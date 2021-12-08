use std::fmt;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn to_rasterized(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];

        let mut x = self.start.x;
        let mut y = self.start.y;

        while x != self.end.x || y != self.end.y {
            points.push(Point { x, y });

            if x < self.end.x {
                x += 1;
            } else if x > self.end.x {
                x -= 1;
            }

            if y < self.end.y {
                y += 1;
            } else if y > self.end.y {
                y -= 1;
            }
        }

        points.push(Point {
            x: self.end.x,
            y: self.end.y,
        });

        points
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}
