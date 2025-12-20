use std::cmp::Ordering;
use crate::aoc_2024::common::Direction::{East, South, West};
use crate::aoc_2024::common::{CharMatrix, Coordinates, Direction};
#[cfg(not(test))]
use log::info;
use std::collections::HashSet;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
use std::time::Instant;
// Solution

pub fn day8(input: &String, nb_of_junctions: usize) -> String {
    let points: Vec<Point> = input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Point::from_str(s).unwrap())
        .collect();

    let links = build_links_sorted(points);

    // set up all the connections
    let mut links_connected: Vec<Link> = Vec::new();
    let mut iteration = 0;
    let nb_of_links = links.len();
    'links_loop: while links_connected.len() < nb_of_junctions && iteration < nb_of_links {
        let link: &Link = links.get(iteration).unwrap();
        iteration+=1;

        for already in &links_connected {
            if link.is_opposite(already) {
                continue 'links_loop;
            }
        }
        links_connected.push(link.clone());
        info!("Link added: {link:?}");
    }

    // build circuits with links
    let mut circuits: Vec<Circuit> = Vec::new();

    let mut last_link = None;

    for l in links_connected {
        let circuit = circuits.iter_mut()
            .filter(|x| x.contains(&l.from) || x.contains(&l.to))
            .fold(Circuit::new_with(l.from.to_owned(), l.to.to_owned()), |acc, element| acc.merge(element));

        circuits.push(circuit);
        // remove all empty circuits

        circuits.retain(|c| c.len() != 0);
        last_link = Some(l);
    }

    // Print circuits size
    circuits.iter().for_each(|c| {
        let length = c.len();
        println!("Circuit: {c:?}: {length}")
    });

    check_points_in_several_circuits(&circuits);

    circuits.sort_by(|x1, x2| x1.len().cmp(&x2.len()));

    circuits.iter().rev().take(3).map(Circuit::len).fold(1, |acc, x| acc * x).to_string()
}


pub fn day8_2(input: &String) -> String {

    let points: Vec<Point> = input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Point::from_str(s).unwrap())
        .collect();

    let nb_of_points = points.len();
    info!("parsed {nb_of_points} points");

    let links = build_links_sorted(points);

    let nb_links = links.len();

    info!("Build links {nb_links} links");

    // build circuits with links
    let mut circuits: Vec<Circuit> = Vec::new();

    let mut last_link_opt = None;
    let mut iter = links.into_iter();

    let mut iteration = 0;

    while single_containing_all_circuit(nb_of_points, &mut circuits) {
        iteration+=1;
        let next_link = iter.next();
        let l: Link;
        if next_link.is_none() {
            break;
        } else {
            l = next_link.unwrap();
        }

        let circuit = circuits.iter_mut()
            .filter(|x| x.contains(&l.from) || x.contains(&l.to))
            .fold(Circuit::new_with(l.from.to_owned(), l.to.to_owned()), |acc, element| acc.merge(element));

        circuits.push(circuit);
        // remove all empty circuits

        circuits.retain(|c| c.len() != 0);
        last_link_opt = Some(l);
        if iteration > 1000 {
            print!("Emergency exit");
        }
    }

    // Print circuits size
    circuits.iter().for_each(|c| {
        let length = c.len();
        println!("Circuit: {c:?}: {length}")
    });

    let last_link = last_link_opt.unwrap();
    (last_link.from.x * last_link.to.x).to_string()
}

fn single_containing_all_circuit(nb_of_points: usize, circuits: &mut Vec<Circuit>) -> bool {
    !(circuits.len() == 1 && circuits.first().unwrap().len() == nb_of_points)
}

fn build_links_sorted(points: Vec<Point>) -> Vec<Link> {
    let mut links: Vec<Link> = Vec::new();

    let mut linked_added = 0;

    let cloned = points.clone();

    let points_len = points.len();
    for i in 0..points_len {
        let point = points[i];
        for j in i..points_len {
            let other = points[j];
            if !point.eq(&other) {
                let new_link = Link::new(point.to_owned(), other.to_owned());
                links.push(new_link);
                    linked_added+= 1;
            }
        }
    }
    info!("links added {linked_added}");

    links.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    links
}

