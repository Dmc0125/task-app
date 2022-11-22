import { createRouter, createWebHistory } from 'vue-router'

import { useUserStore } from '@/stores/user'

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: '/sign-in',
			component: async () => import('@/views/SignInView.vue'),
		},
		{
			path: '/dashboard',
			component: async () => import('@/views/DashboardView.vue'),
		},
	],
})

router.beforeEach(async (to, from) => {
	if (to.path === '/') {
		return {
			path: '/sign-in',
		}
	}
	if (to.path.startsWith('/sign-in') && !from.path.startsWith('/dashboard')) {
		const store = useUserStore()

		if (!store.isSignedIn) {
			await store.fetchUser()

			if (store.isSignedIn) {
				return { path: '/dashboard' }
			}
		}
	}

	if (to.path.startsWith('/dashboard')) {
		const store = useUserStore()
		if (!store.isSignedIn) {
			await store.fetchUser()
			if (!store.isSignedIn) {
				return {
					path: '/sign-in',
				}
			}
		}
	}
})

export default router
