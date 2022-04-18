<template>
  <router-view/>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "main-app",
});
</script>

<style>
.panel-heading {
  line-height: 0.75 !important;
}
</style>