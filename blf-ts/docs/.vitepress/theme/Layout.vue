<script setup lang="ts">
import DefaultTheme from "vitepress/theme";

const { Layout: DefaultLayout } = DefaultTheme;
</script>

<template>
  <div class="blf-layout">
    <div class="blf-hero-bg" aria-hidden="true">
      <div
        class="blf-hero-bg-image"
      />
      <div class="blf-hero-bg-overlay" />
    </div>
    <DefaultLayout />
  </div>
</template>

<style scoped>
.blf-layout {
  position: relative;
  min-height: 100vh;
}

.blf-hero-bg {
  position: fixed;
  inset: 0;
  z-index: 0;
  overflow: hidden;
  pointer-events: none;
}

.blf-hero-bg-image {
  position: absolute;
  inset: -24px;
  background-position: center 35%;
  background-size: cover;
  background-repeat: no-repeat;
  opacity: 0.2;
  filter: blur(4px);
  transform: scale(1.05);
}

.blf-hero-bg-overlay {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    180deg,
    rgba(8, 10, 8, 0.72) 0%,
    rgba(8, 10, 8, 0.86) 100%
  );
}

.blf-layout :deep(.Layout) {
  position: relative;
  z-index: 1;
}
</style>
