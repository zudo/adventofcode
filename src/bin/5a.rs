use adventofcode::*;
use std::collections::HashMap;
fn main() {
    tracing();
    let text = read("input.txt");
    let mut groups = text.split("\n\n").into_iter();
    debug!(?groups);
    let mut map = HashMap::new();
    let seeds = groups
        .next()
        .unwrap()
        .split("seeds: ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    // info!(?seeds);
    for group in groups {
        let mut lines = group.split("\n");
        let first = lines.next().unwrap();
        let k = first.split(" map:").next().unwrap();
        map.insert(k, vec![]);
        for line in lines {
            let mut v = map.get(k).unwrap().clone();
            // info!(?line);
            let a = line
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let destination_range_start = a[0];
            let source_range_start = a[1];
            let range_length = a[2];
            v.push((
                source_range_start..source_range_start + range_length,
                destination_range_start..destination_range_start + range_length,
            ));
            map.insert(k, v.to_vec());
        }
    }
    // for (k, v) in map.iter() {
    //     info!(?k, ?v);
    // }
    let mut locations = vec![];
    // for seed in seeds.iter().cloned().take(1) {
    for seed in seeds {
        let map_next = |i: u64, k: &str| {
            let mut next = i;
            for vec in map.get(k).unwrap() {
                info!(?vec);
                for (j, v) in vec.clone().0.enumerate() {
                    if v == i {
                        next = vec.1.clone().nth(j).unwrap();
                        // info!(?vec);
                    }
                }
            }
            next
        };
        // info!(?seed);
        let soil = map_next(seed, "seed-to-soil");
        // info!(?soil);
        let fertilizer = map_next(soil, "soil-to-fertilizer");
        // info!(?fertilizer);
        let water = map_next(fertilizer, "fertilizer-to-water");
        // info!(?water);
        let light = map_next(water, "water-to-light");
        // info!(?light);
        let temperature = map_next(light, "light-to-temperature");
        // info!(?temperature);
        let humidity = map_next(temperature, "temperature-to-humidity");
        // info!(?humidity);
        let location = map_next(humidity, "humidity-to-location");
        // info!(?location);
        // let water = map_next(seed, "fertilizer-to-water");
        // let light = map_next(seed, "water-to-light");
        // let temperature = map_next(seed, "light-to-temperature");
        // let humidity = map_next(seed, "temperature-to-humidity");
        // let location = map_next(seed, "humidity-to-location");
        locations.push((seed, location));
        info!(?seed);
    }
    info!("asdf");
}
