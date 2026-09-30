import { computed, reactive, watch } from "vue";
import type { Alarm, Weekday } from "@/core/alarmTypes";
import {
  createAlarmId,
  fireKey,
  shouldFire,
} from "@/core/alarmTypes";

const STORAGE_KEY = "toolkit:alarms";

const state = reactive({
  alarms: [] as Alarm[],
  loaded: false,
});

function load() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      state.alarms = [];
    } else {
      const parsed = JSON.parse(raw) as Alarm[];
      state.alarms = Array.isArray(parsed) ? parsed : [];
    }
  } catch {
    state.alarms = [];
  }
  state.loaded = true;
}

function persist() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(state.alarms));
}

export function ensureAlarmsLoaded() {
  if (!state.loaded) load();
}

export const alarms = computed(() => state.alarms);

export const enabledCount = computed(
  () => state.alarms.filter((a) => a.enabled).length,
);

watch(
  () => state.alarms,
  () => {
    if (state.loaded) persist();
  },
  { deep: true },
);

export function listAlarms(): Alarm[] {
  ensureAlarmsLoaded();
  return state.alarms;
}

export function upsertAlarm(input: Omit<Alarm, "id"> & { id?: string }): Alarm {
  ensureAlarmsLoaded();
  if (input.id) {
    const idx = state.alarms.findIndex((a) => a.id === input.id);
    if (idx >= 0) {
      const next: Alarm = {
        ...state.alarms[idx],
        ...input,
        id: input.id,
        days: [...input.days] as Weekday[],
      };
      state.alarms[idx] = next;
      return next;
    }
  }
  const alarm: Alarm = {
    id: createAlarmId(),
    title: input.title.trim() || "闹钟",
    time: input.time,
    enabled: input.enabled,
    once: input.once,
    days: [...input.days] as Weekday[],
    note: input.note ?? "",
    lastFiredKey: input.lastFiredKey,
  };
  state.alarms.unshift(alarm);
  return alarm;
}

export function removeAlarm(id: string) {
  ensureAlarmsLoaded();
  state.alarms = state.alarms.filter((a) => a.id !== id);
}

export function toggleAlarm(id: string, enabled?: boolean) {
  ensureAlarmsLoaded();
  const a = state.alarms.find((x) => x.id === id);
  if (!a) return;
  a.enabled = enabled ?? !a.enabled;
}

export function markFired(id: string, now: Date) {
  const a = state.alarms.find((x) => x.id === id);
  if (!a) return;
  a.lastFiredKey = fireKey(now, a.time);
  if (a.once) a.enabled = false;
}

export function dueAlarms(now = new Date()): Alarm[] {
  ensureAlarmsLoaded();
  return state.alarms.filter((a) => shouldFire(a, now));
}
