import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { setupTools } from "./tools";
import "./styles/main.css";

setupTools();

createApp(App).use(router).mount("#app");
