import React from 'react';
import DiagramEditor, { DefaultNode } from './shared/DiagramEditor';

const ConceptNode = ({ node, selected }) => (
    <>
        <ellipse
            cx={node.width / 2}
            cy={node.height / 2}
            rx={node.width / 2}
            ry={node.height / 2}
            fill="var(--bg-tertiary)"
            stroke={selected ? "var(--accent-color)" : "var(--border-color)"}
            strokeWidth={selected ? 2 : 1}
        />
        <foreignObject width={node.width} height={node.height} className="pointer-events-none">
            <div className="flex items-center justify-center h-full text-sm p-4 text-center leading-tight select-none" style={{ color: "var(--text-primary)" }}>
                {node.text}
            </div>
        </foreignObject>
    </>
);

const ConceptMap = () => {
    const palette = [
        { type: 'concept', label: 'Concept', defaultWidth: 120, defaultHeight: 80 },
        { type: 'note', label: 'Note', defaultWidth: 150, defaultHeight: 100 },
    ];

    const nodeRenderers = {
        concept: ConceptNode,
        note: DefaultNode, // Rectangle
        default: ConceptNode
    };

    return (
        <DiagramEditor
            toolName="Concept Map"
            palette={palette}
            nodeRenderers={nodeRenderers}
        />
    );
};

export default ConceptMap;
