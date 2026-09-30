/**
 * 工具入口：在此集中注册所有工具模块。
 *
 * 新增工具步骤：
 * 1. 在 src/tools/<tool-id>/ 下创建组件与 index.ts
 * 2. 在本文件 import 并加入 tools 数组
 */
import { registerTools } from "@/core/registry";
import type { ToolModule } from "@/types/tool";

import jsonFormatter from "./json-formatter";
import base64 from "./base64";
import timestamp from "./timestamp";
import shutdownTimer from "./shutdown-timer";
import videoToMp3 from "./video-to-mp3";
import colorPicker from "./color-picker";
import alarm from "./alarm";

const tools: ToolModule[] = [
  jsonFormatter,
  base64,
  timestamp,
  alarm,
  shutdownTimer,
  videoToMp3,
  colorPicker,
];

export function setupTools(): void {
  registerTools(tools);
}

export { tools };
