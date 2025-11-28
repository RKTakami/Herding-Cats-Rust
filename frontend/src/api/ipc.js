const pendingRequests = new Map();

// Listen for responses from Rust
window.addEventListener('message', (event) => {
    // Rust sends responses via window.postMessage
    // The data structure matches IpcResponseWrapper in Rust
    const data = event.data;

    if (data && data.id && pendingRequests.has(data.id)) {
        const { resolve, reject } = pendingRequests.get(data.id);
        pendingRequests.delete(data.id);

        // Check for error response type
        // The payload structure depends on the Rust IpcResponse enum serialization
        // IpcResponse::Error { message } serializes to { type: "error", payload: { message: "..." } }
        // Wait, IpcResponseWrapper has { id: "...", response: { type: "...", payload: ... } }
        // because IpcResponse has #[serde(tag = "type", content = "payload")]

        const response = data.response;

        if (response.type === 'error') {
            reject(new Error(response.payload.message));
        } else {
            resolve(response.payload);
        }
    }
});

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
};

export const log = (message) => sendRequest('log', { message });
