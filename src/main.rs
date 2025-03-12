mod project;
mod ser;
use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use project::scan_csproj;
use ser::Project;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    package_props_path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let package_props_path = &args
        .package_props_path
        .unwrap_or("Directory.Packages.props".to_string());
    let solution_path = PathBuf::from(&package_props_path).canonicalize()?;
    let solution_path = solution_path.parent().unwrap();
    let solution_path = solution_path.to_str().unwrap();
    let package_props = Project::from_path(&package_props_path)?;
    let csprojs = scan_csproj(&solution_path);
    let mut versions = package_props.package_versions();

    for csproj in csprojs {
        println!("{:?}", csproj);
        let project = Project::from_path(csproj.to_str().unwrap())?;
        let references = project.package_references();
        for reference in references {
            versions.remove(reference);
        }
    }

    let package_props = fs::read_to_string(&package_props_path)?;

    let quoted_versions = versions
        .iter()
        .map(|v| format!("\"{v}\""))
        .collect::<Vec<String>>();

    let content = package_props
        .lines()
        .into_iter()
        .filter(|line| !quoted_versions.iter().any(|v| line.contains(v)))
        .collect::<Vec<&str>>()
        .join("\n");

    fs::write(&package_props_path, content)?;

    println!("- Removed {} unused package references", versions.len());

    Ok(())
}
