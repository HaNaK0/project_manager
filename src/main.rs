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

    let mut projects = load_projects()?;

    if projects.contains_key(&name) {
        println!("A project with the name {} already exist in projects and will be replaced with new project", name)
    }

    let mut project_path = path.clone();
    project_path.push(".project_manager");
    project_path.push("project.json");

    let project_types = load_project_types()?;

    if !project_path.exists() {
        let project_type = match project_type {
            Some(project_type) => Ok(project_type),
            None => Err("No project file was found and no project type was set"),
        }?;

        let file = File::create(project_path)?;

        serde_json::to_writer_pretty(file, &project_types[&project_type])?;
    }

    projects.insert(name, path);

    let config_path = get_config_path()?;

    let file = File::create(config_path)?;

    serde_json::to_writer_pretty(file, &projects)?;

    Ok(())
} // TODO: figure out how to turn into a transaction

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
    let config_path = get_config_path()?;
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

fn load_project_types() -> Result<HashMap<String, ProjectType>, Box<dyn error::Error>> {
    let mut path = PathBuf::from("./data");
    path.push("project_types.json"); // TODO: find a better place for the project types

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            Ok(serde_json::from_reader(reader)?)
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn error::Error>> {
    let mut config_path: PathBuf =
        if let Some(project_dirs) = &ProjectDirs::from("", "", "project_manager") {
            Ok(project_dirs.project_path())
        } else {
            Err("failed finding the config directory")
        }?
        .into();

    config_path.push("config.json");

    Ok(config_path)
}
