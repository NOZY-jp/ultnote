// Memo types
export type MemoType = 'flash' | 'permanent';

export interface Memo {
	id: string;
	content: string;
	type: MemoType;
	from?: string; // ISO date string (YYYY-MM-DD)
	until?: string; // ISO date string (YYYY-MM-DD)
	tags: string[];
	date_added: string; // ISO datetime string
	access_count: number;
	last_accessed: string; // ISO datetime string
	completed: boolean;
}

export interface CreateMemoRequest {
	content: string;
	type: MemoType;
	from?: string;
	until?: string;
	tags?: string[];
}

export interface UpdateMemoRequest {
	content?: string;
	type?: MemoType;
	from?: string;
	until?: string;
	tags?: string[];
	completed?: boolean;
}

// Search types
export interface SearchFilters {
	from_gte?: string; // from >= this date
	until_lte?: string; // until <= this date
	tags?: string[];
	type?: MemoType;
}

export interface SearchRequest {
	query: string;
	filters?: SearchFilters;
	limit?: number;
}

export interface SearchResult {
	id: string;
	content: string;
	score: number;
	tags: string[];
	from?: string;
	until?: string;
	date_added: string;
	type: MemoType;
	completed: boolean;
}

export interface SearchResponse {
	results: SearchResult[];
	total: number;
}

// Tag types
export interface TagNode {
	name: string;
	path: string;
	children: TagNode[];
}

export interface TagsResponse {
	tags: TagNode[];
}

// Auth state
export interface AuthState {
	isAuthenticated: boolean;
	email?: string;
}

// API error
export interface ApiError {
	message: string;
	status: number;
}

// Date filter presets
export type DatePreset = 'today' | 'tomorrow' | 'this_week' | 'next_week' | 'this_month';

export function getDateRangeForPreset(preset: DatePreset): { from: string; until: string } {
	const today = new Date();
	const yyyy = (d: Date) => d.toISOString().split('T')[0];

	switch (preset) {
		case 'today': {
			const date = yyyy(today);
			return { from: date, until: date };
		}
		case 'tomorrow': {
			const tomorrow = new Date(today);
			tomorrow.setDate(tomorrow.getDate() + 1);
			const date = yyyy(tomorrow);
			return { from: date, until: date };
		}
		case 'this_week': {
			const startOfWeek = new Date(today);
			const day = today.getDay();
			const diff = day === 0 ? -6 : 1 - day; // Monday as start of week
			startOfWeek.setDate(today.getDate() + diff);
			const endOfWeek = new Date(startOfWeek);
			endOfWeek.setDate(startOfWeek.getDate() + 6);
			return { from: yyyy(startOfWeek), until: yyyy(endOfWeek) };
		}
		case 'next_week': {
			const startOfNextWeek = new Date(today);
			const day = today.getDay();
			const diff = day === 0 ? 1 : 8 - day;
			startOfNextWeek.setDate(today.getDate() + diff);
			const endOfNextWeek = new Date(startOfNextWeek);
			endOfNextWeek.setDate(startOfNextWeek.getDate() + 6);
			return { from: yyyy(startOfNextWeek), until: yyyy(endOfNextWeek) };
		}
		case 'this_month': {
			const startOfMonth = new Date(today.getFullYear(), today.getMonth(), 1);
			const endOfMonth = new Date(today.getFullYear(), today.getMonth() + 1, 0);
			return { from: yyyy(startOfMonth), until: yyyy(endOfMonth) };
		}
	}
}
