# shen-nbt (nbt-rust)

> [!IMPORTANT]
> **Unmaintained / 已停止维护**
>
> This Rust NBT parsing project is no longer under active development. **No further feature updates, bug fixes, or releases are planned.** The source code remains available for historical and reference purposes.
>
> 本 Rust NBT 解析库项目 **已停止开发与维护**，不再计划新增功能、修复问题或发布版本。仓库仅保留历史代码供学习和参考。
>
> **Do not assume the existing implementations are complete or suitable for production use. / 请勿默认当前实现已完整或适合生产环境使用。**

一个（堆）由 shenjack 编写的 NBT 解析库。

## Historical implementations / 历史实现

- **[shen-nbt5 (v5)](./shen-nbt5/README.md)** — Earlier implementation / 旧版实现；存在尚未解决的 [正确性与安全性边界问题（Issue #3）](https://github.com/shenjackyuanjie/nbt-rust/issues/3)，以及未完成的 [Serde 支持（Issue #1）](https://github.com/shenjackyuanjie/nbt-rust/issues/1)。
- **[shen-nbt6 (v6)](./shen-nbt6/README.md)** — **Incomplete experiment / 未完成的实验版本**；原定的格式读写与 Serde 支持尚未全部实现，不再继续开发。

The README files in the version directories are preserved as historical development notes. Their TODO lists do **not** indicate ongoing work.

各版本子目录中的 README 作为历史开发记录保留。其中的 TODO **不代表项目仍在开发中**。
