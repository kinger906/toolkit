import type { ToolModule } from "@/types/tool";
import ColorPicker from "./ColorPicker.vue";

const tool: ToolModule = {
  id: "color-picker",
  name: "颜色取色器",
  description: "抓取屏幕任意位置颜色，转换 HEX / RGB / HSL",
  category: "dev",
  icon: "◐",
  keywords: ["color", "picker", "hex", "rgb", "hsl", "取色", "颜色", "吸色"],
  order: 60,
  component: ColorPicker,
};

export default tool;
