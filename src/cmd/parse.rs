use clap::{Parser, Subcommand};
use ethers_solc::remappings::{RelativeRemapping, Remapping};

use crate::formatter::ReportStyle;
use serde::Serialize;
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use super::{analyze::run_analysis, gas::run_gas_metering};

#[derive(Parser)]
#[clap(name = "solhunt")]
#[clap(bin_name = "solhunt")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cmd {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a static analysis
    Analyze(Analyze),
    /// Launch gas meterings
    Gas(GasMetering),
}

// TODO: allow configuring of ignored directories through a .toml file
#[derive(Parser, Debug, Serialize)]
pub struct Analyze {
    #[clap(value_name = "PATH", default_value = ".", help = "root of the project")]
    pub path: String,
    #[clap(short, long, help = "Include only these modules")]
    pub modules: Option<Vec<String>>,
    #[clap(short, long, help = "Exclude these modules")]
    pub except_modules: Option<Vec<String>>,
    // h: High
    // m: Medium
    // l: Low
    // i: Informal
    // g: Gas
    #[clap(short, long, help = "Verbosity of the findings")]
    pub verbosity: Option<String>,
    #[clap(short, long, help = "Style of the report", value_enum)]
    pub style: Option<ReportStyle>,
    pub name: Option<String>,
    #[clap(short, long, help = "specifiy the optimizer runs")]
    pub optimizer_runs: Option<usize>,
    #[clap(
        short,
        long,
        help = "glob path for artifacts to analyze",
        default_value = "{src,contracts}/**/*.sol"
    )]
    pub glob: String,
    #[clap(long, visible_alias = "gh", help = "base location of github path")]
    pub github: Option<String>,
}

#[derive(Parser, Debug, Serialize)]
pub struct GasMetering {
    #[clap(short, long, help = "Exclude these modules")]
    pub except_modules: Option<Vec<String>>,
    #[clap(short, long, help = "Location of files to meter")]
    pub path: Option<PathBuf>,
    #[clap(short, long, help = "reset the metering database")]
    pub reset: bool,
}

#[allow(unused)]
pub fn get_working_path(add_path: String) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(current_dir().expect("could not get current path"));
    path.push(add_path);

    path.canonicalize().expect("Invalid path")
}

pub fn parse() -> eyre::Result<()> {
    let cmd = Cmd::parse();

    // TODO: filter based on rust module name
    // let all_modules = get_all_modules(); // get em' all before loading only those that we want

    // // Only those specified
    // let modules: Vec<DynModule> = match args.modules {
    //     Some(modules_names) => all_modules
    //         .into_iter()
    //         .filter(|module| modules_names.contains(&module.name))
    //         .collect(),
    //     None => all_modules, // don't touch if not specified
    // };

    // // Remove those we don't want
    // let modules: Vec<DynModule> = match args.except_modules {
    //     Some(modules_names) => modules
    //         .into_iter()
    //         .filter(|module| !modules_names.contains(&module.name))
    //         .collect(),

    //     None => modules,
    // };

    match cmd.command {
        Commands::Analyze(args) => run_analysis(args)?,
        Commands::Gas(args) => run_gas_metering(args)?,
    }

    Ok(())
}

#[allow(unused)]
pub fn get_remappings(path: &Path) -> Vec<RelativeRemapping> {
    let base_path = path.to_path_buf();
    let mut remappings: Vec<RelativeRemapping> = Vec::new();

    let remappings_file = base_path.join("remappings.txt");
    if remappings_file.is_file() {
        let content = fs::read_to_string(remappings_file)
            .map_err(|err| err.to_string())
            .unwrap();

        let rem_lines = content.split('\n').collect::<Vec<&str>>();
        let rem = rem_lines
            .iter()
            .filter(|l| l != &&"")
            .map(|l| l.split_once('='))
            .collect::<Vec<Option<(&str, &str)>>>();
        rem.iter().for_each(|pair| {
            if let Some((lib, path)) = pair {
                let full_path = base_path.join(path);
                remappings.push(
                    Remapping {
                        name: lib.to_string(),
                        path: full_path.into_os_string().into_string().unwrap(),
                    }
                    .into_relative(base_path.clone()),
                );
            }
        });
    }

    remappings
}
