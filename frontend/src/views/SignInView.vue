<script lang="ts" setup>
import { API_URL, mergeUrl } from '@/utils/api'

const providers = ['Discord', 'Google']
</script>

<template>
	<div class="sign-in-wrapper">
		<h3>Sing in with</h3>

		<div class="buttons">
			<a
				v-for="provider in providers"
				:key="provider"
				:href="mergeUrl(API_URL, '/api/v1/auth/signin/' + provider.toLowerCase())"
				:class="provider.toLowerCase()"
			>
				<img :src="`/icons/${provider.toLowerCase()}.svg`" :alt="`${provider} logo`" />
				{{ provider }}
			</a>
		</div>
	</div>
</template>

<style lang="scss" scoped>
.sign-in-wrapper {
	width: 100%;
	padding-inline: 5%;
	margin: 2rem auto 0;

	h3 {
		color: var(--font-clr-2);
		font-weight: 500;
		font-size: 1rem;
		text-align: center;
	}
}

.buttons {
	margin-top: 1.5rem;
	display: flex;
	align-items: center;
	justify-content: center;
	gap: 2.5rem;

	a {
		max-width: 10rem;
		width: 100%;
		height: 2.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;

		border-radius: 0.25rem;
		text-decoration: none;
		font-size: 1rem;
		font-weight: 500;

		transition: all 200ms ease-in-out;
	}
}

.discord {
	background: hsl(235, 86%, 65%);
	color: var(--font-clr-1);

	&:hover,
	&:focus {
		background: hsl(235, 86%, 62%);
	}
}

.google {
	background: hsl(0, 0%, 100%);
	color: #000;

	&:hover,
	&:focus {
		background: hsl(0, 0%, 85%);
	}
}
</style>
