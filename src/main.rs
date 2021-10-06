use structopt::StructOpt;
use std::io::{Write};
use std::path::{Path};
use std::fs;
use std::time::SystemTime;

const SRC_DIR: &str = "src/js/app";
const DEFAULT_SPRAVOCHNIK_BASE_DIR: &str = "premiera_legal_entities";

const SINGULAR_BASE_NAME: &str = "legal_entity";
const PLURAL_BASE_NAME: &str = "legal_entities";

const DEFAULT_CLI_PARAMETER: &str = "%DEFAULT%";

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path_to_kinoplan: std::path::PathBuf,

    #[structopt(help = "Название справочника в единственном числе")]
    singular_name: String,
    #[structopt(help = "Название справочника во множественном числе", default_value = DEFAULT_CLI_PARAMETER)]
    plural_name: String
}

struct Naming {
    singular_snake: String,
    plural_snake: String,
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
            .replace(&from.singular_snake, &to.singular_snake)
            .replace(&from.plural_snake, &to.plural_snake)
            .replace(&from.singular_lower_camel, &to.singular_lower_camel)
            .replace(&from.plural_lower_camel, &to.plural_lower_camel)
            .replace(&from.singular_upper_camel, &to.singular_upper_camel)
            .replace(&from.plural_upper_camel, &to.plural_upper_camel)
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

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let time_start = SystemTime::now();

    let args = Cli::from_args();
    let mut stack = Vec::new();
    let base_naming = Naming::new(SINGULAR_BASE_NAME.to_string(), PLURAL_BASE_NAME.to_string());
    let target_naming = Naming::new(args.singular_name, args.plural_name);
    
    let path_to_base = Path::new(&args.path_to_kinoplan)
        .join(SRC_DIR)
        .join(DEFAULT_SPRAVOCHNIK_BASE_DIR);
    let path_to_target = Path::new(&args.path_to_kinoplan)
        .join(SRC_DIR)
        .join("premiera_".to_owned() + &target_naming.plural_snake);

    println!("Берём за основу: {:?}", &path_to_base);

    stack.push(path_to_base);

    fs::create_dir(&path_to_target).unwrap_or_else(|why| {
        println!("! Не могу создать директорию {:?} потому что {:?}", &path_to_target, why.kind())
    });

    while let Some(current_path) = stack.pop() {
        match fs::read_dir(current_path) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(paths) => for path in paths {
                let path = path.unwrap().path();

                if path.is_dir() {
                    let target_path = &path
                        .to_str()
                        .unwrap()
                        .to_string()
                        .replace_naming(&base_naming, &target_naming);

                    fs::create_dir(target_path)
                        .unwrap_or_else(|why| {
                            println!("! Не могу создать директорию {:?} потому что {:?}", target_path, why.kind())
                        });

                    stack.push(path);
                } else {
                    let content = std::fs::read_to_string(&path)
                        .expect("Не могу прочитать файл")
                        .replace_naming(&base_naming, &target_naming);

                    let path_to_new_file = path.to_str().unwrap().to_string().replace_naming(&base_naming, &target_naming);

                    match fs::File::create(&path_to_new_file) {
                        Err(why) => println!("! Не могу создать файл {:?} потому что {:?}", &path_to_new_file, why.kind()),
                        Ok(mut file) => file
                            .write_all(content.as_bytes())
                            .unwrap_or_else(|why| {
                                println!("! Не могу записать в файл {:?} потому что {:?}", &path_to_new_file, why.kind())
                            }),
                    }
                }
            },
        }
    }

    println!(
        "Раздел {:?} создан за {:?} мс",
        target_naming.singular_snake,
        SystemTime::now()
            .duration_since(time_start)
            .unwrap_or_default()
            .subsec_millis()
    );
}
