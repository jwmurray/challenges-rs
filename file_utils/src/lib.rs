use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub enum Direction {
    Up,
    Down,
    UpAndDown,
}

pub fn find_file(
    filename: &str,
    cwd: PathBuf,
    direction: Direction,
    depth: i32,
) -> Result<PathBuf> {
    let mut ret_path_buf = cwd;

    match direction {
        Direction::Up => {
            for _ in 0..depth {
                ret_path_buf = ret_path_buf.join("../");
            }
            ret_path_buf.push(filename);
        }
        // Direction::Down => {
        //     ret_pathBuf.push_str(filename);
        //     for _ in 0..depth {
        //         // get a list of all directories in the current directory
        //         let dirs = std::fs::read_dir(&ret_pathBuf)?;
        //         ret_pathBuf.join("../");
        //         ret_pathBuf.push_str("/new_dir");
        //     }
        // }
        Direction::Down | Direction::UpAndDown => return Err(anyhow!("Not implemented")), // Direction::UpAndDown => {
                                                                                          //     for _ in 0..depth {
                                                                                          //         result.push_str("../");
                                                                                          //     }
                                                                                          //     result.push_str(filename);
                                                                                          //     for _ in 0..depth {
                                                                                          //         result.push_str("/new_dir");
                                                                                          //     }
    }
    Ok(ret_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};

    fn init_test() -> (String, TempDir, PathBuf) {
        let filename = "test.txt".to_string();
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join(&filename);
        // create test.txt in the current directory
        std::fs::File::create(&temp_file).unwrap();
        assert_eq!(temp_file.exists(), true);
        (filename, temp_dir, temp_file)
    }

    #[test]
    fn test_current_directory_exists() {
        let (filename, temp_dir, temp_file) = init_test();
        let cwd = temp_dir.path().to_path_buf();

        let direction = Direction::Up;
        let depth = 1;

        // create test.txt in the current directory
        let depth = 0;
        let result = find_file(&filename, cwd, direction, depth);
        let result = result.unwrap();
        assert_eq!(result, temp_file);
        assert_eq!(result.exists(), true);
    }
}
