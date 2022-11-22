<script lang="ts" setup>
import { onMounted, onUnmounted, ref, watchEffect } from 'vue'

import Button from '@/components/Button.vue'
import { useUserStore } from '@/stores/user'
import { API_URL, mergeUrl } from '@/utils/api'

const userStore = useUserStore()

const showDropdownMenu = ref(false)

const hideDropdownMenu = (e: KeyboardEvent) => {
	if (e.key === 'Escape') {
		showDropdownMenu.value = false
	}
}

watchEffect(() => {
	if (showDropdownMenu.value) {
		document.addEventListener('keydown', hideDropdownMenu)
	} else {
		document.removeEventListener('keydown', hideDropdownMenu)
	}
})

onUnmounted(() => {
	document.removeEventListener('keydown', hideDropdownMenu)
})

const handleClick = (e: Event & { relatedTarget?: HTMLButtonElement }) => {
	if ((e.target as HTMLButtonElement).hasAttribute('data-no-blur')) {
		e.preventDefault()
		return
	}
	showDropdownMenu.value = !showDropdownMenu.value
}

const handleBlur = (e: FocusEvent & { relatedTarget?: HTMLButtonElement }) => {
	if (e.relatedTarget) {
		return
	}
	showDropdownMenu.value = false
}
</script>

<template>
	<header>
		<h1>Task app</h1>

		<div v-if="userStore.isSignedIn">
			<Button @click="handleClick" @blur="handleBlur" class="dropdown-btn">
				{{ userStore.username }}
				<ul class="dropdown-menu" v-if="showDropdownMenu">
					<li>
						<a :href="`${mergeUrl(API_URL, '/api/v1/auth/signout')}`">Sign out</a>
					</li>
					<!-- <li>
						<button data-no-blur>Settings</button>
					</li> -->
				</ul>
			</Button>
		</div>
	</header>

	<slot />
</template>

<style lang="scss" scoped>
header {
	width: 100%;
	height: 4rem;
	padding: 0 5%;

	display: flex;
	align-items: center;
	justify-content: space-between;
}

h1 {
	color: var(--font-clr-1);
	font-weight: 500;
	font-size: 1.75rem;
}

.dropdown-btn {
	position: relative;
}

.dropdown-menu {
	min-width: 6.5rem;
	padding-block: 0.5rem;
	position: absolute;
	top: 100%;
	left: 50%;
	transform: translate(-55%, 0.5rem);
	border-radius: var(--border-radius);

	background: var(--bg-clr-2);
	list-style: none;
	display: flex;
	flex-direction: column;
	gap: 0.25rem;

	li,
	a {
		display: block;
		text-decoration: none;
		color: var(--font-clr-2);
	}

	li {
		height: 2rem;
	}

	a {
		height: 100%;
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;

		&:hover,
		&:focus {
			background: var(--bg-clr-3);
		}
	}
}

@media (min-width: 500px) {
	header {
		height: 5rem;
		padding: 0 10%;

		h1 {
			font-weight: 600;
		}
	}

	.dropdown-menu {
		transform: translate(-50%, 0.5rem);
	}
}
</style>
