import React from 'react';
import DiagramEditor, { DefaultNode } from './shared/DiagramEditor';

const ProcessNode = DefaultNode;

const DecisionNode = ({ node, selected }) => (
    <>
        <polygon
            points={`
                ${node.width / 2},0
                ${node.width},${node.height / 2}
                ${node.width / 2},${node.height}
                0,${node.height / 2}
            `}
            fill="var(--bg-tertiary)"
            stroke={selected ? "var(--accent-color)" : "var(--border-color)"}
            strokeWidth={selected ? 2 : 1}
        />
        <foreignObject width={node.width} height={node.height} className="pointer-events-none">
            <div className="flex items-center justify-center h-full text-xs p-6 text-center leading-tight select-none" style={{ color: "var(--text-primary)" }}>
                {node.text}
            </div>
        </foreignObject>
    </>
);

const TerminatorNode = ({ node, selected }) => (
    <>
        <rect
            width={node.width}
            height={node.height}
            rx={node.height / 2}
            ry={node.height / 2}
            fill="var(--bg-tertiary)"
            stroke={selected ? "var(--accent-color)" : "var(--border-color)"}
            strokeWidth={selected ? 2 : 1}
        />
        <foreignObject width={node.width} height={node.height} className="pointer-events-none">
            <div className="flex items-center justify-center h-full text-sm p-2 text-center leading-tight select-none" style={{ color: "var(--text-primary)" }}>
                {node.text}
            </div>
        </foreignObject>
    </>
);

const IONode = ({ node, selected }) => (
    <>
        <polygon
            points={`
                ${node.width * 0.2},0
                ${node.width},0
                ${node.width * 0.8},${node.height}
                0,${node.height}
            `}
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

const FlowChart = () => {
    const palette = [
        { type: 'process', label: 'Process', defaultWidth: 120, defaultHeight: 60 },
        { type: 'decision', label: 'Decision', defaultWidth: 100, defaultHeight: 100 },
        { type: 'terminator', label: 'Start/End', defaultWidth: 120, defaultHeight: 50 },
        { type: 'io', label: 'Input/Output', defaultWidth: 120, defaultHeight: 60 },
    ];

    const nodeRenderers = {
        process: ProcessNode,
        decision: DecisionNode,
        terminator: TerminatorNode,
        io: IONode,
        default: ProcessNode
    };

    return (
        <DiagramEditor
            toolName="Flow Chart"
            palette={palette}
            nodeRenderers={nodeRenderers}
        />
    );
};

export default FlowChart;
