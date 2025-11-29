import React, { useState, useEffect } from 'react';
import { Folder, FileText, ChevronRight, ChevronDown, Plus, RefreshCw, Trash2, FilePlus } from 'lucide-react';
import { db, app, log } from '../../api/ipc';

const HierarchyItem = ({ item, level = 0, onSelect, expandedItems, toggleExpand, onAddScene, onDelete, onMove, selectedId, dragOverId, setDragOverId }) => {
    const hasChildren = item.children && item.children.length > 0;
    const isExpanded = expandedItems.has(item.id);
    const [isHovered, setIsHovered] = useState(false);
    const isSelected = selectedId === item.id;

    const handleDragStart = (e) => {
        e.dataTransfer.setData('text/plain', item.id);
        e.dataTransfer.effectAllowed = 'move';
    };

    const handleDragOver = (e) => {
        e.preventDefault();
        e.stopPropagation();
        e.dataTransfer.dropEffect = 'move';
        if (dragOverId !== item.id) {
            setDragOverId(item.id);
        }
    };

    const handleDrop = (e) => {
        e.preventDefault();
        e.stopPropagation(); // Stop bubbling to root
        setDragOverId(null);
        setIsHovered(false);
        const draggedId = e.dataTransfer.getData('text/plain');
        if (draggedId && draggedId !== item.id) {
            onMove(draggedId, item);
        }
    };

    return (
        <div className="relative">
            {/* Vertical line for children */}
            {level > 0 && (
                <div
                    className="absolute border-l border-gray-300"
                    style={{
                        left: `${(level - 1) * 16 + 15}px`,
                        top: '0',
                        bottom: '0'
                    }}
                />
            )}

            <div
                className={`flex items-center py-1 px-2 cursor-pointer select-none transition-colors rounded group relative`}
                style={{
                    paddingLeft: `${level * 16 + 8}px`,
                    backgroundColor: dragOverId === item.id ? 'rgba(59, 130, 246, 0.1)' : (isSelected ? '#3b82f6' : (isHovered ? 'var(--bg-secondary-hover, rgba(0,0,0,0.05))' : 'transparent')),
                    color: isSelected ? 'white' : 'var(--text-primary)',
                    boxShadow: dragOverId === item.id ? 'inset 0 -2px 0 0 #3b82f6' : 'none'
                }}
                onClick={() => onSelect(item)}
                onMouseEnter={() => setIsHovered(true)}
                onMouseLeave={() => setIsHovered(false)}
                draggable={true}
                onDragStart={handleDragStart}
                onDragOver={handleDragOver}
                onDrop={handleDrop}
            >
                <div
                    className="mr-1 p-1 rounded hover:bg-gray-200 z-10 relative"
                    onClick={(e) => {
                        e.stopPropagation();
                        toggleExpand(item.id);
                    }}
                >
                    {hasChildren ? (
                        isExpanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />
                    ) : <div className="w-3.5" />}
                </div>

                {item.type === 'manuscript' && <Folder size={16} className="mr-2 text-blue-500 z-10 relative" />}
                {item.type === 'folder' && <Folder size={16} className="mr-2 text-blue-400 z-10 relative" />}
                {item.type === 'chapter' && <Folder size={16} className="mr-2 text-yellow-500 z-10 relative" />}
                {item.type === 'scene' && <FileText size={16} className="mr-2 text-gray-500 z-10 relative" />}

                <span className="text-sm truncate flex-1 z-10 relative">{item.title || 'Untitled'}</span>
            </div>

            {/* Insertion Line Indicator - Removed separate div, using border-b on item for reliability */}

            {isExpanded && hasChildren && (
                <div
                    className="pl-2 min-h-[10px]" // Ensure it has height
                    onDragOver={(e) => {
                        e.preventDefault();
                        e.stopPropagation(); // Stop bubbling
                        e.dataTransfer.dropEffect = 'move';
                    }}
                    onDrop={(e) => {
                        e.preventDefault();
                        e.stopPropagation(); // Stop bubbling
                        setDragOverId(null);
                        const draggedId = e.dataTransfer.getData('text/plain');
                        if (draggedId && draggedId !== item.id) {
                            // If dropping on children container, move to parent (this item)
                            onMove(draggedId, item);
                        }
                    }}
                >
                    {item.children.map(child => (
                        <HierarchyItem
                            key={child.id}
                            item={child}
                            level={level + 1}
                            onSelect={onSelect}
                            expandedItems={expandedItems}
                            toggleExpand={toggleExpand}
                            onAddScene={onAddScene}
                            onDelete={onDelete}
                            onMove={onMove}
                            selectedId={selectedId}
                            dragOverId={dragOverId}
                            setDragOverId={setDragOverId}
                        />
                    ))}
                </div>
            )}
        </div>
    );
};

