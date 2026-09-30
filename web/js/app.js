// MAZZAROTH Shell: 4-Module Controller, Real Reader Content, Telemetry, and Modals
import { getStatus, getDocument, search, createMemoryNode } from './api.js';
import { 
    initGalaxy, 
    renderGalaxy, 
    resizeGalaxy, 
    recenterGalaxy, 
    zoomGalaxyIn, 
    zoomGalaxyOut, 
    setDensity, 
    toggleSynapses, 
    setDomainFilter, 
    selectGalaxyStar,
    HERO_STARS 
} from './galaxy.js';
import { 
    initConstellations, 
    renderConstellations, 
    resizeConstellations, 
    setConstMode, 
    cycleConstSeason, 
    cycleConstPosition, 
    toggleConstLines, 
    recenterConstellations,
    resetConstFilter,
    initLibrarian,
    initMap, 
    renderMap, 
    resizeMap, 
    toggleGraticule, 
    recenterMap,
    zoomMapIn,
    zoomMapOut,
    toggleDownloadPanel,
    startSelectedDownload 
} from './sections.js';

let activeView = 'galaxy';
let currentTheme = 'light';
let isWireframeActive = false;
let currentNode = null;
let currentChapterIndex = 0;
let textFontSize = 12;

const hardwareFrame = document.getElementById('hardware-frame');
const themeBtn = document.getElementById('theme-toggle-btn');
const themeLabel = document.getElementById('theme-label');
const wireframeBtn = document.getElementById('wireframe-toggle-btn');
const wireframeLabel = document.getElementById('wireframe-label');
const wireframeTags = document.querySelectorAll('.wireframe-tag');
const clockEl = document.getElementById('offline-clock');

const viewPanels = {
    galaxy: document.getElementById('view-galaxy'),
    constellations: document.getElementById('view-constellations'),
    librarian: document.getElementById('view-librarian'),
    map: document.getElementById('view-map')
};

const toolDecks = {
    galaxy: document.getElementById('tools-galaxy'),
    constellations: document.getElementById('tools-constellations'),
    librarian: document.getElementById('tools-librarian'),
    map: document.getElementById('tools-map')
};

const navCards = document.querySelectorAll('.nav-deck-item');

document.addEventListener('DOMContentLoaded', () => {
    initTheme();
    initClock();
    initTelemetry();
    setupNavigation();
    setupReaderDeck();
    setupToolbars();
    setupSearchModal();
    setupAddMemory();
    setupWireframe();

    const gCanvas = document.getElementById('galaxy-canvas');
    const cCanvas = document.getElementById('constellations-canvas');
    const mCanvas = document.getElementById('map-canvas');

    initGalaxy(gCanvas, (node) => loadNodeIntoReader(node));
    initConstellations(cCanvas, (node) => loadNodeIntoReader(node));
    initLibrarian();
    initMap(mCanvas, (node) => loadNodeIntoReader(node));

    window.addEventListener('resize', handleResize);
    setTimeout(handleResize, 150);

    showToast('MAZZAROTH Sovereign Vault Online');
});

function initTheme() {
    const saved = localStorage.getItem('mazzaroth-theme') || 'dark';
    applyTheme(saved);

    if (themeBtn) {
        themeBtn.addEventListener('click', () => {
            const next = currentTheme === 'light' ? 'dark' : 'light';
            applyTheme(next);
            showToast(`Theme: ${next.toUpperCase()}`);
        });
    }
}

function applyTheme(theme) {
    currentTheme = theme;
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('mazzaroth-theme', theme);

    if (themeLabel) {
        themeLabel.innerText = theme === 'light' ? 'LIGHT MODE' : 'DARK MODE';
    }

    const icon = document.getElementById('theme-icon');
    if (icon) {
        if (theme === 'dark') {
            icon.innerHTML = `<path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>`;
        } else {
            icon.innerHTML = `
                <circle cx="12" cy="12" r="5"></circle>
                <line x1="12" y1="1" x2="12" y2="3"></line>
                <line x1="12" y1="21" x2="12" y2="23"></line>
                <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                <line x1="1" y1="12" x2="3" y2="12"></line>
                <line x1="21" y1="12" x2="23" y2="12"></line>
                <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            `;
        }
    }

    renderGalaxy();
    renderConstellations();
    renderMap();
}

