// MAZZAROTH Sovereign API Contract Layer (PLAN.md §D)

export async function getGalaxy() {
    const res = await fetch('/api/galaxy');
    if (!res.ok) throw new Error(`getGalaxy failed: ${res.status}`);
    return await res.json();
}

export async function getStatus() {
    const res = await fetch('/api/status');
    if (!res.ok) throw new Error(`getStatus failed: ${res.status}`);
    return await res.json();
}

export async function search(query) {
    if (!query || !query.trim()) return [];
    const res = await fetch(`/api/search?q=${encodeURIComponent(query.trim())}`);
    if (!res.ok) throw new Error(`search failed: ${res.status}`);
    const results = await res.json();
    return results.map(r => ({
        chunk_id: r.chunk_id,
        doc_id: r.doc_id,
        doc_title: r.doc_title || r.title || 'Untitled Document',
        snippet: r.snippet || '',
        rank: r.rank || 0
    }));
}

export async function getModels() {
    const res = await fetch('/api/models');
    if (!res.ok) throw new Error(`models failed: ${res.status}`);
    return await res.json();
}

export async function askStream(question, onToken, onCitations, onDone, onError, model = null) {
    let doneCalled = false;
    const safeDone = () => {
        if (!doneCalled) {
            doneCalled = true;
            if (onDone) onDone();
        }
    };

    try {
        const payload = { question: question.trim() };
        if (model) payload.model = model;

        const res = await fetch('/api/ask', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        if (!res.ok) throw new Error(`ask failed: ${res.status}`);

        const reader = res.body.getReader();
        const decoder = new TextDecoder();
        let buffer = '';

        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            buffer += decoder.decode(value, { stream: true });

            const lines = buffer.split('\n');
            buffer = lines.pop() || '';

            let currentEvent = 'message';
            for (const line of lines) {
                const trimmed = line.trim();
                if (!trimmed) continue;
                if (trimmed.startsWith('event:')) {
                    currentEvent = trimmed.slice(6).trim();
                } else if (trimmed.startsWith('data:')) {
                    const dataStr = trimmed.slice(5).trim();
                    if (currentEvent === 'token') {
                        if (onToken) onToken(dataStr);
                    } else if (currentEvent === 'citations') {
                        try {
                            const parsed = JSON.parse(dataStr);
                            const citationsArray = Array.isArray(parsed) ? parsed : [parsed];
                            if (onCitations) onCitations(citationsArray);
                        } catch (e) {
                            if (onCitations) onCitations([dataStr]);
                        }
                    } else if (currentEvent === 'done') {
                        safeDone();
                    }
                }
            }
        }
        safeDone();
    } catch (err) {
        if (onError) onError(err);
    }
}

export async function createMemoryNode(nodeData) {
    const res = await fetch('/api/memory/node', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            label: nodeData.label || nodeData.name || 'Untitled',
            tier: nodeData.tier || 'personal',
            content: nodeData.content || '',
            tags: nodeData.tags || [],
            pos_x: nodeData.pos_x || 0,
            pos_y: nodeData.pos_y || 0,
            pos_z: nodeData.pos_z || 0
        })
    });
    if (!res.ok) throw new Error(`createMemoryNode failed: ${res.status}`);
    return await res.json();
}

export async function getDocument(docId) {
    const res = await fetch(`/api/read/${encodeURIComponent(docId)}`);
    if (!res.ok) throw new Error(`getDocument failed: ${res.status}`);
    return await res.json();
}

export async function getDocuments() {
    const res = await fetch('/api/documents');
    if (!res.ok) throw new Error(`getDocuments failed: ${res.status}`);
    return await res.json();
}

export async function getCategories() {
    const res = await fetch('/api/categories');
    if (!res.ok) throw new Error(`getCategories failed: ${res.status}`);
    return await res.json();
}

export async function getConstellations() {
    const res = await fetch('/api/sections/constellations');
    if (!res.ok) throw new Error(`getConstellations failed: ${res.status}`);
    return await res.json();
}

export async function getMaps() {
    const res = await fetch('/api/maps');
    if (!res.ok) throw new Error(`getMaps failed: ${res.status}`);
    return await res.json();
}

export async function startMapFetch(bbox, name, maxzoom) {
    const res = await fetch('/api/maps/fetch', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ bbox, name, maxzoom })
    });
    if (!res.ok) {
        let msg = '';
        try { msg = (await res.text()).trim(); } catch (e) {}
        throw new Error(msg || `fetch start failed: ${res.status}`);
    }
    return await res.json();
}

export async function getMapFetchStatus() {
    const res = await fetch('/api/maps/fetch');
    if (!res.ok) throw new Error(`fetch status failed: ${res.status}`);
    return await res.json();
}

export async function deleteMapRegion(filename) {
    const res = await fetch(`/api/maps/${encodeURIComponent(filename)}`, { method: 'DELETE' });
    if (!res.ok) {
        let msg = '';
        try { msg = (await res.text()).trim(); } catch (e) {}
        throw new Error(msg || `delete failed: ${res.status}`);
    }
    return true;
}

export async function getNotes() {
    const res = await fetch('/api/notes');
    if (!res.ok) throw new Error(`getNotes failed: ${res.status}`);
    return await res.json();
}

export async function createNote(title, content) {
    const res = await fetch('/api/notes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title, content })
    });
    if (!res.ok) throw new Error(`createNote failed: ${res.status}`);
    return await res.json();
}

export async function getKnowledgeTree() {
    const res = await fetch('/api/tree');
    if (!res.ok) throw new Error(`getKnowledgeTree failed: ${res.status}`);
    return await res.json();
}

export async function getSkyProjection(params = {}) {
    const searchParams = new URLSearchParams();
    if (params.lat !== undefined) searchParams.set('lat', params.lat);
    if (params.lon !== undefined) searchParams.set('lon', params.lon);
    if (params.time !== undefined) searchParams.set('time', params.time);
    if (params.radius !== undefined) searchParams.set('radius', params.radius);
    
    const qs = searchParams.toString();
    const res = await fetch(`/api/sky${qs ? '?' + qs : ''}`);
    if (!res.ok) throw new Error(`getSkyProjection failed: ${res.status}`);
    return await res.json();
}

export async function getScriptureLanguagesDetailed() {
    const res = await fetch('/api/scripture/languages/detailed');
    if (!res.ok) throw new Error(`getScriptureLanguagesDetailed failed: ${res.status}`);
    return await res.json();
}

export async function getScriptureVersions(lang = 'eng') {
    const res = await fetch(`/api/scripture/versions?lang=${encodeURIComponent(lang)}`);
    if (!res.ok) throw new Error(`getScriptureVersions failed: ${res.status}`);
    return await res.json();
}

export async function getScriptureMeta(lang = 'eng', version = 'kjv') {
    const res = await fetch(`/api/scripture/meta?lang=${encodeURIComponent(lang)}&version=${encodeURIComponent(version)}`);
    if (!res.ok) throw new Error(`getScriptureMeta failed: ${res.status}`);
    return await res.json();
}

export async function getScriptureChapter(lang = 'eng', version = 'kjv', book = 'Genesis', chapter = 1) {
    const res = await fetch(`/api/scripture/chapter?lang=${encodeURIComponent(lang)}&version=${encodeURIComponent(version)}&book=${encodeURIComponent(book)}&chapter=${encodeURIComponent(chapter)}`);
    if (!res.ok) throw new Error(`getScriptureChapter failed: ${res.status}`);
    return await res.json();
}


