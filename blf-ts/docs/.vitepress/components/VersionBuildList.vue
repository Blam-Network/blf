<script setup lang="ts">
import { computed } from "vue";
import data from "../version-exports.json";

const props = defineProps<{
  game: string;
}>();

const builds = computed(() =>
  data.bundles.filter((bundle) => bundle.game === props.game)
);
</script>

<template>
  <ul v-if="builds.length" class="version-build-list">
    <li v-for="bundle in builds" :key="bundle.buildId">
      <a :href="bundle.docLink">{{ bundle.buildLabel }}</a>
      —
      <code>{{ bundle.buildId }}</code>
    </li>
  </ul>
  <p v-else class="empty">No builds documented for this game.</p>
</template>

<style scoped>
.version-build-list {
  padding-left: 1.25rem;
}

.empty {
  font-size: 0.9em;
  opacity: 0.75;
}
</style>
