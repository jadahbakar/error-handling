pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod fs;
use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?;
    println!("{files:#?}");

    Ok(())
}

// fn list_files(path: &str) -> Result<Vec<String>> {
//     let files: Vec<String> = std::fs::read_dir(path)?
//         // let files: Vec<String> = std::fs::read_dir(path)
//         // .map_err(|err| format!("error while reading dir: cause {err}"))?
//         .filter_map(|re| re.ok())
//         .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
//         .filter_map(|e| e.file_name().into_string().ok())
//         .collect();
//     if files.is_empty() {
//         return Err("Cannot list empty folder".into());
//     }
//     Ok(files)
// }
