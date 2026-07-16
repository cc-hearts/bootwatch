# BootWatch 🔍

> 跨平台的开机启动项管理 TUI 工具，用 Rust 编写。

BootWatch 让你在终端里一览系统的开机启动项，并支持快速删除 / 刷新，告别在系统设置或注册表里来回翻找。

目前支持 **macOS** 与 **Windows** 两个平台。

---

## ✨ 功能特性

- 📋 **统一展示**：在一个列表中汇总系统各类启动项，含图标、类型标签、名称与路径。
- 🗑️ **安全删除**：选中后按 `d` 删除，二次确认避免误操作。
- 🔄 **一键刷新**：删除或外部改动后随时重新加载列表。
- ⌨️ **Vim 风格键位**：`j`/`k` 或方向键移动，符合终端习惯。
- 🖥️ **跨平台**：同一套界面，适配 macOS 与 Windows。

### 各平台识别的启动项类型

| 平台    | 类型                                   | 来源                                                                 |
| ------- | -------------------------------------- | -------------------------------------------------------------------- |
| macOS   | Plist                                  | `~/Library/LaunchAgents`、`/Library/LaunchAgents`、`/Library/LaunchDaemons` |
| macOS   | Login Item                             | System Events 登录项                                                  |
| Windows | Registry                               | `HKCU` / `HKLM` 下 `...\CurrentVersion\Run`                          |
| Windows | StartupFolder                          | `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`            |

---

## 📦 安装

### 从 Release 下载（推荐）

前往 [Releases 页面](https://github.com/cc-hearts/bootwatch/releases) 下载对应平台的预编译二进制：

| 平台 | 文件 |
| ---- | ---- |
| macOS (Apple Silicon) | `bootwatch-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `bootwatch-x86_64-apple-darwin.tar.gz` |
| Windows (x86_64) | `bootwatch-x86_64-pc-windows-msvc.zip` |

解压后将 `bootwatch` / `bootwatch.exe` 放入 `PATH` 即可。

或使用 [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) 一键安装：

```bash
cargo binstall --git https://github.com/cc-hearts/bootwatch bootwatch
```

### 从源码编译

需要已安装 [Rust](https://www.rust-lang.org/tools/install)（建议通过 rustup）。

```bash
git clone https://github.com/cc-hearts/bootwatch.git
cd bootwatch
cargo build --release
```

编译产物位于 `target/release/bootwatch`（Windows 下为 `bootwatch.exe`），可自行加入 `PATH`。

### 直接运行

```bash
cargo run
```

> 💡 删除启动项属于敏感操作，部分系统级启动项可能需要管理员 / root 权限才能成功移除。

---

## 🎮 使用方法

启动后进入交互式 TUI：

```
 BootWatch 🔍  开机启动项管理
┌─ 启动项 (N) ─────────────────────────────┐
│ ▶ 📝 [Plist] com.example.agent.plist     │
│      /Users/you/Library/LaunchAgents/...  │
│   🚀 [Login Item] Dropbox                │
│      -                                     │
└───────────────────────────────────────────┘
 📦 共发现 N 个开机启动项
 ↑/↓ 或 j/k 移动 · d 删除 · r 刷新 · q 退出
```

### 键位说明

| 按键            | 功能           |
| --------------- | -------------- |
| `↑` / `↓`       | 上下移动       |
| `j` / `k`       | 上下移动（Vim）|
| `d` / `Delete`  | 删除选中项     |
| `r`             | 刷新列表       |
| `y` / `Enter`   | 确认删除       |
| 其它任意键      | 取消删除       |
| `q` / `Esc`     | 退出程序       |

### 删除行为

- **macOS Plist**：先 `launchctl unload` 卸载，再删除 plist 文件。
- **macOS Login Item**：通过 `osascript` 调用 System Events 移除登录项。
- **Windows Registry**：按来源 hive 调用 `reg delete` 删除对应值（HKLM 需管理员权限）。
- **Windows StartupFolder**：直接删除 Startup 文件夹中的快捷方式文件。

---

## 🛠️ 技术栈

- [ratatui](https://github.com/ratatui/ratatui) + [crossterm](https://github.com/crossterm-rs/crossterm) — 终端 UI 与跨平台事件处理
- [shellexpand](https://crates.io/crates/shellexpand) — 展开 `~` 路径
- [winreg](https://crates.io/crates/winreg) — 读取 Windows 注册表（仅 Windows）
- [encoding_rs](https://crates.io/crates/encoding_rs) — 处理 Windows 命令输出的 GBK 编码

### 目录结构

```
bootwatch/
├── src/
│   ├── main.rs            # 程序入口
│   ├── tui.rs             # TUI 交互与渲染逻辑
│   └── platform/
│       ├── mod.rs         # 跨平台统一接口
│       ├── helper.rs      # 展示用数据结构
│       ├── macos.rs       # macOS 启动项读取 / 删除
│       └── windows.rs     # Windows 启动项读取 / 删除
├── Cargo.toml
└── Cargo.lock
```

---

## 🤝 贡献

欢迎提 Issue 或 PR！提 PR 前，请确保：

- `cargo fmt` 通过格式检查
- `cargo clippy` 无警告
- 已在对应平台实测过改动效果

---

## 📄 许可证

本项目基于 [MIT License](./LICENSE) 开源。
