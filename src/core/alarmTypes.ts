export type Weekday = 0 | 1 | 2 | 3 | 4 | 5 | 6; // 0=周日 … 6=周六

export interface Alarm {
  id: string;
  title: string;
  /** 24h "HH:mm" */
  time: string;
  enabled: boolean;
  /** 仅响一次后自动关闭 */
  once: boolean;
  /** 重复星期；once=false 时至少选一天；once=true 时可为空 */
  days: Weekday[];
  note: string;
  /** 上次触发的本地日期 YYYY-MM-DD，防止同一分钟重复响 */
  lastFiredKey?: string;
}

export const WEEKDAY_LABELS = ["日", "一", "二", "三", "四", "五", "六"] as const;

export const WEEKDAY_PRESETS: { id: string; label: string; days: Weekday[] }[] = [
  { id: "everyday", label: "每天", days: [0, 1, 2, 3, 4, 5, 6] },
  { id: "weekdays", label: "工作日", days: [1, 2, 3, 4, 5] },
  { id: "weekend", label: "周末", days: [0, 6] },
];

export function createAlarmId(): string {
  return `alarm_${Date.now().toString(36)}_${Math.random().toString(36).slice(2, 7)}`;
}

export function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

export function formatDays(alarm: Alarm): string {
  if (alarm.once) return "仅一次";
  if (alarm.days.length === 7) return "每天";
  const sorted = [...alarm.days].sort((a, b) => a - b);
  if (sorted.join(",") === "1,2,3,4,5") return "工作日";
  if (sorted.join(",") === "0,6") return "周末";
  return sorted.map((d) => `周${WEEKDAY_LABELS[d]}`).join("、");
}

export function nextFireHint(alarm: Alarm, now = new Date()): string {
  if (!alarm.enabled) return "已关闭";
  const [hh, mm] = alarm.time.split(":").map(Number);
  if (!Number.isFinite(hh) || !Number.isFinite(mm)) return "";

  if (alarm.once) {
    const d = new Date(now);
    d.setSeconds(0, 0);
    d.setHours(hh, mm, 0, 0);
    if (d.getTime() <= now.getTime()) {
      d.setDate(d.getDate() + 1);
    }
    return `下次 ${pad2(d.getMonth() + 1)}-${pad2(d.getDate())} ${alarm.time}`;
  }

  if (!alarm.days.length) return "请选择重复日";

  for (let offset = 0; offset < 8; offset++) {
    const d = new Date(now);
    d.setSeconds(0, 0);
    d.setDate(now.getDate() + offset);
    d.setHours(hh, mm, 0, 0);
    if (d.getTime() <= now.getTime()) continue;
    const dow = d.getDay() as Weekday;
    if (alarm.days.includes(dow)) {
      if (offset === 0) return `今天 ${alarm.time}`;
      if (offset === 1) return `明天 ${alarm.time}`;
      return `周${WEEKDAY_LABELS[dow]} ${alarm.time}`;
    }
  }
  return "";
}

/** 判断当前这一分钟是否应触发 */
export function shouldFire(alarm: Alarm, now: Date): boolean {
  if (!alarm.enabled) return false;
  const [hh, mm] = alarm.time.split(":").map(Number);
  if (now.getHours() !== hh || now.getMinutes() !== mm) return false;

  const key = `${now.getFullYear()}-${pad2(now.getMonth() + 1)}-${pad2(now.getDate())}T${alarm.time}`;
  if (alarm.lastFiredKey === key) return false;

  if (alarm.once) return true;
  return alarm.days.includes(now.getDay() as Weekday);
}

export function fireKey(now: Date, time: string): string {
  return `${now.getFullYear()}-${pad2(now.getMonth() + 1)}-${pad2(now.getDate())}T${time}`;
}
