use std::io::Write;
use std::fs::{self, read_to_string, File};
use std::path::PathBuf;
use std::time::SystemTime;

use crate::{Naming, ReplaceNaming};

pub fn replace_naming(
  base_naming: Naming,
  target_naming: Naming,
  path_to_base: PathBuf,
  path_to_target: PathBuf,
) {
    let time_start = SystemTime::now();
    let mut stack = Vec::new();

    println!("Берём за основу: {:?}", &path_to_base);

    stack.push(path_to_base);

    fs::create_dir(&path_to_target).unwrap_or_else(|why| {
        println!("! Не могу создать директорию {:?} потому что {:?}", &path_to_target, why.to_string())
    });

    while let Some(current_path) = stack.pop() {
        match fs::read_dir(current_path) {
            Err(why) => println!("! {:?}", why.to_string()),
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
                            println!("! Не могу создать директорию {:?} потому что {:?}", target_path, why.to_string())
                        });

                    stack.push(path);
                } else {
                    let content = read_to_string(&path)
                        .expect("Не могу прочитать файл")
                        .replace_naming(&base_naming, &target_naming);

                    let path_to_new_file = path.to_str().unwrap().to_string().replace_naming(&base_naming, &target_naming);

                    match File::create(&path_to_new_file) {
                        Err(why) => println!("! Не могу создать файл {:?} потому что {:?}", &path_to_new_file, why.to_string()),
                        Ok(mut file) => file
                            .write_all(content.as_bytes())
                            .unwrap_or_else(|why| {
                                println!("! Не могу записать в файл {:?} потому что {:?}", &path_to_new_file, why.to_string())
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