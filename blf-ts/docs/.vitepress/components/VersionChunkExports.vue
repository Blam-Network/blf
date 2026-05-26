<script setup lang="ts">
import { computed } from "vue";
import data from "../version-exports.json";

type ChunkExport = {
  name: string;
  source: string;
  signature: string;
  version: string;
};

type Bundle = {
  game: string;
  buildId: string;
  label: string;
  buildLabel: string;
  importPath: string;
  docLink: string;
  chunks: ChunkExport[];
};

const props = defineProps<{
  /** `src/versions/` folder name, e.g. `haloreach` */
  game?: string;
  /** Barrel filename without `.ts` */
  buildId?: string;
}>();

const sourceRepoBase =
  (data as { sourceRepoBase?: string }).sourceRepoBase ??
  "https://github.com/Blam-Network/blf/blob/main/blf-ts";

const bundles = computed(() =>
  (data.bundles as Bundle[]).filter((bundle) => {
    if (props.game && bundle.game !== props.game) {
      return false;
    }
    if (props.buildId && bundle.buildId !== props.buildId) {
      return false;
    }
    return true;
  })
);

function sourceHref(source: string): string {
  return `${sourceRepoBase}/${source}`;
}

function chunkLabel(chunk: ChunkExport): string {
  if (chunk.signature === "—" || chunk.version === "—") {
    return "—";
  }
  return `${chunk.signature} ${chunk.version}`;
}
</script>

<template>
  <div class="version-chunk-exports">
    <p v-if="!game && !buildId" class="autogen-note">
      Chunk classes below are generated from
      <code>src/versions/&lt;game&gt;/&lt;build&gt;.ts</code>
      (run <code>npm run docs:gen</code> after changing a barrel).
    </p>

    <template v-if="bundles.length">
      <section
        v-for="bundle in bundles"
        :key="`${bundle.game}/${bundle.buildId}`"
        class="bundle"
      >
        <header v-if="!buildId" class="bundle-header">
          <h3>{{ bundle.buildLabel }}</h3>
          <p class="import-path">
            <span class="import-label">npm</span>
            <code>{{ bundle.importPath }}</code>
          </p>
        </header>

        <table v-if="bundle.chunks.length" class="chunk-table">
          <colgroup>
            <col class="col-chunk" />
            <col class="col-class" />
            <col class="col-link" />
          </colgroup>
          <thead>
            <tr>
              <th scope="col">Chunk</th>
              <th scope="col">Class</th>
              <th scope="col" class="col-link-header"><span class="sr-only">Source</span></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="chunk in bundle.chunks" :key="chunk.name">
              <td>
                <code class="chunk-sig">{{ chunkLabel(chunk) }}</code>
              </td>
              <td>
                <code class="chunk-name">{{ chunk.name }}</code>
              </td>
              <td class="col-link-header">
                <a
                  class="chunk-source-link"
                  :href="sourceHref(chunk.source)"
                  target="_blank"
                  rel="noopener noreferrer"
                  :aria-label="`View ${chunk.name} on GitHub`"
                >
                  <svg
                    class="chunk-source-icon"
                    xmlns="http://www.w3.org/2000/svg"
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"
                    />
                  </svg>
                  <span>GitHub</span>
                </a>
              </td>
            </tr>
          </tbody>
        </table>

        <p v-else class="empty">No chunk classes exported from this bundle.</p>
      </section>
    </template>

    <p v-else class="empty">No bundles matched.</p>
  </div>
</template>

<style scoped>
.version-chunk-exports {
  width: 100%;
  margin: 1.25rem 0 1.75rem;
}

.autogen-note {
  font-size: 0.9em;
  color: var(--vp-c-text-2);
  margin-bottom: 1rem;
}

.bundle + .bundle {
  margin-top: 2rem;
}

.bundle-header h3 {
  margin: 0 0 0.35rem;
  font-size: 1.1rem;
}

.import-path {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0 0 1rem;
  font-size: 0.9em;
}

.import-label {
  flex-shrink: 0;
  padding: 0.1rem 0.45rem;
  border-radius: 4px;
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--vp-c-brand-1);
  background: var(--vp-c-brand-soft);
  border: 1px solid rgba(52, 211, 153, 0.22);
}

.import-path code {
  font-size: 0.88em;
}

.col-chunk {
  width: 9rem;
}

.col-class {
  width: auto;
}

.col-link {
  width: 7.5rem;
}

.chunk-sig {
  font-size: 0.95em;
  font-weight: 600;
  color: var(--vp-c-text-1);
  background: transparent;
  border: none;
  padding: 0;
  white-space: nowrap;
}

.chunk-name {
  font-size: 0.92em;
  font-weight: 500;
  color: var(--vp-c-brand-1);
  background: transparent;
  border: none;
  padding: 0;
}

.col-link-header {
  text-align: right;
  white-space: nowrap;
}

.chunk-source-link {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.32rem 0.7rem;
  border-radius: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  line-height: 1;
  color: var(--vp-c-text-1);
  text-decoration: none;
  background: var(--vp-c-bg-elv);
  border: 1px solid var(--vp-c-border);
  transition:
    color 0.2s,
    border-color 0.2s,
    background 0.2s;
}

.chunk-source-link:hover {
  color: var(--vp-c-brand-1);
  border-color: rgba(52, 211, 153, 0.45);
  background: var(--vp-c-brand-soft);
}

.chunk-source-icon {
  opacity: 0.85;
}

.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.empty {
  font-size: 0.9em;
  color: var(--vp-c-text-2);
}
</style>
