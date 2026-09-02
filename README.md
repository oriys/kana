# Kana

Kana 是一个面向中文母语零基础学习者的日语学习项目。

目标不是做一个“背五十音 + 刷 JLPT 题”的题库，而是构建一个以真实沟通能力（Can-do）为主线、中文母语迁移规律为重点、本地优先的完整学习系统。

## 当前状态

仓库已经包含第一个可运行的 Rust / Dioxus Desktop 学习闭环。应用从 **Stage 0 第一课「日语的声音不是“汉字读音”」** 开始，先建立 mora（拍）和 あ・い・う・え・お 的声音意识，再进入假名与 A1 场景课程。课程按学习目标、输入、中文讲解、中文学习者提醒、练习、主动输出与 Can-do 检查逐步呈现。

在 macOS 上运行：

```bash
cargo run -p kana-desktop
```

校验所有课程内容：

```bash
cargo run -p kana-content-lint -- content/zh-CN
```

运行核心测试：

```bash
cargo test --workspace --exclude kana-desktop
```

## 核心原则

1. **Can-do 驱动**：先定义“学完能做什么”，再组织词汇、语法、发音、汉字与练习。
2. **Chinese-first**：不是翻译英文教材；显式处理汉字正迁移、同形异义、中文音韵干扰和中文学习者常见发音问题。
3. **先理解再练习**：课程必须包含讲解、输入、注意、使用与反馈，不能一进入学习就直接做题。
4. **声音优先**：零基础阶段先建立日语的 mora、长音、促音、拨音、拗音等声音模型，再逐步淡出罗马音。
5. **本地优先**：核心课程、学习状态与复习调度默认离线可用；账号和云同步不是第一阶段依赖。
6. **Content as Code**：课程内容独立于 UI，以结构化数据维护，经过 lint / compile 后进入应用。
7. **可解释学习状态**：不使用单一“掌握/未掌握”；按 recognition、listening、reading、production、usage 等技能记录掌握度。

## 第一阶段范围

- Stage 0：声音与假名基础
- Stage 1：A1 日常沟通课程
- FSRS 间隔复习基础设施
- macOS / Desktop 本地客户端
- 课程内容编译与校验工具

云账号、多端同步、ASR 发音评分和 AI 对话放在教学闭环验证之后。

## 技术方向

- Rust workspace（MSRV 1.88）
- Dioxus 0.7 Desktop
- SQLite + SQLx
- Serde + YAML（课程源文件）
- FSRS（后续接入 `fsrs-rs`）
- Axum + PostgreSQL（多端同步阶段再启用）

## 仓库结构

```text
kana/
├── apps/desktop/          # Dioxus 桌面客户端
├── crates/domain/         # 核心领域模型
├── crates/curriculum/     # 课程加载、校验与编译
├── content/zh-CN/         # 中文课程源内容
├── docs/                  # 产品、教学与架构规范
└── tools/content-lint/    # 内容静态检查
```

## 开发顺序

当前优先把 **Stage 0 的教学闭环** 做正确：讲解 → 输入 → 练习 → 真实任务 → 复习。不会为了“功能齐全”提前堆登录、排行榜或云端。

详见 `docs/PRODUCT.md`、`docs/PEDAGOGY.md`、`docs/CURRICULUM.md` 与 `docs/ARCHITECTURE.md`。
