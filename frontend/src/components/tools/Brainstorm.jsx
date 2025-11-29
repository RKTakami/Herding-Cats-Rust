import React, { useState, useEffect, useRef } from 'react';
import { Send, Bot, User, RefreshCw } from 'lucide-react';
import { ai, log } from '../../api/ipc';

const Message = ({ role, text }) => {
    const isUser = role === 'user';
    return (
        <div className={`flex w-full mb-4 ${isUser ? 'justify-end' : 'justify-start'}`}>
            <div className={`flex max-w-[80%] ${isUser ? 'flex-row-reverse' : 'flex-row'}`}>
                <div className={`flex-shrink-0 h-8 w-8 rounded-full flex items-center justify-center mx-2 ${isUser ? 'bg-blue-500' : 'bg-green-500'}`}>
                    {isUser ? <User size={16} className="text-white" /> : <Bot size={16} className="text-white" />}
                </div>
                <div className={`p-3 rounded-lg ${isUser ? 'bg-blue-100 text-blue-900' : 'bg-gray-100 text-gray-900'}`}>
                    <p className="whitespace-pre-wrap text-sm">{text}</p>
                </div>
            </div>
        </div>
    );
};

const Brainstorm = () => {
    const [messages, setMessages] = useState([
        { role: 'ai', text: 'Hello! I am your creative assistant. How can I help you brainstorm today?' }
    ]);
    const [input, setInput] = useState('');
    const [loading, setLoading] = useState(false);
    const messagesEndRef = useRef(null);

    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    const handleSend = async () => {
        if (!input.trim() || loading) return;

        const userMessage = { role: 'user', text: input };
        setMessages(prev => [...prev, userMessage]);
        setInput('');
        setLoading(true);

        try {
            log(`[Brainstorm] Sending request: ${userMessage.text}`);
            // TODO: Pass context if needed (e.g., current document content)
            const responseText = await ai.request(userMessage.text);

            const aiMessage = { role: 'ai', text: responseText };
            setMessages(prev => [...prev, aiMessage]);
        } catch (err) {
            console.error('AI Request failed:', err);
            log(`[Brainstorm] Error: ${err.message}`);
            setMessages(prev => [...prev, { role: 'ai', text: `Error: ${err.message}` }]);
        } finally {
            setLoading(false);
        }
    };

    const handleKeyDown = (e) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    };

    return (
        <div className="flex flex-col h-full" style={{ backgroundColor: 'var(--bg-primary)', color: 'var(--text-primary)' }}>
            {/* Header */}
            <div className="p-2 border-b flex justify-between items-center" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-secondary)' }}>
                <span className="font-semibold text-sm flex items-center gap-2">
                    <Bot size={16} />
                    Brainstorm
                </span>
                <button
                    onClick={() => setMessages([{ role: 'ai', text: 'Hello! I am your creative assistant. How can I help you brainstorm today?' }])}
                    className="p-1 hover:bg-gray-200 rounded"
                    title="Reset Chat"
                >
                    <RefreshCw size={14} />
                </button>
            </div>

            {/* Messages Area */}
            <div className="flex-1 overflow-y-auto p-4">
                {messages.map((msg, index) => (
                    <Message key={index} role={msg.role} text={msg.text} />
                ))}
                {loading && (
                    <div className="flex w-full mb-4 justify-start">
                        <div className="flex max-w-[80%] flex-row">
                            <div className="flex-shrink-0 h-8 w-8 rounded-full flex items-center justify-center mx-2 bg-green-500">
                                <Bot size={16} className="text-white" />
                            </div>
                            <div className="p-3 rounded-lg bg-gray-100 text-gray-500 italic">
                                <p className="text-sm">Thinking...</p>
                            </div>
                        </div>
                    </div>
                )}
                <div ref={messagesEndRef} />
            </div>

            {/* Input Area */}
            <div className="p-4 border-t" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-secondary)' }}>
                <div className="flex gap-2">
                    <textarea
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder="Type your idea or question..."
                        className="flex-1 p-2 rounded border resize-none focus:ring-2 focus:ring-blue-500 outline-none"
                        style={{
                            backgroundColor: 'var(--bg-primary)',
                            color: 'var(--text-primary)',
                            borderColor: 'var(--border-color)',
                            minHeight: '44px',
                            maxHeight: '120px'
                        }}
                        rows={1}
                    />
                    <button
                        onClick={handleSend}
                        disabled={loading || !input.trim()}
                        className={`p-2 rounded-full flex items-center justify-center transition-colors ${loading || !input.trim() ? 'bg-gray-300 cursor-not-allowed' : 'bg-blue-500 hover:bg-blue-600 text-white'}`}
                        style={{ width: '44px', height: '44px' }}
                    >
                        <Send size={20} />
                    </button>
                </div>
                <div className="text-xs text-gray-400 mt-1 text-center">
                    Shift + Enter for new line
                </div>
            </div>
        </div>
    );
};

export default Brainstorm;
