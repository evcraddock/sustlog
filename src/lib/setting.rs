use std::env;

pub fn get_root_folder() -> String {
    env::var("TODUIT_ROOT_FOLDER").expect("root folder variable not set")
}

pub fn get_project_folder() -> String {
    env::var("TODUIT_PROJECT_FOLDER").expect("project folder variable not set")
}

pub fn get_project_folder_name() -> String {
    env::var("TODUIT_PROJECT_FOLDER_NAME").expect("project folder name variable not set")
}

pub fn get_journal_folder() -> String {
    env::var("TODUIT_JOURNAL_FOLDER").expect("journal folder variable not set")
}

pub fn get_health_journal_root() -> String {
    env::var("SUSTLOG_HEALTH_JOURNAL_ROOT").expect("health journal root variable not set")
}

pub fn get_health_journal_folder() -> String {
    env::var("SUSTLOG_HEALTH_JOURNAL_FOLDER").expect("health journal folder variable not set")
}

pub fn get_relative_template_folder() -> String {
    env::var("SUSTLOG_RELATIVE_TEMPLATE_FOLDER").expect("relative health journal path variable not set")
}

pub fn get_todo_list() -> String {
    env::var("TODUIT_TODO_LISTS").expect("todo lists variable not set")
}

