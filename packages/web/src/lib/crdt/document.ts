import * as Y from 'yjs';
import { IndexeddbPersistence } from 'y-indexeddb';

export interface CRDTDocumentOptions {
	pageId: string;
	onSync?: () => void;
	onUpdate?: (update: Uint8Array) => void;
}

export class CRDTDocument {
	readonly doc: Y.Doc;
	readonly pageId: string;
	private persistence: IndexeddbPersistence | null = null;
	private onUpdate?: (update: Uint8Array) => void;

	constructor(options: CRDTDocumentOptions) {
		this.pageId = options.pageId;
		this.doc = new Y.Doc();
		this.onUpdate = options.onUpdate;

		// Set up update listener
		this.doc.on('update', (update: Uint8Array, origin: any) => {
			// Don't broadcast updates from IndexedDB sync
			if (origin !== 'indexeddb') {
				this.onUpdate?.(update);
			}
		});

		// Initialize IndexedDB persistence
		this.initPersistence(options.onSync);
	}

	private initPersistence(onSync?: () => void) {
		const dbName = `notion-killer-${this.pageId}`;
		this.persistence = new IndexeddbPersistence(dbName, this.doc);

		this.persistence.on('synced', () => {
			console.log(`[CRDT] Document ${this.pageId} synced from IndexedDB`);
			onSync?.();
		});
	}

	/**
	 * Get the Y.XmlFragment for editor content
	 */
	getContent(): Y.XmlFragment {
		return this.doc.getXmlFragment('content');
	}

	/**
	 * Get the Y.Text for page title
	 */
	getTitle(): Y.Text {
		return this.doc.getText('title');
	}

	/**
	 * Get the Y.Map for page metadata
	 */
	getMetadata(): Y.Map<any> {
		return this.doc.getMap('metadata');
	}

	/**
	 * Apply an update from remote
	 */
	applyUpdate(update: Uint8Array, origin: string = 'remote') {
		Y.applyUpdate(this.doc, update, origin);
	}

	/**
	 * Get the current state as update
	 */
	getState(): Uint8Array {
		return Y.encodeStateAsUpdate(this.doc);
	}

	/**
	 * Get state vector for sync
	 */
	getStateVector(): Uint8Array {
		return Y.encodeStateVector(this.doc);
	}

	/**
	 * Get diff from a state vector
	 */
	getDiff(stateVector: Uint8Array): Uint8Array {
		return Y.encodeStateAsUpdate(this.doc, stateVector);
	}

	/**
	 * Destroy the document and clean up
	 */
	destroy() {
		this.persistence?.destroy();
		this.doc.destroy();
	}
}

/**
 * Document store for managing multiple CRDT documents
 */
class DocumentStore {
	private documents = new Map<string, CRDTDocument>();

	/**
	 * Get or create a document for a page
	 */
	getDocument(pageId: string, options?: Partial<CRDTDocumentOptions>): CRDTDocument {
		let doc = this.documents.get(pageId);

		if (!doc) {
			doc = new CRDTDocument({
				pageId,
				...options
			});
			this.documents.set(pageId, doc);
		}

		return doc;
	}

	/**
	 * Check if a document exists
	 */
	hasDocument(pageId: string): boolean {
		return this.documents.has(pageId);
	}

	/**
	 * Close and remove a document
	 */
	closeDocument(pageId: string) {
		const doc = this.documents.get(pageId);
		if (doc) {
			doc.destroy();
			this.documents.delete(pageId);
		}
	}

	/**
	 * Close all documents
	 */
	closeAll() {
		for (const doc of this.documents.values()) {
			doc.destroy();
		}
		this.documents.clear();
	}
}

export const documentStore = new DocumentStore();
