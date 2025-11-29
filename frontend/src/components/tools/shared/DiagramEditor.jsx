import React, { useState, useRef, useEffect, useCallback } from 'react';

const Resizer = ({ x, y, onResizeStart }) => (
  <rect
    x={x}
    y={y}
    width="10"
    height="10"
    fill="var(--accent-color)"
    className="cursor-se-resize"
    onMouseDown={(e) => {
      e.stopPropagation();
      onResizeStart(e);
    }}
  />
);

const ConnectorHandle = ({ x, y, onConnectStart }) => (
    <circle
        cx={x}
        cy={y}
        r="6"
        fill="#10b981"
        className="cursor-crosshair hover:fill-green-400"
        onMouseDown={(e) => {
            e.stopPropagation();
            onConnectStart(e);
        }}
    />
);

export const DefaultNode = ({ node, selected }) => (
    <>
        <rect
            width={node.width}
            height={node.height}
            fill="var(--bg-tertiary)"
            stroke={selected ? "var(--accent-color)" : "var(--border-color)"}
            strokeWidth={selected ? 2 : 1}
            rx="4"
        />
         <foreignObject width={node.width} height={node.height} className="pointer-events-none">
            <div className="flex items-center justify-center h-full text-sm p-2 text-center break-words leading-tight select-none" style={{ color: "var(--text-primary)" }}>
                {node.text}
            </div>
        </foreignObject>
    </>
);

