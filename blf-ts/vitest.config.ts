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
        // @Blam-Network/blf/haloreach/<build> → src/versions/haloreach/<build>.ts
        find: /^@Blam-Network\/blf\/(haloreach|halo3|halo3odst)\/(.+)$/,
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
