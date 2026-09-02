# Kana

Kana 是一个面向中文母语零基础学习者的日语学习 CLI。

当前第一阶段只聚焦三件事：

1. **假名**：从声音系统开始，完整掌握平假名、片假名、浊音、半浊音、拗音、促音、长音、拨音。
2. **N5 词汇**：按主题和使用场景组织，不做孤立词表；每个词包含读音、中文义、例句、常见搭配和中文学习者提醒。
3. **N5 语法**：按“能表达什么”组织，从句型功能出发，再解释助词、动词变化和形容词变化。

暂时不扩展到 N4、不做 GUI、不做云端、不做 AI 对话。

## 第一阶段学习路径

```text
假名与声音
  ↓
基础高频词汇
  ↓
基础句型
  ↓
N5 词汇 + 语法
  ↓
综合阅读 / 听辨 / 造句
```

## CLI

```bash
cargo run -p kana-cli -- list
cargo run -p kana-cli -- learn foundation.sound.l01
cargo run -p kana-cli -- learn kana.hiragana.a-row
cargo run -p kana-cli -- learn n5.vocab.people.001
cargo run -p kana-cli -- learn n5.grammar.desu
```

## 内容原则

- 所有讲解使用中文。
- 假名不是只靠五十音表死记，必须包含声音、辨认、输入和混淆对比。
- N5 词汇按主题和语境组织，并显式标注中日汉字关系与同形异义风险。
- N5 语法不按术语堆砌，而按“我现在能表达什么”组织。
- 每个 Lesson 必须有讲解、练习、主动输出和最终 Can-do 检查。
- JLPT N5 只作为范围和阶段验收参考，不把课程做成题库。

## 仓库结构

```text
kana/
├── apps/cli/
├── crates/domain/
├── crates/curriculum/
├── content/zh-CN/
│   ├── foundation/
│   ├── kana/
│   └── n5/
│       ├── vocabulary/
│       └── grammar/
├── docs/
└── tools/content-lint/
```

## 当前优先级

P0：完整假名课程结构与第一批课程。

P1：N5 词汇分类、词条格式、第一批高频词汇课程。

P2：N5 语法地图、第一批基础句型课程。

P3：CLI 学习体验、学习状态和复习机制。
