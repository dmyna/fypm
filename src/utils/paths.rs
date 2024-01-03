pub struct Paths {
    pub tmp_dir: String,
}

pub fn get_paths() -> Paths {
    let paths = Paths {
        tmp_dir: String::from("/tmp/fypm"),
    };

    paths
}
