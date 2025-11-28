import React, { createContext, useState, useEffect, useContext } from 'react';

const ThemeContext = createContext();

export const useTheme = () => useContext(ThemeContext);

export const ThemeProvider = ({ children }) => {
    // Default to 'dark' if no theme is saved
    const [theme, setTheme] = useState(localStorage.getItem('app-theme') || 'dark');

    useEffect(() => {
        // Apply theme class to document element
        const root = document.documentElement;

        // Remove all known theme classes
        root.classList.remove('theme-light', 'theme-dark', 'theme-warm', 'theme-terminal-green', 'theme-terminal-amber');

        // Add current theme class
        root.classList.add(`theme-${theme}`);

        // Save to local storage
        localStorage.setItem('app-theme', theme);
    }, [theme]);

    const value = {
        theme,
        setTheme,
        themes: [
            { id: 'light', name: 'Light' },
            { id: 'dark', name: 'Dark' },
            { id: 'warm', name: 'Warm' },
            { id: 'terminal-green', name: 'Terminal Green' },
            { id: 'terminal-amber', name: 'Terminal Amber' },
        ]
    };

    return (
        <ThemeContext.Provider value={value}>
            {children}
        </ThemeContext.Provider>
    );
};