fn check_points_in_several_circuits(circuits: &Vec<Circuit>) {
    for circuit in circuits {
        for point in &circuit.points {
            let mut time_pres = 0;
            for circuit in circuits {
                if circuit.contains(point) {
                    time_pres += 1;
                }
            }
            if time_pres > 1 {
                info!("Point {point:?} present {time_pres} times!");
            }
        }
    }
}


// Tooling

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point{
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x: isize = split.next().unwrap().parse().unwrap();
        let y: isize = split.next().unwrap().parse().unwrap();
        let z: isize = split.next().unwrap().parse().unwrap();
        Ok(Point {x, y, z})
    }
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2) + self.z.abs_diff(other.z).pow(2)) as f64).sqrt()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Link {
    distance: f64,
    from: Point,
    to: Point
}

impl Link {

    pub fn contain(&self, p0: &Point) -> bool {
        self.from.eq(p0) || self.to.eq(p0)
    }

    pub fn is_opposite(&self, other: &Link) -> bool {
        other.to.eq(&self.from) && other.from.eq(&self.to)
    }

    pub fn share_point(&self, other: &Link) -> bool {
        self.from.eq(&other.from)
        || self.from.eq(&other.to)
        || self.to.eq(&other.from)
        || self.to.eq(&other.to)
    }

    pub fn points(&self) -> [Point; 2] {
        [self.from.clone(), self.to.clone()]
    }
    fn new(from: Point, to: Point) -> Link {
        Link { distance: from.distance(&to), from, to}
    }
}

#[derive(Debug)]
struct Circuit {
    points: HashSet<Point>
}

impl Circuit {

    pub fn merge(mut self, other: &mut Circuit) -> Self{
        other.points
            .drain()
            .for_each(|v| { self.points.insert(v); });

        let len = self.points.len();
        info!("Merged! new point list is: {len}");
        self
    }

    pub fn push(&mut self, other: Point) {
        self.points.insert(other);
    }

    fn new() -> Circuit {
        Circuit { points: HashSet::new() }
    }

    fn new_with(first: Point, second: Point) -> Circuit {
        let mut links = HashSet::new();
        links.insert(first);
        links.insert(second);
        Circuit { points: links }
    }

    fn share_point(&self, other: &Circuit) -> bool {
        other.points.iter().any(|p| self.contains(p))
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn point_distance() {
        assert_eq!(1f64, Point::from_str("0,0,0").unwrap().distance(&Point::from_str("1,0,0").unwrap()));
        assert_eq!(1f64, Point::from_str("0,0,0").unwrap().distance(&Point::from_str("0,1,0").unwrap()));
        assert_eq!(1f64, Point::from_str("0,0,0").unwrap().distance(&Point::from_str("0,0,1").unwrap()));

        assert_eq!(2f64.sqrt(), Point::from_str("0,0,0").unwrap().distance(&Point::from_str("1,1,0").unwrap()));
        assert_eq!(3f64.sqrt() , Point::from_str("0,0,0").unwrap().distance(&Point::from_str("1,1,1").unwrap()));

        assert_eq!(2f64.sqrt(), Point::from_str("5,5,5").unwrap().distance(&Point::from_str("5,6,6").unwrap()));
        assert_eq!(3f64.sqrt(), Point::from_str("5,5,5").unwrap().distance(&Point::from_str("6,4,6").unwrap()));
    }

    #[test]
    fn test_day8() {
        let input = String::from(TEST_INPUT);

        let result = day8(&input, 10);

        assert_eq!(String::from("40"), result);
    }

    #[test]
    fn test_day8_2() {
        let input = String::from(TEST_INPUT);

        let result = day8_2(&input);

        assert_eq!(String::from("25272"), result);
    }
}
