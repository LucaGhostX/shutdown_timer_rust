# 关机计时器 (Rust 版本) 2.4

[English](README.md) | [Italiano](README.it.md) | [Français](README.fr.md) | [Español](README.es.md) | **中文**

[![Vibecoded](https://img.shields.io/badge/development-vibecoded-ff69b4?style=flat-square)](https://github.com/LucaGhostX/shutdown_timer_rust)
[![Rust](https://img.shields.io/badge/rust-2021-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

> **注意**：本项目完全是通过 **vibecoded** 方式，从原项目 [**shutdown_timer**](https://github.com/LucaGhostX/shutdown_timer)（最初使用 Python/CustomTkinter 开发）移植至 Rust 的高性能版本。

一款紧凑、现代且响应迅速的计算机定时关机工具，具有原生深色/浅色模式图形界面、多语言支持、跨平台兼容性（**Windows**、**macOS**、**Linux**）以及智能睡眠阻止功能。

---

## 屏幕截图

![english (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807640-a37306bb-3599-416f-9863-6f5b32117d58.png)
![italian (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807707-e8c55222-fae6-4193-8b74-d32b1916ab25.png)
![french (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807746-c0ef6038-158d-4b31-a13d-76437af47d99.png)
![spanish (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807762-68fcffcb-4077-46dc-ba41-53b781ec15fa.png)
![chinese (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807783-f678cf79-e9b6-4831-bd4c-13d4d835b829.png)
![pause_reset functions (Personalizzato)](https://user-images.githubusercontent.com/64761832/210807849-ce10fce9-3c73-4b4e-8164-3f08ab96c3da.png)

---

## 功能特性

1. **高性能与即时响应**：
   - 使用 Rust 和 `eframe`/`egui` 编译为原生机器码。
   - 单一独立可执行文件，秒速启动，内存占用极低（< 30 MB）。
   - 不依赖 Python 或任何重量级运行时环境。

2. **设置面板与主题切换**：
   - 右下角齿轮图标，一键打开快捷设置窗口。
   - **主题切换**：可即时选择：
     - **深色**（*Dark Mode*）
     - **浅色**（*Light Mode*）
     - **系统**（*Auto* - 跟随操作系统设置）
   - **多语言支持 (i18n)**：
     - 英语 (`en`)
     - 意大利语 (`it`)
     - 法语 (`fr`)
     - 西班牙语 (`sp`)
     - 中文 (`ch`)
     - 首次启动时自动检测系统语言。

3. **便携式与安装式混合模式**：
   - 默认零残留（初次运行不创建任何配置文件）。
   - 用户在设置中修改语言或主题后，自动生成 `config.json`。
   - 在**便携模式**下（或存在 `portable.txt` 时），`config.json` 保存在程序所在目录下。
   - 在**安装模式**下，`config.json` 保存在操作系统标准用户配置目录：
     - **Windows**：%APPDATA%\ShutdownTimer\config.json
     - **macOS**：~/Library/Application Support/ShutdownTimer/config.json
     - **Linux**：$XDG_CONFIG_HOME/ShutdownTimer/config.json 或 ~/.config/ShutdownTimer/config.json

4. **原生跨平台系统集成**：
   - **Windows**：
     - 通过 `SetThreadExecutionState` 和 `PowerCreateRequest` 自动阻止电脑及显示器休眠。
     - 使用 `shutdown /s /t <秒数>` 执行关机，支持通过 `shutdown /a` 随时取消。
     - 自动从 Windows 注册表读取深色模式配置。
   - **macOS**：
     - 通过原生 `caffeinate` 工具阻止系统睡眠。
     - 通过 AppleScript（`osascript` 调用 `System Events`）在用户层级安全关机，无需 root 或 sudo 权限。
     - 通过 `defaults read -g AppleLocale` 检测系统语言。
   - **Linux**：
     - 通过 `systemd-inhibit` 阻止待机休眠。
     - 通过 `systemctl poweroff` / `shutdown` 执行计划关机，支持中断取消。
     - 自动检测 GNOME/freedesktop 深色模式配置。

5. **直观的计时器控制**：
   - 3 个数字微调框分别控制小时 (0-99)、分钟 (0-59) 和秒数 (0-59)。
   - 快捷 `+` 和 `-` 按钮支持循环回绕，文本框亦可直接键盘输入。
   - `HH:MM:SS` 格式的大字体数字倒计时显示。
   - **开始** (*Start*)、**暂停** (*Pause*)、**恢复** (*Resume*) 与 **重启** (*Reset*) 按钮。

6. **安全保护与确认弹窗**：
   - 设定时间为零时点击开始会弹出错误提示。
   - 点击重置 (*Reset*) 时提供确认弹窗，防止误触。
   - 计时进行中点击窗口关闭按钮 `[X]` 时拦截并弹出退出确认提示。

---

## 使用 GitHub Actions 构建与打包

仓库在 [`.github/workflows/build-and-release.yml`](.github/workflows/build-and-release.yml) 中配置了完整的自动化 CI/CD 工作流，在推送版本标签 (`v*`) 或手动触发时自动打包所有格式：

| 平台 | 软件包格式 | 生成的文件 |
| :--- | :--- | :--- |
| **Windows** | **便携版** (独立 Zip) | `ShutdownTimer-Windows-Portable.zip` |
| **Windows** | **安装包** (Inno Setup) | `ShutdownTimer-Windows-Setup.exe` |
| **macOS** | **Apple 磁盘映像与应用包** | `ShutdownTimer-macOS.dmg`, `.tar.gz` |
| **Linux** | **AppImage** | `ShutdownTimer-x86_64.AppImage` |
| **Linux** | **Flatpak 包** | `ShutdownTimer.flatpak` |

---

## 本地编译构建

### 前置条件
- **Rust 和 Cargo**（可通过 [rustup.rs](https://rustup.rs) 安装）

### 编译发布版本：
```bash
cargo build --release
```

### 运行测试套件：
```bash
cargo test
```

---

## 项目结构

```
shutdown_timer_rust/
├── .github/
│   └── workflows/
│       └── build-and-release.yml    # CI/CD 工作流 (Windows, macOS, AppImage, Flatpak)
├── assets/
│   ├── GFSNeohellenic-Bold.ttf      # 内嵌字体
│   ├── power.ico                    # 程序图标 (Windows)
│   ├── power_icon.png               # 窗口图标 (跨平台)
│   ├── settings_icon.png            # 深色模式齿轮图标
│   ├── settings_icon_dark.png       # 浅色模式齿轮图标
│   └── shutdown_timer_logo.png      # 内嵌 Logo
├── installer/
│   ├── flatpak/
│   │   └── io.github.lucag.ShutdownTimer.yml  # Flatpak 清单
│   ├── linux/
│   │   ├── io.github.lucag.ShutdownTimer.desktop
│   │   └── io.github.lucag.ShutdownTimer.metainfo.xml
│   ├── macos/
│   │   └── Info.plist               # macOS App Bundle 配置
│   └── windows/
│       └── setup.iss                # Windows 安装包 Inno Setup 脚本
├── src/
│   ├── main.rs                      # 程序入口、窗口初始化与字体加载
│   ├── app.rs                       # 应用主逻辑、界面绘制与设置弹窗
│   ├── spinbox.rs                   # 小时/分钟/秒微调框组件
│   ├── config.rs                    # 混合配置文件管理 (便携/系统安装)
│   ├── i18n.rs                      # 多语言文本与系统语言检测
│   ├── platform.rs                  # 原生跨平台抽象 (Windows, macOS, Linux)
│   ├── theme.rs                     # 调色板与深色/浅色/自动主题
│   └── timer.rs                     # 倒计时核心引擎
├── Cargo.toml                       # Rust 项目清单
├── LICENSE                          # MIT 许可证
└── README.md
```

---

## 许可证

本项目基于 **MIT** 许可证开源。详细条款请参见 [`LICENSE`](LICENSE) 文件。
