use std::collections::{HashMap, HashSet, VecDeque};

pub fn sum_middle_pages_correctly_ordered(input: &str) -> String {
    let (edges, updates) = parse_input(input);

    // Graph adjacency sets
    let mut e: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Parse edges
    for line in edges.lines() {
        let mut split = line.split('|');
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        e.entry(y).or_default().insert(x);
    }

    let mut total_sum = 0;

    // Process updates
    for update in updates.lines() {
        let pages: Vec<i32> = update
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert!(pages.len() % 2 == 1);

        let mut is_valid = true;

        for (i, &x) in pages.iter().enumerate() {
            for (j, &y) in pages.iter().enumerate() {
                if i < j && e.get(&x).map_or(false, |deps| deps.contains(&y)) {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }

        if is_valid {
            total_sum += pages[pages.len() / 2]; // Add the middle page number
        }
    }

    total_sum.to_string()
}

pub fn sum_middle_pages_after_fixing_order(input: &str) -> String {
    let (edges, updates) = parse_input(input);

    // Graph adjacency sets
    let mut e: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut er: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Parse edges
    for line in edges.lines() {
        let mut split = line.split('|');
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        e.entry(y).or_default().insert(x);
        er.entry(x).or_default().insert(y);
    }

    let mut total_sum = 0;

    // Process updates
    for update in updates.lines() {
        let pages: Vec<i32> = update
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert!(pages.len() % 2 == 1);

        let mut is_valid = true;

        for (i, &x) in pages.iter().enumerate() {
            for (j, &y) in pages.iter().enumerate() {
                if i < j && e.get(&x).map_or(false, |deps| deps.contains(&y)) {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }

        if !is_valid {
            // Fix the order using topological sorting
            let mut fixed_order = Vec::new();
            let mut q: VecDeque<i32> = VecDeque::new();
            let mut dependencies: HashMap<i32, usize> = HashMap::new();

            for &page in &pages {
                let dependency_count = e.get(&page).map_or(0, |deps| {
                    deps.intersection(&pages.iter().cloned().collect()).count()
                });
                dependencies.insert(page, dependency_count);
                if dependency_count == 0 {
                    q.push_back(page);
                }
            }

            while let Some(page) = q.pop_front() {
                fixed_order.push(page);
                if let Some(children) = er.get(&page) {
                    for &child in children {
                        if dependencies.contains_key(&child) {
                            *dependencies.get_mut(&child).unwrap() -= 1;
                            if dependencies[&child] == 0 {
                                q.push_back(child);
                            }
                        }
                    }
                }
            }

            // Add the middle page number of the fixed order
            total_sum += fixed_order[fixed_order.len() / 2];
        }
    }

    total_sum.to_string()
}

fn parse_input(input: &str) -> (&str, &str) {
    let mut parts = input.split("\n\n");
    let edges = parts.next().unwrap();
    let updates = parts.next().unwrap();
    (edges, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_middle_pages_correctly_ordered() {
        let input = "47|53\n97|13\n97|61\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n";
        assert_eq!(sum_middle_pages_correctly_ordered(input), "143");
    }

    #[test]
    fn test_sum_middle_pages_after_fixing_order() {
        let input = "47|53\n97|13\n97|61\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n";
        assert_eq!(sum_middle_pages_after_fixing_order(input), "0");
    }
}
