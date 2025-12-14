type Input = Vec<Vec<usize>>;

fn to_index(s: &str) -> usize {
    s.bytes()
        .take(3)
        .fold(0, |acc, b| 26 * acc + usize::from(b - b'a'))
}

fn parse_graph(input: &str) -> Input {
    let mut graph = vec![vec![]; 26 * 26 * 26];

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let from = parts[0].trim();
            let from_idx = to_index(from);
            let outputs: Vec<&str> = parts[1].split_whitespace().collect();
            graph[from_idx].extend(outputs.iter().map(|&s| to_index(s)));
        }
    }

    graph
}

fn paths(input: &Input, from: &str, to: &str) -> u64 {
    let mut cache = vec![u64::MAX; input.len()];
    dfs(input, &mut cache, to_index(from), to_index(to))
}

fn dfs(input: &Input, cache: &mut [u64], node: usize, end: usize) -> u64 {
    if node == end {
        1
    } else if cache[node] == u64::MAX {
        let result = input[node]
            .iter()
            .map(|&next| dfs(input, cache, next, end))
            .sum();
        cache[node] = result;
        result
    } else {
        cache[node]
    }
}

pub fn count_paths_to_out(input: &str) -> String {
    let graph = parse_graph(input);
    paths(&graph, "you", "out").to_string()
}

pub fn count_paths_to_out_part2(input: &str) -> String {
    let graph = parse_graph(input);

    // Paths from svr to out that visit both dac and fft can be split into:
    // 1. svr -> fft -> dac -> out
    // 2. svr -> dac -> fft -> out
    let one =
        paths(&graph, "svr", "fft") * paths(&graph, "fft", "dac") * paths(&graph, "dac", "out");
    let two =
        paths(&graph, "svr", "dac") * paths(&graph, "dac", "fft") * paths(&graph, "fft", "out");

    (one + two).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(count_paths_to_out(input), "5");
    }

    #[test]
    fn test_example_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(count_paths_to_out_part2(input), "2");
    }
}
