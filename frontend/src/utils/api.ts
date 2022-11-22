export const API_URL = (import.meta.env.API_URL as string) || 'http://localhost:8000'

export const mergeUrl = (base: string, path: string) => new URL(path, base).toString()

type Config<B = Record<string, unknown>> = {
	method?: 'GET' | 'POST' | 'PATCH' | 'DELETE'
	path: string
	queryParams?: URLSearchParams
	body?: B
}

type SuccessResponse<D> = {
	success: true
	data: D
}

type ErrorResponse = {
	success: false
	error: string
}

type FetchResponse<D> = SuccessResponse<D> | ErrorResponse

export const apiFetch = async <R, B = Record<string, unknown>>({
	method = 'GET',
	path,
	queryParams,
	body,
}: Config<B>): Promise<FetchResponse<R>> => {
	try {
		const headers =
			method !== 'GET'
				? {
						'content-type': 'application/json',
				  }
				: undefined
		const res = (await (
			await fetch(
				mergeUrl(API_URL, `/api/v1${path}${queryParams ? `?${queryParams.toString()}` : ''}`),
				{
					method,
					headers,
					body: method !== 'GET' ? JSON.stringify(body) : undefined,
					credentials: 'include',
				},
			)
		).json()) as FetchResponse<R>
		return res
	} catch (error) {
		console.log(error)
		return {
			success: false,
			error: 'Unknown error, please check your connection and try again later',
		}
	}
}
