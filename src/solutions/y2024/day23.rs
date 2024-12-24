use std::collections::{HashMap, HashSet};

/// Parses the input and creates an adjacency list for the network graph.
fn parse_network(input: &str) -> HashMap<String, HashSet<String>> {
    let mut network = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let (a, b) = (parts[0].to_string(), parts[1].to_string());

        network
            .entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());
        network.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    network
}

/// Finds all sets of three interconnected nodes (triangles) in the network.
fn count_triangles_with_t(network: &HashMap<String, HashSet<String>>) -> usize {
    let mut count = 0;

    let nodes: Vec<&String> = network.keys().collect();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let c = nodes[k];

                if network.get(b).map_or(false, |n| n.contains(a))
                    && network.get(c).map_or(false, |n| n.contains(a))
                    && network.get(c).map_or(false, |n| n.contains(b))
                    && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Finds the largest clique in the network deterministically.
fn find_largest_clique(network: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut nodes: Vec<&String> = network.keys().collect();
    nodes.sort_unstable(); // Sort nodes for deterministic results

    let mut largest_clique = Vec::new();

    for &node in &nodes {
        let mut clique = vec![node.clone()];

        for &neighbor in &nodes {
            if neighbor != node
                && clique.iter().all(|clique_node| {
                    network
                        .get(clique_node)
                        .map_or(false, |n| n.contains(neighbor))
                })
            {
                clique.push(neighbor.clone());
            }
        }

        if clique.len() > largest_clique.len() {
            largest_clique = clique.to_vec();
        }
    }

    largest_clique
}

/// Solves Part 1 of the puzzle.
pub fn solve_part1(input: &str) -> String {
    let network = parse_network(input);
    let count = count_triangles_with_t(&network);
    count.to_string()
}

/// Solves Part 2 of the puzzle: Finds the largest clique.
pub fn solve_part2(input: &str) -> String {
    let network = parse_network(input);
    let largest_clique = find_largest_clique(&network);
    largest_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

        assert_eq!(solve_part1(input), "7");
    }

    #[test]
    fn test_example_part2() {
        let input = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

        assert_eq!(solve_part2(input), "co,de,ka,ta");
    }
}
