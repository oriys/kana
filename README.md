# Kana

Kana 是一个面向中文母语零基础学习者的日语学习 CLI。

目标不是做一个“背五十音 + 刷 JLPT 题”的题库，而是构建一个以真实沟通能力（Can-do）为主线、中文母语迁移规律为重点、本地优先的完整学习系统。

## 当前形态

**只做 CLI，不做桌面 GUI。**

核心学习流程全部在终端完成：

```text
课程列表
  ↓
选择课程
  ↓
目标 / 场景 / 讲解
  ↓
练习
  ↓
主动输出
  ↓
Can-do 检查
```

运行：

```bash
cargo run -p kana-cli -- list
cargo run -p kana-cli -- learn foundation.sound.l01
cargo run -p kana-cli -- learn a1.u01.l01
```

校验课程内容：

```bash
cargo run -p kana-content-lint -- content/zh-CN
```

运行测试：

```bash
cargo test --workspace
```

## 核心原则

1. **Can-do 驱动**：先定义“学完能做什么”，再组织词汇、语法、发音、汉字与练习。
2. **Chinese-first**：不是翻译英文教材；显式处理汉字正迁移、同形异义、中文音韵干扰和中文学习者常见发音问题。
3. **先理解再练习**：课程必须包含讲解、输入、注意、使用与反馈，不能一进入学习就直接做题。
4. **声音优先**：零基础阶段先建立日语的 mora、长音、促音、拨音、拗音等声音模型，再逐步淡出罗马音。
5. **本地优先**：核心课程、学习状态与复习调度默认离线可用。
6. **Content as Code**：课程内容独立于 CLI，以结构化 YAML 维护，经过 lint 后进入学习流程。
7. **可解释学习状态**：后续按 recognition、listening、reading、production、usage 等技能记录掌握度。

## 第一阶段范围

- Stage 0：声音与假名基础
- Stage 1：A1 日常沟通课程
- CLI 学习器
- 内容 lint / test
- FSRS 间隔复习基础设施（后续）
- SQLite 本地学习状态（后续）

账号、多端同步、GUI、ASR 发音评分和 AI 对话不属于当前阶段。

## 技术方向

- Rust workspace
- 纯终端 CLI
- Serde + YAML（课程源文件）
- SQLite + SQLx（学习状态阶段）
- FSRS（复习阶段）

## 仓库结构

```text
kana/
├── apps/cli/              # 终端学习器
├── crates/domain/         # 核心领域模型
├── crates/curriculum/     # 课程加载与校验
├── content/zh-CN/         # 中文课程源内容
├── docs/                  # 产品、教学与架构规范
└── tools/content-lint/    # 内容静态检查
```

详见 `docs/PRODUCT.md`、`docs/PEDAGOGY.md`、`docs/CURRICULUM.md` 与 `docs/ARCHITECTURE.md`。
