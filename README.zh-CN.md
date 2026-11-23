[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **简体中文**

# bmo-persona

以文件形式保存聊天智能体的人设层和设定(lore)库。基础层是智能体系统提示词中携带的人设;语气层和房间层按房间
叠加在其上;设定库是作为参考资料提供给智能体的片段索引。各层中的个人值都是占位符，由部署根据自身配置填入。本仓库
不运行任何服务:检查使内容符合读取方所依赖的格式，部署将内容放到读取方查找的位置，安装程序则把内容写成供聊天
智能体的大脑使用的大脑文件。

## 目录结构

| 文件夹 | 内容 |
|---|---|
| `layers/` | 基础层、语气层，以及 `rooms/` 下的房间层 |
| `lore/` | 设定索引、`chunks/` 下的片段，以及切分出这些片段的源文档 |
| `src/` | 安装程序:按 `docs/format.md` 的规定读取内容，并写成大脑文件 |
| `tests/` | 使内容符合格式、仓库符合规则的检查，以及安装程序的测试 |
| `docs/` | 关于格式、边界和清单的权威文档 |

根目录下有安装程序和检查所用的 Cargo 清单、版本、附带韩文台账(ledger)的 `testbed.toml`、`rustfmt.toml`、MIT
`LICENSE` 和 `CLAUDE.md`。

## 文档

| 文档 | 内容 |
|---|---|
| `docs/format.md` | 各层及其语气行的格式、设定索引及其片段，以及占位符 |
| `docs/boundary.md` | 本仓库提供什么、部署如何放置它、安装程序写出的大脑文件、哪个单元接收什么，以及源码树的各个读取方去往何处 |
| `docs/inventory.md` | 每个内容文件的来源和路径、被排除的内容，以及替换个人值的每个占位符 |
| `docs/PERSONAL-DATA.md` | 绝不进入本仓库的值、允许的两个值，以及保障这一点的检查 |

## 版本

- **整个仓库只有一个版本，** 采用 semver。`VERSION` 保存该版本，包的版本与之相同。
- **变更如何推动版本**，见 `docs/boundary.md` 的「What this repository is」。

## 命令

```sh
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --bin bmo-persona-install -- --out <file>
```

最后一条命令写出大脑文件(`docs/boundary.md`,「The brain file」);将其注入大脑是运维人员的步骤。
