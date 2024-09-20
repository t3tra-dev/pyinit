pub mod license;

macro_rules! templates {
    ($( $path:literal as $Struct:ident $(<$lt:lifetime>)? { $( $field:ident: $Type:ty, )* } )*) => {
        $(
            #[derive(askama::Template)]
            #[template(path = $path, escape = "none")]
            pub struct $Struct $(<$lt>)? {
                $( pub $field: $Type, )*
            }
            
            impl $(<$lt>)? $Struct $(<$lt>)? {
                pub fn render_in(self, dir: impl AsRef<std::path::Path>) -> crate::Result<()> {
                    std::fs::write(dir.as_ref().join($path), askama::Template::render(&self)?)?;
                    Ok(())
                }
            }
        )*
    };
}

templates! {
    "requirements.txt" as RequirementsText {
    }
    "README.md" as README<'t> {
        name: &'t str,
        desc: &'t str,
    }
    "__init__.py" as InitPy<'t> {
        name: &'t str,
        desc: &'t str,
        year: u16,
        author: &'t str,
        license: &'t str,
    }
    "setup.py" as SetupPy<'t> {
        name: &'t str,
        desc: &'t str,
        author: &'t str,
        license: &'t str,
    }
}
