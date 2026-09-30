# ToolKit

基于 **Tauri 2 + Vue 3** 的本地工具集合平台基座。桌面图标式首页、搜索、工具注册与路由已就绪，按约定新增模块即可扩展。

## 快速开始

```bash
npm install
npm run tauri:dev
```

仅前端预览（无桌面壳）：

```bash
npm run dev
```

打包：

```bash
npm run tauri:build
```

## 架构一览

```
src/
  core/registry.ts      # 工具注册表
  types/tool.ts         # ToolModule 类型与分类
  tools/                # 各工具模块（一工具一目录）
    index.ts            # 集中注册入口
    json-formatter/
    base64/
    timestamp/
  layouts/              # 顶栏 + 桌面式内容区
  views/                # 桌面首页 / 工具宿主页
  router/               # 路由
src-tauri/
  src/commands/         # Rust 侧命令（按模块扩展）
```

首页是桌面图标网格：按分类排列全部工具，点图标进入详情；工具页顶栏可「返回桌面」。

## 如何新增一个工具

### 1. 创建目录

```
src/tools/my-tool/
  MyTool.vue      # 工具 UI
  index.ts        # 元数据 + 导出
```

### 2. 编写 `index.ts`

```ts
import type { ToolModule } from "@/types/tool";
import MyTool from "./MyTool.vue";

const tool: ToolModule = {
  id: "my-tool",           // 路由 /tools/my-tool
  name: "我的工具",
  description: "一句话说明",
  category: "dev",         // text | encode | time | dev | system | other
  icon: "✦",
  keywords: ["my", "tool"],
  order: 40,
  component: MyTool,
};

export default tool;
```

### 3. 注册

在 `src/tools/index.ts` 中：

```ts
import myTool from "./my-tool";

const tools: ToolModule[] = [
  // ...existing
  myTool,
];
```

保存后桌面首页会自动出现该工具图标。

## 需要调用 Rust 时

1. 在 `src-tauri/src/commands/` 新增模块，例如 `hash.rs`
2. 在 `commands/mod.rs` 声明，并在 `lib.rs` 的 `invoke_handler` 中注册
3. 前端：

```ts
import { invoke } from "@tauri-apps/api/core";
const result = await invoke<string>("your_command", { arg: "..." });
```

已内置示例命令：`get_app_info`。

## 内置示例工具

| 工具 | 说明 |
|------|------|
| JSON 格式化 | 格式化 / 压缩 / 语法校验 |
| Base64 编解码 | UTF-8 安全编解码 |
| 时间戳转换 | Unix ↔ 本地 / ISO |

## 技术栈

- Tauri 2
- Vue 3 + TypeScript + Vite
- Vue Router 4
