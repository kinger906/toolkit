<script setup lang="ts">
import { computed, onMounted, reactive, ref } from "vue";
import {
  alarms,
  ensureAlarmsLoaded,
  enabledCount,
  removeAlarm,
  toggleAlarm,
  upsertAlarm,
} from "@/core/alarmStore";
import type { Alarm, Weekday } from "@/core/alarmTypes";
import {
  WEEKDAY_LABELS,
  WEEKDAY_PRESETS,
  formatDays,
  nextFireHint,
  pad2,
} from "@/core/alarmTypes";

const editingId = ref<string | null>(null);
const error = ref("");
const toast = ref("");

const form = reactive({
  title: "闹钟",
  time: "08:00",
  once: false,
  days: [1, 2, 3, 4, 5] as Weekday[],
  note: "",
  enabled: true,
});

const sortedAlarms = computed(() =>
  [...alarms.value].sort((a, b) => a.time.localeCompare(b.time)),
);

onMounted(() => {
  ensureAlarmsLoaded();
  const now = new Date();
  form.time = `${pad2(now.getHours())}:${pad2((now.getMinutes() + 1) % 60)}`;
});

function resetForm() {
  editingId.value = null;
  form.title = "闹钟";
  const now = new Date();
  form.time = `${pad2(now.getHours())}:${pad2((now.getMinutes() + 1) % 60)}`;
  form.once = false;
  form.days = [1, 2, 3, 4, 5];
  form.note = "";
  form.enabled = true;
  error.value = "";
}

function editAlarm(alarm: Alarm) {
  editingId.value = alarm.id;
  form.title = alarm.title;
  form.time = alarm.time;
  form.once = alarm.once;
  form.days = [...alarm.days];
  form.note = alarm.note;
  form.enabled = alarm.enabled;
  error.value = "";
}

function toggleDay(day: Weekday) {
  if (form.once) return;
  const idx = form.days.indexOf(day);
  if (idx >= 0) form.days.splice(idx, 1);
  else form.days.push(day);
  form.days.sort((a, b) => a - b);
}

function applyPreset(days: Weekday[]) {
  form.once = false;
  form.days = [...days];
}

function setOnce() {
  form.once = true;
  form.days = [];
}

function save() {
  error.value = "";
  if (!/^\d{2}:\d{2}$/.test(form.time)) {
    error.value = "请填写有效时间（HH:mm）";
    return;
  }
  if (!form.once && form.days.length === 0) {
    error.value = "请选择至少一个重复日，或改为「仅一次」";
    return;
  }

  upsertAlarm({
    id: editingId.value ?? undefined,
    title: form.title.trim() || "闹钟",
    time: form.time,
    once: form.once,
    days: form.once ? [] : ([...form.days] as Weekday[]),
    note: form.note.trim(),
    enabled: form.enabled,
  });

  toast.value = editingId.value ? "闹钟已更新" : "闹钟已添加";
  setTimeout(() => {
    if (toast.value.startsWith("闹钟已")) toast.value = "";
  }, 1500);
  resetForm();
}

function remove(id: string) {
  removeAlarm(id);
  if (editingId.value === id) resetForm();
}
</script>

