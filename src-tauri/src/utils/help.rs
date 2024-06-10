use anyhow::{anyhow, bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::btree_set::Range, fs, path::PathBuf, str::FromStr};
use tauri::{
    api::shell::{open, Program},
    Manager,
};

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// /// generate the uid
// pub fn get_uid(prefix: &str) -> String {
//     let id = nanoid!(11, &ALPHABET);
//     format!("{prefix}{id}")
// }

/// parse the string
/// xxx=123123; => 123123
pub fn parse_str<T: FromStr>(target: &str, key: &str) -> Option<T> {
    target.split(';').map(str::trim).find_map(|s| {
        let mut parts = s.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) if k == key => v.parse::<T>().ok(),
            _ => None,
        }
    })
}

/// open file
/// use vscode by default
pub fn open_file(app: tauri::AppHandle, path: PathBuf) -> Result<()> {
    #[cfg(target_os = "macos")]
    let code = "Visual Studio Code";
    #[cfg(not(target_os = "macos"))]
    let code = "code";

    let _ = match Program::from_str(code) {
        Ok(code) => open(&app.shell_scope(), path.to_string_lossy(), Some(code)),
        Err(err) => {
            log::error!(target: "app", "Can't find VScode `{err}`");
            // default open
            open(&app.shell_scope(), path.to_string_lossy(), None)
        }
    };

    Ok(())
}

#[macro_export]
macro_rules! error {
    ($result: expr) => {
        log::error!(target: "app", "{}", $result);
    };
}

#[macro_export]
macro_rules! log_err {
    ($result: expr) => {
        if let Err(err) = $result {
            log::error!(target: "app", "{err}");
        }
    };

    ($result: expr, $err_str: expr) => {
        if let Err(_) = $result {
            log::error!(target: "app", "{}", $err_str);
        }
    };
}

#[macro_export]
macro_rules! trace_err {
    ($result: expr, $err_str: expr) => {
        if let Err(err) = $result {
            log::trace!(target: "app", "{}, err {}", $err_str, err);
        }
    }
}

/// wrap the anyhow error
/// transform the error to String
#[macro_export]
macro_rules! wrap_err {
    ($stat: expr) => {
        match $stat {
            Ok(a) => Ok(a),
            Err(err) => {
                log::error!(target: "app", "{}", err.to_string());
                Err(format!("{}", err.to_string()))
            }
        }
    };
}

/// return the string literal error
#[macro_export]
macro_rules! ret_err {
    ($str: expr) => {
        return Err($str.into())
    };
}

#[test]
fn test_parse_value() {
    let test_1 = "upload=111; download=2222; total=3333; expire=444";
    let test_2 = "attachment; filename=Clash.yaml";

    assert_eq!(parse_str::<usize>(test_1, "upload").unwrap(), 111);
    assert_eq!(parse_str::<usize>(test_1, "download").unwrap(), 2222);
    assert_eq!(parse_str::<usize>(test_1, "total").unwrap(), 3333);
    assert_eq!(parse_str::<usize>(test_1, "expire").unwrap(), 444);
    assert_eq!(
        parse_str::<String>(test_2, "filename").unwrap(),
        format!("Clash.yaml")
    );

    assert_eq!(parse_str::<usize>(test_1, "aaa"), None);
    assert_eq!(parse_str::<usize>(test_1, "upload1"), None);
    assert_eq!(parse_str::<usize>(test_1, "expire1"), None);
    assert_eq!(parse_str::<usize>(test_2, "attachment"), None);
}