import { documentStore, CRDTDocument } from './document';

interface CRDTState {
	currentPageId: string | null;
	document: CRDTDocument | null;
	isSynced: boolean;
	isSaving: boolean;
	lastSaved: Date | null;
}

function createCRDTStore() {
	let currentPageId = $state<string | null>(null);
	let document = $state<CRDTDocument | null>(null);
	let isSynced = $state(false);
	let isSaving = $state(false);
	let lastSaved = $state<Date | null>(null);

	/**
	 * Open a document for editing
	 */
	function openDocument(pageId: string): CRDTDocument {
		// Close previous document if different
		if (currentPageId && currentPageId !== pageId) {
			closeDocument();
		}

		currentPageId = pageId;
		isSynced = false;

		document = documentStore.getDocument(pageId, {
			onSync: () => {
				isSynced = true;
			},
			onUpdate: (update) => {
				// Mark as needing save
				isSaving = true;
				// Auto-save is handled by IndexedDB persistence
				// This callback could be used for WebSocket sync
				setTimeout(() => {
					isSaving = false;
					lastSaved = new Date();
				}, 100);
			}
		});

		return document;
	}

	/**
	 * Close current document
	 */
	function closeDocument() {
		if (currentPageId) {
			documentStore.closeDocument(currentPageId);
			currentPageId = null;
			document = null;
			isSynced = false;
		}
	}

	/**
	 * Apply remote update to current document
	 */
	function applyRemoteUpdate(update: Uint8Array) {
		document?.applyUpdate(update, 'remote');
	}

	/**
	 * Get current document state
	 */
	function getState(): Uint8Array | null {
		return document?.getState() ?? null;
	}

	return {
		get currentPageId() {
			return currentPageId;
		},
		get document() {
			return document;
		},
		get isSynced() {
			return isSynced;
		},
		get isSaving() {
			return isSaving;
		},
		get lastSaved() {
			return lastSaved;
		},
		openDocument,
		closeDocument,
		applyRemoteUpdate,
		getState
	};
}

export const crdtStore = createCRDTStore();
