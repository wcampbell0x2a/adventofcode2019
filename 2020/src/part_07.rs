use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Bags {
    bags: HashMap<String, Vec<(String, usize)>>,
}

impl Bags {
    fn part1(&self, color: &str) -> bool {
        self.bags[color]
            .iter()
            .any(|(color, _)| color == "shiny gold" || self.part1(color))
    }

    fn part2(&self, color: &str) -> usize {
        self.bags[color]
            .iter()
            .map(|(c, count)| count * self.part2(c))
            .sum::<usize>()
            + 1
    }
}

#[must_use] pub fn seven() -> (usize, usize) {
    let input = std::fs::read_to_string("data/07.in").unwrap();
    let mut bags = HashMap::new();

    for line in input.lines() {
        let parse_iter = line.split("bags contain").collect::<Vec<&str>>();

        let key = parse_iter[0].trim();
        let value = parse_iter[1].trim();

        let value = value
            .split(&[',', '.'][..])
            .filter_map(|x| if x.is_empty() { None } else { Some(x.trim()) })
            .collect::<Vec<_>>();

        if value == ["no other bags"] {
            bags.entry(key.to_string()).or_insert(vec![]);
            continue;
        }

        for bag in value {
            let bag_value = bag.split_ascii_whitespace().collect::<Vec<&str>>();
            let num = bag_value[0];
            let adjective = bag_value[1];
            let color = bag_value[2];

            let entry = bags.entry(key.to_string()).or_insert(vec![]);

            entry.push((
                format!("{} {}", adjective, color),
                num.parse::<usize>().unwrap(),
            ));
        }
    }

    let bags = Bags { bags };

    let part1 = bags
        .bags
        .iter()
        .filter(|&(color, _)| bags.part1(color))
        .count();

    let part2 = bags.part2("shiny gold") - 1;
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven() {
        assert_eq!((274, 158_730), seven());
    }
}
