import type { ToolModule } from "@/types/tool";
import AlarmTool from "./AlarmTool.vue";

const tool: ToolModule = {
  id: "alarm",
  name: "闹钟提醒",
  description: "多个闹钟，支持按星期多周期重复提醒",
  category: "time",
  icon: "⏰",
  keywords: ["alarm", "reminder", "clock", "闹钟", "提醒", "周期", "定时"],
  order: 35,
  component: AlarmTool,
};

export default tool;
