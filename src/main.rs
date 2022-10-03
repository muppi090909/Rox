use clap::{Parser, Subcommand};
use config::Config;
use std::env::current_exe;
mod config;
use std::fs::read_to_string;

mod basic_funcs;
use basic_funcs::add::add;
use basic_funcs::install::install;
use basic_funcs::install::install_db;
use config::RawConfig;

mod parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // install a package
    Install {
        package: String,
        ver: Option<String>,
    },
    // remove the source code of the packages
    Remove {
        package: String,
    },
    // Add new repo to db
    Add {
        package: String,
        ver: Option<String>,
    },
}

/// Convert package to [`Vec`]
fn parse_name(url: &str) -> Vec<&str> {
    let url = url.split("/");
    let mut parsed_arr = Vec::new();
    for s in url {
        parsed_arr.push(s);
    }
    return parsed_arr;
}

fn main() {
    let mut conf_path = current_exe().unwrap();
    conf_path.pop();
    conf_path.push("config.toml");
    let conf: Config = RawConfig::from_str(&read_to_string(conf_path).unwrap())
        .unwrap()
        .into();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package, ver } => {
            let pkg_name = parse_name(package);

            // Validation and Appropriation
            // Package
            if package == "" {
                panic!("A valid URL must be provided")
            } else if pkg_name.len() > 1 {
                let fldr = pkg_name[4].to_string();
                println!("fldr: {}", fldr);
                let mut path = conf.clone().path;
                path.push("pkgs");
                let path = format!("{}/{}", path.to_str().unwrap(), fldr);
                install_db(
                    package.to_string(),
                    pkg_name[4].to_string(),
                    path,
                    ver.clone().unwrap().to_string(),
                    &conf
                )
            } else if pkg_name.len() == 1 {
                let fldr = package.to_string();
                let mut path = conf.clone().path;
                path.push("pkgs");
                let path = format!("{}/{}", path.to_str().unwrap(), fldr);
                install(package.to_string(), path, &conf) // url in this case is link to rox official repo
            }
        }
        // Remove command
        Commands::Remove { package: _ } => {
            todo!()
        }
        // Add command
        Commands::Add { package, ver } => {
            let pkg_name = parse_name(package);
            // Validation and Appropriation
            // Package
            if pkg_name.len() > 1 {
                let fldr = pkg_name[4].to_string();
                println!("fldr: {}", fldr);
                add(
                    package.to_string(),
                    pkg_name[4].to_string(),
                    ver.clone().unwrap().to_string(),
                    &conf
                )
            } else {
                panic!("A URL must be provided")
            }
        }
    }
}
