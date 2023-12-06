#[derive(Debug)]
struct Part {
    number: String,
    begin: (usize, usize),
    end: (usize, usize),
}

impl Part {
    fn is_set(&self) -> bool {
        self.number != ""
    }

    fn is_valid(&self, schema: &Vec<&[u8]>) -> bool {
        let max_col = schema[0].len() - 1;
        let max_row = schema.len() - 1;
        let surrounding_begin = [
            "left",
            "bottom",
            "top",
            "top_left",
            "top_right",
            "bottom_left",
            "bottom_right",
        ];
        let surrounding_end = [
            "right",
            "bottom",
            "top",
            "top_left",
            "top_right",
            "bottom_left",
            "bottom_right",
        ];
        let symbol_around_begin = surrounding_begin.iter().any(|location| {
            let (br, bc) = self.begin;
            match *location {
                "left" => {
                    if bc == 0 {
                        false
                    } else {
                        schema[br][bc - 1] != b'.'
                    }
                }
                "top" => {
                    if br == 0 {
                        false
                    } else {
                        schema[br - 1][bc] != b'.'
                    }
                }
                "bottom" => {
                    if br == max_row {
                        false
                    } else {
                        schema[br + 1][bc] != b'.'
                    }
                }
                "top_left" => {
                    if br == 0 || bc == 0 {
                        false
                    } else {
                        schema[br - 1][bc - 1] != b'.'
                    }
                }
                "top_right" => {
                    if br == 0 || bc == max_col {
                        false
                    } else {
                        schema[br - 1][bc + 1] != b'.'
                    }
                }
                "bottom_left" => {
                    if br == max_row || bc == 0 {
                        false
                    } else {
                        schema[br + 1][bc - 1] != b'.'
                    }
                }
                "bottom_right" => {
                    if br == max_row || bc == max_col {
                        false
                    } else {
                        schema[br + 1][bc + 1] != b'.'
                    }
                }
                _ => false,
            }
        });

        let symbol_around_end = surrounding_end.iter().any(|location| {
            let (er, ec) = self.end;
            match *location {
                "right" => {
                    if ec == max_col {
                        false
                    } else {
                        schema[er][ec + 1] != b'.'
                    }
                }
                "top" => {
                    if er == 0 {
                        false
                    } else {
                        schema[er - 1][ec] != b'.'
                    }
                }
                "bottom" => {
                    if er == max_row {
                        false
                    } else {
                        schema[er + 1][ec] != b'.'
                    }
                }
                "top_left" => {
                    if er == 0 || ec == 0 {
                        false
                    } else {
                        schema[er - 1][ec - 1] != b'.'
                    }
                }
                "top_right" => {
                    if er == 0 || ec == max_col {
                        false
                    } else {
                        schema[er - 1][ec + 1] != b'.'
                    }
                }
                "bottom_left" => {
                    if er == max_row || ec == 0 {
                        false
                    } else {
                        schema[er + 1][ec - 1] != b'.'
                    }
                }
                "bottom_right" => {
                    if er == max_row || ec == max_col {
                        false
                    } else {
                        schema[er + 1][ec + 1] != b'.'
                    }
                }
                _ => false,
            }
        });
        symbol_around_begin || symbol_around_end
    }
}

#[derive(Debug)]
struct Gear {
    position: (usize, usize),
}

