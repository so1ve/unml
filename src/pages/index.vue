<script setup lang="ts">
import { pathToFileURL } from "node:url";

const { callNodeCommand } = useClient();

console.log(
  await fetch(
    pathToFileURL(await callNodeCommand("ui:getViews").then((r) => r[0].path)),
  ).then((r) => r.text()),
);
</script>

<template>
  <div>11</div>
</template>
