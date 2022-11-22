import { defineStore } from 'pinia'
import { ref } from 'vue'

import { apiFetch } from '@/utils/api'

type UserResponse = {
	provider_type: string
	username: string
	avatar: string | null
}

export const useUserStore = defineStore('user', () => {
	const isSignedIn = ref(false)
	const username = ref<String | null>(null)
	const avatar = ref<String | null>(null)

	async function fetchUser() {
		const user = await apiFetch<UserResponse>({
			path: '/user',
		})
		if (user.success) {
			const { username: _username, avatar: _avatar } = user.data
			username.value = _username
			avatar.value = _avatar
			isSignedIn.value = true
		}
	}

	return {
		isSignedIn,
		username,
		avatar,
		fetchUser,
	}
})
