use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use shell_words;

fn main() {
    println!("Hello, world!");
    let z = get_config().unwrap();
    println!("XX: {:?}", z);
}

// use crate::dirs::PROJECT_DIRS;
// use crate::util::transpose;

// pub fn config_file() -> PathBuf {
//     env::var("BAT_CONFIG_PATH")
//         .ok()
//         .map(PathBuf::from)
//         .filter(|config_path| config_path.is_file())
//         .unwrap_or_else(|| dirs_rs.home_dir().join("config"))
// }

pub fn transpose<T, E>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    opt.map_or(Ok(None), |res| res.map(Some))
}

pub fn get_config() -> Result<Vec<OsString>, shell_words::ParseError> {
    let mut config_file_path = vec!["/etc/hosts", "/etc/profile"];
    config_file_path.push("/home/deepak/.bashrc");
    let mut y = vec![];
    for path in config_file_path.iter() {
        let x = get_args_from_config_file(PathBuf::from(path));
        y.append(&mut x.unwrap());
        println!("X: {:?}", y);
    }
    Ok(y)
}

pub fn get_args_from_config_file(config_file: PathBuf) -> Result<Vec<OsString>, shell_words::ParseError> {
    // let config_file_path = ["/etc/hosts", "/etc/profile", "some-random-path"];
    // let xx = config_file_path
    //     .iter()
    //     .map(|path| PathBuf::from(path))
    //     .filter(|path| path.is_file())
    //     .map(|path| read_to_string(path))
    //     .collect::<Result<Vec<_>, _>>();
    
    // for x in xx {
    //     println!("item: {:#?}", x);
    // }

    Ok(transpose(
        fs::read_to_string(config_file)
            .ok()
            .map(|content| get_args_from_str(&content)),
    )?
    .unwrap_or_else(|| vec![]))
}

pub fn get_args_from_env_var() -> Option<Result<Vec<OsString>, shell_words::ParseError>> {
    env::var("BAT_OPTS").ok().map(|s| get_args_from_str(&s))
}

fn get_args_from_str(content: &str) -> Result<Vec<OsString>, shell_words::ParseError> {
    let args_per_line = content
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with('#'))
        .map(|line| shell_words::split(line))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(args_per_line
        .iter()
        .flatten()
        .map(|line| line.into())
        .collect())
}

#[test]
fn empty() {
    let args = get_args_from_str("").unwrap();
    assert!(args.is_empty());
}

#[test]
fn single() {
    assert_eq!(vec!["--plain"], get_args_from_str("--plain").unwrap());
}

#[test]
fn multiple() {
    assert_eq!(
        vec!["--plain", "--language=cpp"],
        get_args_from_str("--plain --language=cpp").unwrap()
    );
}

#[test]
fn quotes() {
    assert_eq!(
        vec!["--theme", "Sublime Snazzy"],
        get_args_from_str("--theme \"Sublime Snazzy\"").unwrap()
    );
}

#[test]
fn multi_line() {
    let config = "
    -p
    --style numbers,changes

    --color=always
    ";
    assert_eq!(
        vec!["-p", "--style", "numbers,changes", "--color=always"],
        get_args_from_str(config).unwrap()
    );
}

#[test]
fn comments() {
    let config = "
    # plain style
    -p

    # show line numbers and Git modifications
    --style numbers,changes

    # Always show ANSI colors
    --color=always
    ";
    assert_eq!(
        vec!["-p", "--style", "numbers,changes", "--color=always"],
        get_args_from_str(config).unwrap()
    );
}
