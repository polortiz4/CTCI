pub mod bit_manipulation;
pub mod math_and_logic_puzzles;
pub mod stacks_and_queues;
pub mod trees_and_graphs;


#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write;

    #[test]
    fn main_test() -> std::io::Result<()> {
        let paths = fs::read_dir("./").unwrap();
        let mut text = String::new();
        let mut idx = 0;
        for path in paths {
            // let path_str = path.unwrap().path().display().to_string();
            let path = path.unwrap().path();
            let path_str = if path.is_dir() {
                path.display().to_string()
            } else {
                let mut path = path.display().to_string();
                path.push_str("[file]");
                path
            };
            let path_trimed = path_str.trim_start_matches("./");
            idx += 1;
            println!("{} -> file/folder: {}", idx + 1, path_trimed);
            text.push_str(&path_trimed);
            text.push_str("\n");
        }
        // println!("{}", text);
        // writing the string to file
        let mut file = fs::File::create("file_list.txt")?;
        file.write_all(&text.as_bytes())?;
        Ok(())
    }
}
