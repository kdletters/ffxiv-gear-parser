use std::env;
use ffxiv_gear_parser::{parse_gearset, ParseResult};

fn print_usage() {
    println!("Usage: ffxiv-share-parser <share_string>");
    println!("       ffxiv-share-parser --help");
    println!();
    println!("Examples:");
    println!("  ffxiv-share-parser 1OUOJLa40M28M25Zx9onPbVFINb9tatYuSsZ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
        std::process::exit(1);
    }

    let share_string = &args[1];

    if share_string == "--help" || share_string == "-h" {
        print_usage();
        return;
    }

    println!("Parsing share string: {}", share_string);
    println!("{}", "=".repeat(50));

    match parse_gearset(share_string) {
        Ok(ParseResult::Gearset(gearset)) => {
            println!("✓ Successfully parsed gearset!");
            println!();
            println!("Job: {:?}", gearset.job);
            println!("Job Level: {}", gearset.job_level);
            if let Some(sync_level) = gearset.sync_level {
                println!("Sync Level: {}", sync_level);
            } else {
                println!("Sync Level: None");
            }
            println!();
            println!("Gears ({} items):", gearset.gears.len());
            for (i, gear) in gearset.gears.iter().enumerate() {
                println!("  {}. Gear ID: {}", i + 1, gear.id);
                if !gear.materias.is_empty() {
                    println!("     Materias: {:?}", gear.materias);
                }
                if let Some(ref stats) = gear.custom_stats {
                    println!("     Custom Stats: {:?}", stats);
                }
            }
        }
        Ok(ParseResult::Shb) => {
            println!("⚠ This is a Shadowbringers (5.x) format share string");
            println!("This format is not supported by this parser version.");
        }
        Ok(ParseResult::Ew) => {
            println!("⚠ This is an Endwalker (6.x) format share string");
            println!("This format is not supported by this parser version.");
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
            std::process::exit(1);
        }
    }
}