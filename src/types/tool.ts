import type { Component } from "vue";

/** 工具分类 */
export type ToolCategory =
  | "text"
  | "encode"
  | "time"
  | "dev"
  | "system"
  | "other";

export interface ToolMeta {
  /** 唯一标识，对应路由 /tools/:id */
  id: string;
  /** 显示名称 */
  name: string;
  /** 简短描述 */
  description: string;
  /** 分类，用于桌面分组 */
  category: ToolCategory;
  /** 搜索关键词 */
  keywords?: string[];
  /** 桌面图标（emoji 或单个字符） */
  icon: string;
  /** 排序权重，越小越靠前 */
  order?: number;
}

export interface ToolModule extends ToolMeta {
  /** 工具页面组件 */
  component: Component;
}

export const CATEGORY_LABELS: Record<ToolCategory, string> = {
  text: "文本处理",
  encode: "编码转换",
  time: "时间日期",
  dev: "开发辅助",
  system: "系统工具",
  other: "其他",
};

export const CATEGORY_ORDER: ToolCategory[] = [
  "text",
  "encode",
  "time",
  "dev",
  "system",
  "other",
];
