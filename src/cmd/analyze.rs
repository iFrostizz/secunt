use std::collections::HashMap;

use crate::formatter::Report;
use crate::loader::get_all_visitors;
use crate::solidity::{build_source_maps, Solidity};
use crate::walker::{Severity, Walker};

use super::parse::{get_working_path, Analyze};

pub fn run_analysis(args: Analyze) {
    let mut severities = HashMap::from([
        ('h', Severity::High),
        ('m', Severity::Medium),
        ('l', Severity::Low),
        ('g', Severity::Gas),
        ('i', Severity::Informal),
    ]);

    let verbosity: Vec<Severity> = if let Some(args_verb) = args.verbosity {
        args_verb
            .chars()
            .filter_map(|c| severities.remove(&c))
            .collect()
    } else {
        severities.values().map(|s| s.to_owned()).collect()
    };

    let path = get_working_path(args.path);
    let report_style = args.style;

    let solidity = Solidity::default().with_path_root(path.clone());

    let compiled = solidity.compile_with_foundry().expect("Compilation failed");
    let output = compiled.clone().output();

    let source_map = build_source_maps(output);

    // TODO: configurable with glob
    let included_folders: Vec<String> = vec![String::from("src")];

    let artifacts = compiled
        .into_artifacts()
        .filter(|(id, _art)| {
            let root_path = &path;
            if root_path.is_dir() {
                // only filter if not "file-only"
                let abs_path = &id.source;
                let other_path = abs_path.strip_prefix(root_path).unwrap_or_else(|e| {
                    panic!(
                        "Failed to strip root path: {} from {}",
                        root_path.to_string_lossy(),
                        abs_path.to_string_lossy()
                    )
                });

                let first_folder = other_path
                    .iter()
                    .next()
                    .expect("Failed to get first folder");
                // only take included folders
                included_folders.contains(&first_folder.to_string_lossy().to_string())
            } else {
                false
            }
        })
        .collect();

    let visitors = get_all_visitors();

    let mut walker = Walker::new(artifacts, source_map, visitors);

    println!("Starting the analysis...");

    let findings = walker.traverse().expect("failed to traverse ast");
    let num_findings = findings.len();
    println!("Caught {num_findings} findings");

    let report = Report::new(report_style, path, findings, verbosity);
    report.format();
}