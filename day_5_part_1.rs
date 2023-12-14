use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    
    let start_time = Instant::now();
    let mut input = String::new(); 
    File::open("/home/apu/Projects/aoc/day_5/input.txt").expect("e").read_to_string(&mut input).unwrap();
    let input_lines:Vec<&str> = input.lines().collect();

    let seeds:Vec<u64> = input_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s|s.parse::<u64>().unwrap())
        .collect();

    let maps = parse_into_maps(input.as_str());

    let mut final_location_seeds = Vec::<u64>::new();
    for seed in seeds {
        let mut locations = Vec::<u64>::new();
        locations.push(seed);

        for map in maps.iter() {
            let current_seed = *locations.last().unwrap();
            
            let mut new_locations = Vec::<u64>::new(); // temp vector to store new locations

            for e in map.lines.iter() {
                let new_location_num = e.check_range(&current_seed);

                if current_seed != new_location_num {
                    new_locations.push(new_location_num);
                    break;
                } else {
                    continue;
                }
            }
            locations.extend(new_locations);
        }
        if let Some(last_location) = locations.last().cloned() {
            final_location_seeds.push(last_location);
        }
    }
    let lowest_location = final_location_seeds.sort();
    let elapsed = start_time.elapsed();
    dbg!(elapsed); 
}

fn parse_into_maps(input:&str) -> Vec<Map> {
    let map_blocks:Vec<&str> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .skip(1)
        .filter(|e|!e.is_empty())
        .collect();

    let mut map_vec = Vec::<Map>::new();
    for block in map_blocks {
        let mut new_number_mapping = Map::default();
        for number_set in block.split('\n').skip(1) {
            let number_set:Vec<&str> = number_set.split_whitespace().collect();
            new_number_mapping.lines.push(
                MapElement{
                    des_range: number_set[0].parse::<u64>().unwrap(),
                    src_range: number_set[1].parse::<u64>().unwrap(),
                    range_len: number_set[2].parse::<u64>().unwrap(),
                }
            )
        }
        map_vec.push(new_number_mapping);
    }

    map_vec
}

#[derive(Debug,Default)]
struct Map {
    lines: Vec<MapElement>,
}
impl Map {
    fn find_location_num(&self, seed:&u64) -> u64 {
        let mut location_num:&u64 = seed;
        for set in self.lines.iter() {
            let location_num = &set.check_range(seed);
            if location_num != seed{
                return *location_num;
            }
        }
        *seed
    } 
}
#[derive(Debug,Default)]
struct MapElement {
    des_range: u64,
    src_range: u64,
    range_len: u64,
}
impl MapElement {
    fn check_range(&self, seed:&u64) -> u64 { 
        let range = self.src_range..self.src_range+self.range_len;
        if !range.contains(seed) {
            return *seed;
        } 

        let x = seed-self.src_range;

        self.des_range+x
    }
}

