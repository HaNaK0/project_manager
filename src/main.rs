use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{BufReader, ErrorKind},
    path::PathBuf,
};
use structopt::StructOpt;

mod pm_error;

#[derive(StructOpt)]
enum Opt {
    Add {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        project_type: Option<String>,
        name: Option<String>,
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
    commands: Vec<Command>,
}

#[derive(Serialize, Deserialize)]
struct Command {
    command: String,
    arguments: Vec<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Add { path , project_type, name} => add(path, project_type, name)?,
        Opt::List => list()?,
        Opt::Other(projects) => open_project(projects.first().ok_or_else(|| pm_error::Error)?)?,
    }

    Ok(())
}

fn add(path: PathBuf, project_type: Option<String>, name: Option<String>) -> Result<(), Box<dyn error::Error>> {
    let name = if let Some(name) = name {
        name
    } else {
        let file_name = path.file_name().unwrap(); // error handling does not want to work here because E0277
        file_name.to_str().unwrap().into() // error handling does not want to work here because E0277
    };

    

    let projects = load_projects()?;

    Ok(())
}

fn list() -> Result<(), Box<dyn error::Error>> {
    let projects = load_projects()?;

    for (name, path) in projects {
        println!("{} at {}", name, path.display());
    }

    Ok(())
}

fn open_project(project_name: &str) -> Result<(), Box<dyn error::Error>> {
    todo!()
}

fn load_projects() -> Result<HashMap<String, PathBuf>, Box<dyn error::Error>> {
    let mut config_path: PathBuf =
        if let Some(project_dirs) = &ProjectDirs::from("", "", "project_manager") {
            Ok(project_dirs.project_path())
        } else {
            Err("failed finding the config directory")
        }?
        .into();

    config_path.push("config.json");
    match File::open(config_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            Ok(serde_json::from_reader(reader)?)
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                Ok(HashMap::new())
            } else {
                Err(err.into())
            }
        }
    }
}
