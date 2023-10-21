use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::{File, create_dir_all}, io::{BufReader, ErrorKind}, path::PathBuf};
use structopt::StructOpt;
use anyhow::{Result, anyhow, Context};
use log::info;

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
    open_commands: Vec<Command>,
}

#[derive(Serialize, Deserialize)]
struct Command {
    command: String,
    arguments: Vec<String>,
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    let opt = Opt::from_args();

    match opt {
        Opt::Add {
            path,
            project_type,
            name,
        } => add(path, project_type, name)?,
        Opt::List => list()?,
        Opt::Other(projects) => open_project(projects.first().ok_or(pm_error::Error)?)?,
    }

    Ok(())
}

fn add(
    path: PathBuf,
    project_type: Option<String>,
    name: Option<String>,
) -> Result<()> {
    let name = if let Some(name) = name {
        name
    } else {
        let file_name = path.file_name().unwrap(); // error handling does not want to work here because E0277
        file_name.to_str().unwrap().into() // error handling does not want to work here because E0277
    };

    let mut projects = load_projects().with_context(|| "failed to load projects")?;

    if projects.contains_key(&name) {
        info!("A project with the name {} already exist in projects and will be replaced with new project", name)
    }

    let mut project_path = path.clone();
    project_path.push(".project_manager");
    project_path.push("project.json");

    let project_types = load_project_types().with_context(|| "failed to load project types")?;

    if !project_path.exists() {
        let project_type = match project_type {
            Some(project_type) => Ok(project_type),
            None => Err(anyhow!("No project file was found and no project type was set")),
        }?;

        let folder_path = project_path.parent().ok_or_else(|| anyhow!("failed to get parent to project file"))?;
        create_dir_all(folder_path).with_context(|| format!("Failed to create folder to store project type. Path: {}", folder_path.display()))?;
        let file = File::create(project_path.clone()).with_context(|| format!("Failed to create file at path {}", project_path.display()))?;

        serde_json::to_writer_pretty(file, &project_types[&project_type]).with_context(|| "failed to write project file")?;
    }

    projects.insert(name, path);

    let config_path = get_config_path().with_context(|| "failed to get the config path")?;
    let config_directory = config_path.parent().ok_or_else(|| anyhow!("failed to get config directory path"))?;
    create_dir_all(config_directory).with_context(|| format!("failed to create config directory at {}", config_directory.display()))?;
    let file = File::create(config_path.clone()).with_context(|| format!("failed to open config file at path{}", config_path.display()))?;

    serde_json::to_writer_pretty(file, &projects).with_context(|| "failed to write projects file")?;

    Ok(())
} // TODO: figure out how to turn into a transaction

fn list() -> Result<()> {
    let projects = load_projects()?;

    for (name, path) in projects {
        info!("{} at {}", name, path.display());
    }

    Ok(())
}

fn open_project(project_name: &str) -> Result<()> {
    todo!()
}

fn load_projects() -> Result<HashMap<String, PathBuf>> {
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

fn load_project_types() -> Result<HashMap<String, ProjectType>> {
    let mut path = PathBuf::from("./data");
    path.push("project_types.json"); // TODO: find a better place for the project types

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            Ok(serde_json::from_reader(reader)?)
        }
        Err(err) => Err(err.into()),
    }
}

fn get_config_path() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("", "HaNa", "project_manager");
    let mut config_path: PathBuf =
        if let Some(project_dirs) = &project_dirs {
            Ok(project_dirs.config_dir())
        } else {
            Err(anyhow!("failed finding the config directory"))
        }?
        .into();

    config_path.push("config.json");

    Ok(config_path)
}
