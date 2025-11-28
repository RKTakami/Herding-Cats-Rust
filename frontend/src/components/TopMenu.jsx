import React, { useState, useEffect, useRef } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { app, log } from '../api/ipc';
import { ChevronDown, File, Folder, PenTool, Eye, HelpCircle } from 'lucide-react';

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
                className={`flex items-center space-x-1 px-3 py-1.5 rounded hover:bg-gray-700 text-sm ${isOpen ? 'bg-gray-700' : ''}`}
                onClick={() => setIsOpen(!isOpen)}
            >
                {Icon && <Icon size={14} className="mr-1" />}
                <span>{label}</span>
                <ChevronDown size={12} className="opacity-50" />
            </button>

            {isOpen && (
                <div className="absolute top-full left-0 mt-1 w-48 bg-gray-800 border border-gray-700 rounded-lg shadow-xl z-50 py-1">
                    {items.map((item, index) => (
                        item.type === 'separator' ? (
                            <div key={index} className="h-px bg-gray-700 my-1" />
                        ) : (
                            <button
                                key={index}
                                className="w-full text-left px-4 py-2 text-sm hover:bg-gray-700 flex items-center justify-between group"
                                onClick={() => {
                                    item.action();
                                    setIsOpen(false);
                                }}
                            >
                                <span>{item.label}</span>
                                {item.shortcut && <span className="text-xs text-gray-500 group-hover:text-gray-400">{item.shortcut}</span>}
                            </button>
                        )
                    ))}
                </div>
            )}
        </div>
    );
};

const TopMenu = () => {
    const navigate = useNavigate();

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
            ]
        },
        {
            label: 'View',
            icon: Eye,
            items: [
                { label: 'Toggle Sidebar', action: () => log('View > Toggle Sidebar clicked') },
                { label: 'Toggle Dark Mode', action: () => log('View > Toggle Dark Mode clicked') },
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
        <div className="h-10 bg-gray-900 border-b border-gray-800 flex items-center px-2 select-none">
            <div className="flex items-center space-x-1">
                {menuStructure.map((menu, index) => (
                    <MenuDropdown key={index} {...menu} />
                ))}
            </div>
        </div>
    );
};

export default TopMenu;