function setupNavigation() {
    navCards.forEach(card => {
        card.addEventListener('click', () => {
            const target = card.getAttribute('data-view');
            if (target === activeView) return;

            activeView = target;

            navCards.forEach(c => {
                c.classList.remove('active-nav');
                c.style.borderColor = 'var(--border-subtle)';
                c.style.backgroundColor = 'var(--panel-bg)';
            });
            card.classList.add('active-nav');
            card.style.borderColor = 'var(--panel-border)';
            card.style.backgroundColor = 'var(--panel-bg-subtle)';

            Object.keys(viewPanels).forEach(key => {
                if (viewPanels[key]) {
                    viewPanels[key].classList.toggle('hidden', key !== target);
                    viewPanels[key].classList.toggle('block', key === target && key !== 'librarian');
                    viewPanels[key].classList.toggle('flex', key === target && key === 'librarian');
                }
            });

            Object.keys(toolDecks).forEach(key => {
                if (toolDecks[key]) {
                    toolDecks[key].classList.toggle('hidden', key !== target);
                    toolDecks[key].classList.toggle('flex', key === target);
                }
            });

            handleResize();
            showToast(`View: ${target.toUpperCase()}`);
        });
    });
}

function setupReaderDeck() {
    const decBtn = document.getElementById('btn-font-dec');
    const incBtn = document.getElementById('btn-font-inc');
    const copyBtn = document.getElementById('btn-copy-payload');
    const prevBtn = document.getElementById('btn-chapter-prev');
    const nextBtn = document.getElementById('btn-chapter-next');
    const lockBtn = document.getElementById('btn-target-lock');

    if (decBtn) {
        decBtn.addEventListener('click', () => {
            textFontSize = Math.max(10, textFontSize - 1);
            updateReaderFontSize();
        });
    }

    if (incBtn) {
        incBtn.addEventListener('click', () => {
            textFontSize = Math.min(18, textFontSize + 1);
            updateReaderFontSize();
        });
    }

    if (copyBtn) {
        copyBtn.addEventListener('click', () => {
            const box = document.getElementById('reader-content-box');
            if (box) {
                navigator.clipboard.writeText(box.innerText);
                showToast('Reader text copied to clipboard');
            }
        });
    }

    if (prevBtn) {
        prevBtn.addEventListener('click', () => {
            if (!currentNode || !currentNode.chapters) return;
            if (currentChapterIndex > 0) {
                currentChapterIndex--;
                renderChapterBody();
            }
        });
    }

    if (nextBtn) {
        nextBtn.addEventListener('click', () => {
            if (!currentNode || !currentNode.chapters) return;
            if (currentChapterIndex < currentNode.chapters.length - 1) {
                currentChapterIndex++;
                renderChapterBody();
            }
        });
    }

    if (lockBtn) {
        lockBtn.addEventListener('click', () => {
            if (currentNode) {
                selectGalaxyStar(currentNode.id);
                showToast(`Locked: ${currentNode.name || currentNode.id}`);
            }
        });
    }
}

export function loadNodeIntoReader(node) {
    if (!node) return;
    currentNode = node;
    currentChapterIndex = 0;

    const tagEl = document.getElementById('reader-tag');
    const idEl = document.getElementById('reader-id');
    const titleEl = document.getElementById('reader-title');
    const synBox = document.getElementById('reader-synapse-pills');

    if (tagEl) tagEl.innerText = (node.tags && node.tags[0]) || node.category || node.arm || 'STAR_NODE';
    if (idEl) idEl.innerText = `#${node.id || 'NEXUS-0'}`;
    if (titleEl) titleEl.innerText = node.name || node.label || 'Prime Core Nexus';

    if (synBox) {
        synBox.innerHTML = '';
        const conns = node.connections || ["NEXUS-0"];
        conns.forEach(targetId => {
            const btn = document.createElement('button');
            btn.className = 'px-1.5 py-0.5 rounded border text-[8px] font-mono hover:opacity-80 transition-opacity';
            btn.style.backgroundColor = 'var(--panel-bg-subtle)';
            btn.style.borderColor = 'var(--border-subtle)';
            btn.style.color = 'var(--text-main)';
            btn.innerText = targetId;
            btn.addEventListener('click', () => {
                selectGalaxyStar(targetId);
            });
            synBox.appendChild(btn);
        });
    }

    renderChapterBody();

    // Map real backend document structure (/api/read/{doc_id})
    const docLookupId = node.doc_id || node.id;
    if (docLookupId && docLookupId !== 'NEXUS-0') {
        getDocument(docLookupId).then(doc => {
            if (doc && doc.structure && Array.isArray(doc.structure) && doc.structure.length > 0) {
                currentNode.name = doc.title || currentNode.name;
                currentNode.category = (doc.category || currentNode.category || '').toUpperCase();
                currentNode.provenance = doc.provenance;
                currentNode.chapters = doc.structure.map(ch => {
                    const header = ch.title ? `${ch.title}\n\n` : '';
                    const body = (ch.sections || []).map(s => {
                        const secTitle = s.title ? `${s.title}\n` : '';
                        return secTitle + s.text;
                    }).join('\n\n');
                    return (header + body).trim();
                });
                currentChapterIndex = 0;

                if (titleEl) titleEl.innerText = currentNode.name;
                if (tagEl) tagEl.innerText = currentNode.category;
                renderChapterBody();
            }
        }).catch(() => {});
    }
}

