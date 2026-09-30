import { reactive, computed } from "vue";
import type { ToolModule, ToolCategory } from "@/types/tool";
import { CATEGORY_ORDER } from "@/types/tool";

const tools = reactive<ToolModule[]>([]);

/** 注册单个工具 */
export function registerTool(tool: ToolModule): void {
  const idx = tools.findIndex((t) => t.id === tool.id);
  if (idx >= 0) {
    tools[idx] = tool;
    return;
  }
  tools.push(tool);
}

/** 批量注册 */
export function registerTools(list: ToolModule[]): void {
  list.forEach(registerTool);
}

/** 按 id 获取工具 */
export function getTool(id: string): ToolModule | undefined {
  return tools.find((t) => t.id === id);
}

/** 全部工具（按 order / name 排序） */
export const allTools = computed(() =>
  [...tools].sort((a, b) => {
    const oa = a.order ?? 100;
    const ob = b.order ?? 100;
    if (oa !== ob) return oa - ob;
    return a.name.localeCompare(b.name, "zh-CN");
  }),
);

/** 按分类分组 */
export const toolsByCategory = computed(() => {
  const map = new Map<ToolCategory, ToolModule[]>();
  for (const cat of CATEGORY_ORDER) {
    map.set(cat, []);
  }
  for (const tool of allTools.value) {
    const list = map.get(tool.category) ?? [];
    list.push(tool);
    map.set(tool.category, list);
  }
  return [...map.entries()].filter(([, list]) => list.length > 0);
});

/** 关键词搜索 */
export function searchTools(query: string): ToolModule[] {
  const q = query.trim().toLowerCase();
  if (!q) return allTools.value;
  return allTools.value.filter((t) => {
    const hay = [t.id, t.name, t.description, ...(t.keywords ?? [])]
      .join(" ")
      .toLowerCase();
    return hay.includes(q);
  });
}