impl Gear {
    fn gear_ratio(&self, parts: &Vec<Part>, schema: &Vec<&[u8]>) -> u64 {
        let max_col = schema[0].len() - 1;
        let max_row = schema.len() - 1;
        let surroundings = [
            "left",
            "right",
            "bottom",
            "top",
            "top_left",
            "top_right",
            "bottom_left",
            "bottom_right",
        ];
        let surrounding_part_numbers: Vec<u64> = parts
            .iter()
            .filter(|part| {
                surroundings.iter().any(|location| {
                    let (r, c) = self.position;
                    match *location {
                        "left" => {
                            if c == 0 {
                                false
                            } else {
                                (r, c - 1) == part.begin
                                    || (r, c - 1) == part.begin
                                    || (r, c - 1) == part.end
                                    || (r, c - 1) == part.end
                            }
                        }
                        "right" => {
                            if c == max_col {
                                false
                            } else {
                                (r, c + 1) == part.begin
                                    || (r, c + 1) == part.begin
                                    || (r, c + 1) == part.end
                                    || (r, c - 1) == part.end
                            }
                        }
                        "top" => {
                            if r == 0 {
                                false
                            } else {
                                (r - 1, c) == part.begin
                                    || (r - 1, c) == part.begin
                                    || (r - 1, c) == part.end
                                    || (r - 1, c) == part.end
                            }
                        }
                        "bottom" => {
                            if r == max_row {
                                false
                            } else {
                                (r + 1, c) == part.begin
                                    || (r + 1, c) == part.begin
                                    || (r + 1, c) == part.end
                                    || (r + 1, c) == part.end
                            }
                        }
                        "top_left" => {
                            if r == 0 || c == 0 {
                                false
                            } else {
                                (r - 1, c - 1) == part.begin
                                    || (r - 1, c - 1) == part.begin
                                    || (r - 1, c - 1) == part.end
                                    || (r - 1, c - 1) == part.end
                            }
                        }
                        "top_right" => {
                            if r == 0 || c == max_col {
                                false
                            } else {
                                (r - 1, c + 1) == part.begin
                                    || (r - 1, c + 1) == part.begin
                                    || (r - 1, c + 1) == part.end
                                    || (r - 1, c + 1) == part.end
                            }
                        }
                        "bottom_left" => {
                            if r == max_row || c == 0 {
                                false
                            } else {
                                (r + 1, c - 1) == part.begin
                                    || (r + 1, c - 1) == part.begin
                                    || (r + 1, c - 1) == part.end
                                    || (r + 1, c - 1) == part.end
                            }
                        }
                        "bottom_right" => {
                            if r == max_row || c == max_col {
                                false
                            } else {
                                (r + 1, c + 1) == part.begin
                                    || (r + 1, c + 1) == part.begin
                                    || (r + 1, c + 1) == part.end
                                    || (r + 1, c + 1) == part.end
                            }
                        }
                        _ => false,
                    }
                })
            })
            .map(|part| part.number.parse::<u64>().unwrap())
            .collect();
        if surrounding_part_numbers.len() == 2 {
            surrounding_part_numbers[0] * surrounding_part_numbers[1]
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct EngineSchematic<'a> {
    schema: &'a Vec<&'a [u8]>,
}

impl<'a> EngineSchematic<'a> {
    fn get_parts(&self) -> Vec<Part> {
        let mut parts: Vec<Part> = vec![];
        let mut current_part = Part {
            number: "".to_string(),
            begin: (0, 0),
            end: (0, 0),
        };
        for (row, line) in self.schema.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c >= b'0' && *c <= b'9' {
                    if current_part.is_set() {
                        current_part.number.push(*c as char);
                    } else {
                        current_part.number.push(*c as char);
                        current_part.begin = (row, col);
                    }
                } else if current_part.is_set() && col == 0 {
                    current_part.end = (row - 1, self.max_col());
                    parts.push(current_part);
                    current_part = Part {
                        number: "".to_string(),
                        begin: (0, 0),
                        end: (0, 0),
                    };
                } else if current_part.is_set() && col != 0 {
                    current_part.end = (row, col - 1);
                    parts.push(current_part);
                    current_part = Part {
                        number: "".to_string(),
                        begin: (0, 0),
                        end: (0, 0),
                    };
                } else { // if current_part.is_none()
                }
            }
        }
        if current_part.is_set() {
            current_part.end = (self.max_row(), self.max_col());
            parts.push(current_part);
        }
        parts
    }

    fn get_gears(&self) -> Vec<Gear> {
        let mut gears: Vec<Gear> = vec![];
        for (row, line) in self.schema.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c == b'*' {
                    gears.push(Gear {
                        position: (row, col),
                    });
                }
            }
        }
        gears
    }

    fn max_col(&self) -> usize {
        self.schema[0].len() - 1
    }

    fn max_row(&self) -> usize {
        self.schema.len() - 1
    }
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let mut schema: Vec<&[u8]> = vec![];
    for line in lines {
        schema.push(line.as_bytes());
    }
    let es = EngineSchematic { schema: &schema };
    let parts = es.get_parts();
    let part1: u32 = parts
        .iter()
        .filter(|part| part.is_valid(&schema))
        .map(|part| part.number.parse::<u32>().unwrap())
        .sum();
    dbg!(part1);

    let gears = es.get_gears();
    let part2: u64 = gears
        .iter()
        .map(|gear| gear.gear_ratio(&parts, &schema))
        .sum();
    dbg!(part2);
}