function renderChapterBody() {
    const box = document.getElementById('reader-content-box');
    const counter = document.getElementById('chapter-counter');
    if (!box || !currentNode) return;

    const chapters = currentNode.chapters || [
        "Galactic epicenter of the Mazzaroth repository. Sovereign root node orchestrating cataloged memory stars across 5 logarithmic spiral arms."
    ];

    const total = chapters.length;
    if (counter) counter.innerText = `CH ${currentChapterIndex + 1} OF ${total}`;

    const text = chapters[currentChapterIndex] || '';
    
    // Format real provenance footer
    const provFooter = currentNode.provenance 
        ? `<div class="mt-3 pt-2 border-t font-mono text-[9px]" style="border-color: var(--border-subtle); color: var(--text-muted);">
            <div><strong>PROVENANCE:</strong> ${escapeHtml(currentNode.provenance.publisher || currentNode.provenance.source || 'Verified Source')}</div>
            <div class="text-[8px] opacity-75">License: ${escapeHtml(currentNode.provenance.license || 'public-domain')} • Date: ${escapeHtml(currentNode.provenance.retrieved_date || '2026-09-28')}</div>
           </div>`
        : `<div class="mt-3 pt-2 border-t font-mono text-[9px]" style="border-color: var(--border-subtle); color: var(--text-muted);">PROVENANCE: LOCAL SOVEREIGN ARCHIVE</div>`;

    box.innerHTML = `<p class="text-justify font-sans leading-relaxed whitespace-pre-line">${escapeHtml(text)}</p>${provFooter}`;
    updateReaderFontSize();
}

function updateReaderFontSize() {
    const box = document.getElementById('reader-content-box');
    if (box) {
        box.style.fontSize = `${textFontSize}px`;
    }
}

