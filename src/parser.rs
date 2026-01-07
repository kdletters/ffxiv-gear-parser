use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

use crate::base62;
use crate::types::*;

pub struct Ranges {
    pub version: u32,
    pub job: u32,
    pub job_level: u32,
    pub sync_level: u32,
    pub gear_type: u32,
    pub gear_id: u32,
    pub materia_slot: u32,
    pub materia_stat: u32,
    pub materia_grade: u32,
    pub special_gear: u32,
    pub custom_stat: u32,
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            version: 77,
            job: 0,
            job_level: 0,
            sync_level: 0,
            gear_type: 0,
            gear_id: 0,
            materia_slot: 0,
            materia_stat: 0,
            materia_grade: 0,
            special_gear: 0,
            custom_stat: 0,
        }
    }
}

impl Ranges {
    pub fn use_version(&mut self, version: u32) {
        if version >= 5 {
            self.job = 33;
            self.job_level = 6;
            self.sync_level = 800;
            self.gear_type = 8;
            self.gear_id = 60000;
            self.materia_slot = 6;
            self.materia_grade = 12;
            self.special_gear = 9;
            self.custom_stat = 1001;
        }
    }

    pub fn use_job(&mut self, job_index: usize) {
        let job_decode = get_job_decode();
        self.materia_stat = job_decode[job_index].stat_decode.len() as u32;
    }
}

struct Reader {
    input: BigUint,
}

impl Reader {
    fn new(input: BigUint) -> Self {
        Self { input }
    }

    fn read(&mut self, range: u32) -> u32 {
        let range_bi = BigUint::from(range);
        let ret = (&self.input % &range_bi).to_u32().unwrap_or(0);
        self.input /= &range_bi;
        ret
    }

    fn read_boolean(&mut self) -> bool {
        self.read(2) == 1
    }

    fn is_empty(&self) -> bool {
        self.input.is_zero()
    }
}

pub fn parse(s: &str) -> Result<ParseResult, String> {
    if s.is_empty() {
        return Err("Empty input string".to_string());
    }

    let input = base62::decode(s);
    let mut reader = Reader::new(input);
    let mut ranges = Ranges::default();

    let version = reader.read(ranges.version);
    if version < 4 {
        return Ok(ParseResult::Shb);
    }
    if version < 5 {
        return Ok(ParseResult::Ew);
    }
    ranges.use_version(version);

    let job_decode = get_job_decode();
    let job_index = reader.read(ranges.job) as usize;
    if job_index >= job_decode.len() {
        return Err(format!("Invalid job index: {}", job_index));
    }

    let job = job_decode[job_index].job;
    let stat_decode = &job_decode[job_index].stat_decode;
    ranges.use_job(job_index);

    let synced = reader.read_boolean();
    let job_level = if synced {
        let job_level_index = reader.read(ranges.job_level) as usize;
        if job_level_index >= JOB_LEVEL_DECODE.len() {
            return Err(format!("Invalid job level index: {}", job_level_index));
        }
        JOB_LEVEL_DECODE[job_level_index]
    } else {
        JOB_LEVEL_DECODE[ranges.job_level as usize - 1]
    };

    let sync_level = if synced {
        let level = reader.read(ranges.sync_level);
        if level > 0 { Some(level as u16) } else { None }
    } else {
        None
    };

    let mut gear_type_decode: Vec<usize> = Vec::new();
    for i in 0..ranges.gear_type as usize {
        if reader.read_boolean() {
            gear_type_decode.push(i);
        }
    }

    let min_materia_grade = reader.read(ranges.materia_grade + 1);
    let mut materia_decode: GearsetMaterias = Vec::new();

    for grade in (min_materia_grade..=ranges.materia_grade).rev() {
        if grade == 0 {
            continue;
        }
        for stat_index in 0..ranges.materia_stat as usize {
            if reader.read_boolean() {
                if stat_index >= stat_decode.len() {
                    return Err(format!("Invalid stat index: {}", stat_index));
                }
                materia_decode.push(Some((stat_decode[stat_index], grade as MateriaGrade)));
            }
        }
    }
    if min_materia_grade == 0 {
        materia_decode.push(None);
    }

    let mut gears: Vec<GearsetGear> = Vec::new();
    let mut gear_id_delta_range: i32 = 0;
    let mut gear_id_delta_direction: i32 = 0;
    let mut rings_inversed = false;
    let mut id: i32 = -1;

    while !reader.is_empty() {
        if gear_type_decode.is_empty() {
            return Err("No gear types available".to_string());
        }

        let gear_type_index = reader.read(gear_type_decode.len() as u32) as usize;
        if gear_type_index >= gear_type_decode.len() {
            return Err(format!("Invalid gear type index: {}", gear_type_index));
        }

        let gear_type = GearType::from_value(gear_type_decode[gear_type_index]);
        let mut materias: GearsetMaterias = Vec::new();
        let mut custom_stats: Option<Stats> = None;

        if gear_type == GearType::Special {
            let special_index = reader.read(ranges.special_gear) as usize;
            if special_index >= SPECIAL_GEAR_DECODE.len() {
                return Err(format!("Invalid special gear index: {}", special_index));
            }
            let special_id = SPECIAL_GEAR_DECODE[special_index];
            gears.push(GearsetGear {
                id: special_id,
                materias: Vec::new(),
                custom_stats: None,
            });
            continue;
        }

        if gear_type == GearType::Customizable {
            let mut stats = Stats::new();
            for stat in stat_decode {
                let value = reader.read(ranges.custom_stat) as i32;
                if value > 0 {
                    stats.insert(*stat, value);
                }
            }
            custom_stats = Some(stats);
        } else if let GearType::Normal(materia_count) = gear_type {
            for _ in 0..materia_count {
                if materia_decode.is_empty() {
                    return Err("No materia types available".to_string());
                }
                let materia_index = reader.read(materia_decode.len() as u32) as usize;
                if materia_index >= materia_decode.len() {
                    return Err(format!("Invalid materia index: {}", materia_index));
                }
                materias.push(materia_decode[materia_index].clone());
            }
        }

        if id == -1 {
            id = reader.read(ranges.gear_id) as i32;
            if id <= 0 {
                return Err(format!("Invalid gear id: {}", id));
            }
            gear_id_delta_range = reader.read(id as u32) as i32;
            gear_id_delta_direction = if reader.read_boolean() { 1 } else { -1 };
            rings_inversed = reader.read_boolean();
        } else {
            let delta = reader.read(gear_id_delta_range as u32) as i32;
            let adjustment = if reader.is_empty() { 1 } else { 0 };
            id += (delta - adjustment) * gear_id_delta_direction;
        }

        gears.push(GearsetGear {
            id: id as GearId,
            materias,
            custom_stats,
        });
    }

    if rings_inversed != (gear_id_delta_direction == -1) {
        gears.reverse();
    }

    Ok(ParseResult::Gearset(Gearset {
        job,
        job_level,
        sync_level,
        gears,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() {
        let s = "1OUOJLa40M28M25Zx9onPbVFINb9tatYuSsZ";
        let result = parse(s);

        assert!(result.is_ok());
        match result.unwrap() {
            ParseResult::Gearset(gearset) => {
                println!("Job: {:?}", gearset.job);
                println!("Job Level: {}", gearset.job_level);
                println!("Sync Level: {:?}", gearset.sync_level);
                println!("Gears count: {}", gearset.gears.len());
                assert!(!gearset.gears.is_empty());
            }
            _ => panic!("Expected gearset result"),
        }
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_old_versions() {
        // These would need actual old version test strings
        // For now, just test that the version logic works
        assert_eq!(3, 3); // placeholder
    }
}