use dioxus::prelude::*;
use kana_curriculum::load_lesson;
use kana_domain::LessonStep;

const LESSON_YAML: &str = include_str!("../../../content/zh-CN/a1/unit-01/lesson-01.yaml");
const APP_CSS: &str = r#"
:root {
  font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "PingFang SC", "Hiragino Sans", sans-serif;
  color: #1d1d1f;
  background: #f5f5f7;
}
* { box-sizing: border-box; }
body { margin: 0; }
button { font: inherit; }
.shell { min-height: 100vh; display: grid; grid-template-columns: 300px 1fr; }
.sidebar { background: rgba(255,255,255,.86); border-right: 1px solid #e6e6e8; padding: 28px 24px; }
.brand { font-size: 22px; font-weight: 760; margin-bottom: 36px; }
.eyebrow { font-size: 12px; color: #6e6e73; text-transform: uppercase; letter-spacing: .08em; }
.lesson-title { margin: 8px 0 8px; font-size: 26px; line-height: 1.2; }
.can-do { color: #515154; font-size: 14px; line-height: 1.6; }
.progress-track { height: 7px; background: #ececef; border-radius: 999px; overflow: hidden; margin: 24px 0 8px; }
.progress-fill { height: 100%; background: #1d1d1f; border-radius: inherit; }
.progress-label { color: #6e6e73; font-size: 12px; }
.main { padding: 56px min(8vw, 96px); display: flex; justify-content: center; }
.lesson { width: min(760px, 100%); }
.step-kind { color: #6e6e73; font-size: 13px; margin-bottom: 10px; }
.card { background: #fff; border: 1px solid #e7e7e9; border-radius: 22px; padding: 34px; box-shadow: 0 10px 34px rgba(0,0,0,.04); min-height: 390px; }
.card h1, .card h2 { margin-top: 0; }
.card h1 { font-size: 34px; letter-spacing: -.02em; }
.card h2 { font-size: 25px; }
.body { white-space: pre-wrap; line-height: 1.85; color: #333336; font-size: 16px; }
.dialogue-line { padding: 14px 0; border-bottom: 1px solid #f0f0f2; }
.dialogue-line:last-child { border-bottom: 0; }
.speaker { color: #6e6e73; font-size: 12px; margin-bottom: 5px; }
.japanese { font-size: 23px; margin-bottom: 4px; }
.reading { font-size: 13px; color: #86868b; }
.translation { font-size: 14px; color: #515154; margin-top: 5px; }
.note { border-left: 4px solid #1d1d1f; padding-left: 18px; }
.choice { display: block; width: 100%; margin: 10px 0; padding: 14px 16px; text-align: left; border: 1px solid #dedee2; background: #fafafa; border-radius: 12px; }
.answer { margin-top: 22px; padding: 14px 16px; border-radius: 12px; background: #f5f5f7; line-height: 1.6; }
.criteria { line-height: 1.9; padding-left: 22px; }
.nav { display: flex; align-items: center; justify-content: space-between; margin-top: 18px; }
.nav button { border: 0; border-radius: 999px; padding: 11px 20px; cursor: pointer; }
.secondary { background: #e9e9ec; color: #1d1d1f; }
.primary { background: #1d1d1f; color: white; }
.primary:disabled, .secondary:disabled { opacity: .35; cursor: default; }
@media (max-width: 760px) { .shell { grid-template-columns: 1fr; } .sidebar { border-right: 0; border-bottom: 1px solid #e6e6e8; } .main { padding: 28px 18px; } }
"#;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let lesson = load_lesson(LESSON_YAML).expect("bundled lesson must be valid");
    let total = lesson.steps.len();
    let mut current = use_signal(|| 0usize);
    let index = *current.read();
    let step = lesson.steps[index].clone();
    let progress = ((index + 1) as f64 / total as f64) * 100.0;

    rsx! {
        style { "{APP_CSS}" }
        div { class: "shell",
            aside { class: "sidebar",
                div { class: "brand", "かな · Kana" }
                div { class: "eyebrow", "A1 · Unit 1 · Lesson 1" }
                h1 { class: "lesson-title", "{lesson.title}" }
                for goal in &lesson.can_do {
                    p { class: "can-do", "{goal}" }
                }
                div { class: "progress-track",
                    div { class: "progress-fill", style: "width: {progress:.1}%" }
                }
                div { class: "progress-label", "{index + 1} / {total}" }
            }
            main { class: "main",
                section { class: "lesson",
                    div { class: "step-kind", "{step_label(&step)}" }
                    div { class: "card", {render_step(step)} }
                    div { class: "nav",
                        button {
                            class: "secondary",
                            disabled: index == 0,
                            onclick: move |_| if index > 0 { current.set(index - 1) },
                            "上一步"
                        }
                        button {
                            class: "primary",
                            disabled: index + 1 >= total,
                            onclick: move |_| if index + 1 < total { current.set(index + 1) },
                            if index + 1 >= total { "课程完成" } else { "继续" }
                        }
                    }
                }
            }
        }
    }
}

fn step_label(step: &LessonStep) -> &'static str {
    match step {
        LessonStep::Goal { .. } => "学习目标",
        LessonStep::Explanation { .. } => "理解",
        LessonStep::Dialogue { .. } => "场景对话",
        LessonStep::Comprehension { .. } => "理解检查",
        LessonStep::ChineseLearnerNote { .. } => "中文学习者提醒",
        LessonStep::Exercise { .. } => "练习",
        LessonStep::ProductionTask { .. } => "主动输出",
        LessonStep::CanDoCheck { .. } => "Can-do 检查",
    }
}

fn render_step(step: LessonStep) -> Element {
    match step {
        LessonStep::Goal { text } => rsx! {
            h1 { "今天你会做到什么" }
            p { class: "body", "{text}" }
        },
        LessonStep::Explanation { title, body } => rsx! {
            h2 { "{title}" }
            div { class: "body", "{body}" }
        },
        LessonStep::Dialogue { context, lines } => rsx! {
            h2 { "先进入场景" }
            p { class: "body", "{context}" }
            div {
                for line in lines {
                    div { class: "dialogue-line",
                        div { class: "speaker", "{line.speaker}" }
                        div { class: "japanese", "{line.japanese}" }
                        if let Some(reading) = line.reading {
                            div { class: "reading", "{reading}" }
                        }
                        div { class: "translation", "{line.zh_cn}" }
                    }
                }
            }
        },
        LessonStep::Comprehension {
            prompt,
            choices,
            answer,
            explanation,
        } => rsx! {
            h2 { "{prompt}" }
            for choice in &choices {
                button { class: "choice", "{choice}" }
            }
            div { class: "answer",
                strong { "参考答案：" }
                "{choices[answer]}"
                br {}
                "{explanation}"
            }
        },
        LessonStep::ChineseLearnerNote { title, body } => rsx! {
            div { class: "note",
                h2 { "{title}" }
                div { class: "body", "{body}" }
            }
        },
        LessonStep::Exercise {
            prompt,
            answer,
            explanation,
            ..
        } => rsx! {
            h2 { "自己试一下" }
            p { class: "body", "{prompt}" }
            div { class: "answer",
                strong { "答案：{answer}" }
                br {}
                "{explanation}"
            }
        },
        LessonStep::ProductionTask {
            prompt,
            success_criteria,
        } => rsx! {
            h2 { "现在由你来说" }
            p { class: "body", "{prompt}" }
            ul { class: "criteria",
                for criterion in success_criteria {
                    li { "{criterion}" }
                }
            }
        },
        LessonStep::CanDoCheck { prompt } => rsx! {
            h1 { "这一课完成了" }
            p { class: "body", "{prompt}" }
            p { class: "body", "能做到就继续；还不稳定也没关系，之后会通过间隔复习再次遇到这些内容。" }
        },
    }
}