function setupToolbars() {
    const densityBtn = document.getElementById('tool-density');
    let densityState = 'high';
    if (densityBtn) {
        densityBtn.addEventListener('click', () => {
            densityState = densityState === 'high' ? 'med' : (densityState === 'med' ? 'sparse' : 'high');
            setDensity(densityState);
            showToast(`Density: ${densityState.toUpperCase()}`);
        });
    }

    const synBtn = document.getElementById('tool-synapse');
    if (synBtn) {
        synBtn.addEventListener('click', () => {
            const active = toggleSynapses();
            showToast(`Synapses: ${active ? 'ON' : 'OFF'}`);
        });
    }

    const gRecenterBtn = document.getElementById('tool-recenter');
    if (gRecenterBtn) {
        gRecenterBtn.addEventListener('click', () => {
            recenterGalaxy();
            showToast('Galaxy Recentered');
        });
    }

    const zoomInBtn = document.getElementById('tool-zoom-in');
    const zoomOutBtn = document.getElementById('tool-zoom-out');
    if (zoomInBtn) {
        zoomInBtn.addEventListener('click', () => {
            zoomGalaxyIn();
        });
    }
    if (zoomOutBtn) {
        zoomOutBtn.addEventListener('click', () => {
            zoomGalaxyOut();
        });
    }

    const constModeBtn = document.getElementById('btn-const-mode');
    let cMode = 'POSTER';
    if (constModeBtn) {
        constModeBtn.addEventListener('click', () => {
            cMode = cMode === 'POSTER' ? 'SPHERE' : 'POSTER';
            constModeBtn.innerText = cMode;
            setConstMode(cMode);
            showToast(`Layout: ${cMode}`);
        });
    }

    const seasonBtn = document.getElementById('btn-season-cycle');
    if (seasonBtn) {
        seasonBtn.addEventListener('click', () => {
            const season = cycleConstSeason();
            seasonBtn.innerText = season;
            showToast(`Season: ${season}`);
        });
    }

    const posBtn = document.getElementById('btn-position-cycle');
    if (posBtn) {
        posBtn.addEventListener('click', () => {
            const pos = cycleConstPosition();
            posBtn.innerText = pos;
            showToast(`Coord: ${pos}`);
        });
    }

    const constLinesBtn = document.getElementById('tool-const-lines');
    if (constLinesBtn) {
        constLinesBtn.addEventListener('click', () => {
            const active = toggleConstLines();
            showToast(`Constellation Lines: ${active ? 'ON' : 'OFF'}`);
        });
    }

    const constRecenterBtn = document.getElementById('tool-const-recenter');
    if (constRecenterBtn) {
        constRecenterBtn.addEventListener('click', () => {
            recenterConstellations();
            showToast('Constellations Recentered');
        });
    }

    const resetFilterBtn = document.getElementById('btn-reset-const-filter');
    if (resetFilterBtn) {
        resetFilterBtn.addEventListener('click', () => {
            resetConstFilter();
            if (seasonBtn) seasonBtn.innerText = 'ALL';
            if (posBtn) posBtn.innerText = 'ALL';
            showToast('Filters Reset');
        });
    }

    const mapGratBtn = document.getElementById('tool-map-graticule');
    if (mapGratBtn) {
        mapGratBtn.addEventListener('click', () => {
            toggleGraticule();
            showToast('Graticule Toggled');
        });
    }

    const mapRecenterBtn = document.getElementById('tool-map-recenter');
    if (mapRecenterBtn) {
        mapRecenterBtn.addEventListener('click', () => {
            recenterMap();
            showToast('Map Recentered');
        });
    }

    const mapZoomInBtn = document.getElementById('tool-map-zoom-in');
    if (mapZoomInBtn) {
        mapZoomInBtn.addEventListener('click', () => zoomMapIn());
    }

    const mapZoomOutBtn = document.getElementById('tool-map-zoom-out');
    if (mapZoomOutBtn) {
        mapZoomOutBtn.addEventListener('click', () => zoomMapOut());
    }

    const mapDlBtn = document.getElementById('tool-map-download');
    if (mapDlBtn) {
        mapDlBtn.addEventListener('click', () => toggleDownloadPanel());
    }

    const mapDlClose = document.getElementById('map-download-close');
    if (mapDlClose) {
        mapDlClose.addEventListener('click', () => toggleDownloadPanel());
    }

    const mapDlGo = document.getElementById('map-download-go');
    if (mapDlGo) {
        mapDlGo.addEventListener('click', () => startSelectedDownload());
    }
}

// ⌘K Search (Nodes + BM25)
function setupSearchModal() {
    const sModal = document.getElementById('modal-search');
    const sBtn = document.getElementById('btn-star-search');
    const sClose = document.getElementById('btn-close-search');
    const sInput = document.getElementById('modal-input');
    const sResults = document.getElementById('modal-results');

    const openSearch = () => {
        if (!sModal) return;
        sModal.classList.remove('hidden');
        if (sInput) {
            sInput.value = '';
            sInput.focus();
            renderSearchResults('');
        }
    };

    const closeSearch = () => {
        if (!sModal) return;
        sModal.classList.add('hidden');
    };

    if (sBtn) sBtn.addEventListener('click', openSearch);
    if (sClose) sClose.addEventListener('click', closeSearch);

    window.addEventListener('keydown', (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
            e.preventDefault();
            if (sModal && sModal.classList.contains('hidden')) openSearch();
            else closeSearch();
        }
        if (e.key === 'Escape') closeSearch();
    });

    if (sInput) {
        sInput.addEventListener('input', async () => {
            const q = sInput.value.trim();
            renderSearchResults(q);
        });
    }

    async function renderSearchResults(q) {
        if (!sResults) return;
        sResults.innerHTML = '';

        if (!q) {
            sResults.innerHTML = `<div class="p-3 font-mono text-[10px] text-center" style="color: var(--text-muted);">Type to search database nodes and BM25 index...</div>`;
            return;
        }

        try {
            const hits = [];

            // 1. Backend BM25 chunks
            try {
                const bm25Hits = await search(q);
                if (bm25Hits && Array.isArray(bm25Hits)) {
                    bm25Hits.forEach(h => hits.push({
                        id: h.doc_id,
                        title: h.doc_title,
                        subtitle: h.snippet,
                        badge: 'BM25'
                    }));
                }
            } catch (e) {}

            if (hits.length > 0) {
                hits.slice(0, 10).forEach(hit => {
                    const row = document.createElement('div');
                    row.className = 'p-2.5 rounded-xl border flex items-center justify-between cursor-pointer hover:opacity-80 transition-all font-mono text-xs';
                    row.style.backgroundColor = 'var(--panel-bg-subtle)';
                    row.style.borderColor = 'var(--border-subtle)';
                    row.innerHTML = `
                        <div class="pr-2">
                            <div class="font-bold truncate" style="color: var(--text-main);">${escapeHtml(hit.title)}</div>
                            <div class="text-[10px] truncate" style="color: var(--text-muted);">${escapeHtml(hit.subtitle)}</div>
                        </div>
                        <span class="text-[9px] px-1.5 py-0.5 rounded border flex-shrink-0" style="border-color: var(--panel-border); color: var(--text-main);">${hit.badge}</span>
                    `;
                    row.addEventListener('click', () => {
                        loadNodeIntoReader({
                            id: hit.id,
                            name: hit.title,
                            doc_id: hit.id,
                            tags: ["SEARCH_RESULT"],
                            chapters: [hit.subtitle],
                            connections: ["NEXUS-0"]
                        });
                        closeSearch();
                    });
                    sResults.appendChild(row);
                });
            } else {
                sResults.innerHTML = `<div class="p-3 font-mono text-[10px] text-center" style="color: var(--text-muted);">No database records found for '${escapeHtml(q)}'.</div>`;
            }
        } catch (err) {
            console.error('Search error:', err);
        }
    }
}

