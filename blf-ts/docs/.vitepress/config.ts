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
  appearance: "force-dark",
  head: [
    ["link", { rel: "preconnect", href: "https://fonts.googleapis.com" }],
    [
      "link",
      { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" },
    ],
    [
      "link",
      {
        rel: "stylesheet",
        href: "https://fonts.googleapis.com/css2?family=Overpass:wght@400;600;700&display=swap",
      },
    ],
  ],
  markdown: {
    theme: {
      light: "github-light",
      dark: "github-dark",
    },
  },
  themeConfig: {
    siteTitle:
      '<span class="blf-site-title"><span class="blf-scope">@blamnetwork/</span><span class="blf-name">blf</span></span>',
    nav: [
      { text: "Blam Network", link: "https://blam.network" },
      { text: "Guide", link: "/guide/quick-start" },
      { text: "Changelog", link: "/changelog" },
      {
        text: "npm",
        link: "https://www.npmjs.com/package/@blamnetwork/blf",
      },
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
          { text: "Reading chunks", link: "/guide/reading" },
          { text: "Writing chunks", link: "/guide/writing" },
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
      {
        text: "Reference",
        items: [{ text: "Changelog", link: "/changelog" }],
      },
    ],
    socialLinks: [
      {
        icon: "npm",
        link: "https://www.npmjs.com/package/@blamnetwork/blf",
        ariaLabel: "npm",
      },
      {
        icon: "github",
        link: "https://github.com/Blam-Network/blf",
      },
      {
        icon: "discord",
        link: "https://discord.gg/77ZAgXv8a6",
        ariaLabel: "Discord",
      },
    ],
    footer: {
      message: "MIT Licensed",
      copyright:
        'Copyright © <a href="https://discord.gg/77ZAgXv8a6" target="_blank" rel="noopener noreferrer">Blam Network</a>',
    },
  },
});