const DiagramEditor = ({
    initialNodes = [],
    initialEdges = [],
    nodeRenderers = {},
    palette = [],
    toolName = 'Diagram'
}) => {
    const [nodes, setNodes] = useState(initialNodes);
    const [edges, setEdges] = useState(initialEdges);
    const [selection, setSelection] = useState(null);
    const [tempEdge, setTempEdge] = useState(null);

    const svgRef = useRef(null);
    const dragInfo = useRef(null);

    const getMousePos = (e) => {
        const rect = svgRef.current.getBoundingClientRect();
        return {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };
    };

    const handleMouseDown = useCallback((e, nodeId, type = 'move') => {
        e.stopPropagation();
        if (e.button !== 0) return;

        const { x, y } = getMousePos(e);
        const node = nodes.find(n => n.id === nodeId);

        if (type === 'move') {
            setSelection(nodeId);
            dragInfo.current = {
                type: 'move',
                id: nodeId,
                startX: x,
                startY: y,
                initialX: node.x,
                initialY: node.y
            };
        } else if (type === 'resize') {
             dragInfo.current = {
                type: 'resize',
                id: nodeId,
                startX: x,
                startY: y,
                initialWidth: node.width,
                initialHeight: node.height
            };
        } else if (type === 'connect') {
             dragInfo.current = {
                type: 'connect',
                id: nodeId,
                startX: x,
                startY: y
            };
            setTempEdge({ x1: node.x + node.width/2, y1: node.y + node.height/2, x2: x, y2: y });
        }
    }, [nodes]);

    const handleMouseMove = useCallback((e) => {
        if (!dragInfo.current) return;

        const { x, y } = getMousePos(e);
        const info = dragInfo.current;

        if (info.type === 'move') {
            const dx = x - info.startX;
            const dy = y - info.startY;
            setNodes(ns => ns.map(n =>
                n.id === info.id ? { ...n, x: info.initialX + dx, y: info.initialY + dy } : n
            ));
        } else if (info.type === 'resize') {
            const dx = x - info.startX;
            const dy = y - info.startY;
             setNodes(ns => ns.map(n =>
                n.id === info.id ? { ...n, width: Math.max(50, info.initialWidth + dx), height: Math.max(30, info.initialHeight + dy) } : n
            ));
        } else if (info.type === 'connect') {
             const source = nodes.find(n => n.id === info.id);
             if(source) {
                  setTempEdge({
                      x1: source.x + source.width/2,
                      y1: source.y + source.height/2,
                      x2: x,
                      y2: y
                  });
             }
        }
    }, [nodes]);

    const handleMouseUp = useCallback((e) => {
        if (!dragInfo.current) return;

        if (dragInfo.current.type === 'connect') {
             const { x, y } = getMousePos(e);
             const targetNode = nodes.find(n =>
                 x >= n.x && x <= n.x + n.width &&
                 y >= n.y && y <= n.y + n.height &&
                 n.id !== dragInfo.current.id
             );

             if (targetNode) {
                 setEdges(es => [...es, {
                     id: Date.now().toString(),
                     from: dragInfo.current.id,
                     to: targetNode.id
                 }]);
             }
             setTempEdge(null);
        }

        dragInfo.current = null;
    }, [nodes]);

    useEffect(() => {
        const handleGlobalMove = (e) => handleMouseMove(e);
        const handleGlobalUp = (e) => handleMouseUp(e);

        window.addEventListener('mousemove', handleGlobalMove);
        window.addEventListener('mouseup', handleGlobalUp);

        return () => {
            window.removeEventListener('mousemove', handleGlobalMove);
            window.removeEventListener('mouseup', handleGlobalUp);
        };
    }, [handleMouseMove, handleMouseUp]);

    const handleDrop = (e) => {
        e.preventDefault();
        if(!svgRef.current) return;
        const rect = svgRef.current.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        const type = e.dataTransfer.getData('application/react-diagram-type');
        const text = e.dataTransfer.getData('text/plain');

        if (type) {
             const paletteItem = palette.find(p => p.type === type);
             const newNode = {
                id: Date.now().toString(),
                type,
                x,
                y,
                width: paletteItem?.defaultWidth || 120,
                height: paletteItem?.defaultHeight || 60,
                text: text || "New Node"
             };
             setNodes(prev => [...prev, newNode]);
        } else if (text) {
             const newNode = {
                id: Date.now().toString(),
                type: palette[0]?.type || 'default',
                x,
                y,
                width: 150,
                height: 80,
                text: text
             };
             setNodes(prev => [...prev, newNode]);
        }
    };

    return (
        <div className="flex h-full w-full" style={{ backgroundColor: "var(--bg-primary)", color: "var(--text-primary)" }}>
            <div className="w-48 border-r flex flex-col" style={{ backgroundColor: "var(--bg-secondary)", borderColor: "var(--border-color)" }}>
                <div className="p-3 border-b font-bold" style={{ borderColor: "var(--border-color)" }}>
                    {toolName}
                </div>
                <div className="p-2 overflow-y-auto flex-1">
                    <div className="text-xs font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>Palette</div>
                    {palette.map(item => (
                        <div
                            key={item.type}
                            className="p-3 mb-2 rounded cursor-grab shadow-sm transition-colors flex items-center gap-2 select-none hover:bg-opacity-80"
                            style={{ backgroundColor: "var(--bg-tertiary)", borderColor: "var(--border-color)", borderWidth: '1px' }}
                            draggable
                            onDragStart={(e) => {
                                e.dataTransfer.setData('application/react-diagram-type', item.type);
                                e.dataTransfer.setData('text/plain', item.label);
                            }}
                        >
                            <div className="w-3 h-3 rounded-full opacity-75" style={{ backgroundColor: "var(--accent-color)" }}></div>
                            <span className="text-sm">{item.label}</span>
                        </div>
                    ))}

                    <div className="mt-4 p-3 text-xs rounded border" style={{ backgroundColor: "var(--bg-tertiary)", borderColor: "var(--border-color)", color: "var(--text-secondary)" }}>
                        <p className="mb-2 font-semibold" style={{ color: "var(--text-primary)" }}>Tips:</p>
                        <ul className="list-disc pl-3 space-y-1">
                            <li>Drag shapes from palette</li>
                            <li>Drag text from other tools</li>
                            <li>Drag <span className="text-green-400">Green</span> dot to connect</li>
                            <li>Drag <span style={{ color: "var(--accent-color)" }}>Blue</span> square to resize</li>
                        </ul>
                    </div>
                </div>
            </div>

            <div
                className="flex-1 relative overflow-hidden"
                style={{ backgroundColor: "var(--bg-primary)" }}
                onDrop={handleDrop}
                onDragOver={(e) => e.preventDefault()}
                onClick={() => setSelection(null)}
            >
                <svg
                    ref={svgRef}
                    className="w-full h-full"
                >
                    <defs>
                        <pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse">
                            <path d="M 20 0 L 0 0 0 20" fill="none" stroke="var(--border-color)" strokeWidth="0.5"/>
                        </pattern>
                        <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
                            <polygon points="0 0, 10 3.5, 0 7" fill="var(--text-secondary)" />
                        </marker>
                    </defs>
                    <rect width="100%" height="100%" fill="url(#grid)" />

                    {edges.map(edge => {
                         const from = nodes.find(n => n.id === edge.from);
                         const to = nodes.find(n => n.id === edge.to);
                         if (!from || !to) return null;

                         const x1 = from.x + from.width/2;
                         const y1 = from.y + from.height/2;
                         const x2 = to.x + to.width/2;
                         const y2 = to.y + to.height/2;

                         return (
                            <g key={edge.id}>
                                <line
                                    x1={x1} y1={y1} x2={x2} y2={y2}
                                    stroke="var(--text-secondary)"
                                    strokeWidth="2"
                                    markerEnd="url(#arrowhead)"
                                />
                            </g>
                         );
                    })}

                    {tempEdge && (
                        <line
                            x1={tempEdge.x1} y1={tempEdge.y1} x2={tempEdge.x2} y2={tempEdge.y2}
                            stroke="var(--accent-color)"
                            strokeWidth="2"
                            strokeDasharray="5,5"
                        />
                    )}

                    {nodes.map(node => {
                        const Renderer = nodeRenderers[node.type] || DefaultNode;
                        const isSelected = selection === node.id;

                        return (
                            <g
                               key={node.id}
                               transform={`translate(${node.x}, ${node.y})`}
                               onMouseDown={(e) => handleMouseDown(e, node.id, 'move')}
                               className="cursor-move"
                            >
                                <Renderer
                                    node={node}
                                    selected={isSelected}
                                />

                                {isSelected && (
                                    <>
                                        <Resizer
                                            x={node.width - 5}
                                            y={node.height - 5}
                                            onResizeStart={(e) => handleMouseDown(e, node.id, 'resize')}
                                        />
                                        <ConnectorHandle
                                            x={node.width + 10}
                                            y={node.height / 2}
                                            onConnectStart={(e) => handleMouseDown(e, node.id, 'connect')}
                                        />
                                    </>
                                )}
                            </g>
                        );
                    })}
                </svg>
            </div>
        </div>
    );
};

export default DiagramEditor;
