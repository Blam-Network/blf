import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import VersionBuildList from "../components/VersionBuildList.vue";
import VersionChunkExports from "../components/VersionChunkExports.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("VersionBuildList", VersionBuildList);
    app.component("VersionChunkExports", VersionChunkExports);
  },
} satisfies Theme;
