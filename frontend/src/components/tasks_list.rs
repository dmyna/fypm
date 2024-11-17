use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use fypm_lib::values::structs::TaskWarriorExported;

use crate::request;

pub enum Message {
    Success(Vec<TaskWarriorExported>),
    Waiting,
    Error,
}

pub struct TasksList {
    tasks: Vec<TaskWarriorExported>,
    tasks_html: Html,
    first_exec: bool,
}

impl TasksList {
    pub fn get_filter(&self) -> String {
        let current_wt = "Calm"; //. ONLY TESTING

        let essential_string = "(+TODAY and +INSTANCE)";
        let scheduled_string =
            "((scheduled.after:today or scheduled:today) and scheduled.before:tomorrow)";

        let worktime_filter = format!(
            "(WT:{} or WT:AllDay) and ((({}) and WT.not:AllDay!) or {})",
            current_wt, essential_string, scheduled_string
        );

        let tw_filter = "+ACTIVE or ((+OVERDUE) and WT.not:AllDay!) or (due:today and (WT:Quantify or WT:NonSched))";

        format!("({} or {}) and status:pending", tw_filter, worktime_filter)
    }
    pub fn success(&mut self) {
        let mut entries: Vec<Html> = Vec::new();

        self.tasks.sort_by(|a, b| {
            b.urgency
                .partial_cmp(&a.urgency)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for task in &self.tasks {
            entries.push(html! {
                <div class="bg-gray-700 rounded-lg p-2">
                    <p>{task.description.clone()}</p>
                </div>
            });
        }

        self.tasks_html = html! { <div id="tasks" class="flex flex-col gap-1"> { for entries } </div> };
    }
    pub fn waiting(&mut self) {
        self.first_exec = false;
        self.tasks_html = html! { <div> {"Loading tasks..."} </div> };
    }
    pub fn error(&mut self) {
        self.tasks_html = html! { <div> {"Failed to load tasks"} </div> };
    }
}
impl Component for TasksList {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tasks: Vec::new(),
            tasks_html: html! { <div /> },
            first_exec: true,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Success(tasks) => {
                self.tasks = tasks;
                self.success();
                true
            }
            Message::Waiting => {
                self.waiting();
                true
            }
            Message::Error => {
                self.error();
                true
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link = _ctx.link();

        let filter = self.get_filter();
        let link_bind = link.clone();

        let update_tasks = || {
            spawn_local(async move {
                let response = request::task::get_by_filter(filter).await;

                match response {
                    Ok(tasks) => {
                        link_bind.send_message(Message::Success(tasks));
                    }
                    Err(_e) => {
                        link_bind.send_message(Message::Error);
                    }
                }
            });
        };

        if self.first_exec {
            link.send_message(Message::Waiting);

            update_tasks();
        }

        html! {
            <div class="h-full">
                {self.tasks_html.clone()}
            </div>
        }
    }
}
