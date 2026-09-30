import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { setupTools } from "./tools";
import { startAlarmScheduler } from "./core/alarmScheduler";
import "./styles/main.css";

setupTools();
startAlarmScheduler();

createApp(App).use(router).mount("#app");