// Add Memory Action (Truthful Toast)
function setupAddMemory() {
    const btn = document.getElementById('btn-add-memory');
    const pillAdd = document.getElementById('pill-quick-add');

    const handleAdd = async () => {
        const label = prompt("Enter Memory Star Name:");
        if (!label || !label.trim()) return;

        try {
            await createMemoryNode({ label: label.trim(), tier: 'personal', tags: ['MEMORY'] });
            showToast(`Memory star '${label}' saved to vault`);
        } catch (e) {
            showToast(`Memory save failed: ${e.message}`);
        }
    };

    if (btn) btn.addEventListener('click', handleAdd);
    if (pillAdd) pillAdd.addEventListener('click', handleAdd);
}

function setupWireframe() {
    if (wireframeBtn) {
        wireframeBtn.addEventListener('click', () => {
            isWireframeActive = !isWireframeActive;
            if (hardwareFrame) hardwareFrame.classList.toggle('wireframe-active', isWireframeActive);
            if (wireframeLabel) wireframeLabel.innerText = isWireframeActive ? 'ON' : 'OFF';
            wireframeTags.forEach(tag => tag.classList.toggle('hidden', !isWireframeActive));
            showToast(`Wireframe: ${isWireframeActive ? 'ON' : 'OFF'}`);
        });
    }
}

function initClock() {
    function updateClock() {
        if (!clockEl) return;
        const now = new Date();
        const timeStr = now.toLocaleTimeString('en-GB', { hour12: false });
        clockEl.innerText = `${timeStr} LOCAL`;
    }
    setInterval(updateClock, 1000);
    updateClock();
}

async function initTelemetry() {
    try {
        const status = await getStatus();
        const starEl = document.getElementById('spec-star-count');
        const linkEl = document.getElementById('spec-link-count');

        if (starEl && status.total_documents) starEl.innerText = `${status.total_documents.toLocaleString()} DOCS`;
        if (linkEl && status.total_chunks) linkEl.innerText = `${status.total_chunks.toLocaleString()} CHUNKS`;
    } catch (e) {}
}

function handleResize() {
    if (activeView === 'galaxy') resizeGalaxy();
    if (activeView === 'constellations') resizeConstellations();
    if (activeView === 'map') resizeMap();
}

export function showToast(msg) {
    const toast = document.getElementById('toast-message');
    const text = document.getElementById('toast-text');
    if (!toast || !text) return;

    text.innerText = msg;
    toast.classList.remove('opacity-0', 'translate-y-3');
    toast.classList.add('opacity-100', 'translate-y-0');

    setTimeout(() => {
        toast.classList.remove('opacity-100', 'translate-y-0');
        toast.classList.add('opacity-0', 'translate-y-3');
    }, 2400);
}

function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.innerText = text;
    return div.innerHTML;
}
