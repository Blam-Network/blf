import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import VersionBuildList from "../components/VersionBuildList.vue";
import VersionChunkExports from "../components/VersionChunkExports.vue";
import Layout from "./Layout.vue";
import "./style.css";

export default {
  extends: DefaultTheme,
  Layout,
  enhanceApp({ app }) {
    app.component("VersionBuildList", VersionBuildList);
    app.component("VersionChunkExports", VersionChunkExports);
  },
} satisfies Theme;
