diesel::table! {
    worktimes (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        style -> Text,
        start_time -> Text,
        end_time -> Text,
        polybar_background -> Text,
        polybar_foreground -> Text,
    }
}
