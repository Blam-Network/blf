import path from "node:path";
import { fileURLToPath } from "node:url";
import swc from "unplugin-swc";
import { defineConfig } from "vitest/config";

const root = path.dirname(fileURLToPath(import.meta.url));

const swcDecorators = {
  legacyDecorator: false,
  decoratorVersion: "2022-03" as const,
  decoratorMetadata: false,
  useDefineForClassFields: true,
};

const versionsRoot = path.join(root, "src/versions").replace(/\\/g, "/");

export default defineConfig({
  resolve: {
    alias: [
      {
        // Exact package root only (do not prefix-match subpaths).
        find: /^@blamnetwork\/blf$/,
        replacement: path.join(root, "src/index.ts"),
      },
      {
        // @blamnetwork/blf/haloreach/<build> → src/versions/haloreach/<build>.ts
        find: /^@blamnetwork\/blf\/(haloreach|halo3|halo3odst|haloreach_mcc)\/(.+)$/,
        replacement: `${versionsRoot}/$1/$2.ts`,
      },
    ],
  },
  plugins: [
    swc.vite({
      jsc: {
        parser: { syntax: "typescript", decorators: true },
        transform: swcDecorators,
        target: "es2022",
      },
    }),
  ],
  test: {
    include: ["src/**/*.test.ts", "tests/**/*.test.ts"],
    server: {
      deps: {
        inline: ["@craftycodie/cstruct"],
      },
    },
    deps: {
      optimizer: {
        swc: {
          jsc: {
            parser: { syntax: "typescript", decorators: true },
            transform: swcDecorators,
            target: "es2022",
          },
        },
      },
    },
  },
});
