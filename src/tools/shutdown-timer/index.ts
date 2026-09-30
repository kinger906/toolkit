import type { ToolModule } from "@/types/tool";
import ShutdownTimer from "./ShutdownTimer.vue";

const tool: ToolModule = {
  id: "shutdown-timer",
  name: "定时关机",
  description: "倒计时关机 / 重启，支持快捷时长与指定时刻",
  category: "system",
  icon: "⏻",
  keywords: ["shutdown", "restart", "timer", "关机", "重启", "定时", "倒计时"],
  order: 40,
  component: ShutdownTimer,
};

export default tool;
