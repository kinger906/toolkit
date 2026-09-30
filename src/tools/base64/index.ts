import type { ToolModule } from "@/types/tool";
import Base64Tool from "./Base64Tool.vue";

const tool: ToolModule = {
  id: "base64",
  name: "Base64 编解码",
  description: "支持 UTF-8 文本的 Base64 编码与解码",
  category: "encode",
  icon: "64",
  keywords: ["base64", "encode", "decode", "编码", "解码"],
  order: 20,
  component: Base64Tool,
};

export default tool;
