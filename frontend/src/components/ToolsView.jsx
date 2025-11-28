import React from 'react';

const ToolsView = () => {
    return (
        <div className="p-6">
            <h2 className="text-2xl font-bold mb-4">Tools</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h3 className="font-semibold mb-2">Dictionary</h3>
                    <p className="text-gray-400 text-sm">Look up definitions and synonyms.</p>
                </div>
                <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h3 className="font-semibold mb-2">AI Assistant</h3>
                    <p className="text-gray-400 text-sm">Get help with brainstorming.</p>
                </div>
            </div>
        </div>
    );
};

export default ToolsView;
