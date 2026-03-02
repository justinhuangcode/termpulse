# termpulse

[English](./README.md) | **中文**

[![CI](https://github.com/justinhuangcode/termpulse/actions/workflows/ci.yml/badge.svg)](https://github.com/justinhuangcode/termpulse/actions/workflows/ci.yml)
[![Release](https://github.com/justinhuangcode/termpulse/actions/workflows/release.yml/badge.svg)](https://github.com/justinhuangcode/termpulse/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/termpulse?style=flat-square)](https://crates.io/crates/termpulse)
[![docs.rs](https://img.shields.io/docsrs/termpulse?style=flat-square&logo=docs.rs&logoColor=white)](https://docs.rs/termpulse)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/justinhuangcode/termpulse?style=flat-square&logo=github)](https://github.com/justinhuangcode/termpulse/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/justinhuangcode/termpulse?style=flat-square)](https://github.com/justinhuangcode/termpulse/commits/main)
[![Issues](https://img.shields.io/github/issues/justinhuangcode/termpulse?style=flat-square)](https://github.com/justinhuangcode/termpulse/issues)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey?style=flat-square)](https://github.com/justinhuangcode/termpulse)

通过 [OSC 9;4](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC) 实现原生终端进度指示器 — 智能终端检测、优雅降级、零配置。

## 为什么选择 termpulse？

构建工具、CI 管道和下载脚本都需要显示进度。但终端进度比想象中要难：

- **没有标准 API** — OSC 9;4 是最接近的方案，但每个终端的支持方式不同
- **静默失败** — 发送错误的转义序列会导致输出乱码
- **没有降级方案** — 如果终端不支持 OSC，进度信息直接消失

现有方案无法解决这些问题：

| | termpulse | [osc-progress](https://github.com/steipete/osc-progress) | 原始转义码 |
| --- | --- | --- | --- |
| 语言 | Rust（含 CLI） | TypeScript（仅库） | 任意 |
| 终端检测 | 10+ 终端，自动检测 | 3 个终端，手动 | 无 |
| 优雅降级 | OSC > ASCII > 静默 | 无 | 无 |
| tmux / screen | DCS 透传 | 不支持 | 手动包装 |
| 遵循 `NO_COLOR` | 是 | 否 | 手动 |
| Ctrl+C 清理 | 是 | 否 | 手动 |
| 节流 / 去重 | 内置 150ms | 手动 | 手动 |
| ETA 估算 | EMA 算法 | 无 | 手动 |
| 标签注入安全 | 已消毒 | 无 | 手动 |
| 序列解析器 | 完整往返 | 无 | 手动 |
| `no_std` 核心 | 零依赖，WASM 就绪 | 无 | 不适用 |

termpulse 自动检测终端，选择最佳输出方式，优雅降级。你只需调用 `termpulse set 50`，一切就绑定好了。

## 功能特性

- **零配置检测** — 自动检测 Ghostty、WezTerm、iTerm2、Kitty、Windows Terminal、VS Code、ConEmu、Contour、foot、Rio 等终端
- **三级降级** — OSC 9;4 原生进度 > ASCII 进度条（stderr）> 静默模式；不崩溃、不污染输出
- **tmux DCS 透传** — 检测 `TMUX` 环境变量，为 tmux 3.3+ 包装 DCS 透传信封
- **`wrap` 命令** — 运行任意 shell 命令并自动显示不确定进度；退出时发送完成/失败信号；转发子进程退出码
- **`pipe` 命令** — 透明的 stdin 到 stdout 管道，跟踪字节或行数；使用 `--total` 显示百分比，否则显示不确定计数器
- **节流去重引擎** — 将后端写入限制为 150ms 间隔；去重相同状态；状态和标签变更立即通过
- **ETA 估算** — 指数移动平均 (EMA) 算法，可配置 alpha；人类可读显示，上限 24 小时
- **标签消毒** — 零开销剥离 ESC、BEL、C1 ST 和控制字符；防止 OSC 转义注入
- **信号处理** — 在 `wrap` 模式下安装 `ctrlc` 处理器，即使按下 Ctrl+C 也会在退出前清除进度指示器
- **`NO_COLOR` 支持** — 遵循 [no-color.org](https://no-color.org/) 约定；`TERMPULSE_FORCE` 可覆盖
- **`no_std` 核心** — `termpulse-core` 零依赖、`#![no_std]`、`forbid(unsafe_code)`；适用于嵌入式、WASM 和 FFI 场景
- **依赖注入** — 所有 I/O 通过 trait（`Backend`、`EnvLookup`、`Write`）完成；111 个测试全覆盖

## 安装

### 预编译二进制（推荐）

从 [GitHub Releases](https://github.com/justinhuangcode/termpulse/releases) 下载：

| 平台 | 文件 |
| --- | --- |
| Linux x86_64 | `termpulse-x86_64-unknown-linux-gnu.tar.gz` |
| Linux aarch64 | `termpulse-aarch64-unknown-linux-gnu.tar.gz` |
| macOS x86_64 | `termpulse-x86_64-apple-darwin.tar.gz` |
| macOS Apple Silicon | `termpulse-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `termpulse-x86_64-pc-windows-msvc.zip` |

### 通过 Cargo

```bash
cargo install termpulse-cli
```

### 从源码构建

```bash
git clone https://github.com/justinhuangcode/termpulse.git
cd termpulse
cargo install --path crates/termpulse-cli
```

**系统要求：** Rust 1.85+

## 快速开始

### CLI

```bash
# 设置进度为 50%
termpulse set 50 -l "Building"

# 不确定状态旋转器
termpulse start -l "Compiling"

# 包装命令 — 显示进度，转发退出码
termpulse wrap -- cargo build --release

# 管道进度跟踪
curl -sL https://example.com/file.tar.gz \
  | termpulse pipe --total 104857600 -l "Downloading" \
  > file.tar.gz

# 发送完成信号
termpulse done -l "Build complete"
termpulse fail -l "Build failed"

# 检测终端能力
termpulse detect --json
```

### Rust 库

```rust
use termpulse::Controller;

let mut ctrl = Controller::auto();
ctrl.set(25, "Downloading");
ctrl.set(50, "Downloading");
ctrl.set(75, "Downloading");
ctrl.done("Complete");
```

### 核心库（`no_std`）

```rust
use termpulse_core::{OscSequence, ProgressState, Terminator};

let seq = OscSequence::normal_with_label(50, "Building");
let mut buf = [0u8; 256];
let n = seq.write_to(&mut buf).unwrap();
// buf[..n] = b"\x1b]9;4;1;50;Building\x1b\\"
```

## 命令

| 命令 | 说明 |
| --- | --- |
| `set <percent> [-l label]` | 设置进度百分比（0-100） |
| `start [-l label]` | 启动不确定进度 |
| `done [-l label]` | 发送成功信号（100% 然后清除） |
| `fail [-l label]` | 发送失败信号（错误状态然后清除） |
| `wrap -- <command...>` | 包装 shell 命令并显示进度 |
| `pipe [--total N] [--lines]` | 管道 stdin 到 stdout 并显示进度 |
| `clear` | 清除/移除进度指示器 |
| `detect` | 显示终端能力 |
| `completions <shell>` | 生成 shell 补全脚本（bash、zsh、fish、powershell、elvish） |

### 全局标志

| 标志 | 说明 |
| --- | --- |
| `--json` | 以 JSON 格式输出 |

### `wrap` 标志

| 标志 | 默认值 | 说明 |
| --- | --- | --- |
| `-l, --label` | `Running` | 执行期间显示的标签 |
| `--done-label` | `Done` | 成功时显示的标签 |
| `--fail-label` | `Failed` | 失败时显示的标签 |

### `pipe` 标志

| 标志 | 默认值 | 说明 |
| --- | --- | --- |
| `-t, --total` | — | 预期总字节数（启用百分比） |
| `--lines` | `false` | 按行而非字节计数 |
| `--buffer-size` | `8192` | 读取缓冲区大小（字节） |
| `-l, --label` | `Piping` | 管道期间显示的标签 |

## 工作原理

1. `Controller::auto()` 读取环境变量（`TERM_PROGRAM`、`WT_SESSION`、`TMUX` 等）
2. 检测终端并选择最佳后端：**OSC 9;4**、**ASCII** 或 **静默**
3. 如果在 tmux 内，将 OSC 序列包装在 DCS 透传中（`\ePtmux;...\e\\`）
4. 节流引擎将写入频率限制为 150ms；去重相同更新
5. 标签消毒器在嵌入前剥离危险字节（ESC、BEL、控制字符）
6. 完成/失败时，发送最终状态并清除指示器

```
termpulse set 50 -l "Building"
        |
        v
  检测终端（环境变量）
        |
        v
  选择后端（OSC / ASCII / 静默）
        |
        v
  节流 + 去重（150ms，跳过相同状态）
        |
        v
  消毒标签（剥离 ESC/BEL/控制字符）
        |
        v
  输出到 stderr（\x1b]9;4;1;50;Building\x1b\\）
```

## 架构

```
                    Cargo 工作区
+------------------+    +------------------+    +------------------+
|  termpulse-core  |    |    termpulse     |    |  termpulse-cli   |
| (no_std, 0 依赖) |--->| (库, 1 依赖)     |--->|  (二进制, 5 依赖) |
+------------------+    +------------------+    +------------------+
| OscSequence      |    | Controller       |    | set / start      |
| ProgressState    |    | detect()         |    | done / fail      |
| find_sequences() |    | Backend trait    |    | wrap / pipe      |
| sanitize_label() |    |   OscBackend     |    | clear / detect   |
| strip_sequences()|    |   TmuxBackend    |    |                  |
|                  |    |   AsciiBackend   |    |                  |
|                  |    |   SilentBackend  |    |                  |
|                  |    | Throttle         |    |                  |
|                  |    | Estimator        |    |                  |
+------------------+    +------------------+    +------------------+
```

**核心紧凑，外层渐宽** — 内层 crate 有最严格的约束（`no_std`、零依赖、`forbid(unsafe_code)`），外层 crate 逐步增加能力：

| Crate | `no_std` | 依赖数 | 用途 |
| --- | --- | --- | --- |
| `termpulse-core` | 是 | 0 | OSC 9;4 构建、解析、消毒、剥离 |
| `termpulse` | 否 | 1 (termpulse-core) | 检测、后端、节流、ETA |
| `termpulse-cli` | 否 | 5 (anyhow, clap, ctrlc, serde, serde_json) | CLI 二进制 |

## 终端支持

| 终端 | 检测方式 | 支持级别 |
| --- | --- | --- |
| Ghostty | `TERM_PROGRAM=ghostty` | OSC 9;4 原生 |
| WezTerm | `TERM_PROGRAM=wezterm` | OSC 9;4 原生 |
| iTerm2 | `TERM_PROGRAM=iTerm.app` | OSC 9;4 原生 |
| Kitty | `TERM_PROGRAM=kitty` | OSC 9;4 原生 |
| Windows Terminal | `WT_SESSION` 环境变量 | OSC 9;4 原生 |
| VS Code 终端 | `TERM_PROGRAM=vscode` | OSC 9;4 原生 |
| ConEmu | `ConEmuPID` 环境变量 | OSC 9;4 原生 |
| Contour | `TERM_PROGRAM=contour` | OSC 9;4 原生 |
| foot | `TERM=foot*` | OSC 9;4 原生 |
| Rio | `TERM_PROGRAM=rio` | OSC 9;4 原生 |
| tmux | `TMUX` 环境变量 | DCS 透传 |
| 其他 TTY | 是 TTY | ASCII 降级 `[====>    ] 50%` |
| 非 TTY（管道、文件） | 非 TTY | 静默（无输出） |

## 环境变量

| 变量 | 效果 |
| --- | --- |
| `TERMPULSE_FORCE=1` | 强制 OSC 模式，忽略检测结果 |
| `TERMPULSE_DISABLE=1` | 禁用 OSC，使用 ASCII 降级或静默 |
| `NO_COLOR` | 避免转义序列，使用 ASCII 降级（[no-color.org](https://no-color.org/)） |

## 项目结构

```
termpulse/
├── Cargo.toml                          # 工作区根（共享依赖、lint、元数据）
├── rust-toolchain.toml                 # 固定 stable + rustfmt + clippy
├── .github/workflows/ci.yml           # CI：测试、clippy、fmt、doc（Linux/macOS/Windows）
├── crates/
│   ├── termpulse-core/                 # no_std，零依赖
│   │   └── src/
│   │       ├── osc.rs                  # OscSequence, ProgressState, Terminator
│   │       ├── parse.rs                # find_sequences() — 零分配解析器
│   │       ├── sanitize.rs             # sanitize_label() — 注入防护
│   │       └── strip.rs               # strip_sequences() — 从文本中移除 OSC
│   ├── termpulse/                      # 主库
│   │   └── src/
│   │       ├── controller.rs           # 高级 Controller API
│   │       ├── detect.rs               # 终端 + 多路复用器检测
│   │       ├── throttle.rs             # 150ms 速率限制 + 去重
│   │       ├── estimate.rs             # ETA 估算（EMA 算法）
│   │       └── backend/                # OSC、tmux、ASCII、静默后端
│   └── termpulse-cli/                  # CLI 二进制
│       ├── src/cmd/                    # set, start, done, fail, wrap, pipe, clear, detect
│       └── tests/cli_integration.rs    # 20 个集成测试 (assert_cmd)
├── CHANGELOG.md
├── CONTRIBUTING.md                     # 贡献指南
├── AGENTS.md                           # 开发者指南
├── LICENSE                             # MIT
└── README.md
```

## 设计决策

- **选择 OSC 9;4 而非自定义协议** — OSC 9;4 是最广泛支持的终端进度协议，由 ConEmu 发起，被 Ghostty、WezTerm、iTerm2、Kitty、Windows Terminal 等采用
- **stderr 而非 stdout** — 进度输出到 stderr，不会污染管道数据；`pipe` 命令将 stdin 原样传递到 stdout
- **尽力写入** — 终端写入错误被静默忽略；进度是信息性的，不是关键的
- **150ms 节流** — 平衡视觉流畅度和终端性能；状态和标签变更绕过计时器
- **保守的多路复用器支持** — tmux 透传已启用（tmux 3.3+ 良好支持）；GNU screen 透传已禁用（跨版本太不可靠）

## 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

## 更新日志

请查看 [CHANGELOG.md](CHANGELOG.md) 了解版本历史。

## 致谢

灵感来自 [osc-progress](https://github.com/steipete/osc-progress)。

## 许可证

[MIT](LICENSE)
