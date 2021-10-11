use std::{error, path::PathBuf};
use structopt::StructOpt;
use serde::{Deserialize, Serialize};

mod pm_error;

#[derive(StructOpt)]
enum Opt {
    Add {
        #[structopt(parse(from_os_str))]
        path: PathBuf
    },
    List,
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    path: PathBuf,
    project_type: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectType {
    name: String,
    commands: Vec<Command>
}

#[derive(Serialize, Deserialize)]
struct Command {
    command: String,
    arguments: Vec<String>
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Add { path } => add(path)?,
        Opt::List => list()?,
        Opt::Other(projects) => load_project(projects.first().ok_or_else(|| pm_error::Error)?)?,
    }

    Ok(())
}

fn add(path: PathBuf) -> Result<(), Box<dyn error::Error>> {
    println!("Path {}", path.display());

    Ok(())
}

fn list() -> Result<(), Box<dyn error::Error>> {
    todo!()
}

fn load_project(project_name: &str) -> Result<(), Box<dyn error::Error>> {
    todo!()
}