use kana_domain::{Lesson, LessonStep};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurriculumError {
    #[error("invalid yaml: {0}")]
    InvalidYaml(#[from] serde_yaml::Error),
    #[error("lesson validation failed: {0}")]
    Validation(String),
}

pub fn load_lesson(yaml: &str) -> Result<Lesson, CurriculumError> {
    let lesson: Lesson = serde_yaml::from_str(yaml)?;
    validate_lesson(&lesson)?;
    Ok(lesson)
}

pub fn validate_lesson(lesson: &Lesson) -> Result<(), CurriculumError> {
    if lesson.id.0.trim().is_empty() {
        return Err(CurriculumError::Validation("lesson id is empty".into()));
    }
    if lesson.title.trim().is_empty() {
        return Err(CurriculumError::Validation("lesson title is empty".into()));
    }
    if lesson.can_do.is_empty() || lesson.can_do.iter().any(|v| v.trim().is_empty()) {
        return Err(CurriculumError::Validation(
            "lesson must define at least one non-empty Can-do".into(),
        ));
    }
    if lesson.steps.is_empty() {
        return Err(CurriculumError::Validation("lesson has no teaching steps".into()));
    }

    let has_explanation = lesson
        .steps
        .iter()
        .any(|step| matches!(step, LessonStep::Explanation { .. }));
    let has_production = lesson
        .steps
        .iter()
        .any(|step| matches!(step, LessonStep::ProductionTask { .. }));
    let has_can_do_check = lesson
        .steps
        .iter()
        .any(|step| matches!(step, LessonStep::CanDoCheck { .. }));

    if !has_explanation {
        return Err(CurriculumError::Validation(
            "lesson must contain an explanation step".into(),
        ));
    }
    if !has_production {
        return Err(CurriculumError::Validation(
            "lesson must contain an active production task".into(),
        ));
    }
    if !has_can_do_check {
        return Err(CurriculumError::Validation(
            "lesson must end with a Can-do check".into(),
        ));
    }

    for step in &lesson.steps {
        match step {
            LessonStep::Comprehension {
                choices, answer, ..
            } => {
                if choices.len() < 2 {
                    return Err(CurriculumError::Validation(
                        "comprehension question needs at least two choices".into(),
                    ));
                }
                if *answer >= choices.len() {
                    return Err(CurriculumError::Validation(
                        "comprehension answer index is out of range".into(),
                    ));
                }
            }
            LessonStep::Dialogue { lines, .. } if lines.is_empty() => {
                return Err(CurriculumError::Validation(
                    "dialogue must contain at least one line".into(),
                ));
            }
            LessonStep::ProductionTask {
                success_criteria, ..
            } if success_criteria.is_empty() => {
                return Err(CurriculumError::Validation(
                    "production task needs success criteria".into(),
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_a_lesson_that_is_only_a_quiz() {
        let yaml = r#"
id: bad.lesson
title: 只有测试
can_do:
  - 能完成测试
steps:
  - type: comprehension
    prompt: 请选择
    choices: [A, B]
    answer: 0
    explanation: 因为 A 正确
"#;

        let error = load_lesson(yaml).unwrap_err();
        assert!(error.to_string().contains("explanation"));
    }
}
