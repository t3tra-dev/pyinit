pub struct License(Repr);

enum Repr {
    Builtin(BuiltinLicense),
    Custom { name: String },
}

macro_rules! BuiltinLicense {
    ($( $path:literal as $Struct:ident # $doc:literal ),* $(,)?) => {
        #[derive(Clone, clap::ValueEnum)]
        pub enum BuiltinLicense {
            $(
                #[doc = $doc]
                $Struct
            ),*
        }

        impl BuiltinLicense {
            const NAMES: &'static [&'static str] = &[$( $Struct::NAME ),*];

            fn from_name(name: &str) -> Option<Self> {
                match name {
                    $( $Struct::NAME => Some(Self::$Struct), )*
                    _ => None
                }
            }

            fn name(&self) -> &str {
                match self {
                    $( Self::$Struct => $Struct::NAME, )*
                }
            }

            fn render_with(self, package_info: PackageInfo) -> crate::Result<String> {
                match self {
                    $( Self::$Struct => $Struct::from_package_info(package_info).render(), )*
                }
            }
        }

        $(
            #[derive(askama::Template)]
            #[template(path = $path, escape = "none")]
            #[allow(unused)]
            struct $Struct {
                year: u16,
                name: String,
                author: String,
            }

            impl $Struct {
                const NAME: &'static str = const_name_of($path);

                fn from_package_info(PackageInfo { name, author }: PackageInfo) -> Self {
                    let year = crate::util::current_year();
                    Self { year, name, author }
                }

                fn render(self) -> crate::Result<String> {
                    let r = <Self as askama::Template>::render(&self)?;
                    Ok(r)
                }
            }
        )*
    };
}

BuiltinLicense! {
    "licenses/Apache-2.0"   as Apache # "Apache-2.0 license",
    "licenses/BSD 2-Clause" as BSD2   # "BSD 2-Clause license",
    "licenses/BSD 3-Clause" as BSD3   # "BSD 3-Clause license",
    "licenses/CC0-1.0"      as CC0    # "CC0-1.0 license",
    "licenses/EPL-2.0"      as EPL    # "EPL-2.0 license",
    "licenses/GPL-3.0"      as GPL    # "GPL-3.0 license",
    "licenses/MIT"          as MIT    # "MIT license",
    "licenses/MPL-2.0"      as MPL    # "MPL-2.0 license",
}

pub struct PackageInfo {
    pub name:   String,
    pub author: String,
}

impl License {
    pub const BUILTIN_NAMES: &'static [&'static str] = BuiltinLicense::NAMES;

    pub fn builtin(name: &str) -> Option<Self> {
        BuiltinLicense::from_name(name)
            .map(Repr::Builtin).map(Self)
    }

    pub fn custom(name: String) -> Self {
        Self(Repr::Custom { name })
    }

    pub fn name(&self) -> &str {
        match &self.0 {
            Repr::Builtin(b) => b.name(),
            Repr::Custom { name } => &name
        }
    }

    pub fn render_with(self, package_info: PackageInfo) -> crate::Result<String> {
        match self.0 {
            Repr::Builtin(b) => b.render_with(package_info),
            Repr::Custom { .. } => Ok(String::new())
        }
    }
}

impl From<BuiltinLicense> for License {
    fn from(builtin: BuiltinLicense) -> Self {
        Self(Repr::Builtin(builtin))
    }
}

const fn const_name_of(path: &'static str) -> &'static str {
    if !path.is_ascii() {
        panic!("unexpected non-ascii path")
    }

    let bytes = path.as_bytes();
    let mut i = path.len() - 1;
    while i > 0 {
        if bytes[i] == b'/' {
            return unsafe {
                std::str::from_utf8_unchecked(
                    std::slice::from_raw_parts(
                        bytes.as_ptr().add(i + 1),
                        bytes.len() - (i + 1)
                    )
                )
            }
        }
        i -= 1;
    }

    path
}
