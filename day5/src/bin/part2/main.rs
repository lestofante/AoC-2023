mod generated;

use rayon::prelude::*;
use crate::generated::*;


#[derive(Debug)]
struct Range{
    destination_range_start: usize,
    source_range_start: usize,
    len: usize,
}

fn loader(section: &str) -> (String, Vec<Range>) {
    let mut lines = section.lines();
    let name = lines.next().unwrap().replace(" ", "-").replace("-", "_").replace(":", "");
    let values: Vec<Range> = lines.map(|line| {
        let values: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        Range{destination_range_start: values[0], source_range_start: values[1], len: values[2]}
    }).collect();
    return (name, values);
}

fn as_file(name:&str, range: &Vec<Range>){
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("generated.rs")).expect("open file fail");

    let mut content = "\n\npub fn ".to_owned()+name+"(i: usize) -> usize{\nreturn match i {";

    for r in range{
        content = format!("{content}\n {}..={} => {} + i - {},",
            r.source_range_start,
            r.source_range_start+r.len - 1,
            r.destination_range_start,
            r.source_range_start,
        );
    }
    content += "\n_ => i\n};\n}";
    // Write the content to the file
    std::io::Write::write_all(&mut file, content.as_bytes()).expect("file creation error");
}

fn get_chain(v: usize) -> usize{
    let ris = seed_to_soil_map(v);
    let ris = soil_to_fertilizer_map(ris);
    let ris = fertilizer_to_water_map(ris);
    let ris = water_to_light_map(ris);
    let ris = light_to_temperature_map(ris);
    let ris = temperature_to_humidity_map(ris);
    let ris = humidity_to_location_map(ris);
    ris
}

fn main() {
/*
    //PART 1: from the data, generate the file with functions and matches
    let mut input_file_sections = include_str!("../../../data/input.txt").split("\n\n");
    
    let mut seeds_ranges = input_file_sections.next().unwrap().split_whitespace().skip(1);

    for section in input_file_sections{
        let touple = loader(section);
        as_file(&touple.0, &touple.1);
    }
*/

    //PART 2: from the generated file 
    let mut input_file_sections = include_str!("../../../data/input.txt").split("\n\n");
    
    let mut seeds_ranges = input_file_sections.next().unwrap().split_whitespace().skip(1);
    let mut min_location = usize::MAX;

    while let (Some(start), Some(number)) = (seeds_ranges.next(), seeds_ranges.next()) {
        let start: usize = start.parse().unwrap();
        let end: usize = start + number.parse::<usize>().unwrap();
        let mut ranges: Vec<_> = (start..=end).collect();
        let v = ranges.par_iter_mut().map(|v| get_chain( *v)).reduce_with(|a, b| a.min(b)).unwrap();
        if v < min_location{
            min_location = v;
        }
    }
    println!("min_location is {min_location}");

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test() {
        assert!(seed_to_soil_map(79) == 81);
        assert!(soil_to_fertilizer_map(81) == 81);
        assert!(fertilizer_to_water_map(81) == 81);
        assert!(water_to_light_map(81) == 74);
        assert!(light_to_temperature_map(74) == 78);
        assert!(temperature_to_humidity_map(78) == 78);
        assert!(humidity_to_location_map(78) == 82);
        assert!(get_chain(79) == 82);

        assert!(seed_to_soil_map(14) == 14);
        assert!(soil_to_fertilizer_map(14) == 53);
        assert!(fertilizer_to_water_map(53) == 49);
        assert!(water_to_light_map(49) == 42);
        assert!(light_to_temperature_map(42) == 42);
        assert!(temperature_to_humidity_map(42) == 43);
        assert!(humidity_to_location_map(43) == 43);
        assert!(get_chain(14) == 43);

        assert!(seed_to_soil_map(55) == 57);
        assert!(soil_to_fertilizer_map(57) == 57);
        assert!(fertilizer_to_water_map(57) == 53);
        assert!(water_to_light_map(53) == 46);
        assert!(light_to_temperature_map(46) == 82);
        assert!(temperature_to_humidity_map(82) == 82);
        assert!(humidity_to_location_map(82) == 86);
        assert!(get_chain(55) == 86);

        assert!(seed_to_soil_map(13) == 13);
        assert!(soil_to_fertilizer_map(13) == 52);
        assert!(fertilizer_to_water_map(52) == 41);
        assert!(water_to_light_map(41) == 34);
        assert!(light_to_temperature_map(34) == 34);
        assert!(temperature_to_humidity_map(34) == 35);
        assert!(humidity_to_location_map(35) == 35);
        assert!(get_chain(13) == 35);
    }

}
