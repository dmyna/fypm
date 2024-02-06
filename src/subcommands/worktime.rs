struct Worktime {
    name: String,
    description: String,
    start_time: String,
    end_time: String,
    polybar_background: String,
    polybar_foreground: String,
}

struct WorktimeHandler {
    worktimes: Vec<Worktime>,
}
impl WorktimeHandler {
    pub fn add(matches: &clap::ArgMatches) {
        // let mut worktimes = Vec::new();

        // Worktime {
        //     name: String::from(""),
        //     description: String::from(""),
        //     start_time: String::from(""),
        //     end_time: String::from(""),
        //     polybar_background: String::from(""),
        //     polybar_foreground: String::from(""),
        // };

        todo!();
    }
}
