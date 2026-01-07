use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use ::ffxiv_gear_parser::{parse_gearset, ParseResult, Gearset, GearsetGear};
use std::collections::HashMap;
use std::panic;

/// Parse a FFXIV gearset share string
/// 
/// Args:
///     share_string (str): The base62-encoded share string
/// 
/// Returns:
///     dict: A dictionary containing the parsed gearset data with keys:
///         - type: "Gearset", "Shb", or "Ew"
///         - data: The parsed gearset data (only for "Gearset" type)
/// 
/// Raises:
///     ValueError: If the share string is invalid or cannot be parsed
/// 
/// Example:
///     >>> import ffxiv_gear_parser
///     >>> result = ffxiv_gear_parser.parse("45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we")
///     >>> print(result['type'])
///     'Gearset'
#[pyfunction]
fn parse(py: Python, share_string: &str) -> PyResult<Py<PyAny>> {
    // Catch any panics and convert them to Python exceptions
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        parse_gearset(share_string)
    }));

    match result {
        Ok(Ok(parse_result)) => {
            let dict = pyo3::types::PyDict::new(py);

            match parse_result {
                ParseResult::Gearset(gearset) => {
                    dict.set_item("type", "Gearset")?;
                    dict.set_item("data", gearset_to_py(py, &gearset)?)?;
                }
                ParseResult::Shb => {
                    dict.set_item("type", "Shb")?;
                    dict.set_item("message", "Old Shadowbringers format (version < 4)")?;
                }
                ParseResult::Ew => {
                    dict.set_item("type", "Ew")?;
                    dict.set_item("message", "Old Endwalker format (version < 5)")?;
                }
            }

            Ok(dict.into())
        }
        Ok(Err(e)) => Err(PyValueError::new_err(format!("Parse error: {}", e))),
        Err(_) => Err(PyValueError::new_err(
            "Internal error: Rust panic occurred during parsing. This may be due to invalid input data."
        ))
    }
}

/// Parse a FFXIV gearset share string and return as JSON string
/// 
/// Args:
///     share_string (str): The base62-encoded share string
/// 
/// Returns:
///     str: A JSON string containing the parsed gearset data
/// 
/// Raises:
///     ValueError: If the share string is invalid or cannot be parsed
/// 
/// Example:
///     >>> import ffxiv_gear_parser
///     >>> import json
///     >>> result = ffxiv_gear_parser.parse_json("45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we")
///     >>> data = json.loads(result)
#[pyfunction]
fn parse_json(share_string: &str) -> PyResult<String> {
    // Catch any panics and convert them to Python exceptions
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        parse_gearset(share_string)
    }));

    match result {
        Ok(Ok(parse_result)) => {
            let json_result = match parse_result {
                ParseResult::Gearset(gearset) => {
                    serde_json::json!({
                        "type": "Gearset",
                        "data": gearset_to_json(&gearset)
                    })
                }
                ParseResult::Shb => {
                    serde_json::json!({
                        "type": "Shb",
                        "message": "Old Shadowbringers format (version < 4)"
                    })
                }
                ParseResult::Ew => {
                    serde_json::json!({
                        "type": "Ew",
                        "message": "Old Endwalker format (version < 5)"
                    })
                }
            };

            serde_json::to_string_pretty(&json_result)
                .map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))
        }
        Ok(Err(e)) => Err(PyValueError::new_err(format!("Parse error: {}", e))),
        Err(_) => Err(PyValueError::new_err(
            "Internal error: Rust panic occurred during parsing. This may be due to invalid input data."
        ))
    }
}

fn gearset_to_py(py: Python, gearset: &Gearset) -> PyResult<Py<PyAny>> {
    // Catch any panics during conversion
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| -> PyResult<Py<PyAny>> {
        let dict = pyo3::types::PyDict::new(py);

        dict.set_item("job", format!("{:?}", gearset.job))?;
        dict.set_item("job_level", gearset.job_level)?;
        dict.set_item("sync_level", gearset.sync_level)?;

        let gears_list = pyo3::types::PyList::empty(py);
        for gear in &gearset.gears {
            let gear_dict = pyo3::types::PyDict::new(py);
            gear_dict.set_item("id", gear.id)?;

            let materias_list = pyo3::types::PyList::empty(py);
            for materia in &gear.materias {
                if let Some((stat, grade)) = materia {
                    let materia_dict = pyo3::types::PyDict::new(py);
                    materia_dict.set_item("stat", format!("{:?}", stat))?;
                    materia_dict.set_item("grade", *grade)?;
                    materias_list.append(materia_dict)?;
                } else {
                    materias_list.append(py.None())?;
                }
            }
            gear_dict.set_item("materias", materias_list)?;

            if let Some(ref stats) = gear.custom_stats {
                let stats_dict = pyo3::types::PyDict::new(py);
                for (stat, value) in stats {
                    stats_dict.set_item(format!("{:?}", stat), *value)?;
                }
                gear_dict.set_item("custom_stats", stats_dict)?;
            } else {
                gear_dict.set_item("custom_stats", py.None())?;
            }

            gears_list.append(gear_dict)?;
        }
        dict.set_item("gears", gears_list)?;

        Ok(dict.into())
    }));

    match result {
        Ok(r) => r,
        Err(_) => Err(PyValueError::new_err(
            "Internal error: Panic occurred during data conversion"
        ))
    }
}

fn gearset_to_json(gearset: &Gearset) -> serde_json::Value {
    // This function should not panic, but we're being defensive
    serde_json::json!({
        "job": format!("{:?}", gearset.job),
        "job_level": gearset.job_level,
        "sync_level": gearset.sync_level,
        "gears": gearset.gears.iter().map(|gear| {
            let mut gear_json = serde_json::json!({
                "id": gear.id,
                "materias": gear.materias.iter().map(|m| {
                    m.as_ref().map(|(stat, grade)| {
                        serde_json::json!({
                            "stat": format!("{:?}", stat),
                            "grade": grade
                        })
                    })
                }).collect::<Vec<_>>()
            });

            if let Some(ref stats) = gear.custom_stats {
                let stats_map: HashMap<String, i32> = stats.iter()
                    .map(|(k, v)| (format!("{:?}", k), *v))
                    .collect();
                gear_json["custom_stats"] = serde_json::json!(stats_map);
            }

            gear_json
        }).collect::<Vec<_>>()
    })
}

/// FFXIV Gearset Share String Parser
/// 
/// This module provides functions to parse FFXIV gearset share strings
/// used by gearing tools like Etro and Ariyala.
#[pymodule]
fn ffxiv_gear_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    Ok(())
}
