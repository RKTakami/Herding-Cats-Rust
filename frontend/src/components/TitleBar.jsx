import React from 'react';
import { X, Minus, Square } from 'lucide-react';
import { app } from '../api/ipc';

const TitleBar = ({ title, showControls = true }) => {
    return (
        <div className="titlebar flex items-center justify-between select-none" style={{
            height: '30px',
            backgroundColor: 'var(--bg-secondary)',
            borderBottom: '1px solid var(--border-color)',
            WebkitAppRegion: 'drag',
            zIndex: 50,
            cursor: 'default'
        }}
        >
            <div className="flex items-center gap-2" style={{ WebkitAppRegion: 'no-drag', width: '70px' }}>
                {/* Native controls will appear here */}
            </div>

            <div className="flex-1 text-center text-xs font-medium" style={{ color: 'var(--text-secondary)' }}>
                {title}
            </div>
            {/* Spacer to balance controls */}
            <div className="w-14"></div>
        </div>
    );
};

export default TitleBar;
