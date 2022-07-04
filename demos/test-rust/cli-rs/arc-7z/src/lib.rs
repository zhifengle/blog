use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
};

use clap::{arg, ArgMatches, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

// &'a str ??
#[derive(Debug)]
pub struct Config<'a> {
    exclude_dirs: Vec<&'a str>,
    password: Option<&'a str>,
    target_dir: &'a str,
    output_dir: &'a str,
}

impl<'a> Config<'a> {
    // 把 matches 放在里面会报错。new 内部对 matches 的数据进行了 borrow
    pub fn new(matches: &'a ArgMatches) -> MyResult<Config<'a>> {
        let exclude_dirs: Vec<&str> = matches.values_of("exclude").unwrap().collect();
        let target_dir = matches.value_of("target_dir").unwrap();
        let mut output_dir = matches.value_of("output_dir").unwrap();
        if output_dir == "-" {
            output_dir = target_dir;
        }
        Ok(Config {
            exclude_dirs,
            password: matches.value_of("password"),
            target_dir,
            output_dir,
        })
    }
    pub fn is_target(&self, dir: &str) -> bool {
        self.exclude_dirs
            .iter()
            .find(|s| dir.contains(*s))
            .is_none()
    }
    fn get_pw_arg_7z(&self) -> Option<String> {
        self.password.map(|p| "-p".to_string() + p)
    }
    fn get_args_7z(&self, folder: &str) -> Vec<String> {
        let target_folder = Path::new(self.target_dir)
            .join(folder)
            .display()
            .to_string();
        let output_file = Path::new(self.output_dir)
            .join(folder)
            .display()
            .to_string()
            + ".7z";
        let mut args = vec!["a".to_string(), output_file, "-mhe".to_string()];
        args.push(target_folder);
        if let Some(password) = self.get_pw_arg_7z() {
            args.push(password);
        }
        args
    }
    pub fn archive_folder_7z(&self, folder: &str) -> MyResult<()> {
        // 不能像 nodejs 那样 exec("7z a xx.7z")
        ProcessCommand::new("7z")
            .args(self.get_args_7z(folder))
            .status()?;
        Ok(())
    }
}

pub fn get_matches() -> ArgMatches {
    return Command::new("archive")
        .version("0.0.2")
        .author("Alan Yang")
        .about("archive folders")
        .arg(arg!(target_dir: <TARGET_DIR> "target dir"))
        .arg(arg!(output_dir: -o [OUTPUT_DIR] "output dir").default_value("-"))
        .arg(arg!(password: -p --password [PASSWORD] "archive password"))
        .arg(arg!(exclude: -e --exclude [EXCLUDE] ... "exclude dir").default_value("done"))
        .get_matches();
}

pub fn run(config: &Config) -> MyResult<()> {
    for entry in fs::read_dir(PathBuf::from(config.target_dir))? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap();
        if path.is_dir() && config.is_target(file_name) {
            config.archive_folder_7z(file_name)?;
        }
        // println!("{:?}", path);

        // let metadata = fs::metadata(&path)?;
        // println!("{:?}", metadata);
    }
    Ok(())
}
