import type { ToolModule } from "@/types/tool";
import VideoToMp3 from "./VideoToMp3.vue";

const tool: ToolModule = {
  id: "video-to-mp3",
  name: "视频转 MP3",
  description: "提取音频为 MP3，支持按起止时间剪切",
  category: "system",
  icon: "♪",
  keywords: ["video", "mp3", "audio", "ffmpeg", "视频", "音频", "转码", "剪切"],
  order: 50,
  component: VideoToMp3,
};

export default tool;
