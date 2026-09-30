import type { ToolModule } from "@/types/tool";
import JsonFormatter from "./JsonFormatter.vue";

const tool: ToolModule = {
  id: "json-formatter",
  name: "JSON 格式化",
  description: "格式化 / 压缩 JSON，快速校验语法",
  category: "text",
  icon: "{ }",
  keywords: ["json", "format", "beautify", "minify", "格式化"],
  order: 10,
  component: JsonFormatter,
};

export default tool;
