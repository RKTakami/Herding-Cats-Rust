import React, { createContext, useState, useContext } from 'react';

const StatsContext = createContext();

export const useStats = () => useContext(StatsContext);

export const StatsProvider = ({ children }) => {
    const [wordCount, setWordCount] = useState(0);

    const value = {
        wordCount,
        setWordCount
    };

    return (
        <StatsContext.Provider value={value}>
            {children}
        </StatsContext.Provider>
    );
};
