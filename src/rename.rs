use std::{fs::{read_to_string, read_dir, OpenOptions}, path::PathBuf, io::Write, time::SystemTime};

pub fn rename(path_to_project: PathBuf, from: String, to: String) {
    let time_start = SystemTime::now();
    let mut stack = Vec::new();

    stack.push(path_to_project);

    while let Some(current_path) = stack.pop() {
        match read_dir(current_path) {
            Err(why) => println!("! {:?}", why.to_string()),
            Ok(paths) => for path in paths {
                let path = path.unwrap().path();

                if path.is_dir() {
                    stack.push(path);
                } else {
                    let content = read_to_string(&path)
                        .expect("Не могу прочитать файл")
                        .replace(&from, &to);

                    match OpenOptions::new().write(true).truncate(true).open(&path) {
                        Err(why) => println!("! Не могу открыть файл {:?} потому что {:?}", &path, why.to_string()),
                        Ok(mut file) => file
                            .write_all(content.as_bytes())
                            .unwrap_or_else(|why| {
                                println!("! Не могу записать в файл {:?} потому что {:?}", &file, why.to_string())
                            }),
                    }

                    let new_path = path.to_str().unwrap().to_string().replace(&from, &to);
                    println!("Переименовываю {:?} в {:?}", &path, &new_path);

                    std::fs::rename(&path, &new_path)
                        .unwrap_or_else(|why| {
                            println!("! Не могу переименовать файл {:?} потому что {:?}", &path, why.to_string())
                        });
                }
            },
        }
    }

    println!(
        "Скрипт выполнен за {:?} мс",
        SystemTime::now()
            .duration_since(time_start)
            .unwrap_or_default()
            .subsec_millis()
    );
}