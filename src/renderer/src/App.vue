<script setup lang="ts">
import Versions from "./components/Versions.vue";

const ipcHandle = (): void => window.electron.ipcRenderer.send("ping");
</script>

<template>
	<img alt="logo" class="logo" src="./assets/electron.svg" />
	<div class="creator">Powered by electron-vite</div>
	<div class="text">
		Build an Electron app with
		<span class="vue">Vue</span>
		and
		<span class="ts">TypeScript</span>
	</div>
	<p class="tip">
		Please try pressing
		<code>F12</code>
		to open the devTool
	</p>
	<div class="actions">
		<div class="action">
			<a href="https://electron-vite.org/" rel="noreferrer" target="_blank">
				Documentation
			</a>
		</div>
		<div class="action">
			<a rel="noreferrer" target="_blank" @click="ipcHandle">Send IPC</a>
		</div>
	</div>
	<Versions />
</template>
