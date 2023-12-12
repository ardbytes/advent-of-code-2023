use regex::Regex;
use std::collections::HashMap;

fn next_step<'a>(map: &'a HashMap<String, Vec<String>>, key: &'a String, step: &char) -> &'a String {
    if step == &'L' {
        map.get(key).unwrap().first().unwrap()
    } else {
        map.get(key).unwrap().last().unwrap()
    }
}

fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();

    let steps_regex = Regex::new(r"(?<steps>(^[LR]+$))").unwrap();
    let sep_regex = Regex::new(r"[\(\),]").unwrap();

    let mut steps = vec![];

    if let Some(captures) = steps_regex.captures(lines[0]) {
        steps.append(&mut captures["steps"].chars().collect::<Vec<_>>());
    }

    let graph = lines[1..].iter().fold(HashMap::new(), |mut result, line|{
        if line.contains(" = ") {
            let l = sep_regex.replace_all(line, "");
            let mut path_components = l.split(" = ").collect::<Vec<_>>();
            let source = path_components.remove(0).to_string();
            let children: Vec<String> = path_components.remove(0).split_whitespace().map(|c| c.to_string()).collect::<Vec<_>>();
            result.insert(source, children);
        }
        result
    });

    let mut step_cycles = steps.iter().cycle();
    let mut current = &"AAA".to_string();
    let end = &"ZZZ".to_string();
    let mut part1 = 0;
    while current != end {
        let step = step_cycles.next().unwrap();
        current = next_step(&graph, &current, step);
        part1 += 1;
    }

    dbg!(part1);

    step_cycles = steps.iter().cycle();
    let mut current_p2: Vec<_> = graph.keys().filter(|k| k.ends_with("A")).collect();
    let mut part2 = 0;
    loop {
        let step = step_cycles.next().unwrap();
        current_p2.iter_mut().for_each(|k|{
            *k = next_step(&graph, k, step)
        });
        part2 += 1;
        if part2 % 10000000 == 0 {
            dbg!(&current_p2);
        }
        let ended = current_p2.iter().all(|k|{
            k.ends_with("Z")
        });
        if ended {
            break;
        }
    }
    dbg!(part2);
}
