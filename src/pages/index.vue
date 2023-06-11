<script setup lang="ts">
const { callNodeCommand } = useClient();

console.log(
  await fetch("unml-resource://c/Workspace/unml/package.json").then((r) =>
    r.text(),
  ),
);
</script>

<template>
  <div>11</div>
</template>
