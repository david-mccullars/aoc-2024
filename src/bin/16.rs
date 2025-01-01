#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::HashSet;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Maze::new(input).min_score())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Maze::new(input).best_seats())
}

struct Maze {
    walls: HashSet<Pos>,
    start: Pos,
    end: Pos,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#SE")).parse(input).unwrap();

        Self {
            walls: grid.take_all('#'),
            start: grid.take_one('S'),
            end: grid.take_one('E'),
        }
    }

    fn min_score(&self) -> usize {
        let mut successors = |&n: &Node| self.successor(n);
        let mut success = |&n: &Node| n.0 == self.end;

        let (_, min) = dijkstra(&self.start(), successors, success).unwrap();
        min
    }

    fn best_seats(&self) -> usize {
        let mut successors = |&n: &Node| self.successor(n);
        let mut success = |&n: &Node| n.0 == self.end;

        let (visited, _) = dijkstra_multi(&self.start(), successors, success).unwrap();
        visited.into_iter().unique_by(|n| n.0).count()
    }

    fn start(&self) -> Node {
        (self.start.clone(), Direction::East)
    }

    fn successor<'a>(&'a self, node: Node) -> impl Iterator<Item = (Node, usize)> + 'a {
        let (pos, dir) = node;
        [dir, dir.turn_left(), dir.turn_right()]
            .into_iter()
            .filter_map(move |dir2| {
                let cost = if dir == dir2 { 1 } else { 1001 };
                let pos2 = dir2.forward_from(&pos);
                if !self.walls.contains(&pos) {
                    Some(((pos2, dir2), cost))
                } else {
                    None
                }
            })
    }
}

type Node = (Pos, Direction);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_a() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_b() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
