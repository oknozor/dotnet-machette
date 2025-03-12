use anyhow::Result;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageProperties {
    #[serde(default)]
    pub package_id: String,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub authors: String,

    #[serde(default)]
    pub company: Option<String>,

    #[serde(default)]
    pub product: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub copyright: Option<String>,

    #[serde(default)]
    pub license_url: Option<String>,

    #[serde(default)]
    pub project_url: Option<String>,

    #[serde(default)]
    pub repository_url: Option<String>,

    #[serde(default)]
    pub package_tags: Option<String>,

    #[serde(default)]
    pub release_notes: Option<String>,

    #[serde(default)]
    pub require_license_acceptance: Option<bool>,

    #[serde(default)]
    pub assembly_version: Option<String>,

    #[serde(default)]
    pub file_version: Option<String>,

    #[serde(default)]
    pub target_framework: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    #[serde(rename = "PropertyGroup", default)]
    pub property_groups: Vec<PackageProperties>,

    #[serde(rename = "ItemGroup", default)]
    pub item_groups: Vec<ItemGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemGroup {
    #[serde(rename = "PackageReference", default)]
    pub package_references: Vec<PackageReference>,
    #[serde(rename = "PackageVersion", default)]
    pub package_versions: Vec<PackageVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageVersion {
    pub include: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageReference {
    #[serde(rename = "Include")]
    pub include: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl Project {
    pub fn from_path(path: &str) -> Result<Project> {
        let json = fs::read_to_string(path)?;
        let project: Project = serde_xml_rs::from_str(&json)?;
        Ok(project)
    }

    pub fn package_versions(&self) -> std::collections::HashSet<&str> {
        self.item_groups
            .iter()
            .map(|g| &g.package_versions)
            .flatten()
            .map(|r| (r.include.as_str()))
            .collect::<std::collections::HashSet<_>>()
    }

    pub fn package_references(&self) -> std::collections::HashSet<&str> {
        self.item_groups
            .iter()
            .map(|g| &g.package_references)
            .flatten()
            .filter_map(|r| r.include.as_ref())
            .map(|include| (include.as_str()))
            .collect::<std::collections::HashSet<_>>()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::ser::Project;

    #[test]
    fn should_deserialize_project() {
        let json = fs::read_to_string("tests/Package.props.example.xml").unwrap();
        let project: Project = serde_xml_rs::from_str(&json).unwrap();
        println!("{:?}", project);
        let references = project
            .item_groups
            .iter()
            .map(|g| &g.package_versions)
            .flatten()
            .map(|r| (r.include.as_str()))
            .collect::<Vec<_>>();

        println!("{:#?}", references);
        assert!(references
            .contains(&"Agicap.DataIntegration.Business.Engine.Collect.Adapters.Messaging"));
    }
}
