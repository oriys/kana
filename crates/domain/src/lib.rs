use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lesson {
    pub id: ContentId,
    pub title: String,
    pub can_do: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<ContentId>,
    #[serde(default)]
    pub concepts: Vec<ConceptRef>,
    pub steps: Vec<LessonStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConceptRef {
    pub id: ContentId,
    pub kind: ConceptKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConceptKind {
    Kana,
    Vocabulary,
    Expression,
    Grammar,
    KanjiWord,
    Pronunciation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LessonStep {
    Goal {
        text: String,
    },
    Explanation {
        title: String,
        body: String,
    },
    Dialogue {
        context: String,
        lines: Vec<DialogueLine>,
    },
    Comprehension {
        prompt: String,
        choices: Vec<String>,
        answer: usize,
        explanation: String,
    },
    ChineseLearnerNote {
        title: String,
        body: String,
    },
    Exercise {
        prompt: String,
        answer: String,
        explanation: String,
        skill: Skill,
    },
    ProductionTask {
        prompt: String,
        success_criteria: Vec<String>,
    },
    CanDoCheck {
        prompt: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub japanese: String,
    #[serde(default)]
    pub reading: Option<String>,
    pub zh_cn: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Skill {
    Recognition,
    Listening,
    Reading,
    Production,
    Usage,
}
