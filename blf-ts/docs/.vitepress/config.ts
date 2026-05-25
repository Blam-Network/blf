import { defineConfig } from "vitepress";
import type { DefaultTheme } from "vitepress/theme";
import versionExports from "./version-exports.json";

function versionBundlesSidebar(): DefaultTheme.SidebarItem[] {
  const items: DefaultTheme.SidebarItem[] = [
    { text: "Overview", link: "/guide/versions/" },
  ];

  const byGame = new Map<string, typeof versionExports.bundles>();
  for (const bundle of versionExports.bundles) {
    const list = byGame.get(bundle.game) ?? [];
    list.push(bundle);
    byGame.set(bundle.game, list);
  }

  for (const game of [...byGame.keys()].sort()) {
    const gameBundles = byGame.get(game) ?? [];
    gameBundles.sort((a, b) => a.buildId.localeCompare(b.buildId));
    items.push({
      text: gameBundles[0]?.label ?? game,
      collapsed: true,
      items: [
        { text: "Overview", link: `/guide/versions/${game}/` },
        ...gameBundles.map((bundle) => ({
          text: bundle.buildLabel,
          link: bundle.docLink,
        })),
      ],
    });
  }

  return items;
}

export default defineConfig({
  title: "@blamnetwork/blf",
  description:
    "TypeScript library for reading and writing Halo BLF chunk files.",
  base: "/blf/",
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: "Guide", link: "/guide/quick-start" },
      {
        text: "GitHub",
        link: "https://github.com/Blam-Network/blf",
      },
    ],
    sidebar: [
      {
        text: "Introduction",
        items: [
          { text: "What is blf?", link: "/" },
          { text: "Install & quick start", link: "/guide/quick-start" },
        ],
      },
      {
        text: "Usage",
        items: [
          { text: "Reading chunks", link: "/guide/chunks" },
          { text: "Bitstream", link: "/guide/bitstream" },
        ],
      },
      {
        text: "Helpers",
        items: [
          {
            text: "Converting Reach Gametypes",
            link: "/guide/converting-reach-gametypes",
          },
        ],
      },
      {
        text: "Version bundles",
        items: versionBundlesSidebar(),
      },
      {
        text: "Contributing",
        items: [{ text: "Development", link: "/guide/development" }],
      },
    ],
    socialLinks: [
      {
        icon: "github",
        link: "https://github.com/Blam-Network/blf",
      },
    ],
  },
});
