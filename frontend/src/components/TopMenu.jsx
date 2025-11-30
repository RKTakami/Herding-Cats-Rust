import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { app, log } from '../api/ipc';
import {
    File, Folder, PenTool, Eye, HelpCircle
} from 'lucide-react';

const MenuDropdown = ({ label, icon: Icon, items }) => {
    const [isOpen, setIsOpen] = useState(false);
    const dropdownRef = useRef(null);

    useEffect(() => {
        const handleClickOutside = (event) => {
            if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
                setIsOpen(false);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => document.removeEventListener('mousedown', handleClickOutside);
    }, []);

    return (
        <div className="relative" ref={dropdownRef}>
            <button
                className={`flex items-center rounded bg-transparent hover:text-white transition-colors font-medium ${isOpen ? 'text-blue-400' : ''}`}
                style={{
                    color: 'var(--text-primary)',
                    fontSize: '14px',
                    padding: '4px 12px',
                    WebkitAppRegion: 'no-drag'
                }}
                onClick={() => setIsOpen(!isOpen)}
            >
                <span>{label}</span>
            </button>

            {isOpen && (
                <div className="absolute top-full left-0 mt-1 w-48 rounded-lg shadow-xl z-50 py-1" style={{ backgroundColor: 'var(--bg-secondary)', border: '1px solid var(--border-color)' }}>
                    {items.map((item, index) => (
                        item.type === 'separator' ? (
                            <div key={index} className="h-px my-1" style={{ backgroundColor: 'var(--border-color)' }} />
                        ) : (
                            <button
                                key={index}
                                className="w-full text-left px-4 py-1 text-sm bg-transparent flex items-center justify-between group transition-colors"
                                style={{ color: 'var(--text-primary)' }}
                                onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--bg-tertiary)'}
                                onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                                onClick={() => {
                                    item.action();
                                    setIsOpen(false);
                                }}
                            >
                                <span>{item.label}</span>
                                {item.shortcut && <span className="text-xs" style={{ color: 'var(--text-secondary)' }}>{item.shortcut}</span>}
                            </button>
                        )
                    ))}
                </div>
            )}
        </div>
    );
};

import { useTheme } from '../contexts/ThemeContext';

import { X, Minus, Square } from 'lucide-react';

const TopMenu = () => {
    const navigate = useNavigate();
    const { theme, setTheme, themes } = useTheme();

    const handleExit = async () => {
        log('Exiting application...');
        await app.exit();
    };

    const menuStructure = [
        {
            label: 'File',
            icon: File,
            items: [
                { label: 'New', action: () => log('File > New clicked'), shortcut: '⌘N' },
                { label: 'Open', action: () => log('File > Open clicked'), shortcut: '⌘O' },
                { label: 'Save', action: () => log('File > Save clicked'), shortcut: '⌘S' },
                { type: 'separator' },
                { label: 'Delete', action: () => log('File > Delete clicked') },
                { type: 'separator' },
                { label: 'Exit', action: handleExit, shortcut: '⌘Q' },
            ]
        },
        {
            label: 'Project',
            icon: Folder,
            items: [
                { label: 'New Project', action: () => log('Project > New clicked') },
                { label: 'Open Project', action: () => log('Project > Open clicked') },
                { label: 'Save Project', action: () => log('Project > Save clicked') },
                { type: 'separator' },
                { label: 'Import Project', action: () => log('Project > Import clicked') },
            ]
        },
        {
            label: 'Tools',
            icon: PenTool,
            items: [
                { label: 'Hierarchy', action: () => app.openTool('hierarchy') },
                { label: 'Codex', action: () => app.openTool('codex') },
                { label: 'Plot', action: () => app.openTool('plot') },
                { label: 'Notes', action: () => app.openTool('notes') },
                { label: 'Research', action: () => app.openTool('research') },
                { label: 'Mindmap', action: () => app.openTool('mindmap') },
                { label: 'Brainstorm', action: () => app.openTool('brainstorm') },
                { label: 'Concept Map', action: () => app.openTool('concept-map') },
                { label: 'Flow Chart', action: () => app.openTool('flow-chart') },
            ]
        },
        {
            label: 'View',
            icon: Eye,
            items: [
                { label: 'Toggle Sidebar', action: () => log('View > Toggle Sidebar clicked') },
                { type: 'separator' },
                ...themes.map(t => ({
                    label: `Theme: ${t.name}`,
                    action: () => setTheme(t.id),
                    shortcut: theme === t.id ? '✓' : ''
                }))
            ]
        },
        {
            label: 'Help',
            icon: HelpCircle,
            items: [
                { label: 'About', action: () => log('Help > About clicked') },
                { label: 'Documentation', action: () => log('Help > Documentation clicked') },
            ]
        }
    ];

    return (
        <div className="w-full flex items-center select-none" style={{
            backgroundColor: 'var(--bg-primary)',
            borderBottom: '1px solid var(--border-color)',
            height: '30px',
            padding: '0 16px',
            gap: '20px',
            zIndex: 40,
            // WebkitAppRegion: 'drag' // Removed drag, now handled by TitleBar
        }}>
            {/* Traffic Lights removed - moved to TitleBar */}

            <div className="flex items-center w-full" style={{ gap: '20px' }}>
                {menuStructure.map((menu, index) => (
                    <MenuDropdown key={index} {...menu} />
                ))}
            </div>
        </div>
    );
};

export default TopMenu;
