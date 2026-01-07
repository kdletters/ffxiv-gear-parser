use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type GearId = i32;
pub type MateriaGrade = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Job {
    PLD, WAR, DRK, GNB,
    WHM, SCH, AST, SGE,
    MNK, DRG, NIN, SAM, RPR, VPR,
    BRD, MCH, DNC,
    BLM, SMN, RDM, PCT, BLU,
    CRP, BSM, ARM, GSM, LTW, WVR, ALC, CUL,
    MIN, BTN, FSH,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    CRT, DET, DHT, SKS, SPS, TEN, PIE,
    CMS, CRL, CP,
    GTH, PCP, GP,
}

pub type JobLevel = u16;

pub type Stats = HashMap<Stat, i32>;

pub type Materia = Option<(Stat, MateriaGrade)>;
pub type GearsetMaterias = Vec<Materia>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearsetGear {
    pub id: GearId,
    pub materias: GearsetMaterias,
    pub custom_stats: Option<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gearset {
    pub job: Job,
    pub job_level: JobLevel,
    pub sync_level: Option<u16>,
    pub gears: Vec<GearsetGear>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseResult {
    Gearset(Gearset),
    Shb,
    Ew,
}

impl PartialEq for Gearset {
    fn eq(&self, other: &Self) -> bool {
        self.job == other.job && 
        self.job_level == other.job_level && 
        self.sync_level == other.sync_level &&
        self.gears.len() == other.gears.len()
    }
}

pub struct JobDecodeEntry {
    pub job: Job,
    pub stat_decode: Vec<Stat>,
}

pub fn get_job_decode() -> Vec<JobDecodeEntry> {
    use Job::*;
    use Stat::*;
    vec![
        JobDecodeEntry { job: PLD, stat_decode: vec![CRT, DET, DHT, SKS, TEN] },
        JobDecodeEntry { job: WAR, stat_decode: vec![CRT, DET, DHT, SKS, TEN] },
        JobDecodeEntry { job: DRK, stat_decode: vec![CRT, DET, DHT, SKS, TEN] },
        JobDecodeEntry { job: GNB, stat_decode: vec![CRT, DET, DHT, SKS, TEN] },
        JobDecodeEntry { job: WHM, stat_decode: vec![CRT, DET, DHT, SPS, PIE] },
        JobDecodeEntry { job: SCH, stat_decode: vec![CRT, DET, DHT, SPS, PIE] },
        JobDecodeEntry { job: AST, stat_decode: vec![CRT, DET, DHT, SPS, PIE] },
        JobDecodeEntry { job: SGE, stat_decode: vec![CRT, DET, DHT, SPS, PIE] },
        JobDecodeEntry { job: MNK, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: DRG, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: NIN, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: SAM, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: RPR, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: VPR, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: BRD, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: MCH, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: DNC, stat_decode: vec![CRT, DET, DHT, SKS] },
        JobDecodeEntry { job: BLM, stat_decode: vec![CRT, DET, DHT, SPS] },
        JobDecodeEntry { job: SMN, stat_decode: vec![CRT, DET, DHT, SPS] },
        JobDecodeEntry { job: RDM, stat_decode: vec![CRT, DET, DHT, SPS] },
        JobDecodeEntry { job: PCT, stat_decode: vec![CRT, DET, DHT, SPS] },
        JobDecodeEntry { job: BLU, stat_decode: vec![CRT, DET, DHT, SPS] },
        JobDecodeEntry { job: CRP, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: BSM, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: ARM, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: GSM, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: LTW, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: WVR, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: ALC, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: CUL, stat_decode: vec![CMS, CRL, CP] },
        JobDecodeEntry { job: MIN, stat_decode: vec![GTH, PCP, GP] },
        JobDecodeEntry { job: BTN, stat_decode: vec![GTH, PCP, GP] },
        JobDecodeEntry { job: FSH, stat_decode: vec![GTH, PCP, GP] },
    ]
}

pub const JOB_LEVEL_DECODE: [JobLevel; 6] = [50, 60, 70, 80, 90, 100];

pub const SPECIAL_GEAR_DECODE: [GearId; 9] = [
    10337, 10338, 10339, 10340, 10341, 10342, 10343, 10344,  // Soul of the Crafter
    17726,  // Spearfishing Gig
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GearType {
    Normal(u8),  // 0-5 with materia count
    Special,     // 6
    Customizable, // 7
}

impl GearType {
    pub fn from_value(value: usize) -> Self {
        match value {
            0..=5 => GearType::Normal(value as u8),
            6 => GearType::Special,
            7 => GearType::Customizable,
            _ => panic!("Invalid gear type: {}", value),
        }
    }
}