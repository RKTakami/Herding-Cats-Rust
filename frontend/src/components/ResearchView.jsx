import React from 'react';

const ResearchView = () => {
    return (
        <div className="p-6">
            <h2 className="text-2xl font-bold mb-4">Research</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h3 className="font-semibold mb-2">Characters</h3>
                    <p className="text-gray-400 text-sm">Manage your story characters.</p>
                </div>
                <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h3 className="font-semibold mb-2">Locations</h3>
                    <p className="text-gray-400 text-sm">Map out your world.</p>
                </div>
                <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h3 className="font-semibold mb-2">Timeline</h3>
                    <p className="text-gray-400 text-sm">Track events chronologically.</p>
                </div>
            </div>
        </div>
    );
};

export default ResearchView;
