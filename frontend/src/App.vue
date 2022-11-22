<script setup lang="ts">
import { RouterView } from 'vue-router'

import GlobalLayout from './layouts/GlobalLayout.vue'
</script>

<template>
	<GlobalLayout>
		<RouterView />
	</GlobalLayout>
</template>

<style>
@font-face {
	font-family: 'WorkSans';
	font-weight: 100 1000;
	src: url('/WorkSans-VariableFont_wght.ttf');
}

* {
	margin: 0;
	padding: 0;
	box-sizing: border-box;
	font-family: 'WorkSans', sans-serif;
}

html,
body {
	width: 100%;
	min-height: 100vh;
}

:root {
	--bg-clr-1: #252e3d;
	--bg-clr-2: hsl(218, 24%, 15%);
	--bg-clr-3: hsl(218, 26%, 25%);

	--font-clr-1: rgb(233, 233, 233);
	--font-clr-2: #afafaf;
	--font-clr-3: #5b6d79;

	--border-radius: 0.25rem;

	background: var(--bg-clr-1);
}
</style>
