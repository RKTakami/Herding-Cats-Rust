import React, { useState, useEffect } from 'react';
import { Plus, Save, Trash2, Search, BookOpen, User, MapPin, Box, Clock, FileText, Download } from 'lucide-react';
import { db, log } from '../../api/ipc';

const CATEGORIES = [
    { id: 'summary', label: 'Story Summary', icon: BookOpen },
    { id: 'character', label: 'Characters', icon: User },
    { id: 'item', label: 'Items', icon: Box },
    { id: 'time', label: 'Time', icon: Clock },
    { id: 'location', label: 'Place', icon: MapPin },
];

const Codex = () => {
    const [activeCategory, setActiveCategory] = useState('summary');
    const [entries, setEntries] = useState([]);
    const [selectedEntry, setSelectedEntry] = useState(null);
    const [loading, setLoading] = useState(false);
    const [importModalOpen, setImportModalOpen] = useState(false);
    const [importCandidates, setImportCandidates] = useState([]);
    const [importLoading, setImportLoading] = useState(false);

    // Fetch entries for the active category
    const fetchEntries = async () => {
        setLoading(true);
        try {
            // We store codex entries as documents with type 'codex_entry'
            // and metadata.codex_type = activeCategory
            const result = await db.query(
                "SELECT id, title, metadata FROM documents WHERE document_type = 'codex_entry'"
            );

            if (result.data) {
                const parsed = result.data.map(d => {
                    let metadata = {};
                    try {
                        metadata = d.metadata ? JSON.parse(d.metadata) : {};
                    } catch (e) {
                        metadata = {};
                    }
                    return { ...d, metadata };
                }).filter(d => d.metadata.codex_type === activeCategory);

                setEntries(parsed);

                // If we have a selected entry, refresh it, otherwise clear selection
                if (selectedEntry) {
                    const found = parsed.find(e => e.id === selectedEntry.id);
                    setSelectedEntry(found || null);
                }
            }
        } catch (err) {
            log(`Error fetching codex entries: ${err.message}`);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchEntries();
        setSelectedEntry(null); // Clear selection on category switch
    }, [activeCategory]);

    const handleCreate = async () => {
        const id = crypto.randomUUID();
        const title = "New Entry";
        const metadata = {
            codex_type: activeCategory,
            description: '',
            // Add specific fields based on category
            ...(activeCategory === 'character' ? { role: '', age: '', traits: '' } : {}),
            ...(activeCategory === 'location' ? { type: '', significance: '' } : {}),
            ...(activeCategory === 'item' ? { type: '', abilities: '' } : {}),
            ...(activeCategory === 'time' ? { date: '', era: '' } : {}),
            ...(activeCategory === 'summary' ? { logline: '', themes: '' } : {}),
        };

        try {
            await db.execute(
                "INSERT INTO documents (id, project_id, title, content, document_type, metadata, created_at, updated_at, checksum) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)",
                [id, 'default-project', title, '', 'codex_entry', JSON.stringify(metadata), '']
            );
            await fetchEntries();
            // Select the new entry
            const newEntry = { id, title, metadata };
            setSelectedEntry(newEntry);
        } catch (err) {
            log(`Error creating entry: ${err.message}`);
        }
    };

    const handleSave = async () => {
        if (!selectedEntry) return;
        try {
            await db.execute(
                "UPDATE documents SET title = ?, metadata = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
                [selectedEntry.title, JSON.stringify(selectedEntry.metadata), selectedEntry.id]
            );
            log(`Saved entry: ${selectedEntry.title}`);
            await fetchEntries();
        } catch (err) {
            log(`Error saving entry: ${err.message}`);
        }
    };

    const handleDelete = async () => {
        if (!selectedEntry) return;
        if (!confirm(`Are you sure you want to delete "${selectedEntry.title}"?`)) return;
        try {
            await db.execute("DELETE FROM documents WHERE id = ?", [selectedEntry.id]);
            setSelectedEntry(null);
            await fetchEntries();
        } catch (err) {
            log(`Error deleting entry: ${err.message}`);
        }
    };

    const updateField = (field, value) => {
        if (!selectedEntry) return;
        if (field === 'title') {
            setSelectedEntry({ ...selectedEntry, title: value });
        } else {
            setSelectedEntry({
                ...selectedEntry,
                metadata: { ...selectedEntry.metadata, [field]: value }
            });
        }
    };

    // Heuristic Import Logic
    const scanForEntities = async () => {
        setImportLoading(true);
        setImportModalOpen(true);
        try {
            // Fetch all chapters and scenes
            const result = await db.query(
                "SELECT content FROM documents WHERE document_type IN ('chapter', 'scene')"
            );

            if (!result.data) {
                setImportCandidates([]);
                setImportLoading(false);
                return;
            }

            const text = result.data.map(d => {
                // Content is stored as Tiptap JSON string. We need to extract text.
                // For simplicity, we'll just regex the JSON for now, or try to parse if robust.
                // A simple regex to find "text": "..." values might be safer/faster than parsing huge JSONs.
                // Or just naive regex on the raw string.
                return d.content;
            }).join(' ');

            // Simple heuristic: Find Capitalized Words that appear frequently
            // Exclude common stopwords (very basic list)
            const stopWords = new Set(['The', 'A', 'An', 'In', 'On', 'At', 'To', 'For', 'Of', 'With', 'And', 'But', 'Or', 'So', 'It', 'He', 'She', 'They', 'We', 'You', 'I', 'Is', 'Was', 'Are', 'Were', 'Be', 'Have', 'Has', 'Had', 'Do', 'Does', 'Did', 'Can', 'Could', 'Will', 'Would', 'Should', 'May', 'Might', 'Must']);

            const words = text.match(/\b[A-Z][a-z]+\b/g) || [];
            const counts = {};
            words.forEach(w => {
                if (!stopWords.has(w)) {
                    counts[w] = (counts[w] || 0) + 1;
                }
            });

            // Filter by frequency > 2
            const candidates = Object.entries(counts)
                .filter(([_, count]) => count > 2)
                .map(([word, count]) => ({ word, count }))
                .sort((a, b) => b.count - a.count);

            setImportCandidates(candidates);

        } catch (err) {
            log(`Error scanning entities: ${err.message}`);
        } finally {
            setImportLoading(false);
        }
    };

    const importEntity = async (name) => {
        // Create a new entry with this name in the current category
        const id = crypto.randomUUID();
        const metadata = {
            codex_type: activeCategory,
            description: 'Imported from story text.',
        };
        try {
            await db.execute(
                "INSERT INTO documents (id, project_id, title, content, document_type, metadata, created_at, updated_at, checksum) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)",
                [id, 'default-project', name, '', 'codex_entry', JSON.stringify(metadata), '']
            );
            // Remove from candidates list locally
            setImportCandidates(prev => prev.filter(c => c.word !== name));
            log(`Imported ${name} to ${activeCategory}`);
            await fetchEntries();
        } catch (err) {
            log(`Error importing entity: ${err.message}`);
        }
    };

    return (
        <div className="flex flex-col h-full" style={{ backgroundColor: 'var(--bg-primary)', color: 'var(--text-primary)' }}>
            {/* Top Header: Categories and Toolbar */}
            <div className="flex items-center justify-between border-b" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-secondary)', padding: '12px 24px' }}>
                {/* Categories */}
                <div className="flex overflow-x-auto" style={{ gap: '48px' }}>
                    {CATEGORIES.map(cat => {
                        const Icon = cat.icon;
                        return (
                            <button
                                key={cat.id}
                                onClick={() => setActiveCategory(cat.id)}
                                className={`flex items-center gap-3 px-4 py-2 rounded-lg transition-colors whitespace-nowrap text-sm ${activeCategory === cat.id ? 'bg-blue-100 text-blue-600 font-medium' : 'text-gray-500 hover:bg-gray-200'}`}
                                title={cat.label}
                            >
                                <Icon size={18} />
                                <span>{cat.label}</span>
                            </button>
                        );
                    })}
                </div>

                {/* Toolbar */}
                <div className="flex gap-2 border-l pl-4" style={{ borderColor: 'var(--border-color)' }}>
                    <button onClick={scanForEntities} className="flex items-center gap-1 px-3 py-1.5 hover:bg-purple-100 text-purple-600 rounded transition-colors" title="Heuristic Import">
                        <Download size={16} />
                        <span className="text-sm">Import</span>
                    </button>
                    <button onClick={handleCreate} className="flex items-center gap-1 px-3 py-1.5 hover:bg-blue-100 text-blue-600 rounded transition-colors" title="Add Entry">
                        <Plus size={16} />
                        <span className="text-sm">Add</span>
                    </button>
                </div>
            </div>

            {/* Main Content Area */}
            <div className="flex-1 flex overflow-hidden">
                {/* Left Panel: Entry List */}
                <div className="w-64 border-r flex flex-col" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-secondary)' }}>
                    <div className="p-3 font-medium text-sm text-gray-500 border-b" style={{ borderColor: 'var(--border-color)' }}>
                        {CATEGORIES.find(c => c.id === activeCategory)?.label} Entries
                    </div>
                    <div className="flex-1 overflow-y-auto p-2">
                        {entries.map(entry => (
                            <div
                                key={entry.id}
                                onClick={() => setSelectedEntry(entry)}
                                className={`p-2 rounded cursor-pointer mb-1 text-sm ${selectedEntry?.id === entry.id ? 'bg-blue-500 text-white' : 'hover:bg-gray-200'}`}
                            >
                                {entry.title}
                            </div>
                        ))}
                        {entries.length === 0 && (
                            <div className="text-gray-400 text-center mt-8 text-sm italic">No entries yet.</div>
                        )}
                    </div>
                </div>

                {/* Right Panel: Editor */}
                <div className="flex-1 flex flex-col h-full bg-white" style={{ backgroundColor: 'var(--bg-primary)' }}>
                    {selectedEntry ? (
                        <>
                            <div className="p-6 border-b flex justify-between items-center" style={{ borderColor: 'var(--border-color)' }}>
                                <input
                                    type="text"
                                    value={selectedEntry.title}
                                    onChange={(e) => updateField('title', e.target.value)}
                                    className="text-2xl font-bold bg-transparent border-none focus:outline-none w-full"
                                    placeholder="Entry Name"
                                />
                                <div className="flex gap-2">
                                    <button onClick={handleSave} className="flex items-center gap-1 px-3 py-1.5 bg-blue-500 text-white rounded hover:bg-blue-600 shadow-sm">
                                        <Save size={16} /> Save
                                    </button>
                                    <button onClick={handleDelete} className="flex items-center gap-1 px-3 py-1.5 bg-white text-red-500 border border-red-200 rounded hover:bg-red-50 shadow-sm">
                                        <Trash2 size={16} /> Delete
                                    </button>
                                </div>
                            </div>
                            <div className="p-8 overflow-y-auto flex-1">
                                {/* Dynamic Fields based on Category */}
                                <div className="space-y-6 max-w-3xl mx-auto">

                                    {/* Common Description */}
                                    <div>
                                        <label className="block text-sm font-semibold text-gray-500 mb-2">Description</label>
                                        <textarea
                                            value={selectedEntry.metadata.description || ''}
                                            onChange={(e) => updateField('description', e.target.value)}
                                            className="w-full p-3 border rounded-lg h-32 bg-transparent focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                                            style={{ borderColor: 'var(--border-color)' }}
                                            placeholder="Enter description..."
                                        />
                                    </div>

                                    {activeCategory === 'character' && (
                                        <div className="grid grid-cols-2 gap-6">
                                            <div className="col-span-2">
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Role</label>
                                                <input
                                                    type="text"
                                                    value={selectedEntry.metadata.role || ''}
                                                    onChange={(e) => updateField('role', e.target.value)}
                                                    className="w-full p-2 border rounded bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Age</label>
                                                <input
                                                    type="text"
                                                    value={selectedEntry.metadata.age || ''}
                                                    onChange={(e) => updateField('age', e.target.value)}
                                                    className="w-full p-2 border rounded bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Traits</label>
                                                <input
                                                    type="text"
                                                    value={selectedEntry.metadata.traits || ''}
                                                    onChange={(e) => updateField('traits', e.target.value)}
                                                    className="w-full p-2 border rounded bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                        </div>
                                    )}

                                    {activeCategory === 'location' && (
                                        <>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Type</label>
                                                <input
                                                    type="text"
                                                    value={selectedEntry.metadata.type || ''}
                                                    onChange={(e) => updateField('type', e.target.value)}
                                                    className="w-full p-2 border rounded bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Significance</label>
                                                <textarea
                                                    value={selectedEntry.metadata.significance || ''}
                                                    onChange={(e) => updateField('significance', e.target.value)}
                                                    className="w-full p-2 border rounded h-24 bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                        </>
                                    )}

                                    {activeCategory === 'summary' && (
                                        <>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Logline</label>
                                                <textarea
                                                    value={selectedEntry.metadata.logline || ''}
                                                    onChange={(e) => updateField('logline', e.target.value)}
                                                    className="w-full p-2 border rounded h-24 bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                            <div>
                                                <label className="block text-sm font-semibold text-gray-500 mb-2">Themes</label>
                                                <input
                                                    type="text"
                                                    value={selectedEntry.metadata.themes || ''}
                                                    onChange={(e) => updateField('themes', e.target.value)}
                                                    className="w-full p-2 border rounded bg-transparent"
                                                    style={{ borderColor: 'var(--border-color)' }}
                                                />
                                            </div>
                                        </>
                                    )}
                                </div>
                            </div>
                        </>
                    ) : (
                        <div className="flex-1 flex flex-col items-center justify-center text-gray-300">
                            <BookOpen size={64} className="mb-4 opacity-20" />
                            <p>Select an entry from the list or create a new one.</p>
                        </div>
                    )}
                </div>
            </div>

            {/* Import Modal */}
            {importModalOpen && (
                <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div className="bg-white p-6 rounded-lg shadow-xl w-96 max-h-[80vh] flex flex-col" style={{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-primary)', border: '1px solid var(--border-color)' }}>
                        <div className="flex justify-between items-center mb-4">
                            <h3 className="text-lg font-bold">Heuristic Import</h3>
                            <button onClick={() => setImportModalOpen(false)} className="text-gray-400 hover:text-gray-600">
                                <Plus size={20} className="rotate-45" />
                            </button>
                        </div>
                        <p className="text-sm text-gray-500 mb-4">Suggested entities found in your story:</p>

                        {importLoading ? (
                            <div className="text-center py-8 text-blue-500">Scanning story text...</div>
                        ) : (
                            <div className="flex-1 overflow-y-auto space-y-2 mb-4 pr-2">
                                {importCandidates.length > 0 ? (
                                    importCandidates.map((c, i) => (
                                        <div key={i} className="flex justify-between items-center p-3 border rounded hover:bg-gray-100 transition-colors" style={{ borderColor: 'var(--border-color)' }}>
                                            <span className="font-medium">{c.word} <span className="text-xs text-gray-400 ml-1">({c.count}x)</span></span>
                                            <button
                                                onClick={() => importEntity(c.word)}
                                                className="px-3 py-1 text-xs bg-blue-500 text-white rounded-full hover:bg-blue-600 transition-colors"
                                            >
                                                Import
                                            </button>
                                        </div>
                                    ))
                                ) : (
                                    <div className="text-center text-gray-400 py-8">No new entities found.</div>
                                )}
                            </div>
                        )}

                        <button
                            onClick={() => setImportModalOpen(false)}
                            className="w-full py-2 bg-gray-100 rounded hover:bg-gray-200 text-gray-600 transition-colors"
                        >
                            Done
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};

export default Codex;