const Hierarchy = () => {
    const [items, setItems] = useState([]);
    const [expandedItems, setExpandedItems] = useState(new Set());
    const [loading, setLoading] = useState(false);
    const [selectedId, setSelectedId] = useState(null);
    const [selectedItem, setSelectedItem] = useState(null);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
    const [itemToDelete, setItemToDelete] = useState(null);
    const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
    const [newChapterTitle, setNewChapterTitle] = useState('');
    const [dragOverId, setDragOverId] = useState(null);

    const fetchDocuments = async () => {
        setLoading(true);
        try {
            const result = await db.query("SELECT id, title, document_type, metadata FROM documents ORDER BY title");
            if (result.data) {
                const docs = result.data.map(d => {
                    let metadata = {};
                    try {
                        if (d.metadata) {
                            // Handle escaped quotes from DB/IPC
                            let cleanMeta = d.metadata;
                            if (typeof cleanMeta === 'string') {
                                // Remove backslashes before quotes
                                cleanMeta = cleanMeta.replace(/\\"/g, '"');
                                // Remove leading/trailing quotes if it's a stringified string
                                if (cleanMeta.startsWith('"') && cleanMeta.endsWith('"')) {
                                    cleanMeta = cleanMeta.slice(1, -1);
                                }
                            }
                            metadata = JSON.parse(cleanMeta);
                        }
                    } catch (e) {
                        console.warn(`Failed to parse metadata for doc ${d.id}:`, d.metadata);
                    }
                    return {
                        ...d,
                        metadata,
                        children: [] // Initialize children array
                    };
                });

                const docMap = {};
                docs.forEach(d => docMap[d.id] = d);

                const manuscript = { id: 'manuscript', title: 'Manuscript', type: 'manuscript', children: [] };
                const unassigned = { id: 'unassigned', title: 'Unassigned', type: 'folder', children: [] };

                // Build tree
                docs.forEach(doc => {
                    if (doc.id === 'manuscript') return;

                    const parentId = doc.metadata?.parent_id;
                    if (parentId && docMap[parentId]) {
                        docMap[parentId].children.push(doc);
                    } else if (parentId === 'manuscript') {
                        manuscript.children.push(doc);
                    } else {
                        // Heuristic fallback
                        if (doc.title && doc.title.startsWith('Chapter')) {
                            doc.type = 'chapter'; // Ensure type is set
                            manuscript.children.push(doc);
                        } else {
                            doc.type = 'scene'; // Default to scene
                            unassigned.children.push(doc);
                        }
                    }
                });

                // Recursive sort function
                const sortChildren = (items) => {
                    items.sort((a, b) => {
                        // Use explicit order if available
                        const orderA = a.metadata?.order ?? 9999;
                        const orderB = b.metadata?.order ?? 9999;
                        if (orderA !== orderB) return orderA - orderB;

                        // Fallback to type then title
                        const typeOrder = { 'folder': 0, 'chapter': 1, 'scene': 2 };
                        const ta = typeOrder[a.type] ?? 99;
                        const tb = typeOrder[b.type] ?? 99;
                        if (ta !== tb) return ta - tb;
                        return a.title.localeCompare(b.title);
                    });
                    items.forEach(item => {
                        if (item.children && item.children.length > 0) {
                            sortChildren(item.children);
                        }
                    });
                };

                sortChildren(manuscript.children);
                sortChildren(unassigned.children);

                // Always add Unassigned folder, Unassigned FIRST as requested
                const rootItems = [unassigned, manuscript];

                setItems(rootItems);
                setExpandedItems(prev => new Set([...prev, 'manuscript', 'unassigned']));
            }
        } catch (err) {
            log(`Error fetching documents: ${err.message}`);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchDocuments();
    }, []);

    const toggleExpand = (id) => {
        setExpandedItems(prev => {
            const next = new Set(prev);
            if (next.has(id)) next.delete(id);
            else next.add(id);
            return next;
        });
    };

    const handleSelect = (item) => {
        log(`[Hierarchy] Selected item: ${item.id} (${item.title})`);
        setSelectedId(item.id);
        setSelectedItem(item);
        if (item.id && item.id !== 'unassigned') {
            app.openDocument(item.id);
        }
    };

    const openCreateChapterModal = () => {
        setNewChapterTitle('Chapter ');
        setIsCreateModalOpen(true);
    };

    const confirmCreateChapter = async () => {
        if (!newChapterTitle.trim()) return;

        const id = crypto.randomUUID();
        const title = newChapterTitle;
        const metadata = JSON.stringify({ parent_id: 'manuscript' });

        // Initial Tiptap JSON: Heading 1 + Empty Paragraph
        const initialContent = {
            type: 'doc',
            content: [
                {
                    type: 'heading',
                    attrs: { level: 1 },
                    content: [{ type: 'text', text: title }]
                },
                {
                    type: 'paragraph'
                }
            ]
        };
        const contentString = JSON.stringify(initialContent);

        try {
            log(`[Hierarchy] Creating chapter: ${title} (${id})`);
            await db.execute(
                "INSERT INTO documents (id, project_id, title, content, document_type, metadata, created_at, updated_at, checksum) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)",
                [id, 'default-project', title, contentString, 'chapter', metadata, '']
            );
            setIsCreateModalOpen(false);
            log(`[Hierarchy] Chapter created. Fetching documents...`);
            await fetchDocuments();
            log(`[Hierarchy] Documents fetched. Opening document: ${id}`);
            // Auto-open new chapter
            app.openDocument(id);
            // Select it in hierarchy
            setSelectedId(id);
            // We don't set selectedItem here easily without finding it, but that's ok
        } catch (err) {
            log(`Error adding chapter: ${err.message}`);
        }
    };

    const addScene = async (parentId) => {
        const id = crypto.randomUUID();
        const title = "New Scene";
        const metadata = JSON.stringify({ parent_id: parentId });
        try {
            await db.execute(
                "INSERT INTO documents (id, project_id, title, content, document_type, metadata, created_at, updated_at, checksum) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, ?)",
                [id, 'default-project', title, '', 'scene', metadata, '']
            );
            fetchDocuments();
            // Auto-expand parent
            setExpandedItems(prev => new Set([...prev, parentId]));
        } catch (err) {
            log(`Error adding scene: ${err.message}`);
        }
    };

    const moveItem = async (draggedId, targetItem) => {
        if (!draggedId || !targetItem) return;
        if (draggedId === targetItem.id) return;

        let newParentId = targetItem.id;
        let insertAfterId = null; // If null, append to end. If set, insert after this ID.

        // Strategy:
        // 1. If target is a Container (Folder/Chapter/Manuscript/Unassigned), we move INSIDE it (append to end).
        // 2. If target is a Scene, we move to its PARENT, and insert AFTER the target scene.

        if (targetItem.type === 'scene') {
            newParentId = targetItem.metadata?.parent_id || 'unassigned';
            insertAfterId = targetItem.id;
        } else {
            // It's a container, so we append to it.
            // Exception: If we drag a Chapter onto another Chapter, do we nest? 
            // Current logic: Yes, nest inside.
            newParentId = targetItem.id;
            insertAfterId = null; // Append
        }

        // log(`[Hierarchy] Moving ${draggedId} to ${newParentId} (Insert After: ${insertAfterId})`);

        try {
            // 1. Get all current children of the new parent to determine order
            // We need to fetch them to re-index.
            // We can reuse the logic from fetchDocuments but scoped? 
            // Simpler: Fetch all docs, filter by parent.
            const result = await db.query("SELECT id, metadata FROM documents");
            if (!result.data) return;

            const allDocs = result.data.map(d => {
                let metadata = {};
                try {
                    if (d.metadata) {
                        let cleanMeta = d.metadata;
                        if (typeof cleanMeta === 'string') {
                            cleanMeta = cleanMeta.replace(/\\"/g, '"');
                            if (cleanMeta.startsWith('"') && cleanMeta.endsWith('"')) {
                                cleanMeta = cleanMeta.slice(1, -1);
                            }
                        }
                        metadata = JSON.parse(cleanMeta);
                    }
                } catch (e) {
                    // log(`[Hierarchy] Metadata parse error for ${d.id}: ${e.message}. Raw: ${d.metadata}`);
                    metadata = {};
                }
                return {
                    id: d.id,
                    metadata
                };
            });

            // Filter siblings of the NEW parent
            // Exclude the dragged item from the current list (it might be there if moving within same parent)
            let siblings = allDocs.filter(d => d.metadata.parent_id === newParentId && d.id !== draggedId);

            // Sort siblings by current order to maintain stability
            siblings.sort((a, b) => (a.metadata.order ?? 9999) - (b.metadata.order ?? 9999));

            // Insert dragged item
            const draggedDoc = allDocs.find(d => d.id === draggedId);
            if (!draggedDoc) return; // Should not happen

            // Update dragged doc's parent
            draggedDoc.metadata.parent_id = newParentId;

            const newSiblings = [];
            if (insertAfterId) {
                let inserted = false;
                for (const sibling of siblings) {
                    newSiblings.push(sibling);
                    if (sibling.id === insertAfterId) {
                        newSiblings.push(draggedDoc);
                        inserted = true;
                    }
                }
                // Fallback if target not found (shouldn't happen)
                if (!inserted) newSiblings.push(draggedDoc);
            } else {
                // Append to end
                newSiblings.push(...siblings);
                newSiblings.push(draggedDoc);
            }

            // 2. Update all siblings with new order
            // We use a transaction or batch updates if possible. 
            // For now, sequential updates.
            for (let i = 0; i < newSiblings.length; i++) {
                const doc = newSiblings[i];
                doc.metadata.order = i;
                await db.execute(
                    "UPDATE documents SET metadata = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
                    [JSON.stringify(doc.metadata), doc.id]
                );
            }

            fetchDocuments();
            setExpandedItems(prev => new Set([...prev, newParentId]));

        } catch (err) {
            log(`Error moving item: ${err.message}`);
        }
    };

    const handleToolbarAddScene = () => {
        let parentId = 'unassigned';
        if (selectedItem) {
            if (selectedItem.type === 'chapter' || selectedItem.type === 'folder') {
                parentId = selectedItem.id;
            } else if (selectedItem.type === 'scene') {
                // Add as sibling
                parentId = selectedItem.metadata?.parent_id || 'unassigned';
            }
        }
        addScene(parentId);
    };

    const requestDelete = (id) => {
        if (!id || id === 'manuscript' || id === 'unassigned') return;
        setItemToDelete(id);
        setIsDeleteModalOpen(true);
    };

    const confirmDelete = async () => {
        if (!itemToDelete) return;
        try {
            log(`[Hierarchy] Deleting item: ${itemToDelete}`);
            await db.execute("DELETE FROM documents WHERE id = ?", [itemToDelete]);
            setIsDeleteModalOpen(false);
            setItemToDelete(null);
            await fetchDocuments();
            if (selectedId === itemToDelete) {
                setSelectedId(null);
                setSelectedItem(null);
            }
        } catch (err) {
            log(`Error deleting item: ${err.message}`);
        }
    };

    return (
        <div
            className="flex flex-col h-full relative"
            style={{ backgroundColor: 'var(--bg-primary)', color: 'var(--text-primary)' }}
            onDragOver={(e) => {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'move';
            }}
            onDrop={(e) => {
                e.preventDefault();
                const draggedId = e.dataTransfer.getData('text/plain');
                // Default to Unassigned if dropped on empty space
                if (draggedId) {
                    // Pass a fake target item representing Unassigned
                    moveItem(draggedId, { id: 'unassigned', type: 'folder', title: 'Unassigned' });
                }
            }}
        >
            <div className="p-2 border-b flex items-center gap-2" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-secondary)' }}>
                <span className="font-semibold text-sm mr-2">Hierarchy</span>
                <div className="flex gap-1">
                    <button onClick={openCreateChapterModal} className="p-1 hover:bg-gray-200 rounded" title="New Chapter">
                        <Plus size={14} />
                    </button>
                    <button onClick={handleToolbarAddScene} className="p-1 hover:bg-gray-200 rounded" title="New Scene">
                        <FilePlus size={14} />
                    </button>
                    <button
                        onClick={() => requestDelete(selectedId)}
                        className={`p-1 rounded ${selectedId && selectedId !== 'manuscript' && selectedId !== 'unassigned' ? 'hover:bg-red-200 text-red-500' : 'text-gray-300 cursor-not-allowed'}`}
                        title="Delete Selected"
                        disabled={!selectedId || selectedId === 'manuscript' || selectedId === 'unassigned'}
                    >
                        <Trash2 size={14} />
                    </button>
                    <button onClick={fetchDocuments} className="p-1 hover:bg-gray-200 rounded" title="Refresh">
                        <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
                    </button>
                </div>
            </div>
            <div className="flex-1 overflow-y-auto p-2">
                {items.map(item => (
                    <HierarchyItem
                        key={item.id}
                        item={item}
                        onSelect={handleSelect}
                        expandedItems={expandedItems}
                        toggleExpand={toggleExpand}
                        onAddScene={addScene}
                        onDelete={requestDelete}
                        onMove={moveItem}
                        selectedId={selectedId}
                        dragOverId={dragOverId}
                        setDragOverId={setDragOverId}
                    />
                ))}
            </div>

            {/* Create Chapter Modal */}
            {isCreateModalOpen && (
                <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div className="bg-white p-4 rounded shadow-lg w-64" style={{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-primary)', border: '1px solid var(--border-color)' }}>
                        <h3 className="text-sm font-semibold mb-2">New Chapter</h3>
                        <input
                            type="text"
                            value={newChapterTitle}
                            onChange={(e) => setNewChapterTitle(e.target.value)}
                            className="w-full p-1 border rounded mb-3 text-sm"
                            placeholder="Chapter Title"
                            autoFocus
                            onKeyDown={(e) => {
                                if (e.key === 'Enter') confirmCreateChapter();
                                if (e.key === 'Escape') setIsCreateModalOpen(false);
                            }}
                            style={{ backgroundColor: 'var(--bg-primary)', color: 'var(--text-primary)', borderColor: 'var(--border-color)' }}
                        />
                        <div className="flex justify-end gap-2">
                            <button
                                onClick={() => setIsCreateModalOpen(false)}
                                className="px-2 py-1 text-xs rounded hover:bg-gray-200"
                                style={{ color: 'var(--text-secondary)' }}
                            >
                                Cancel
                            </button>
                            <button
                                onClick={confirmCreateChapter}
                                className="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600"
                            >
                                Create
                            </button>
                        </div>
                    </div>
                </div>
            )}

            {/* Delete Confirmation Modal */}
            {isDeleteModalOpen && (
                <div className="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div className="bg-white p-4 rounded shadow-lg w-64" style={{ backgroundColor: 'var(--bg-secondary)', color: 'var(--text-primary)', border: '1px solid var(--border-color)' }}>
                        <h3 className="text-sm font-semibold mb-2">Confirm Delete</h3>
                        <p className="text-xs mb-4">Are you sure you want to delete this item?</p>
                        <div className="flex justify-end gap-2">
                            <button
                                onClick={() => setIsDeleteModalOpen(false)}
                                className="px-2 py-1 text-xs rounded hover:bg-gray-200"
                                style={{ color: 'var(--text-secondary)' }}
                            >
                                Cancel
                            </button>
                            <button
                                onClick={confirmDelete}
                                className="px-2 py-1 text-xs bg-red-500 text-white rounded hover:bg-red-600"
                            >
                                Delete
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default Hierarchy;
