<script setup lang="ts">
import { usePlugin } from "@unml/ui-kit";
import { ref } from "vue";

const output = ref("");
const time = ref("");
const { call, on } = usePlugin();

async function callBackend() {
  try {
    output.value = "Calling...";
    const result = await call("hello", { time: Date.now() });
    output.value = JSON.stringify(result, null, 2);
  } catch (e: any) {
    output.value = `Error: ${e.message}`;
  }
}

on("time-update", (data: { time: string }) => {
  time.value = data.time;
});
</script>

<template>
  <div>
    <h1>Hello from Built-in Plugin (Vue)!</h1>
    <button @click="callBackend">Call Backend</button>
    <pre>{{ output }}</pre>
    <pre>{{ time }}</pre>
  </div>
</template>

<style>
body {
  font-family: sans-serif;
  padding: 20px;
}
</style>
