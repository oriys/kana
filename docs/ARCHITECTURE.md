# Architecture

## 1. 目标

架构服务于三个核心目标：

1. 本地优先、离线可学；
2. 课程内容与应用代码分离；
3. 未来支持多端和同步，但不提前引入分布式复杂度。

## 2. 技术栈

- Rust 2024 edition
- Dioxus Desktop
- Tokio
- SQLite
- SQLx
- Serde
- serde_yaml
- tracing
- FSRS：后续接入 `fsrs-rs`
- Server：后续 Axum + PostgreSQL

## 3. 分层

```text
UI (Dioxus)
    ↓
Application services
    ↓
Domain / Learning / Curriculum
    ↓
Repository traits
    ↓
SQLite adapters
```

桌面应用内部不通过 localhost HTTP 通信。

## 4. Workspace

```text
apps/desktop
crates/domain
crates/curriculum
crates/learning        # 后续
crates/srs             # 后续
tools/content-lint
```

### domain

只放稳定的业务概念和数据结构，不依赖 UI 或数据库。

### curriculum

负责：

- 解析课程源文件；
- 校验引用；
- 校验 lesson 结构；
- 将结构化内容转成 domain 模型；
- 后续编译成内容数据库。

### learning

负责：

- lesson session；
- exercise session；
- mastery；
- 错误分类；
- 学习路径选择。

### srs

负责：

- memory item；
- FSRS 状态与调度；
- review queue。

## 5. 领域方向

```text
Course
 └── Unit
      └── Lesson
           ├── CanDo
           ├── Step
           └── ConceptRef

Concept
 ├── Kana
 ├── Vocabulary
 ├── Expression
 ├── Grammar
 ├── KanjiWord
 └── Pronunciation

Mastery
 └── Concept × Skill

Skill
 ├── Recognition
 ├── Listening
 ├── Reading
 ├── Production
 └── Usage
```

`Lesson` 不直接等于一组 questions。Step 可以是 explanation、audio input、dialogue、exercise、production task 等不同教学行为。

## 6. Content as Code

源文件使用 YAML，便于人工编写与 review。

```text
content/zh-CN/**/*.yaml
        ↓
content-lint
        ↓
content compiler
        ↓
content.db / embedded pack
        ↓
Desktop
```

源内容必须可版本化、可 diff、可做 CI 静态校验。

## 7. 本地状态与内容分离

建议最终维护两个逻辑数据库：

```text
content.db       只读课程包，可升级 / 替换
user.db          用户学习状态、复习记录、设置
```

好处：

- 更新课程不会修改学习历史；
- 内容包可以版本化；
- 用户数据库更容易同步；
- 测试容易构造稳定 fixture。

## 8. ID 稳定性

课程内容中的 ID 一旦正式发布，不应随意变化。

例如：

```text
foundation.sound.mora
foundation.kana.a
vocab.taberu
grammar.desu
a1.u01.l01
```

用户学习历史和复习状态通过稳定 ID 关联内容。

## 9. 多端阶段

当确实需要跨设备同步时，再增加：

```text
Desktop/Mobile/Web
       ↓
      Axum
       ↓
 PostgreSQL/Object Storage
```

同步协议使用领域 ID 和版本号，而不是把 SQLite 文件整体上传。

服务端不能成为学习核心逻辑的唯一实现；尽可能复用 Rust crates。

## 10. 第一阶段架构门槛

在开始大量 UI 开发前，至少完成：

- domain 的课程核心类型；
- Lesson YAML schema 的可运行版本；
- content-lint；
- 第一课 fixture；
- 单元测试；
- 最小 desktop shell 能加载第一课。
