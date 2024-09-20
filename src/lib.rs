mod util;
mod error;
mod interact;
mod templates;

use error::Error;
use util::AlphaNumeric;
use templates::license::{License, BuiltinLicense, PackageInfo};
use templates::{InitPy, RequirementsText, SetupPy, README};
use ::std::{path::PathBuf, fs};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(clap::Parser)]
#[command(version, about = "CLI tool to create Python library scaffolding")]
pub struct PyInitArgs {
    /// Library name
    #[arg(short = 'n')]
    name: Option<AlphaNumeric>,

    /// Library description
    #[arg(short = 'd')]
    description: Option<String>,

    /// Library author's name"
    #[arg(short = 'a')]
    author: Option<AlphaNumeric>,

    /// License type
    #[arg(short = 'l')]
    license: Option<BuiltinLicense>,
}

pub struct PyInit {
    name: AlphaNumeric,
    description: Option<String>,
    author: AlphaNumeric,
    license: License,
}

impl PyInit {
    pub fn from_interaction_or_args(args: PyInitArgs) -> Result<Self> {
        let name = match args.name {
            Some(name) => name,
            None => interact::text_required("Enter the library name")?
        };

        if util::is_available_name_on_pypi(&name)? {
            return Err(Error::LibraryNameTaken { name: name.to_string() })
        }

        let description = match args.description {
            Some(description) => Some(description),
            None => interact::text_optional("Enter a description for the library (optional)")?
        };

        let author = match args.author {
            Some(author) => author,
            None => interact::text_required("Enter the author's name")?
        };

        let license = match args.license {
            Some(a_builtin_license) => a_builtin_license.into(),
            None => {
                let options = [License::BUILTIN_NAMES, &["Other (custom)"]].concat();
                match License::builtin(interact::select_one("Choose a license", &options)?) {
                    Some(a_builtin_license) => a_builtin_license,
                    None => License::custom(interact::text_required("Enter your custom license (optional)")?)
                }
            }
        };

        Ok(Self { name, description, author, license })
    }

    pub fn run(self) -> Result<()> {
        let project_dir = PathBuf::from_iter([".", &self.name]);
        fs::create_dir_all(&project_dir)?;

        let module_dir = project_dir.join(&*self.name);
        fs::create_dir(&module_dir)?;

        InitPy {
            name: &self.name,
            desc: self.description.as_deref().unwrap_or_default(),
            year: util::current_year(),
            author: &self.author,
            license: self.license.name()
        }.render_in(&module_dir)?;

        SetupPy {
            name: &self.name,
            desc: self.description.as_deref().unwrap_or_default(),
            author: &self.author,
            license: self.license.name()
        }.render_in(&project_dir)?;

        README {
            name: &self.name,
            desc: self.description.as_deref().unwrap_or_default()
        }.render_in(&project_dir)?;

        RequirementsText {
        }.render_in(&project_dir)?;

        fs::write(project_dir.join("LICENSE"), self.license.render_with(PackageInfo {
            name: self.name.to_string(),
            author: self.author.to_string()
        })?)?;

        println!("Project files created successfully!");

        Ok(())
    }
}
