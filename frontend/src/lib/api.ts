import type {
	Memo,
	CreateMemoRequest,
	UpdateMemoRequest,
	SearchRequest,
	SearchResponse,
	TagsResponse,
	ApiError
} from './types';

const API_BASE = '/api';

class ApiClient {
	private async request<T>(
		path: string,
		options: RequestInit = {}
	): Promise<T> {
		const url = `${API_BASE}${path}`;
		const response = await fetch(url, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			}
		});

		if (!response.ok) {
			const error: ApiError = {
				message: `API error: ${response.statusText}`,
				status: response.status
			};
			try {
				const body = await response.json();
				if (body.message) {
					error.message = body.message;
				}
			} catch {
				// Ignore JSON parse error
			}
			throw error;
		}

		return response.json();
	}

	// Memo operations
	async createMemo(data: CreateMemoRequest): Promise<Memo> {
		return this.request<Memo>('/memo', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}

	async getMemo(id: string): Promise<Memo> {
		return this.request<Memo>(`/memo/${id}`);
	}

	async updateMemo(id: string, data: UpdateMemoRequest): Promise<Memo> {
		return this.request<Memo>(`/memo/${id}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	}

	async deleteMemo(id: string): Promise<void> {
		await this.request<void>(`/memo/${id}`, {
			method: 'DELETE'
		});
	}

	// Search operations
	async search(data: SearchRequest): Promise<SearchResponse> {
		return this.request<SearchResponse>('/search', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}

	async demoSearch(data: SearchRequest): Promise<SearchResponse> {
		return this.request<SearchResponse>('/demo/search', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}

	// Tag operations
	async getTags(): Promise<TagsResponse> {
		return this.request<TagsResponse>('/tags');
	}

	// Health check
	async health(): Promise<{ status: string; version: string }> {
		return this.request<{ status: string; version: string }>('/health');
	}
}

export const api = new ApiClient();
