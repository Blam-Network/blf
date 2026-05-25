<script setup lang="ts">
import { computed } from "vue";
import data from "../version-exports.json";

const props = defineProps<{
  /** `src/versions/` folder name, e.g. `haloreach` */
  game?: string;
  /** Barrel filename without `.ts` */
  buildId?: string;
}>();

const bundles = computed(() =>
  data.bundles.filter((bundle) => {
    if (props.game && bundle.game !== props.game) {
      return false;
    }
    if (props.buildId && bundle.buildId !== props.buildId) {
      return false;
    }
    return true;
  })
);
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
      >
        <h3 v-if="!buildId">{{ bundle.buildLabel }}</h3>
        <p>
          <code>{{ bundle.importPath }}</code>
        </p>
        <ul v-if="bundle.chunks.length">
          <li v-for="chunk in bundle.chunks" :key="chunk">
            <code>{{ chunk }}</code>
          </li>
        </ul>
        <p v-else class="empty">No chunk classes exported from this bundle.</p>
      </section>
    </template>
    <p v-else class="empty">No bundles matched.</p>
  </div>
</template>

<style scoped>
.autogen-note {
  font-size: 0.9em;
  opacity: 0.85;
}

section {
  margin-top: 1rem;
}

section h3 {
  margin-bottom: 0.25rem;
}

ul {
  margin: 0.5rem 0 0;
  padding-left: 1.25rem;
}

.empty {
  font-size: 0.9em;
  opacity: 0.75;
}
</style>
