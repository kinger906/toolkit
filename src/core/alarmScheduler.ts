import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { dueAlarms, ensureAlarmsLoaded, markFired } from "@/core/alarmStore";
import type { Alarm } from "@/core/alarmTypes";

let started = false;
let timer: ReturnType<typeof setInterval> | null = null;

function beep() {
  try {
    const ctx = new AudioContext();
    const now = ctx.currentTime;
    const notes = [880, 1174.7, 880];
    notes.forEach((freq, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = "sine";
      osc.frequency.value = freq;
      gain.gain.setValueAtTime(0.0001, now + i * 0.22);
      gain.gain.exponentialRampToValueAtTime(0.2, now + i * 0.22 + 0.02);
      gain.gain.exponentialRampToValueAtTime(0.0001, now + i * 0.22 + 0.2);
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.start(now + i * 0.22);
      osc.stop(now + i * 0.22 + 0.22);
    });
    setTimeout(() => void ctx.close(), 1200);
  } catch {
    // ignore audio failures
  }
}

async function ensureNotifyPermission() {
  let granted = await isPermissionGranted();
  if (!granted) {
    const perm = await requestPermission();
    granted = perm === "granted";
  }
  return granted;
}

async function revealWindow() {
  try {
    const win = getCurrentWindow();
    await win.setSkipTaskbar(false);
    await win.show();
    await win.unminimize();
    await win.setFocus();
  } catch {
    // ignore
  }
}

async function ring(alarm: Alarm) {
  markFired(alarm.id, new Date());
  beep();
  await revealWindow();

  const body = alarm.note?.trim()
    ? `${alarm.time} · ${alarm.note}`
    : `${alarm.time} 到了`;

  try {
    if (await ensureNotifyPermission()) {
      sendNotification({
        title: alarm.title || "闹钟提醒",
        body,
      });
    }
  } catch {
    // notification optional
  }
}

function tick() {
  const now = new Date();
  const due = dueAlarms(now);
  for (const alarm of due) {
    void ring(alarm);
  }
}

/** 全局闹钟调度：应用启动后常驻，收起到托盘也会继续检查 */
export function startAlarmScheduler() {
  if (started) return;
  started = true;
  ensureAlarmsLoaded();
  void ensureNotifyPermission();
  tick();
  timer = setInterval(tick, 5_000);
}

export function stopAlarmScheduler() {
  if (timer) clearInterval(timer);
  timer = null;
  started = false;
}
