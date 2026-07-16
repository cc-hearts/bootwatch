# Changelog

本文件记录 BootWatch 各版本的显著变更，格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

## [0.1.0] - 2026-07-16

首发版本：基于 ratatui 的跨平台（macOS / Windows）开机启动项管理 TUI。

### Added

- macOS：读取 `LaunchAgents` / `LaunchDaemons` plist 与登录项，支持卸载并删除。
- Windows：读取 `HKCU` / `HKLM` 下 `Run` 键与 Startup 文件夹，支持删除。
- TUI 交互：`j/k` 或方向键移动、`d` 删除（带二次确认）、`r` 刷新、`q` 退出。
- 单元测试：覆盖删除令牌解析、AppleScript 字符串转义等纯逻辑。
- `rust-toolchain.toml`：固定 stable 工具链与 `rustfmt`/`clippy` 组件。
- GitHub Actions CI：macOS 与 Windows 上运行 `fmt` / `clippy` / `test` / `build`。
- GitHub Actions Release：tag 触发，自动构建 macOS (Apple Silicon/Intel) 与 Windows 二进制并发布到 Release。
- 支持 `cargo binstall --git` 安装预编译二进制。

### Fixed

- **Windows 删除逻辑**：此前删除操作无视启动项来源，统一对 `HKCU\...\Run` 执行 `reg delete`，导致 HKLM 项与 StartupFolder 项无法删除。现按类型分流——Registry 项按来源 hive 构造完整键路径删除，StartupFolder 项直接删除对应文件。
- **macOS 删除不再 panic**：移除 `launchctl unload` / `fs::remove_file` / `osascript` 调用处的 `.expect()`，改为向上返回 `Err`，避免在 TUI 的 alternate screen 下崩溃导致终端卡死。
- **macOS 登录项注入风险**：对传入 AppleScript 的名称做转义，避免项名称中包含 `"` / `\` 时破坏脚本。
- **终端崩溃恢复**：`tui::run` 设置 panic hook，运行中 panic 时先还原终端（退出 raw mode 与 alternate screen）再传播 panic。

### Changed

- **删除令牌统一化**：各平台在构造启动项时生成结构化删除令牌（如 `plist|<path>`、`reg|<key>|<name>`、`file|<path>`），删除时由对应平台解析，避免此前依赖 `label` 字符串匹配判断类型的脆弱写法。
- **依赖按平台门控**：`encoding_rs` 移入 `[target.'cfg(windows)'.dependencies]`，macOS 构建不再拉取该 crate。
- `.gitignore` 补充 `.DS_Store`、`.idea/`、`.vscode/` 等常见忽略项。
