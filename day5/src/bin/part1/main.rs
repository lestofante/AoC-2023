use std::collections::HashMap;

#[derive(Debug)]
struct Range{
    destination_range_start: usize,
    source_range_start: usize,
    len: usize,
}

fn loader(section: &str) -> (String, Vec<Range>) {
    let mut lines = section.lines();
    let name = lines.next().unwrap().replace(" ", "-").replace(":", "");
    let values: Vec<Range> = lines.map(|line| {
        let values: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        Range{destination_range_start: values[0], source_range_start: values[1], len: values[2]}
    }).collect();
    return (name, values);
}

fn get_from_range(value: usize, range: &Vec<Range>) -> usize{
    for r in range{
        if value >= r.source_range_start && value <= r.source_range_start + r.len{
            return r.destination_range_start + value - r.source_range_start;
        }
    }
    return value;
}

fn get_chain(value: usize, all_maps: &HashMap<String, Vec<Range>>) -> usize{
    let list = ["seed-to-soil-map", "soil-to-fertilizer-map", "fertilizer-to-water-map", "water-to-light-map", "light-to-temperature-map", "temperature-to-humidity-map", "humidity-to-location-map"];
    
    let mut val = value;
    for str in list{
        let map = all_maps.get(str).unwrap();
        val = get_from_range(val, map);
    }
    val
}

fn main() {
    let mut input_file_sections = include_str!("../../../data/input.txt").split("\n\n");
    
    let seeds: Vec<usize> = input_file_sections.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap()).collect();

    let mut map: HashMap<String, Vec<Range>> = HashMap::new();
    for section in input_file_sections{
        let touple = loader(section);
        map.insert(touple.0, touple.1);
        
    }
    println!("map: {map:#?}");

    let mut min_location = usize::MAX;
    for s in seeds{
        let v = get_chain(s, &map);
        if v < min_location{
            min_location = v;
        }
        println!("got {v} min_location for seed {s} is {min_location}");
    }
    println!("min_location is {min_location}");
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test() {
        let mut input_file_sections = include_str!("../../../data/test.txt").split("\n\n");
    
        let seeds = input_file_sections.next().unwrap();

        let mut map: HashMap<String, Vec<Range>> = HashMap::new();
        for section in input_file_sections{
            let touple = loader(section);
            map.insert(touple.0, touple.1);
        }

        let ris = map.get("seed-to-soil-map").unwrap();
        assert!(get_from_range(79, ris) == 81);

        let ris = map.get("soil-to-fertilizer-map").unwrap();
        assert!(get_from_range(81, ris) == 81);

        let ris = map.get("fertilizer-to-water-map").unwrap();
        assert!(get_from_range(81, ris) == 81);

        let ris = map.get("water-to-light-map").unwrap();
        assert!(get_from_range(81, ris) == 74);

        let ris = map.get("light-to-temperature-map").unwrap();
        assert!(get_from_range(74, ris) == 78);

        let ris = map.get("temperature-to-humidity-map").unwrap();
        assert!(get_from_range(78, ris) == 78);

        let ris = map.get("humidity-to-location-map").unwrap();
        assert!(get_from_range(78, ris) == 82);



        let ris = map.get("seed-to-soil-map").unwrap();
        assert!(get_from_range(14, ris) == 14);

        let ris = map.get("soil-to-fertilizer-map").unwrap();
        assert!(get_from_range(14, ris) == 53);

        let ris = map.get("fertilizer-to-water-map").unwrap();
        assert!(get_from_range(53, ris) == 49);

        let ris = map.get("water-to-light-map").unwrap();
        assert!(get_from_range(49, ris) == 42);

        let ris = map.get("light-to-temperature-map").unwrap();
        assert!(get_from_range(42, ris) == 42);

        let ris = map.get("temperature-to-humidity-map").unwrap();
        assert!(get_from_range(42, ris) == 43);

        let ris = map.get("humidity-to-location-map").unwrap();
        assert!(get_from_range(43, ris) == 43);
    }

}
