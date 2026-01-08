# UNML - Universal Minecraft Launcher

一个使用 Rust 和 GPUI 构建的现代化 Minecraft 启动器。

## 特性

- 🚀 **高性能**: 使用 Rust 构建，充分利用系统资源
- 🎨 **现代化 UI**: 基于 GPUI 框架的流畅界面
- 📦 **模块化设计**: 清晰的 crate 分离，易于维护和扩展
- 🔧 **功能完整**:
  - 游戏下载和启动
  - Java 自动检测
  - Mod 管理（支持 Modrinth 和 CurseForge）
  - 账号系统（离线和微软登录）
  - 多下载源支持（官方和镜像）

## 项目结构

```
unml/
├── crates/
│   ├── unml-core/       # 核心 traits 和数据结构
│   ├── unml-java/       # Java 检测和管理
│   ├── unml-download/   # 下载提供者实现
│   ├── unml-launcher/   # 游戏启动器
│   ├── unml-mods/       # Mod 管理
│   ├── unml-auth/       # 账号系统
│   └── unml-gui/        # GPUI 应用界面
```

## 开发要求

- Rust nightly
- 操作系统: Windows / macOS / Linux

## 构建

```bash
# 检查所有 workspace
cargo check --workspace

# 构建
cargo build --release

# 运行
cargo run --bin unml
```

## 开发工具

项目配置了以下开发工具：

- **Clippy**: 代码质量检查（在 rust-analyzer 中自动运行）
- **Rustfmt**: 代码格式化
- **Edition 2024**: 使用最新的 Rust edition

## 许可证

本项目使用 AGPL-3.0-or-later 许可证分发。

## 贡献

欢迎贡献！请确保：
1. 代码通过 `cargo clippy` 检查
2. 代码格式化符合 `cargo fmt` 标准
3. 所有测试通过

## 路线图

- [ ] 完善下载功能实现
- [ ] 实现 GPUI 界面
- [ ] 添加完整的 Mod 管理功能
- [ ] 实现微软账号登录
- [ ] 添加游戏设置管理
- [ ] 支持更多 Mod 加载器

## 鸣谢

本项目参考了 [HMCL](https://github.com/HMCL-dev/HMCL) 的下载机制设计。
