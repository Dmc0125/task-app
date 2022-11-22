<template>
	<button>
		<slot />
	</button>
</template>

<style lang="scss" scoped>
button {
	padding: 0.5rem 1rem;

	background: none;
	border: 0;
	border-radius: var(--border-radius);
	font-size: 1rem;
	font-weight: 400;
	cursor: pointer;
	color: var(--font-clr-1);
	transition: all 200ms ease-in-out;

	&:hover,
	&:focus {
		background: var(--bg-clr-2);
	}
}
</style>
