use std::io::{self, Read};
use std::cmp::{min, max};
use std::ops::{Index, IndexMut};
//use std::assert

type Point = (i32, i32);

type Path = Vec<Point>;

fn read_path(input: &str) -> Path {
    let mut result: Vec<Point> = vec![(0, 0)];
    for edge in input.split(",").into_iter().map(|x| x.trim()) {
        if edge.len() == 0 {
            continue;
        }
        let (x, y) = result.last().unwrap();
        let length = edge[1..].parse::<i32>().unwrap();
        let next = match edge.chars().nth(0) {
            Some('R') => (x + length, *y),
            Some('L') => (x - length, *y),
            Some('U') => (*x, y - length),
            Some('D') => (*x, y + length),
            _ => panic!("invalid direction: {}")
        };
        result.push(next);
    }
    result
}

fn read_paths(input: &str) -> (Path, Path) {
    let mut paths: Vec<Path> = input.split("\n").into_iter().map(|line| read_path(line)).collect();
    let second = paths.remove(1);
    let first = paths.remove(0);
    (first, second)
}

fn bounding_box(bbox: &(Point, Point), path: &Path) -> (Point, Point) {
    let (mut topleft, mut bottomright) = bbox.clone();
    for (x, y) in path {
        topleft = (min(topleft.0, *x), min(topleft.1, *y));
        bottomright = (max(bottomright.0, *x), max(bottomright.1, *y));
    }
    (topleft, (bottomright.0 + 1, bottomright.1 + 1))
}

struct Field<T: Clone> {
    data: Vec<T>,
    size: (i32, i32),
    offset: (i32, i32),
}

impl<T: Clone> Field<T> {
    fn new(size: (i32, i32), offset: (i32, i32), value: T) -> Field<T> {
        let (cols, rows) = size;
        let data = vec![value; (cols * rows) as usize];
        Field{ data, size, offset}
    }
}

impl<T: Clone> Index<Point> for Field<T> {
    type Output = T;
    fn index(self: & Field<T>, index: Point) -> &T {
        let (x, y) = index;
        let (sx, sy) = (x + self.offset.0, y + self.offset.1);
        assert!(sx >= 0, sx < self.size.0);
        assert!(sy >= 0, sy < self.size.1);
        &self.data[(sy * self.size.1 + sx) as usize]
    }
}

impl<T: Clone> IndexMut<Point> for Field<T> {
    fn index_mut(self: &mut Field<T>, index: Point) -> &mut T {
        let (x, y) = index;
        let (sx, sy) = (x + self.offset.0, y + self.offset.1);
        assert!(sx >= 0, sx < self.size.0);
        assert!(sy >= 0, sy < self.size.1);
        &mut self.data[(sy * self.size.1 + sx) as usize]
    }
}

fn do_path<F>(a: &Point, b: &Point, mut fun: F) where F: FnMut(Point) -> () {
    let mut curr = a.clone();
    while &curr != b {
        if curr.0 > b.0 {
            curr.0 -= 1;
        } else if curr.0 < b.0 {
            curr.0 += 1;
        }
        if curr.1 > b.1 {
            curr.1 -= 1;
        } else if curr.1 < b.1 {
            curr.1 += 1;
        }
        fun(curr)
    }
}

fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

fn from_origin(a: &Point) -> i32 {
    abs( a.0 ) + abs ( a.1 )
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let (path1, path2) = read_paths(&input);

    let mut bbox = ((0, 0), (0, 0));
    bbox = bounding_box(&bbox, &path1);
    bbox = bounding_box(&bbox, &path2);

    let offset = (- bbox.0 .0, - bbox.0 .1);
    let size = (bbox.1 .0 - bbox.0 .0, bbox.1 .1 - bbox.0 .1);

    let mut map: Field<i32> = Field::new(size, offset, 0);

    {
        let mut last = (0, 0);
        let mut steps = 0;
        for piece in &path1 {
            do_path(&last, &piece, |x| {
                steps += 1;
                map[x] = steps;
            });
            last = *piece;
        }
    }

    {
        let mut best_cross: Option<Point> = None;
        let mut last = (0, 0);
        for piece in &path2 {
            do_path(&last, &piece, |x| {
                if map[x] == 0 { return; }
                match best_cross {
                    Some(old_best) => {
                        if from_origin(&x) < from_origin(&old_best) {
                            println!("new best cross: {:?} ({})", x, from_origin(&x));
                            best_cross = Some(x);
                        }
                    },
                    None => {
                        println!("first best cross: {:?} ({})", x, from_origin(&x));
                        best_cross = Some(x);
                    }
                }
            });
            last = *piece;
        }
    }

    {
        let mut last = (0, 0);
        let mut earliest_cross: Option<i32> = None;
        let mut steps = 0;
        for piece in &path2 {
            do_path(&last, &piece, |x| {
                steps += 1;
                if map[x] == 0 { return; }
                let total_steps = map[x] + steps;
                match earliest_cross {
                    Some(old_earliest) => {
                        if total_steps < old_earliest {
                            println!("new earliest cross: {} + {} = {} ({:?})", map[x], steps, total_steps, piece);
                            earliest_cross = Some(total_steps);
                        }
                    },
                    None => {
                        println!("first earliest cross: {} + {} = {} ({:?})", map[x], steps, total_steps, piece);
                        earliest_cross = Some(total_steps);
                    }
                }
            });
            last = *piece;
        }
    }
}
