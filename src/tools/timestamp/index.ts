import type { ToolModule } from "@/types/tool";
import TimestampTool from "./TimestampTool.vue";

const tool: ToolModule = {
  id: "timestamp",
  name: "时间戳转换",
  description: "Unix 时间戳与本地 / ISO 时间互转",
  category: "time",
  icon: "⏱",
  keywords: ["timestamp", "unix", "date", "时间戳", "日期"],
  order: 30,
  component: TimestampTool,
};

export default tool;
