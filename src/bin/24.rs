#[allow(unused_imports)]
use advent_of_code::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use topological_sort::TopologicalSort;

advent_of_code::solution!(24);

const NUMBITS: usize = 45;

pub fn part_one(input: &str) -> Option<u64> {
    Some(Wires::new(input).value("z"))
}

pub fn part_two(input: &str) -> Option<String> {
    Wires::new(input).fix(4)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

struct Wires {
    ids: IdMap<String>,
    wires: HashMap<usize, bool>,
    rules: BiMap<usize, (usize, Op, usize)>,
    rule_parts: HashMap<(usize, Op), usize>,
    swaps: HashMap<usize, usize>,
}

impl Wires {
    fn new(input: &str) -> Self {
        let (section1, section2) = parser!(
            section(lines(
                string(alnum+) ": " u8
            ))
            section(lines(
                string(alnum+)
                " " { "XOR" => Op::Xor, "OR" => Op::Or, "AND" => Op::And } " "
                string(alnum+) " -> " string(alnum+)
            ))
        )
        .parse(input)
        .unwrap();

        let mut ids = IdMap::new();
        let mut wires = HashMap::new();
        let mut rules = BiMap::new();
        let mut rule_parts = HashMap::new();
        let mut swaps = HashMap::new();

        for (k, v) in &section1 {
            wires.insert(ids.id(k), *v > 0);
        }
        for (w1, op, w2, w3) in &section2 {
            let (w1, w2, w3) = (ids.id(w1), ids.id(w2), ids.id(w3));
            rules.insert(
                w3,
                if w1 < w2 {
                    (w1, op.clone(), w2)
                } else {
                    (w2, op.clone(), w1)
                },
            );
            rule_parts.insert((w1, op.clone()), w2);
            rule_parts.insert((w2, op.clone()), w1);
        }

        Self {
            ids,
            wires,
            rules,
            rule_parts,
            swaps,
        }
    }

    fn xyz(&mut self, bit: usize) -> (usize, usize, usize) {
        (
            self.bit_id("x", bit),
            self.bit_id("y", bit),
            self.bit_id("z", bit),
        )
    }

    fn bit_id(&mut self, var: &str, bit: usize) -> usize {
        self.ids.id(&format!("{}{:0>2}", var, bit))
    }

    fn rule(&self, a: usize, op: Op, b: usize) -> Option<usize> {
        self.rules
            .iget(&if a < b { (a, op, b) } else { (b, op, a) })
            .map(|c| self.swaps.get(c).unwrap_or(c))
            .copied()
    }

    fn rule_part(&self, a: usize, op: Op) -> (Option<usize>, Option<usize>) {
        self.rule_parts
            .get(&(a, op.clone()))
            .map(|b| (Some(*b), self.rule(a, op, *b)))
            .unwrap_or_else(|| (None, None))
    }

    fn evaluate(&self) -> Vec<bool> {
        let mut wires = vec![false; self.ids.next_id()];
        let mut ts = TopologicalSort::<usize>::new();

        for (w3, (w1, op, w2)) in &self.rules {
            if !self.wires.contains_key(w1) {
                ts.add_dependency(*w1, *w3);
            }
            if !self.wires.contains_key(w2) {
                ts.add_dependency(*w2, *w3);
            }
        }

        for (w, v) in &self.wires {
            wires[*w] = *v;
        }
        for w in ts {
            let (w1, op, w2) = self.rules.get(&w).unwrap();
            let v1 = wires[*w1];
            let v2 = wires[*w2];
            wires[w] = match op {
                Op::And => v1 & v2,
                Op::Or => v1 | v2,
                Op::Xor => v1 ^ v2,
            };
        }
        wires
    }

    fn value(&self, var: &str) -> u64 {
        let mut value: u64 = 0;
        for (w, v) in self.evaluate().into_iter().enumerate() {
            let name = self.ids.key(&w).unwrap();
            if v && name.starts_with(var) {
                let bit = name[1..].parse::<usize>().unwrap();
                value |= 1 << bit;
            }
        }
        value
    }

    fn possibly_incorrect(&mut self) -> HashSet<usize> {
        let mut incorrect = self.rules.keys().copied().collect::<HashSet<usize>>();
        let mut prev_carry: Option<usize> = None;
        for bit in (0..NUMBITS) {
            let (x, y, z) = self.xyz(bit);
            let gate = NandGate::new(self, x, y, bit == 0);
            if gate.is_correct(z, prev_carry) {
                for w in gate.flatten() {
                    incorrect.remove(&w);
                }
            }
            prev_carry = gate.cout;
        }
        incorrect
    }

    fn possible_swaps(&mut self) -> Vec<HashMap<usize, usize>> {
        let problematic = self.possibly_incorrect();
        problematic
            .iter()
            .copied()
            .tuple_combinations()
            .map(|(i, j)| HashMap::from([(i, j), (j, i)]))
            .filter(|swaps| {
                self.swaps = swaps.clone();
                let still_problematic = self.possibly_incorrect();
                still_problematic.len() < problematic.len()
                    && still_problematic.is_subset(&problematic)
            })
            .collect()
    }

    fn fix(&mut self, swap_count: usize) -> Option<String> {
        for swaps in self.possible_swaps().into_iter().combinations(swap_count) {
            self.swaps = HashMap::new();
            for swap in swaps {
                self.swaps.extend(&swap);
            }
            if self.swaps.len() == swap_count * 2 && self.possibly_incorrect().is_empty() {
                return Some(
                    self.swaps
                        .keys()
                        .map(|i| self.ids.key(i).unwrap())
                        .sorted()
                        .join(","),
                );
            }
        }
        None
    }
}

/*
Given digits x & y with input carry (cin), a full adder is implemented as:

    has = x XOR y       // half-adder sum
    hac = x AND y       // half-adder carry

    int = d AND cin     // intermediate

    sum = has XOR cin   // full-adder sum
    cout = hac OR int   // full-adder carry

As a special case, if cin is 0 (e.g. for the 1's place):

    sum = x XOR y
    cout = x AND y
*/
#[derive(Debug)]
struct NandGate {
    cin: Option<usize>,  // (Expected) input carry
    has: Option<usize>,  // (Expected) Half-adder sum
    hac: Option<usize>,  // (Expected) Half-adder carry
    sum: Option<usize>,  // (Expected) output sum
    int: Option<usize>,  // (Expected) intermediate sum
    cout: Option<usize>, // (Expected) output carry
}

impl NandGate {
    fn new(wires: &Wires, x: usize, y: usize, ones_place: bool) -> Self {
        let has = wires.rule(x, Op::Xor, y);
        let hac = wires.rule(x, Op::And, y);
        let (cin, mut sum) = wires.rule_part(has.expect("HAS must exist"), Op::Xor);
        let (int, mut cout) = wires.rule_part(hac.expect("HAC must exist"), Op::Or);
        if ones_place {
            sum = has;
            cout = hac;
        }
        Self {
            has,
            hac,
            cin,
            sum,
            int,
            cout,
        }
    }

    fn is_correct(&self, sum: usize, prev_cout: Option<usize>) -> bool {
        // 1. Expected and actual sums should match.
        // 2. Assuming we had a previous output carry, it should match the expected input carry.
        // 3. We should have an output carry.
        self.sum == Some(sum)
            && (self.cin == prev_cout || prev_cout.is_none())
            && self.cout.is_some()
    }

    fn flatten(&self) -> impl Iterator<Item = usize> + '_ {
        vec![self.cin, self.has, self.hac, self.int, self.sum, self.cout]
            .into_iter()
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("btb,cmv,mwp,rdg,rmj,z17,z23,z30".to_owned()));
    }
}
