use replace_naming::replace_naming;
use structopt::StructOpt;
use std::path::Path;

use crate::rename::rename;

mod replace_naming;
mod rename;

const DEFAULT_CLI_PARAMETER: &str = "%DEFAULT%";
const DEFAULT_SPRAVOCHNIK_BASE_DIR: &str = "premiera_legal_entities";

const SINGULAR_BASE_NAME: &str = "legal_entity";
const PLURAL_BASE_NAME: &str = "legal_entities";

pub struct Naming {
    singular_snake: String,
    plural_snake: String,
    singular_kebab: String,
    plural_kebab: String,
    singular_lower_camel: String,
    plural_lower_camel: String,
    singular_upper_camel: String,
    plural_upper_camel: String,
}

impl Naming {
    fn new(singular_name: String, plural_name: String) -> Naming {
        let plural_name = if plural_name == DEFAULT_CLI_PARAMETER {
            &singular_name
        } else {
            &plural_name
        };
        
        Naming {
            singular_snake: singular_name.to_lowercase(),
            plural_snake: plural_name.to_lowercase(),
            singular_kebab: singular_name.replace("_", "-"),
            plural_kebab: plural_name.replace("_", "-"),
            singular_lower_camel: to_camel_case(&singular_name, true),
            plural_lower_camel: to_camel_case(plural_name, true),
            singular_upper_camel: to_camel_case(&singular_name, false),
            plural_upper_camel: to_camel_case(plural_name, false),
        }
    }
}

trait ReplaceNaming {
    fn replace_naming(self, from: &Naming, to: &Naming) -> String;
}

impl ReplaceNaming for String {
    fn replace_naming(self, from: &Naming, to: &Naming) -> String {
        self
            .replace(&from.plural_snake, &to.plural_snake)
            .replace(&from.singular_snake, &to.singular_snake)
            .replace(&from.plural_kebab, &to.plural_kebab)
            .replace(&from.singular_kebab, &to.singular_kebab)
            .replace(&from.plural_lower_camel, &to.plural_lower_camel)
            .replace(&from.singular_lower_camel, &to.singular_lower_camel)
            .replace(&from.plural_upper_camel, &to.plural_upper_camel)
            .replace(&from.singular_upper_camel, &to.singular_upper_camel)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Tool to make copy of your project or it's part, but with another naming everywhere")]
struct Cli {
    #[structopt(subcommand)]
    cmd: CliCommand
}

#[derive(StructOpt, Debug)]
enum CliCommand {
    #[structopt(about = "Command with default values of base project which can be changed internally")]
    Default {
        #[structopt(parse(from_os_str))]
        path_to_project: std::path::PathBuf,
    
        #[structopt(help = "Target project singular name")]
        singular_name: String,
        #[structopt(help = "Target project plural name", default_value = DEFAULT_CLI_PARAMETER)]
        plural_name: String
    },
    Clone {
        #[structopt(parse(from_os_str))]
        path_to_project: std::path::PathBuf,
    
        #[structopt(help = "Singular name of project you want to use as a base")]
        base_singular_name: String,
        #[structopt(help = "Target project singular name")]
        singular_name: String,
        #[structopt(help = "Plural name of project you want to use as a base", default_value = DEFAULT_CLI_PARAMETER)]
        base_plural_name: String,
        #[structopt(help = "Target project plural name", default_value = DEFAULT_CLI_PARAMETER)]
        plural_name: String
    },
    Rename {
        #[structopt(parse(from_os_str))]
        path_to_project: std::path::PathBuf,

        from: String,
        to: String,
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn to_camel_case(s: &str, to_lower: bool) -> String {
    s
        .to_lowercase()
        .split("_")
        .enumerate()
        .map(|(index, slice)| {
            if !to_lower || (to_lower && index != 0) {
                capitalize(slice)
            } else {
                slice.to_string()
            }
        })
        .collect()
}

fn main() {
    let base_naming;
    let target_naming;
    let path_to_base;
    let path_to_target;

    match Cli::from_args().cmd {
        CliCommand::Default { path_to_project, singular_name, plural_name } => {
            base_naming = Naming::new(SINGULAR_BASE_NAME.to_string(), PLURAL_BASE_NAME.to_string());
            target_naming = Naming::new(singular_name, plural_name);

            path_to_base = Path::new(&path_to_project)
                .join(DEFAULT_SPRAVOCHNIK_BASE_DIR);
            path_to_target = Path::new(&path_to_project)
                .join("premiera_".to_owned() + &target_naming.plural_snake);

            replace_naming(base_naming, target_naming, path_to_base, path_to_target);
        },

        CliCommand::Clone {
            path_to_project,
            base_singular_name,
            base_plural_name,
            singular_name,
            plural_name
        } => {
            base_naming = Naming::new(base_singular_name, base_plural_name);
            target_naming = Naming::new(singular_name, plural_name);

            path_to_base = Path::new(&path_to_project)
                .join("premiera_".to_owned() + &base_naming.plural_snake);
            path_to_target = Path::new(&path_to_project)
                .join("premiera_".to_owned() + &target_naming.plural_snake);

            replace_naming(base_naming, target_naming, path_to_base, path_to_target);
        },

        CliCommand::Rename {
            path_to_project,
            from,
            to,
        } => {
            rename(path_to_project, from, to);
        }
    }
}