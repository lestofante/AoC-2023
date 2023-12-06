use rayon::prelude::*;
use module::*;

generate_map!("../../data/test.txt");

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

    let mut input_file_sections = include_str!("../../../data/test.txt").split("\n\n");
    
    let mut seeds_ranges = input_file_sections.next().unwrap().split_whitespace().skip(1);
    let mut min_location = usize::MAX;

    while let (Some(start), Some(number)) = (seeds_ranges.next(), seeds_ranges.next()) {
        let start: usize = start.parse().unwrap();
        let end: usize = start + number.parse::<usize>().unwrap();
        let mut ranges: Vec<_> = (start..=end).collect();
        let v = ranges.iter_mut().map(|v| get_chain( *v)).reduce(|a, b| a.min(b)).unwrap();
        if v < min_location{
            min_location = v;
        }
    }
    println!("min_location is {min_location}");
    //assert!(min_location == 47909639);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;    

    generate_map!("../../data/test.txt");
    #[test]
    fn test() {
        
        assert!(seed_to_soil_map(79) == 81);
        assert!(soil_to_fertilizer_map(81) == 81);
        assert!(fertilizer_to_water_map(81) == 81);
        assert!(water_to_light_map(81) == 74);
        assert!(light_to_temperature_map(74) == 78);
        assert!(temperature_to_humidity_map(78) == 78);
        assert!(humidity_to_location_map(78) == 82);
        // assert!(get_chain(79) == 82); //this break as fuction may use real input file

        assert!(seed_to_soil_map(14) == 14);
        assert!(soil_to_fertilizer_map(14) == 53);
        assert!(fertilizer_to_water_map(53) == 49);
        assert!(water_to_light_map(49) == 42);
        assert!(light_to_temperature_map(42) == 42);
        assert!(temperature_to_humidity_map(42) == 43);
        assert!(humidity_to_location_map(43) == 43);
        // assert!(get_chain(14) == 43);

        assert!(seed_to_soil_map(55) == 57);
        assert!(soil_to_fertilizer_map(57) == 57);
        assert!(fertilizer_to_water_map(57) == 53);
        assert!(water_to_light_map(53) == 46);
        assert!(light_to_temperature_map(46) == 82);
        assert!(temperature_to_humidity_map(82) == 82);
        assert!(humidity_to_location_map(82) == 86);
        // assert!(get_chain(55) == 86);

        assert!(seed_to_soil_map(13) == 13);
        assert!(soil_to_fertilizer_map(13) == 52);
        assert!(fertilizer_to_water_map(52) == 41);
        assert!(water_to_light_map(41) == 34);
        assert!(light_to_temperature_map(34) == 34);
        assert!(temperature_to_humidity_map(34) == 35);
        assert!(humidity_to_location_map(35) == 35);
        // assert!(get_chain(13) == 35);
    }

}
