import React from 'react';
import { app } from '../api/ipc';

const ResizeHandle = ({ direction, cursor, style }) => (
    <div
        style={{
            position: 'absolute',
            zIndex: 100,
            cursor: cursor,
            ...style
        }}
        onMouseDown={(e) => {
            e.preventDefault();
            app.startResize(direction);
        }}
    />
);

const ResizeHandles = ({ thickness = 8 }) => {
    return (
        <>
            {/* Edges */}
            <ResizeHandle direction="North" cursor="ns-resize" style={{ top: 0, left: thickness, right: thickness, height: thickness }} />
            <ResizeHandle direction="South" cursor="ns-resize" style={{ bottom: 0, left: thickness, right: thickness, height: thickness }} />
            <ResizeHandle direction="East" cursor="ew-resize" style={{ top: thickness, bottom: thickness, right: 0, width: thickness }} />
            <ResizeHandle direction="West" cursor="ew-resize" style={{ top: thickness, bottom: thickness, left: 0, width: thickness }} />

            {/* Corners */}
            <ResizeHandle direction="NorthWest" cursor="nwse-resize" style={{ top: 0, left: 0, width: thickness, height: thickness }} />
            <ResizeHandle direction="NorthEast" cursor="nesw-resize" style={{ top: 0, right: 0, width: thickness, height: thickness }} />
            <ResizeHandle direction="SouthWest" cursor="nesw-resize" style={{ bottom: 0, left: 0, width: thickness, height: thickness }} />
            <ResizeHandle direction="SouthEast" cursor="nwse-resize" style={{ bottom: 0, right: 0, width: thickness, height: thickness }} />
        </>
    );
};

export default ResizeHandles;
