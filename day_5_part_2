use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::ops::Range;

fn main() {
    
    let mut input = String::new(); 
    File::open("/home/apu/Projects/aoc/day_5/input.txt").expect("e").read_to_string(&mut input).unwrap();
    let input_lines:Vec<&str> = input.lines().collect();
    let maps = parse_into_maps(input.as_str());

    let seeds:Vec<u64> = input_lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s|s.parse::<u64>().unwrap())
        .collect();

    let seed_ranges  = seeds.chunks(2).map(|c|{
        Range {start: c[0], end: c[1]}
    }).collect::<Vec<Range<u64>>>();

    for element in maps {
        seed_ranges = seed_ranges.into_iter().map(|mut range|{
            
        });
    }
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
    fn find_location_num(&self, seed:&u64) -> Option<u64> {
        let mut location_num:&u64 = seed;
        for set in self.lines.iter() {
            let location_num = &set.check_range(seed);
            if location_num != seed{
                return Some(*location_num);
            }
        }
        None
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