<template>
  <div class="tool">
    <div class="summary">
      共 {{ alarms.length }} 个闹钟 · 启用 {{ enabledCount }} 个
      <span class="tip">收起到托盘后仍会提醒</span>
    </div>

    <section class="editor">
      <h3>{{ editingId ? "编辑闹钟" : "新建闹钟" }}</h3>

      <div class="grid">
        <label class="field">
          <span>标题</span>
          <input v-model="form.title" type="text" maxlength="40" placeholder="例如：开会" />
        </label>
        <label class="field">
          <span>时间</span>
          <input v-model="form.time" type="time" />
        </label>
      </div>

      <div class="field">
        <span>周期</span>
        <div class="presets">
          <button
            type="button"
            class="chip"
            :class="{ active: form.once }"
            @click="setOnce"
          >
            仅一次
          </button>
          <button
            v-for="p in WEEKDAY_PRESETS"
            :key="p.id"
            type="button"
            class="chip"
            :class="{
              active:
                !form.once &&
                form.days.slice().sort().join() === p.days.slice().sort().join(),
            }"
            @click="applyPreset(p.days)"
          >
            {{ p.label }}
          </button>
        </div>
        <div class="days">
          <button
            v-for="(label, day) in WEEKDAY_LABELS"
            :key="day"
            type="button"
            class="day"
            :class="{ active: !form.once && form.days.includes(day as Weekday) }"
            :disabled="form.once"
            @click="toggleDay(day as Weekday)"
          >
            {{ label }}
          </button>
        </div>
      </div>

      <label class="field">
        <span>备注</span>
        <input v-model="form.note" type="text" maxlength="80" placeholder="可选" />
      </label>

      <label class="check">
        <input v-model="form.enabled" type="checkbox" />
        <span>创建后立即启用</span>
      </label>

      <div class="form-actions">
        <button type="button" class="btn primary" @click="save">
          {{ editingId ? "保存修改" : "添加闹钟" }}
        </button>
        <button v-if="editingId" type="button" class="btn" @click="resetForm">取消编辑</button>
      </div>
      <p v-if="error" class="err">{{ error }}</p>
    </section>

    <section class="list">
      <article v-for="alarm in sortedAlarms" :key="alarm.id" class="card" :class="{ off: !alarm.enabled }">
        <div class="card-main">
          <div class="time">{{ alarm.time }}</div>
          <div class="info">
            <div class="title">{{ alarm.title }}</div>
            <div class="meta">
              {{ formatDays(alarm) }}
              <template v-if="alarm.note"> · {{ alarm.note }}</template>
            </div>
            <div class="next">{{ nextFireHint(alarm) }}</div>
          </div>
        </div>
        <div class="card-actions">
          <button
            type="button"
            class="btn sm"
            @click="toggleAlarm(alarm.id)"
          >
            {{ alarm.enabled ? "关闭" : "启用" }}
          </button>
          <button type="button" class="btn sm" @click="editAlarm(alarm)">编辑</button>
          <button type="button" class="btn sm danger" @click="remove(alarm.id)">删除</button>
        </div>
      </article>

      <div v-if="!sortedAlarms.length" class="empty">还没有闹钟，先在上方添加一个吧</div>
    </section>

    <p v-if="toast" class="toast">{{ toast }}</p>
  </div>
</template>

<style scoped>
.tool {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.summary {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  align-items: baseline;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.tip {
  font-size: 0.78rem;
  color: var(--text-faint);
}

.editor,
.card {
  padding: 14px;
  border-radius: 14px;
  border: 1px solid var(--border-soft);
  background: var(--bg);
}

.editor h3 {
  margin: 0 0 12px;
  font-size: 0.95rem;
}

.grid {
  display: grid;
  grid-template-columns: 1fr 140px;
  gap: 10px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 12px;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.field input[type="text"],
.field input[type="time"] {
  padding: 9px 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface-strong);
  color: var(--text);
  font-family: inherit;
  font-size: 0.9rem;
}

.field input:focus {
  outline: none;
  border-color: color-mix(in srgb, var(--accent) 45%, transparent);
}

.presets,
.days {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.days {
  margin-top: 8px;
}

.chip,
.day {
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text-muted);
  border-radius: 999px;
  padding: 6px 12px;
  font-size: 0.8rem;
  font-family: inherit;
  cursor: pointer;
}

.day {
  width: 36px;
  height: 36px;
  padding: 0;
  border-radius: 50%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.chip.active,
.day.active {
  background: var(--accent-soft);
  border-color: color-mix(in srgb, var(--accent) 35%, transparent);
  color: var(--accent);
  font-weight: 600;
}

.day:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.check {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  font-size: 0.85rem;
  color: var(--text);
}

.form-actions,
.card-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.btn {
  padding: 8px 14px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface-strong);
  color: var(--text);
  font-size: 0.85rem;
  font-family: inherit;
  cursor: pointer;
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
  font-weight: 600;
}

.btn.sm {
  padding: 5px 10px;
  font-size: 0.75rem;
}

.btn.danger:hover {
  border-color: #e5484d;
  color: #e5484d;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.card {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
}

.card.off {
  opacity: 0.55;
}

.card-main {
  display: flex;
  gap: 14px;
  align-items: center;
  min-width: 0;
}

.time {
  font-size: 1.6rem;
  font-weight: 700;
  font-family: var(--font-mono);
  letter-spacing: 0.02em;
  color: var(--text);
}

.info {
  min-width: 0;
}

.title {
  font-weight: 600;
  color: var(--text);
}

.meta,
.next {
  font-size: 0.78rem;
  color: var(--text-faint);
  margin-top: 2px;
}

.next {
  color: var(--accent);
}

.empty {
  padding: 28px;
  text-align: center;
  color: var(--text-faint);
  font-size: 0.9rem;
}

.err {
  margin: 10px 0 0;
  color: #b45309;
  font-size: 0.82rem;
}

.toast {
  margin: 0;
  padding: 10px 12px;
  border-radius: 8px;
  background: var(--accent-soft);
  color: var(--accent);
  font-size: 0.85rem;
}

@media (max-width: 640px) {
  .grid {
    grid-template-columns: 1fr;
  }

  .card {
    flex-direction: column;
    align-items: flex-start;
  }
}
</style>
