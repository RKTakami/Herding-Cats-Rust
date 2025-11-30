const pendingRequests = new Map();

// Direct callback for Rust
window.__IPC_RECEIVE__ = (data) => {
    if (data && data.id && pendingRequests.has(data.id)) {
        const { resolve, reject } = pendingRequests.get(data.id);
        pendingRequests.delete(data.id);

        if (data.type === 'error') {
            reject(new Error(data.payload ? data.payload.message : 'Unknown error'));
        } else if (data.type === 'db_result') {
            resolve(data.payload); // Return the payload directly
        } else if (data.type === 'ai_response') {
            resolve(data.payload.text);
        } else {
            resolve(data.payload);
        }
    } else if (data && data.type === 'open_document') {
        // Handle unsolicited open_document event from backend
        window.dispatchEvent(new CustomEvent('open-document', { detail: data.payload.id }));
    }
};

function generateId() {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
}

function sendRequest(type, payload) {
    return new Promise((resolve, reject) => {
        const id = generateId();
        // The request structure matches IpcRequest in Rust
        // IpcRequest { id, message: IpcMessage }
        // IpcMessage has #[serde(tag = "type", content = "payload")]
        const request = {
            id,
            type,
            payload
        };

        pendingRequests.set(id, { resolve, reject });

        // Timeout to prevent memory leaks if backend never responds
        setTimeout(() => {
            if (pendingRequests.has(id)) {
                pendingRequests.delete(id);
                reject(new Error('IPC Request Timeout'));
            }
        }, 30000); // 30s timeout

        if (window.ipc) {
            window.ipc.postMessage(JSON.stringify(request));
        } else {
            // Fallback for development in browser without Rust backend
            console.warn('IPC not available, simulating response for', type);
            // Simulate async delay
            setTimeout(() => {
                if (pendingRequests.has(id)) {
                    const { resolve } = pendingRequests.get(id);
                    pendingRequests.delete(id);
                    resolve({ simulated: true, originalRequest: payload });
                }
            }, 500);
        }
    });
}

export const db = {
    query: (sql, params = []) => sendRequest('db_query', { sql, params }),
    execute: (sql, params = []) => sendRequest('db_execute', { sql, params }),
};

export const ai = {
    request: (prompt, context = null) => sendRequest('ai_request', { prompt, context }),
};

export const app = {
    exit: () => sendRequest('app_action', { action: 'exit' }),
    openTool: (toolId) => sendRequest('app_action', { action: `open_tool:${toolId}` }),
    openDocument: (docId) => sendRequest('app_action', { action: `open_document:${docId}` }),
    closeWindow: () => sendRequest('app_action', { action: 'close_window' }),
    minimizeWindow: () => sendRequest('app_action', { action: 'minimize_window' }),
    toggleMaximizeWindow: () => sendRequest('app_action', { action: 'toggle_maximize_window' }),
    startResize: (direction) => sendRequest('app_action', { action: `start_resize:${direction}` }),
    dragWindow: () => sendRequest('app_action', { action: 'drag_window' }),
};

export const log = (message) => sendRequest('log', { message });
