#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Graph::new(input).count_triples(|s| s.starts_with("t")))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(Graph::new(input).find_maximal_clique())
}

struct Graph {
    ids: IdMap<String>,
    adjacent: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let pairs = parser!(lines(string(any_char any_char) "-" string(any_char any_char)))
            .parse(input)
            .unwrap();

        let mut ids = IdMap::new();
        let mut adjacent = HashMap::new();
        for (a, b) in pairs {
            let ai = ids.id(&a);
            let bi = ids.id(&b);
            adjacent.entry(ai).or_insert(HashSet::new()).insert(bi);
            adjacent.entry(bi).or_insert(HashSet::new()).insert(ai);
        }

        Self { ids, adjacent }
    }

    fn key(&self, id: &usize) -> &String {
        self.ids.key(id).unwrap()
    }

    fn neighbors(&self, v: &usize) -> &HashSet<usize> {
        self.adjacent.get(v).unwrap()
    }

    fn count_triples<F>(&self, filter: F) -> usize
    where
        F: Fn(&String) -> bool,
    {
        let mut count = 0;
        for (a, a_neighbors) in &self.adjacent {
            for b in a_neighbors.iter().filter(|b| *a < **b) {
                for c in self
                    .neighbors(&b)
                    .iter()
                    .filter(|c| *b < **c && a_neighbors.contains(*c))
                {
                    if filter(self.key(a)) || filter(self.key(b)) || filter(self.key(c)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn bron_kerbosch_with_pivot(
        &self,
        r: HashSet<usize>,
        mut p: HashSet<usize>,
        mut x: HashSet<usize>,
        cliques: &mut Vec<HashSet<usize>>,
    ) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r);
            return;
        }

        let pivot = p.iter().chain(x.iter()).take(1).next().unwrap();
        for v in p.clone().difference(self.neighbors(&pivot)) {
            let v_neighbors = self.neighbors(&v);

            let mut new_r = r.clone();
            new_r.insert(*v);

            let new_p = p.intersection(v_neighbors).cloned().collect();
            let new_x = x.intersection(v_neighbors).cloned().collect();

            self.bron_kerbosch_with_pivot(new_r, new_p, new_x, cliques);

            p.remove(v);
            x.insert(*v);
        }
    }

    fn find_maximal_clique(&self) -> String {
        let mut cliques = vec![];
        let all_vertices: HashSet<usize> = self.adjacent.keys().cloned().collect();
        self.bron_kerbosch_with_pivot(HashSet::new(), all_vertices, HashSet::new(), &mut cliques);
        cliques.sort_by(|a, b| b.len().cmp(&a.len()));
        cliques
            .into_iter()
            .next()
            .expect("Failed to find maximal clique")
            .iter()
            .map(|id| self.key(id))
            .sorted()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
