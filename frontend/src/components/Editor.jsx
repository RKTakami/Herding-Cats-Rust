import React, { useEffect, useState } from 'react';
import { useEditor, EditorContent } from '@tiptap/react';
import { Extension } from '@tiptap/core';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import { TextStyle } from '@tiptap/extension-text-style';
import FontFamily from '@tiptap/extension-font-family';
import TextAlign from '@tiptap/extension-text-align';
import Underline from '@tiptap/extension-underline';
import {
    Bold, Italic, Strikethrough, Underline as UnderlineIcon,
    AlignLeft, AlignCenter, AlignRight, AlignJustify,
    List, ListOrdered, Quote, Code
} from 'lucide-react';
import { db, log } from '../api/ipc';
import { useTheme } from '../contexts/ThemeContext';
import { useStats } from '../contexts/StatsContext';

// Custom Font Size Extension removed for stability

const MenuBar = ({ editor }) => {
    if (!editor) {
        return null;
    }

    const setFontFamily = (e) => {
        const font = e.target.value;
        if (font === 'default') {
            editor.chain().focus().unsetFontFamily().run();
        } else {
            editor.chain().focus().setFontFamily(font).run();
        }
    };

    // Helper to get current values
    const currentFont = editor.getAttributes('textStyle').fontFamily || 'default';

    return (
        <div className="flex flex-wrap items-center gap-2 p-3 mb-4 sticky top-0 z-10 select-none shadow-sm" style={{ borderBottom: '1px solid var(--border-color)', backgroundColor: 'var(--bg-secondary)' }}>

            {/* Font Family */}
            <select
                value={currentFont}
                onChange={setFontFamily}
                className="p-2 rounded text-base border-none focus:ring-1 focus:ring-blue-500 outline-none w-40 mr-2"
                style={{ backgroundColor: 'var(--bg-primary)', color: 'var(--text-primary)' }}
            >
                <option value="default">Font</option>
                <option value="Inter">Inter</option>
                <option value="Arial">Arial</option>
                <option value="Georgia">Georgia</option>
                <option value="Times New Roman">Times New Roman</option>
                <option value="Courier New">Courier New</option>
            </select>

            <div className="w-px h-8 mx-2 self-center" style={{ backgroundColor: 'var(--border-color)' }}></div>

            {/* Formatting Group */}
            <button
                onClick={() => editor.chain().focus().toggleBold().run()}
                disabled={!editor.can().chain().focus().toggleBold().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('bold') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('bold') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Bold"
            >
                <Bold size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleItalic().run()}
                disabled={!editor.can().chain().focus().toggleItalic().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('italic') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('italic') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Italic"
            >
                <Italic size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleUnderline().run()}
                disabled={!editor.can().chain().focus().toggleUnderline().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('underline') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('underline') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Underline"
            >
                <UnderlineIcon size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleStrike().run()}
                disabled={!editor.can().chain().focus().toggleStrike().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('strike') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('strike') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Strikethrough"
            >
                <Strikethrough size={20} />
            </button>

            <div className="w-px h-8 mx-2 self-center" style={{ backgroundColor: 'var(--border-color)' }}></div>

            {/* Alignment Group */}
            <button
                onClick={() => editor.chain().focus().setTextAlign('left').run()}
                className={`p-2 rounded transition-colors ${editor.isActive({ textAlign: 'left' }) ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive({ textAlign: 'left' }) ? '#2563eb' : 'var(--text-secondary)' }}
                title="Align Left"
            >
                <AlignLeft size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().setTextAlign('center').run()}
                className={`p-2 rounded transition-colors ${editor.isActive({ textAlign: 'center' }) ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive({ textAlign: 'center' }) ? '#2563eb' : 'var(--text-secondary)' }}
                title="Align Center"
            >
                <AlignCenter size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().setTextAlign('right').run()}
                className={`p-2 rounded transition-colors ${editor.isActive({ textAlign: 'right' }) ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive({ textAlign: 'right' }) ? '#2563eb' : 'var(--text-secondary)' }}
                title="Align Right"
            >
                <AlignRight size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().setTextAlign('justify').run()}
                className={`p-2 rounded transition-colors ${editor.isActive({ textAlign: 'justify' }) ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive({ textAlign: 'justify' }) ? '#2563eb' : 'var(--text-secondary)' }}
                title="Justify"
            >
                <AlignJustify size={20} />
            </button>

            <div className="w-px h-8 mx-2 self-center" style={{ backgroundColor: 'var(--border-color)' }}></div>

            {/* Lists & Extras */}
            <button
                onClick={() => editor.chain().focus().toggleBulletList().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('bulletList') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('bulletList') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Bullet List"
            >
                <List size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleOrderedList().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('orderedList') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('orderedList') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Ordered List"
            >
                <ListOrdered size={20} />
            </button>

            <button
                onClick={() => editor.chain().focus().toggleBlockquote().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('blockquote') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('blockquote') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Blockquote"
            >
                <Quote size={20} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleCodeBlock().run()}
                className={`p-2 rounded transition-colors ${editor.isActive('codeBlock') ? 'bg-blue-100 text-blue-600' : 'hover:bg-gray-100'}`}
                style={{ color: editor.isActive('codeBlock') ? '#2563eb' : 'var(--text-secondary)' }}
                title="Code Block"
            >
                <Code size={20} />
            </button>
        </div>
    );
};

const Editor = ({ placeholder = 'Start writing...', documentId = 'scratchpad', trackGlobalStats = false }) => {
    const [content, setContent] = useState('');
    const [title, setTitle] = useState('');
    const [lastSaved, setLastSaved] = useState(null);
    const [isSaving, setIsSaving] = useState(false);

    // Load initial content
    useEffect(() => {
        async function loadContent() {
            try {
                const result = await db.query(`SELECT title, content FROM documents WHERE id = '${documentId}'`);
                if (result.data && result.data.length > 0) {
                    const doc = result.data[0];
                    setTitle(doc.title || documentId);
                    let loadedContent = doc.content;
                    // Try to parse as JSON, if fails treat as HTML string
                    try {
                        const parsed = JSON.parse(loadedContent);
                        // Check if it looks like a ProseMirror document (has type: 'doc')
                        if (parsed && parsed.type === 'doc') {
                            setContent(parsed);
                        } else {
                            setContent(loadedContent);
                        }
                    } catch (e) {
                        // Not JSON, assume legacy HTML
                        setContent(loadedContent);
                    }
                } else {
                    setContent(''); // Reset content if document doesn't exist
                    setTitle(documentId);
                }
            } catch (err) {
                log(`Error loading content: ${err.message}`);
            }
        }
        loadContent();
    }, [documentId]);

    const generateChecksum = async (text) => {
        const msgBuffer = new TextEncoder().encode(text);
        const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
        const hashArray = Array.from(new Uint8Array(hashBuffer));
        const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
        return hashHex;
    };

    const saveContent = async (contentToSave) => {
        try {
            setIsSaving(true);
            // Convert JSON object to string for storage
            const contentString = typeof contentToSave === 'string' ? contentToSave : JSON.stringify(contentToSave);
            const checksum = await generateChecksum(contentString);

            // Upsert content
            await db.execute(`
                INSERT INTO documents (id, project_id, title, content, document_type, checksum, updated_at)
                VALUES (?, 'default-project', ?, ?, 'json', ?, CURRENT_TIMESTAMP)
                ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                document_type = excluded.document_type,
                checksum = excluded.checksum,
                updated_at = CURRENT_TIMESTAMP
            `, [documentId, title || documentId, contentString, checksum]);
            setLastSaved(new Date());
            setIsSaving(false);
        } catch (err) {
            console.error('Failed to save:', err);
            log(`Failed to save: ${err.message}`);
            setIsSaving(false);
        }
    };

    // Debounce save function
    const [saveTimeout, setSaveTimeout] = useState(null);
    const debouncedSave = (contentToSave) => {
        if (saveTimeout) clearTimeout(saveTimeout);
        const timeout = setTimeout(() => saveContent(contentToSave), 1000);
        setSaveTimeout(timeout);
    };

    const { theme } = useTheme();
    const { setWordCount } = useStats();
    const isDarkTheme = ['dark', 'terminal-green', 'terminal-amber'].includes(theme);

    const editor = useEditor({
        extensions: [
            StarterKit,
            Placeholder.configure({
                placeholder,
            }),
            TextStyle,
            FontFamily,
            Underline,
            TextAlign.configure({
                types: ['heading', 'paragraph'],
            }),
        ],
        content: content,
        onUpdate: ({ editor }) => {
            const json = editor.getJSON();
            if (trackGlobalStats) {
                const text = editor.getText();
                const words = text.trim().split(/\s+/).filter(w => w.length > 0).length;
                setWordCount(words);
            }
            debouncedSave(json);
        },
        editorProps: {
            attributes: {
                class: `prose ${isDarkTheme ? 'prose-invert' : ''} max-w-none focus:outline-none min-h-[500px]`,
                style: 'color: var(--text-primary)'
            },
        },
    });

    // Update editor content when loaded from DB
    useEffect(() => {
        if (editor && content !== undefined) {
            // Check if content has changed significantly
            // For JSON, we compare stringified versions or rely on Tiptap's internal check
            // For HTML, we compare strings

            // Simple check: if editor is empty and we have content, set it
            if (editor.isEmpty && content) {
                editor.commands.setContent(content);
                // Update word count
                if (trackGlobalStats) {
                    const text = editor.getText();
                    const words = text.trim().split(/\s+/).filter(w => w.length > 0).length;
                    setWordCount(words);
                }
            }
        }
    }, [content, editor, setWordCount, trackGlobalStats]);

    return (
        <div className="editor-wrapper flex flex-col h-full relative">
            <div className="absolute top-2 right-2 text-xs text-gray-500 z-20">
                {isSaving ? 'Saving...' : lastSaved ? `Saved ${lastSaved.toLocaleTimeString()}` : ''}
            </div>
            <MenuBar editor={editor} />

            {/* Manuscript Title Bar */}
            <div className="w-full flex justify-center" style={{ backgroundColor: 'var(--bg-tertiary)' }}>
                <div
                    className="text-center font-semibold text-lg"
                    style={{
                        paddingTop: '0.25em',
                        paddingBottom: '0.25em',
                        color: 'var(--text-primary)',
                        width: '816px' // Match page width for alignment
                    }}
                >
                    {title}
                </div>
            </div>

            <div className="editor-content flex-1 overflow-y-auto bg-gray-100 p-8 flex justify-center" style={{ backgroundColor: 'var(--bg-tertiary)' }}>
                <div
                    className="min-h-[1056px] w-[816px] bg-white shadow-lg p-[96px] outline-none"
                    style={{
                        backgroundColor: 'var(--bg-primary)',
                        color: 'var(--text-primary)'
                    }}
                    onClick={() => editor?.chain().focus().run()}
                >
                    <EditorContent editor={editor} />
                </div>
            </div>
        </div>
    );
};

export default Editor;
