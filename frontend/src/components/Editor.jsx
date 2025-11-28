import React, { useEffect, useState } from 'react';
import { useEditor, EditorContent } from '@tiptap/react';
import { BubbleMenu, FloatingMenu } from '@tiptap/react/menus';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import BubbleMenuExtension from '@tiptap/extension-bubble-menu';
import FloatingMenuExtension from '@tiptap/extension-floating-menu';
import { Bold, Italic, Strikethrough, Code, Heading1, Heading2, List, ListOrdered, Quote } from 'lucide-react';
import { db, log } from '../api/ipc';

const MenuBar = ({ editor }) => {
    if (!editor) {
        return null;
    }

    return (
        <div className="flex flex-wrap gap-2 p-2 mb-4 bg-gray-800 rounded-lg border border-gray-700 sticky top-0 z-10">
            <button
                onClick={() => editor.chain().focus().toggleBold().run()}
                disabled={!editor.can().chain().focus().toggleBold().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('bold') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Bold"
            >
                <Bold size={18} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleItalic().run()}
                disabled={!editor.can().chain().focus().toggleItalic().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('italic') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Italic"
            >
                <Italic size={18} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleStrike().run()}
                disabled={!editor.can().chain().focus().toggleStrike().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('strike') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Strikethrough"
            >
                <Strikethrough size={18} />
            </button>
            <div className="w-px h-6 bg-gray-600 mx-1 self-center"></div>
            <button
                onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('heading', { level: 1 }) ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Heading 1"
            >
                <Heading1 size={18} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('heading', { level: 2 }) ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Heading 2"
            >
                <Heading2 size={18} />
            </button>
            <div className="w-px h-6 bg-gray-600 mx-1 self-center"></div>
            <button
                onClick={() => editor.chain().focus().toggleBulletList().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('bulletList') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Bullet List"
            >
                <List size={18} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleOrderedList().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('orderedList') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Ordered List"
            >
                <ListOrdered size={18} />
            </button>
            <div className="w-px h-6 bg-gray-600 mx-1 self-center"></div>
            <button
                onClick={() => editor.chain().focus().toggleCodeBlock().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('codeBlock') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Code Block"
            >
                <Code size={18} />
            </button>
            <button
                onClick={() => editor.chain().focus().toggleBlockquote().run()}
                className={`p-2 rounded hover:bg-gray-700 ${editor.isActive('blockquote') ? 'bg-gray-700 text-blue-400' : 'text-gray-300'}`}
                title="Blockquote"
            >
                <Quote size={18} />
            </button>
        </div>
    );
};

const Editor = ({ placeholder = 'Start writing...' }) => {
    const [content, setContent] = useState('');
    const [lastSaved, setLastSaved] = useState(null);
    const [isSaving, setIsSaving] = useState(false);

    // Load initial content
    useEffect(() => {
        async function loadContent() {
            try {
                const result = await db.query("SELECT content FROM documents WHERE id = 'scratchpad'");
                if (result.data && result.data.length > 0) {
                    setContent(result.data[0].content);
                }
            } catch (err) {
                log(`Error loading content: ${err.message}`);
            }
        }
        loadContent();
    }, []);

    const saveContent = async (html) => {
        try {
            setIsSaving(true);
            // Upsert content
            await db.execute(`
                INSERT INTO documents (id, content, updated_at) 
                VALUES ('scratchpad', ?, CURRENT_TIMESTAMP)
                ON CONFLICT(id) DO UPDATE SET 
                content = excluded.content,
                updated_at = CURRENT_TIMESTAMP
            `, [html]);
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
    const debouncedSave = (html) => {
        if (saveTimeout) clearTimeout(saveTimeout);
        const timeout = setTimeout(() => saveContent(html), 1000);
        setSaveTimeout(timeout);
    };

    const editor = useEditor({
        extensions: [
            StarterKit,
            Placeholder.configure({
                placeholder,
            }),
            BubbleMenuExtension,
            FloatingMenuExtension,
        ],
        content: content,
        onUpdate: ({ editor }) => {
            const html = editor.getHTML();
            debouncedSave(html);
        },
        editorProps: {
            attributes: {
                class: 'prose prose-invert max-w-none focus:outline-none min-h-[500px]',
            },
        },
    });

    // Update editor content when loaded from DB
    useEffect(() => {
        if (editor && content && editor.getHTML() !== content) {
            // Only set content if it's significantly different to avoid cursor jumps
            // Ideally we'd use Yjs for real sync, but for now this is a simple load
            if (editor.isEmpty) {
                editor.commands.setContent(content);
            }
        }
    }, [content, editor]);

    return (
        <div className="editor-wrapper flex flex-col h-full relative">
            <div className="absolute top-2 right-2 text-xs text-gray-500 z-20">
                {isSaving ? 'Saving...' : lastSaved ? `Saved ${lastSaved.toLocaleTimeString()}` : ''}
            </div>
            <MenuBar editor={editor} />

            {editor && (
                <BubbleMenu className="bubble-menu bg-gray-800 border border-gray-700 rounded-lg shadow-xl flex p-1 gap-1" tippyOptions={{ duration: 100 }} editor={editor}>
                    <button
                        onClick={() => editor.chain().focus().toggleBold().run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('bold') ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <Bold size={14} />
                    </button>
                    <button
                        onClick={() => editor.chain().focus().toggleItalic().run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('italic') ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <Italic size={14} />
                    </button>
                    <button
                        onClick={() => editor.chain().focus().toggleStrike().run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('strike') ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <Strikethrough size={14} />
                    </button>
                </BubbleMenu>
            )}

            {editor && (
                <FloatingMenu className="floating-menu bg-gray-800 border border-gray-700 rounded-lg shadow-xl flex p-1 gap-1" tippyOptions={{ duration: 100 }} editor={editor}>
                    <button
                        onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('heading', { level: 1 }) ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <Heading1 size={14} />
                    </button>
                    <button
                        onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('heading', { level: 2 }) ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <Heading2 size={14} />
                    </button>
                    <button
                        onClick={() => editor.chain().focus().toggleBulletList().run()}
                        className={`p-1 rounded hover:bg-gray-700 ${editor.isActive('bulletList') ? 'text-blue-400' : 'text-gray-300'}`}
                    >
                        <List size={14} />
                    </button>
                </FloatingMenu>
            )}

            <div className="editor-content flex-1 overflow-y-auto">
                <EditorContent editor={editor} />
            </div>
        </div>
    );
};

export default Editor;
