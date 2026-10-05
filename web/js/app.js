// MAZZAROTH Shell: 4-Module Controller, Real Reader Hierarchy Explorer, Telemetry, and Modals
import { 
    getStatus, 
    getDocument, 
    search, 
    createMemoryNode, 
    getKnowledgeTree, 
    getScriptureLanguagesDetailed, 
    getScriptureMeta 
} from './api.js';
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
    pauseGalaxyAnimation,
    resumeGalaxyAnimation,
    HERO_STARS 
} from './galaxy.js';
import { 
    initConstellations, 
    renderConstellations, 
    resizeConstellations, 
    setConstMode, 
    cycleConstMode,
    cycleConstSeason, 
    cycleConstPosition, 
    toggleConstLines, 
    recenterConstellations,
    resetConstFilter,
    zoomConstIn,
    zoomConstOut,
    stepSkyHour,
    initLibrarian,
    initMap, 
    renderMap, 
    resizeMap, 
    toggleGraticule, 
    recenterMap,
    zoomMapIn,
    zoomMapOut,
    toggleDownloadPanel,
    startSelectedDownload,
    setMapMode,
    cycleMapMode,
    cycleMapDataset,
    openDataMatrixModal
} from './sections.js';

let activeView = 'galaxy';
let currentTheme = 'midnight-gold';
let isWireframeActive = false;
let currentNode = null;
let currentChapterIndex = 0;
let textFontSize = 12;
let cachedKnowledgeTree = null;
let cachedScriptureLangs = null;
let cachedScriptureMetaMap = {};
let hierarchyState = {
    level: 'root', // 'root', 'category', 'language', 'document', 'book'
    selectedCategory: null,
    selectedLanguage: null,
    selectedDoc: null,
    selectedBook: null,
    selectedChapter: null,
    filterText: ''
};

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

    // Load initial knowledge tree
    getKnowledgeTree().then(tree => {
        cachedKnowledgeTree = tree;
    }).catch(() => {});

    window.addEventListener('resize', handleResize);
    setTimeout(handleResize, 150);

    showToast('MAZZAROTH Sovereign Vault Online');
});

const THEME_NAMES = {
    'midnight-gold': 'MIDNIGHT GOLD',
    'synth-magenta': 'SYNTH MAGENTA',
    'obsidian-mono': 'OBSIDIAN MONO',
    'technical-paper': 'TECHNICAL PAPER',
    'dark': 'OBSIDIAN MONO',
    'light': 'TECHNICAL PAPER'
};

function initTheme() {
    const saved = localStorage.getItem('mazzaroth-theme') || 'midnight-gold';
    applyTheme(saved);

    const themeMenuBtn = document.getElementById('theme-menu-btn');
    const themeDropdown = document.getElementById('theme-dropdown');

    if (themeMenuBtn && themeDropdown) {
        themeMenuBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            themeDropdown.classList.toggle('hidden');
        });

        document.querySelectorAll('.theme-option').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                const chosen = btn.getAttribute('data-set-theme');
                if (chosen) {
                    applyTheme(chosen);
                    themeDropdown.classList.add('hidden');
                    showToast(`Theme: ${THEME_NAMES[chosen] || chosen.toUpperCase()}`);
                }
            });
        });

        document.addEventListener('click', (e) => {
            if (!themeDropdown.classList.contains('hidden') && !themeDropdown.contains(e.target) && !themeMenuBtn.contains(e.target)) {
                themeDropdown.classList.add('hidden');
            }
        });
    }
}

function applyTheme(theme) {
    currentTheme = theme;
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('mazzaroth-theme', theme);

    const activeLabel = document.getElementById('theme-active-label');
    if (activeLabel) {
        activeLabel.innerText = THEME_NAMES[theme] || theme.toUpperCase();
    }

    const activeDot = document.getElementById('theme-active-dot');
    if (activeDot) {
        const dotColors = {
            'midnight-gold': '#f59e0b',
            'synth-magenta': '#ec4899',
            'obsidian-mono': '#eef2f6',
            'technical-paper': '#0b0d11',
            'dark': '#eef2f6',
            'light': '#0b0d11'
        };
        activeDot.style.backgroundColor = dotColors[theme] || '#f59e0b';
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

            const tooltip = document.getElementById('galaxy-tooltip');
            if (tooltip) tooltip.classList.add('hidden');

            if (target === 'galaxy') {
                resumeGalaxyAnimation();
            } else {
                pauseGalaxyAnimation();
            }

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
    const expandBtn = document.getElementById('btn-reader-expand');
    const toggleSynBtn = document.getElementById('btn-toggle-synapses');

    let isReaderFullscreen = false;
    let isSynapsesExpanded = true;

    if (expandBtn) {
        expandBtn.addEventListener('click', () => {
            isReaderFullscreen = !isReaderFullscreen;
            const zoneGreen = document.getElementById('zone-green');
            const zonePurple = document.getElementById('zone-purple');
            const icon = document.getElementById('icon-reader-expand');
            if (zoneGreen) zoneGreen.classList.toggle('hidden', isReaderFullscreen);
            if (zonePurple) zonePurple.classList.toggle('hidden', isReaderFullscreen);
            if (icon) {
                icon.innerHTML = isReaderFullscreen 
                    ? '<path d="M4 14h6v6M20 10h-6V4M14 10l7-7M10 14l-7 7"/>' 
                    : '<path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7"/>';
            }
            showToast(isReaderFullscreen ? 'Reader View Maximized' : 'Reader View Restored');
        });
    }

    if (toggleSynBtn) {
        toggleSynBtn.addEventListener('click', () => {
            isSynapsesExpanded = !isSynapsesExpanded;
            const synPills = document.getElementById('reader-synapse-pills');
            const arrow = document.getElementById('synapse-toggle-arrow');
            if (synPills) synPills.classList.toggle('hidden', !isSynapsesExpanded);
            if (arrow) arrow.innerText = isSynapsesExpanded ? '▾' : '▸';
        });
    }

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
                navigator.clipboard.writeText(box.innerText).then(() => {
                    showToast('Reader text copied to clipboard');
                }).catch(() => {
                    showToast('Failed to copy to clipboard');
                });
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

let currentReaderRequestId = 0;

export function loadNodeIntoReader(node) {
    if (!node) return;

    const nodeId = String(node.id || node.doc_id || '');

    // Check for Central Core or Root
    if (nodeId === 'NEXUS-0' || nodeId === 'core:librarian' || node.isCore || node.is_nexus) {
        showHierarchyRoot();
        return;
    }

    // Check for Category stars
    if (nodeId.startsWith('category:')) {
        const catName = nodeId.replace('category:', '');
        hierarchyState.selectedCategory = catName;
        hierarchyState.level = 'category';
        hierarchyState.filterText = '';
        renderHierarchyExplorer();
        return;
    }

    // Check for Scripture Language stars
    if (nodeId.startsWith('lang:')) {
        const langCode = nodeId.replace('lang:', '');
        hierarchyState.selectedCategory = 'scripture';
        const langObj = (cachedScriptureLangs || []).find(l => l.code === langCode) || { code: langCode, name: langCode, versions: [] };
        hierarchyState.selectedLanguage = langObj;
        hierarchyState.level = 'language';
        hierarchyState.filterText = '';
        renderHierarchyExplorer();
        return;
    }

    // Check for Scripture Translation stars
    if (nodeId.startsWith('scripture:')) {
        const parts = nodeId.split(':');
        if (parts.length === 3) {
            const langCode = parts[1];
            const ver = parts[2];
            hierarchyState.selectedCategory = 'scripture';
            hierarchyState.selectedLanguage = { code: langCode, name: langCode, versions: [ver] };
            hierarchyState.selectedDoc = {
                id: nodeId,
                lang: langCode,
                version: ver,
                title: node.name || `Holy Bible (${ver.toUpperCase()})`
            };
            hierarchyState.level = 'document';
            hierarchyState.filterText = '';
            renderHierarchyExplorer();
            return;
        } else if (parts.length >= 4) {
            const langCode = parts[1];
            const ver = parts[2];
            const bookName = parts[3].replace(/_/g, ' ');
            hierarchyState.selectedCategory = 'scripture';
            hierarchyState.selectedLanguage = { code: langCode, name: langCode, versions: [ver] };
            hierarchyState.selectedDoc = { id: `scripture:${langCode}:${ver}`, lang: langCode, version: ver, title: `Holy Bible (${ver.toUpperCase()})` };
            hierarchyState.selectedBook = { name: bookName };
            loadScriptureBookIntoReader(langCode, ver, bookName, 0);
            return;
        }
    }

    currentNode = node;
    currentChapterIndex = 0;
    const reqId = ++currentReaderRequestId;

    const tagEl = document.getElementById('reader-tag');
    const idEl = document.getElementById('reader-id');
    const titleEl = document.getElementById('reader-title');
    const synBox = document.getElementById('reader-synapse-pills');
    const synContainer = document.getElementById('reader-synapses-container');
    const synBadge = document.getElementById('synapse-count-badge');
    const breadcrumbs = document.getElementById('reader-breadcrumbs');

    if (tagEl) tagEl.innerText = (node.tags && node.tags[0]) || node.category || node.arm || 'STAR_NODE';
    if (idEl) idEl.innerText = `#${node.id || 'NEXUS-0'}`;
    if (titleEl) titleEl.innerText = node.name || node.label || 'Prime Core Nexus';

    const conns = node.connections || ["NEXUS-0"];
    if (synContainer) {
        synContainer.classList.remove('hidden');
    }
    if (synBadge) {
        synBadge.innerText = `(${conns.length})`;
    }

    if (synBox) {
        synBox.innerHTML = '';
        conns.forEach(targetId => {
            const btn = document.createElement('button');
            btn.className = 'px-1.5 py-0.5 rounded border text-[8px] font-mono hover:opacity-80 transition-opacity truncate max-w-[140px]';
            btn.style.backgroundColor = 'var(--panel-bg-subtle)';
            btn.style.borderColor = 'var(--border-subtle)';
            btn.style.color = 'var(--text-main)';
            btn.innerText = targetId;
            btn.addEventListener('click', () => {
                if (targetId === 'NEXUS-0' || targetId === 'core:librarian') {
                    showHierarchyRoot();
                } else {
                    selectGalaxyStar(targetId);
                }
            });
            synBox.appendChild(btn);
        });
    }

    if (breadcrumbs) {
        breadcrumbs.classList.add('hidden');
    }

    renderChapterBody();

    // Map real backend document structure (/api/read/{doc_id})
    const docLookupId = node.doc_id || node.id;
    if (docLookupId && docLookupId !== 'NEXUS-0' && docLookupId !== 'core:librarian') {
        getDocument(docLookupId).then(doc => {
            if (reqId !== currentReaderRequestId) return;
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

export function showHierarchyRoot() {
    currentReaderRequestId++;
    hierarchyState = {
        level: 'root',
        selectedCategory: null,
        selectedLanguage: null,
        selectedDoc: null,
        selectedBook: null,
        selectedChapter: null,
        filterText: ''
    };

    const synContainer = document.getElementById('reader-synapses-container');
    if (synContainer) {
        synContainer.classList.add('hidden');
    }

    const tagEl = document.getElementById('reader-tag');
    const idEl = document.getElementById('reader-id');
    const titleEl = document.getElementById('reader-title');

    if (tagEl) tagEl.innerText = 'DATABASE_INDEX';
    if (idEl) idEl.innerText = '#NEXUS-0';
    if (titleEl) titleEl.innerText = 'Knowledge Themes & Hierarchy';

    if (!cachedKnowledgeTree || !cachedScriptureLangs) {
        Promise.all([
            cachedKnowledgeTree ? Promise.resolve(cachedKnowledgeTree) : getKnowledgeTree().catch(() => null),
            cachedScriptureLangs ? Promise.resolve(cachedScriptureLangs) : getScriptureLanguagesDetailed().catch(() => null)
        ]).then(([tree, langs]) => {
            if (tree) cachedKnowledgeTree = tree;
            if (langs) cachedScriptureLangs = langs;
            renderHierarchyExplorer();
        });
    } else {
        renderHierarchyExplorer();
    }
}

function updateBreadcrumbs() {
    const breadcrumbs = document.getElementById('reader-breadcrumbs');
    if (!breadcrumbs) return;

    breadcrumbs.classList.remove('hidden');
    breadcrumbs.innerHTML = '';

    const addCrumb = (label, onClick) => {
        const btn = document.createElement('button');
        btn.className = 'tree-breadcrumb-btn cursor-pointer font-mono text-[9px] hover:underline';
        btn.innerText = label;
        btn.style.color = 'var(--text-secondary)';
        btn.addEventListener('click', onClick);
        breadcrumbs.appendChild(btn);
    };

    const addSeparator = () => {
        const span = document.createElement('span');
        span.className = 'text-[9px] px-0.5';
        span.innerText = '›';
        span.style.color = 'var(--border-subtle)';
        breadcrumbs.appendChild(span);
    };

    addCrumb('HOME (THEMES)', () => showHierarchyRoot());

    if (hierarchyState.selectedCategory) {
        addSeparator();
        addCrumb(hierarchyState.selectedCategory.toUpperCase(), () => {
            hierarchyState.level = 'category';
            hierarchyState.selectedLanguage = null;
            hierarchyState.selectedDoc = null;
            hierarchyState.selectedBook = null;
            hierarchyState.selectedChapter = null;
            hierarchyState.filterText = '';
            renderHierarchyExplorer();
        });
    }

    if (hierarchyState.selectedLanguage) {
        addSeparator();
        const lName = typeof hierarchyState.selectedLanguage === 'object' 
            ? (hierarchyState.selectedLanguage.name || hierarchyState.selectedLanguage.code)
            : hierarchyState.selectedLanguage;
        addCrumb(lName.toUpperCase(), () => {
            hierarchyState.level = 'language';
            hierarchyState.selectedDoc = null;
            hierarchyState.selectedBook = null;
            hierarchyState.selectedChapter = null;
            hierarchyState.filterText = '';
            renderHierarchyExplorer();
        });
    }

    if (hierarchyState.selectedDoc) {
        addSeparator();
        const dTitle = hierarchyState.selectedDoc.title || hierarchyState.selectedDoc.id;
        addCrumb(dTitle.length > 20 ? dTitle.slice(0, 18) + '…' : dTitle, () => {
            hierarchyState.level = 'document';
            hierarchyState.selectedBook = null;
            hierarchyState.selectedChapter = null;
            hierarchyState.filterText = '';
            renderHierarchyExplorer();
        });
    }

    if (hierarchyState.selectedBook) {
        addSeparator();
        addCrumb(hierarchyState.selectedBook.name.toUpperCase(), () => {
            hierarchyState.level = 'book';
            hierarchyState.selectedChapter = null;
            hierarchyState.filterText = '';
            renderHierarchyExplorer();
        });
    }
}

async function loadScriptureBookIntoReader(langCode, versionCode, bookName, chapterIndex = 0) {
    const docId = `scripture:${langCode}:${versionCode}:${bookName}`;
    const box = document.getElementById('reader-content-box');
    if (box) {
        box.innerHTML = `<div class="p-6 text-center font-mono text-xs" style="color: var(--text-muted);">Loading ${escapeHtml(bookName)}...</div>`;
    }
    
    try {
        const fullDoc = await getDocument(docId);
        currentNode = {
            id: docId,
            doc_id: docId,
            name: `${fullDoc.title || bookName}`,
            category: 'SCRIPTURE',
            provenance: fullDoc.provenance,
            chapters: fullDoc.structure.map(ch => {
                const header = ch.title ? `${ch.title}\n\n` : '';
                const body = (ch.sections || []).map(s => {
                    const secTitle = s.title ? `${s.title}\n` : '';
                    return secTitle + s.text;
                }).join('\n\n');
                return (header + body).trim();
            })
        };
        currentChapterIndex = chapterIndex;

        const titleEl = document.getElementById('reader-title');
        const tagEl = document.getElementById('reader-tag');
        const idEl = document.getElementById('reader-id');
        if (titleEl) titleEl.innerText = currentNode.name;
        if (tagEl) tagEl.innerText = currentNode.category;
        if (idEl) idEl.innerText = `#${docId}`;

        hierarchyState.selectedChapter = chapterIndex + 1;
        updateBreadcrumbs();
        renderChapterBody();
    } catch (e) {
        if (box) {
            box.innerHTML = `<div class="p-6 text-center font-mono text-xs text-red-500">Failed to load scripture: ${escapeHtml(e.message)}</div>`;
        }
    }
}

async function loadDocumentIntoReader(docId, chapterIndex = 0) {
    const box = document.getElementById('reader-content-box');
    if (box) {
        box.innerHTML = `<div class="p-6 text-center font-mono text-xs" style="color: var(--text-muted);">Loading document...</div>`;
    }
    
    try {
        const fullDoc = await getDocument(docId);
        currentNode = {
            id: docId,
            doc_id: docId,
            name: fullDoc.title,
            category: (fullDoc.category || '').toUpperCase(),
            provenance: fullDoc.provenance,
            chapters: fullDoc.structure.map(ch => {
                const header = ch.title ? `${ch.title}\n\n` : '';
                const body = (ch.sections || []).map(s => {
                    const secTitle = s.title ? `${s.title}\n` : '';
                    return secTitle + s.text;
                }).join('\n\n');
                return (header + body).trim();
            })
        };
        currentChapterIndex = chapterIndex;

        const titleEl = document.getElementById('reader-title');
        const tagEl = document.getElementById('reader-tag');
        const idEl = document.getElementById('reader-id');
        if (titleEl) titleEl.innerText = currentNode.name;
        if (tagEl) tagEl.innerText = currentNode.category;
        if (idEl) idEl.innerText = `#${docId}`;

        hierarchyState.selectedChapter = chapterIndex + 1;
        updateBreadcrumbs();
        renderChapterBody();
    } catch (e) {
        if (box) {
            box.innerHTML = `<div class="p-6 text-center font-mono text-xs text-red-500">Failed to load document: ${escapeHtml(e.message)}</div>`;
        }
    }
}

function renderHierarchyExplorer() {
    const box = document.getElementById('reader-content-box');
    const counter = document.getElementById('chapter-counter');
    if (!box) return;

    updateBreadcrumbs();
    if (counter) counter.innerText = 'THEME EXPLORER';

    const tree = cachedKnowledgeTree || [
        {
            category: 'scripture',
            languages: [{ language: 'eng', documents: [{ id: 'scripture:eng:kjv', title: 'Holy Bible (KJV, English)', chapters: [] }] }]
        },
        {
            category: 'astronomy',
            languages: [{ language: 'en', documents: [{ id: 'star-navigation-handbook', title: 'Star Navigation & Celestial Lore Handbook', chapters: [] }] }]
        },
        {
            category: 'survival',
            languages: [{ language: 'en', documents: [{ id: 'wilderness-survival-guide', title: 'Wilderness Survival Guide', chapters: [] }] }]
        },
        {
            category: 'medical',
            languages: [{ language: 'en', documents: [{ id: 'emergency-medical-protocols', title: 'Emergency Medical Protocols', chapters: [] }] }]
        },
        {
            category: 'cognitive',
            languages: [{ language: 'en', documents: [{ id: 'cognitive-memory-index', title: 'Cognitive Memory Vault', chapters: [] }] }]
        }
    ];

    // Category meta info (monospace design tokens)
    const categoryIcons = {
        scripture: { 
            icon: 'S', 
            label: 'SCRIPTURE & SACRED TEXTS', 
            desc: '66 Languages, 226 Translations & Thousands of Canonical Books' 
        },
        astronomy: { 
            icon: 'A', 
            label: 'ASTRONOMY & ASTROMETRY', 
            desc: 'Star navigation handbook, celestial lore & ephemeris data' 
        },
        survival: { 
            icon: 'V', 
            label: 'SURVIVAL & EXPEDITION', 
            desc: 'Field manual, water purification, shelter & wilderness tactics' 
        },
        medical: { 
            icon: 'M', 
            label: 'EMERGENCY MEDICAL', 
            desc: 'First responder protocols, triage & wound intervention' 
        },
        cognitive: { 
            icon: 'C', 
            label: 'COGNITIVE MEMORY', 
            desc: 'Sovereign neural memory stars & contextual linkages' 
        }
    };

    // LEVEL 0: ROOT CATEGORIES
    if (hierarchyState.level === 'root') {
        const filter = (hierarchyState.filterText || '').trim().toLowerCase();
        const orderedCategories = ['scripture', 'astronomy', 'survival', 'medical', 'cognitive'];
        const renderedSet = new Set();
        const allCatNames = [];

        orderedCategories.forEach(c => allCatNames.push(c));
        tree.forEach(cat => {
            const low = cat.category.toLowerCase();
            if (!allCatNames.includes(low)) {
                allCatNames.push(low);
            }
        });

        const filteredCatNames = allCatNames.filter(catName => {
            if (!filter) return true;
            const meta = categoryIcons[catName] || { label: catName, desc: '' };
            return catName.includes(filter) ||
                   meta.label.toLowerCase().includes(filter) ||
                   meta.desc.toLowerCase().includes(filter);
        });

        let html = `
            <div class="space-y-3 font-mono">
                <div class="flex items-center justify-between">
                    <span class="text-[11px] font-bold" style="color: var(--text-main);">
                        KNOWLEDGE THEMES (${filteredCatNames.length} MATCHED)
                    </span>
                    <span class="text-[9px]" style="color: var(--text-muted);">Mazzaroth Sovereign Vault</span>
                </div>

                <div class="relative">
                    <input id="input-root-filter" type="text" placeholder="Filter knowledge themes... (e.g. scripture, astronomy, survival, medical, cognitive)"
                           value="${escapeHtml(hierarchyState.filterText || '')}"
                           class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                           style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                </div>

                <div class="grid grid-cols-1 gap-2.5 max-h-[580px] overflow-y-auto reader-scroll pr-1">
        `;

        if (filteredCatNames.length === 0) {
            html += `
                <div class="p-6 text-center text-xs" style="color: var(--text-muted);">
                    No knowledge themes matching "${escapeHtml(filter)}".
                </div>
            `;
        } else {
            const renderCatCard = (catName) => {
                const cat = tree.find(c => c.category.toLowerCase() === catName) || { category: catName, languages: [] };
                const meta = categoryIcons[catName] || {
                    icon: 'D',
                    label: catName.toUpperCase(),
                    desc: 'Indexed domain knowledge and documents'
                };

                let countLabel = '';
                if (catName === 'scripture') {
                    const langCount = cachedScriptureLangs ? cachedScriptureLangs.length : 66;
                    countLabel = `${langCount} LANGS / 226 VERSIONS ›`;
                } else {
                    const totalDocs = (cat.languages || []).reduce((acc, l) => acc + (l.documents || []).length, 0);
                    countLabel = `${totalDocs} ${totalDocs === 1 ? 'DOC' : 'DOCS'} ›`;
                }

                html += `
                    <div data-cat="${escapeHtml(cat.category || catName)}" class="tree-node-card cursor-pointer p-3 rounded-xl border flex items-center justify-between transition-all hover:scale-[1.01]"
                         style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                        <div class="flex items-center space-x-3">
                            <div class="text-xl p-2 rounded-lg border font-bold" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle);">${meta.icon}</div>
                            <div>
                                <div class="font-bold text-xs" style="color: var(--text-main);">${escapeHtml(meta.label)}</div>
                                <div class="text-[9px] mt-0.5" style="color: var(--text-secondary);">${escapeHtml(meta.desc)}</div>
                            </div>
                        </div>
                        <span class="text-[8.5px] px-2.5 py-1 rounded border font-semibold shrink-0" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                            ${countLabel}
                        </span>
                    </div>
                `;
            };

            filteredCatNames.forEach(catName => renderCatCard(catName));
        }

        html += `
                </div>
            </div>
        `;

        box.innerHTML = html;

        const filterInput = document.getElementById('input-root-filter');
        if (filterInput) {
            filterInput.addEventListener('input', (e) => {
                hierarchyState.filterText = e.target.value;
                renderHierarchyExplorer();
                const newInp = document.getElementById('input-root-filter');
                if (newInp) {
                    newInp.focus();
                    newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                }
            });
        }

        box.querySelectorAll('[data-cat]').forEach(el => {
            el.addEventListener('click', () => {
                const catName = el.getAttribute('data-cat');
                hierarchyState.selectedCategory = catName;
                hierarchyState.level = 'category';
                hierarchyState.filterText = '';
                renderHierarchyExplorer();
            });
        });
        return;
    }

    // LEVEL 1: CATEGORY (Show Languages)
    if (hierarchyState.level === 'category') {
        const catName = hierarchyState.selectedCategory.toLowerCase();

        if (catName === 'scripture') {
            const allLangs = cachedScriptureLangs || [];
            const filter = (hierarchyState.filterText || '').trim().toLowerCase();
            const filteredLangs = allLangs.filter(l => {
                if (!filter) return true;
                return (l.code && l.code.toLowerCase().includes(filter)) ||
                       (l.name && l.name.toLowerCase().includes(filter)) ||
                       (l.native_name && l.native_name.toLowerCase().includes(filter));
            });

            let html = `
                <div class="space-y-3 font-mono">
                    <div class="flex items-center justify-between">
                        <span class="text-[11px] font-bold" style="color: var(--text-main);">
                            SCRIPTURE: 66 LANGUAGES (${filteredLangs.length} MATCHED)
                        </span>
                        <button id="btn-tree-back-root" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80"
                                style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ THEMES</button>
                    </div>

                    <div class="relative">
                        <input id="input-lang-filter" type="text" placeholder="Filter 66 languages... (e.g. English, 日本語, Deutsch, Hebrew, Greek)"
                               value="${escapeHtml(hierarchyState.filterText || '')}"
                               class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                               style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 max-h-[580px] overflow-y-auto reader-scroll pr-1">
            `;

            if (filteredLangs.length === 0) {
                html += `
                    <div class="col-span-2 p-6 text-center text-xs" style="color: var(--text-muted);">
                        No languages found matching "${escapeHtml(filter)}".
                    </div>
                `;
            } else {
                filteredLangs.forEach(l => {
                    const vCount = (l.versions || []).length;
                    html += `
                        <div data-lang-code="${escapeHtml(l.code)}" class="tree-node-card cursor-pointer p-2.5 rounded-xl border flex items-center justify-between transition-all hover:scale-[1.01]"
                             style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                            <div class="flex items-center space-x-2.5 min-w-0">
                                <span class="w-9 h-7 rounded border font-bold text-[10px] flex items-center justify-center shrink-0 uppercase"
                                      style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                                    ${escapeHtml(l.code)}
                                </span>
                                <div class="min-w-0">
                                    <div class="font-bold text-xs truncate" style="color: var(--text-main);">${escapeHtml(l.name)}</div>
                                    <div class="text-[9px] truncate" style="color: var(--text-secondary);">${escapeHtml(l.native_name || l.name)}</div>
                                </div>
                            </div>
                            <span class="text-[8px] px-1.5 py-0.5 rounded border font-semibold shrink-0 ml-1"
                                  style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-muted);">
                                ${vCount} ${vCount === 1 ? 'VER' : 'VERS'} ›
                            </span>
                        </div>
                    `;
                });
            }

            html += `
                    </div>
                </div>
            `;

            box.innerHTML = html;

            const backBtn = document.getElementById('btn-tree-back-root');
            if (backBtn) backBtn.addEventListener('click', () => showHierarchyRoot());

            const filterInput = document.getElementById('input-lang-filter');
            if (filterInput) {
                filterInput.addEventListener('input', (e) => {
                    hierarchyState.filterText = e.target.value;
                    renderHierarchyExplorer();
                    const newInp = document.getElementById('input-lang-filter');
                    if (newInp) {
                        newInp.focus();
                        newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                    }
                });
            }

            box.querySelectorAll('[data-lang-code]').forEach(el => {
                el.addEventListener('click', () => {
                    const code = el.getAttribute('data-lang-code');
                    const langObj = (cachedScriptureLangs || []).find(l => l.code === code) || { code, name: code, versions: [] };
                    hierarchyState.selectedLanguage = langObj;
                    hierarchyState.level = 'language';
                    hierarchyState.filterText = '';
                    renderHierarchyExplorer();
                });
            });
            return;
        }

        // Standard non-scripture category
        const catData = tree.find(c => c.category.toLowerCase() === catName) || { category: catName, languages: [] };
        const allLangs = catData.languages || [];
        const filter = (hierarchyState.filterText || '').trim().toLowerCase();
        const filteredLangs = allLangs.filter(l => {
            if (!filter) return true;
            return (l.language && l.language.toLowerCase().includes(filter));
        });

        let html = `
            <div class="space-y-3 font-mono">
                <div class="text-[11px] leading-relaxed flex items-center justify-between" style="color: var(--text-secondary);">
                    <span>THEME: <strong style="color: var(--text-main);">${escapeHtml(hierarchyState.selectedCategory.toUpperCase())}</strong> (${filteredLangs.length} MATCHED)</span>
                    <button id="btn-tree-back-root" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80"
                            style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ THEMES</button>
                </div>

                <div class="relative">
                    <input id="input-nonscrip-lang-filter" type="text" placeholder="Filter languages / corpora... (e.g. en, english)"
                           value="${escapeHtml(hierarchyState.filterText || '')}"
                           class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                           style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                </div>

                <div class="grid grid-cols-1 gap-2 max-h-[580px] overflow-y-auto reader-scroll pr-1">
        `;

        if (filteredLangs.length === 0) {
            html += `
                <div class="p-6 text-center text-xs" style="color: var(--text-muted);">
                    No languages found matching "${escapeHtml(filter)}".
                </div>
            `;
        } else {
            filteredLangs.forEach(l => {
                const docCount = (l.documents || []).length;
                html += `
                    <div data-lang="${escapeHtml(l.language)}" class="tree-node-card cursor-pointer p-3 rounded-xl border flex items-center justify-between"
                         style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                        <div class="flex items-center space-x-3">
                            <div class="w-8 h-8 rounded-lg border flex items-center justify-center font-bold text-xs"
                                 style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                                ${escapeHtml(l.language.toUpperCase())}
                            </div>
                            <div>
                                <div class="font-bold text-xs" style="color: var(--text-main);">${l.language === 'en' ? 'English (en)' : escapeHtml(l.language.toUpperCase())}</div>
                                <div class="text-[9px]" style="color: var(--text-secondary);">${docCount} documents cataloged</div>
                            </div>
                        </div>
                        <span class="text-[9px] px-2 py-0.5 rounded border font-semibold" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                            OPEN ›
                        </span>
                    </div>
                `;
            });
        }

        html += `
                </div>
            </div>
        `;

        box.innerHTML = html;

        const backBtn = document.getElementById('btn-tree-back-root');
        if (backBtn) backBtn.addEventListener('click', () => showHierarchyRoot());

        const filterInput = document.getElementById('input-nonscrip-lang-filter');
        if (filterInput) {
            filterInput.addEventListener('input', (e) => {
                hierarchyState.filterText = e.target.value;
                renderHierarchyExplorer();
                const newInp = document.getElementById('input-nonscrip-lang-filter');
                if (newInp) {
                    newInp.focus();
                    newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                }
            });
        }

        box.querySelectorAll('[data-lang]').forEach(el => {
            el.addEventListener('click', () => {
                const langName = el.getAttribute('data-lang');
                hierarchyState.selectedLanguage = langName;
                hierarchyState.level = 'language';
                hierarchyState.filterText = '';
                renderHierarchyExplorer();
            });
        });
        return;
    }

    // LEVEL 2: LANGUAGE (Show Versions or Documents)
    if (hierarchyState.level === 'language') {
        const catName = (hierarchyState.selectedCategory || '').toLowerCase();

        if (catName === 'scripture') {
            const langObj = typeof hierarchyState.selectedLanguage === 'object'
                ? hierarchyState.selectedLanguage
                : (cachedScriptureLangs || []).find(l => l.code === hierarchyState.selectedLanguage) || { code: hierarchyState.selectedLanguage, name: hierarchyState.selectedLanguage, versions: [] };
            
            const allVersions = langObj.versions || [];
            const filter = (hierarchyState.filterText || '').trim().toLowerCase();
            const filteredVersions = allVersions.filter(v => {
                if (!filter) return true;
                return v.toLowerCase().includes(filter);
            });

            let html = `
                <div class="space-y-3 font-mono">
                    <div class="flex items-center justify-between">
                        <div>
                            <span class="text-xs font-bold" style="color: var(--text-main);">${escapeHtml(langObj.name)}</span>
                            <span class="text-[10px] ml-1" style="color: var(--text-secondary);">(${escapeHtml(langObj.native_name || langObj.code)})</span>
                        </div>
                        <button id="btn-tree-back-langs" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80"
                                style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ ALL LANGUAGES</button>
                    </div>

                    <div class="relative">
                        <input id="input-ver-filter" type="text" placeholder="Filter translations... (e.g. KJV, ASV, Geneva, Darby, Luther, Webster)"
                               value="${escapeHtml(hierarchyState.filterText || '')}"
                               class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                               style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                    </div>

                    <div class="text-[10px]" style="color: var(--text-muted);">
                        Available Bible Translations (${filteredVersions.length} of ${allVersions.length}):
                    </div>

                    <div class="grid grid-cols-1 gap-2.5 max-h-[560px] overflow-y-auto reader-scroll pr-1">
            `;

            if (filteredVersions.length === 0) {
                html += `
                    <div class="p-6 text-center text-xs" style="color: var(--text-muted);">
                        No translations matching "${escapeHtml(filter)}".
                    </div>
                `;
            } else {
                filteredVersions.forEach(ver => {
                    html += `
                        <div data-ver="${escapeHtml(ver)}" class="tree-node-card cursor-pointer p-3 rounded-xl border flex items-center justify-between transition-all hover:scale-[1.01]"
                             style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                            <div class="flex items-center space-x-3">
                                <span class="w-12 h-8 rounded border font-bold text-xs flex items-center justify-center shrink-0 uppercase font-mono"
                                      style="background-color: var(--contrast-ink); border-color: var(--panel-border); color: var(--contrast-paper);">
                                    ${escapeHtml(ver.toUpperCase())}
                                </span>
                                <div>
                                    <div class="font-bold text-xs" style="color: var(--text-main);">Holy Bible (${escapeHtml(ver.toUpperCase())})</div>
                                    <div class="text-[9px] mt-0.5" style="color: var(--text-secondary);">Canonical Scripture Translation • Sovereign Archive</div>
                                </div>
                            </div>
                            <span class="text-[9px] px-2.5 py-1 rounded border font-semibold uppercase"
                                  style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                                BOOKS ›
                            </span>
                        </div>
                    `;
                });
            }

            html += `
                    </div>
                </div>
            `;

            box.innerHTML = html;

            const backBtn = document.getElementById('btn-tree-back-langs');
            if (backBtn) {
                backBtn.addEventListener('click', () => {
                    hierarchyState.level = 'category';
                    hierarchyState.selectedLanguage = null;
                    hierarchyState.filterText = '';
                    renderHierarchyExplorer();
                });
            }

            const filterInput = document.getElementById('input-ver-filter');
            if (filterInput) {
                filterInput.addEventListener('input', (e) => {
                    hierarchyState.filterText = e.target.value;
                    renderHierarchyExplorer();
                    const newInp = document.getElementById('input-ver-filter');
                    if (newInp) {
                        newInp.focus();
                        newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                    }
                });
            }

            box.querySelectorAll('[data-ver]').forEach(el => {
                el.addEventListener('click', async () => {
                    const ver = el.getAttribute('data-ver');
                    const langCode = langObj.code;
                    hierarchyState.selectedDoc = {
                        id: `scripture:${langCode}:${ver}`,
                        lang: langCode,
                        version: ver,
                        title: `Holy Bible (${ver.toUpperCase()}, ${langObj.name})`
                    };
                    hierarchyState.level = 'document';
                    hierarchyState.filterText = '';

                    const metaKey = `${langCode}:${ver}`;
                    if (!cachedScriptureMetaMap[metaKey]) {
                        try {
                            cachedScriptureMetaMap[metaKey] = await getScriptureMeta(langCode, ver);
                        } catch (e) {}
                    }
                    renderHierarchyExplorer();
                });
            });
            return;
        }

        // Standard non-scripture category
        const catData = tree.find(c => c.category.toLowerCase() === catName) || { category: catName, languages: [] };
        const langCode = typeof hierarchyState.selectedLanguage === 'object' ? hierarchyState.selectedLanguage.language : hierarchyState.selectedLanguage;
        const langData = (catData.languages || []).find(l => l.language === langCode) || { language: langCode, documents: [] };
        const allDocs = langData.documents || [];
        const filter = (hierarchyState.filterText || '').trim().toLowerCase();
        const filteredDocs = allDocs.filter(d => {
            if (!filter) return true;
            return (d.title && d.title.toLowerCase().includes(filter)) ||
                   (d.id && d.id.toLowerCase().includes(filter)) ||
                   (d.publisher && d.publisher.toLowerCase().includes(filter));
        });

        let html = `
            <div class="space-y-3 font-mono">
                <div class="text-[11px] leading-relaxed flex items-center justify-between" style="color: var(--text-secondary);">
                    <span>DOCUMENTS [${escapeHtml(langCode.toUpperCase())}] (${filteredDocs.length} MATCHED)</span>
                    <button id="btn-tree-back-cat" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80"
                            style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ LANGUAGES</button>
                </div>

                <div class="relative">
                    <input id="input-doc-filter" type="text" placeholder="Filter documents... (e.g. navigation, emergency, survival, triage)"
                           value="${escapeHtml(hierarchyState.filterText || '')}"
                           class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                           style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                </div>

                <div class="grid grid-cols-1 gap-2 max-h-[560px] overflow-y-auto reader-scroll pr-1">
        `;

        if (filteredDocs.length === 0) {
            html += `
                <div class="p-6 text-center text-xs" style="color: var(--text-muted);">
                    No documents matching "${escapeHtml(filter)}".
                </div>
            `;
        } else {
            filteredDocs.forEach(doc => {
                const chCount = (doc.chapters || []).length;
                html += `
                    <div data-doc-id="${escapeHtml(doc.id)}" class="tree-node-card cursor-pointer p-3 rounded-xl border flex flex-col gap-1.5"
                         style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                        <div class="flex items-center justify-between">
                            <div class="font-bold text-xs" style="color: var(--text-main);">${escapeHtml(doc.title)}</div>
                            <span class="text-[8.5px] px-1.5 py-0.5 rounded border" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-muted);">${escapeHtml(doc.license || 'public-domain')}</span>
                        </div>
                        <div class="flex items-center justify-between text-[9px]" style="color: var(--text-secondary);">
                            <span>Publisher: ${escapeHtml(doc.publisher || 'Verified Archive')}</span>
                            <span class="font-semibold" style="color: var(--text-main);">${chCount} ${chCount === 1 ? 'CHAPTER' : 'CHAPTERS'} ›</span>
                        </div>
                    </div>
                `;
            });
        }

        html += `
                </div>
            </div>
        `;

        box.innerHTML = html;

        const backBtn = document.getElementById('btn-tree-back-cat');
        if (backBtn) {
            backBtn.addEventListener('click', () => {
                hierarchyState.level = 'category';
                hierarchyState.filterText = '';
                renderHierarchyExplorer();
            });
        }

        const filterInput = document.getElementById('input-doc-filter');
        if (filterInput) {
            filterInput.addEventListener('input', (e) => {
                hierarchyState.filterText = e.target.value;
                renderHierarchyExplorer();
                const newInp = document.getElementById('input-doc-filter');
                if (newInp) {
                    newInp.focus();
                    newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                }
            });
        }

        box.querySelectorAll('[data-doc-id]').forEach(el => {
            const docId = el.getAttribute('data-doc-id');
            el.addEventListener('click', () => {
                const foundDoc = allDocs.find(d => d.id === docId);
                if (foundDoc) {
                    hierarchyState.selectedDoc = foundDoc;
                    hierarchyState.level = 'document';
                    hierarchyState.filterText = '';
                    renderHierarchyExplorer();
                }
            });
        });
        return;
    }

    // LEVEL 3: DOCUMENT (Show Books for Scripture, Chapters for Docs)
    if (hierarchyState.level === 'document') {
        const catName = (hierarchyState.selectedCategory || '').toLowerCase();

        if (catName === 'scripture') {
            const doc = hierarchyState.selectedDoc;
            const metaKey = `${doc.lang}:${doc.version}`;
            const meta = cachedScriptureMetaMap[metaKey];

            if (!meta || !meta.books) {
                box.innerHTML = `<div class="p-6 text-center font-mono text-xs" style="color: var(--text-muted);">Loading books catalog...</div>`;
                getScriptureMeta(doc.lang, doc.version).then(m => {
                    cachedScriptureMetaMap[metaKey] = m;
                    if (hierarchyState.level === 'document' && hierarchyState.selectedDoc && hierarchyState.selectedDoc.lang === doc.lang && hierarchyState.selectedDoc.version === doc.version) {
                        renderHierarchyExplorer();
                    }
                }).catch(() => {
                    if (hierarchyState.level === 'document' && hierarchyState.selectedDoc && hierarchyState.selectedDoc.lang === doc.lang && hierarchyState.selectedDoc.version === doc.version) {
                        box.innerHTML = `<div class="p-6 text-center font-mono text-xs text-red-500">Failed to load translation books.</div>`;
                    }
                });
                return;
            }

            const allBooks = meta.books || [];
            const filter = (hierarchyState.filterText || '').trim().toLowerCase();
            const filteredBooks = allBooks.filter(b => {
                if (!filter) return true;
                return b.name.toLowerCase().includes(filter);
            });

            let html = `
                <div class="space-y-3 font-mono">
                    <div class="flex items-center justify-between">
                        <div class="truncate pr-2">
                            <span class="text-xs font-bold" style="color: var(--text-main);">${escapeHtml(doc.title)}</span>
                            <div class="text-[9px]" style="color: var(--text-muted);">${filteredBooks.length} Canonical Books</div>
                        </div>
                        <button id="btn-tree-back-vers" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80 shrink-0"
                                style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ VERSIONS</button>
                    </div>

                    <div class="relative">
                        <input id="input-book-filter" type="text" placeholder="Filter canonical books... (e.g. Genesis, Psalms, John, Romans, Rev)"
                               value="${escapeHtml(hierarchyState.filterText || '')}"
                               class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                               style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 max-h-[560px] overflow-y-auto reader-scroll pr-1">
            `;

            if (filteredBooks.length === 0) {
                html += `
                    <div class="col-span-2 p-6 text-center text-xs" style="color: var(--text-muted);">
                        No books matching "${escapeHtml(filter)}".
                    </div>
                `;
            } else {
                filteredBooks.forEach((b, idx) => {
                    html += `
                        <div data-book-name="${escapeHtml(b.name)}" data-book-chapters="${b.chapters}" class="tree-node-card cursor-pointer p-2.5 rounded-xl border flex items-center justify-between transition-all hover:scale-[1.01]"
                             style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                            <div class="min-w-0 pr-1">
                                <div class="font-bold text-xs truncate" style="color: var(--text-main);">${escapeHtml(b.name)}</div>
                                <div class="text-[8.5px]" style="color: var(--text-secondary);">${b.chapters} ${b.chapters === 1 ? 'Chapter' : 'Chapters'}</div>
                            </div>
                            <div class="flex items-center gap-1 shrink-0">
                                <button data-read-book="${escapeHtml(b.name)}" class="text-[8px] px-2 py-0.5 rounded border font-bold uppercase transition-all"
                                        style="background-color: var(--contrast-ink); border-color: var(--panel-border); color: var(--contrast-paper);">
                                    READ ›
                                </button>
                            </div>
                        </div>
                    `;
                });
            }

            html += `
                    </div>
                </div>
            `;

            box.innerHTML = html;

            const backBtn = document.getElementById('btn-tree-back-vers');
            if (backBtn) {
                backBtn.addEventListener('click', () => {
                    hierarchyState.level = 'language';
                    hierarchyState.selectedDoc = null;
                    hierarchyState.filterText = '';
                    renderHierarchyExplorer();
                });
            }

            const filterInput = document.getElementById('input-book-filter');
            if (filterInput) {
                filterInput.addEventListener('input', (e) => {
                    hierarchyState.filterText = e.target.value;
                    renderHierarchyExplorer();
                    const newInp = document.getElementById('input-book-filter');
                    if (newInp) {
                        newInp.focus();
                        newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                    }
                });
            }

            box.querySelectorAll('[data-read-book]').forEach(el => {
                el.addEventListener('click', (e) => {
                    e.stopPropagation();
                    const bName = el.getAttribute('data-read-book');
                    hierarchyState.selectedBook = { name: bName };
                    loadScriptureBookIntoReader(doc.lang, doc.version, bName, 0);
                });
            });

            box.querySelectorAll('[data-book-name]').forEach(el => {
                el.addEventListener('click', () => {
                    const bName = el.getAttribute('data-book-name');
                    const bChapters = parseInt(el.getAttribute('data-book-chapters') || '1', 10);
                    hierarchyState.selectedBook = { name: bName, chapters: bChapters };
                    hierarchyState.level = 'book';
                    hierarchyState.filterText = '';
                    renderHierarchyExplorer();
                });
            });
            return;
        }

        // Standard non-scripture document chapters
        const doc = hierarchyState.selectedDoc;
        const allChapters = doc.chapters || [];
        const filter = (hierarchyState.filterText || '').trim().toLowerCase();
        const filteredChapters = allChapters.map((ch, idx) => ({ ch, idx })).filter(({ ch, idx }) => {
            if (!filter) return true;
            const title = (ch.title || `Chapter ${idx + 1}`).toLowerCase();
            return title.includes(filter) || `${idx + 1}`.includes(filter);
        });

        let html = `
            <div class="space-y-3 font-mono">
                <div class="text-[11px] leading-relaxed flex items-center justify-between" style="color: var(--text-secondary);">
                    <span class="truncate pr-2 font-bold" style="color: var(--text-main);">${escapeHtml(doc.title)} (${filteredChapters.length} MATCHED)</span>
                    <button id="btn-tree-back-docs" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80 shrink-0"
                            style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ DOCUMENTS</button>
                </div>

                <div class="relative">
                    <input id="input-ch-filter" type="text" placeholder="Filter chapters/topics... (e.g. Chapter 1, water, navigation)"
                           value="${escapeHtml(hierarchyState.filterText || '')}"
                           class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                           style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                </div>

                <div class="grid grid-cols-1 gap-2 max-h-[560px] overflow-y-auto reader-scroll pr-1">
        `;

        if (filteredChapters.length === 0) {
            html += `
                <div class="p-6 text-center text-xs" style="color: var(--text-muted);">
                    No chapters matching "${escapeHtml(filter)}".
                </div>
            `;
        } else {
            filteredChapters.forEach(({ ch, idx }) => {
                const secCount = ch.section_count || (ch.sections || []).length;
                html += `
                    <div data-ch-idx="${idx}" class="tree-node-card cursor-pointer p-3 rounded-xl border flex items-center justify-between"
                         style="background-color: var(--panel-bg); border-color: var(--border-subtle);">
                        <div>
                            <div class="font-bold text-xs" style="color: var(--text-main);">${escapeHtml(ch.title || `Chapter ${idx + 1}`)}</div>
                            <div class="text-[9px] mt-0.5" style="color: var(--text-secondary);">${secCount} sections / passages</div>
                        </div>
                        <span class="text-[9px] px-2.5 py-1 rounded border font-bold uppercase transition-all"
                              style="background-color: var(--contrast-ink); border-color: var(--panel-border); color: var(--contrast-paper);">
                            READ ›
                        </span>
                    </div>
                `;
            });
        }

        html += `
                </div>
            </div>
        `;

        box.innerHTML = html;

        const backBtn = document.getElementById('btn-tree-back-docs');
        if (backBtn) {
            backBtn.addEventListener('click', () => {
                hierarchyState.level = 'language';
                hierarchyState.filterText = '';
                renderHierarchyExplorer();
            });
        }

        const filterInput = document.getElementById('input-ch-filter');
        if (filterInput) {
            filterInput.addEventListener('input', (e) => {
                hierarchyState.filterText = e.target.value;
                renderHierarchyExplorer();
                const newInp = document.getElementById('input-ch-filter');
                if (newInp) {
                    newInp.focus();
                    newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                }
            });
        }

        box.querySelectorAll('[data-ch-idx]').forEach(el => {
            const idx = parseInt(el.getAttribute('data-ch-idx'), 10);
            el.addEventListener('click', () => {
                loadDocumentIntoReader(doc.id, idx);
            });
        });
        return;
    }

    // LEVEL 4: BOOK (Chapters Grid for Scripture)
    if (hierarchyState.level === 'book') {
        const doc = hierarchyState.selectedDoc;
        const book = hierarchyState.selectedBook;
        const totalCh = book.chapters || 1;
        const filter = (hierarchyState.filterText || '').trim();

        const chList = [];
        for (let c = 1; c <= totalCh; c++) {
            if (!filter || `${c}`.includes(filter)) {
                chList.push(c);
            }
        }

        let html = `
            <div class="space-y-3 font-mono">
                <div class="flex items-center justify-between">
                    <div class="truncate pr-2">
                        <span class="text-xs font-bold" style="color: var(--text-main);">${escapeHtml(book.name)}</span>
                        <div class="text-[9px]" style="color: var(--text-muted);">${totalCh} Chapters • ${escapeHtml(doc.title)}</div>
                    </div>
                    <button id="btn-tree-back-booklist" class="text-[9px] px-2 py-0.5 rounded border hover:opacity-80 shrink-0"
                            style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">‹ BOOKS</button>
                </div>

                <div class="relative">
                    <input id="input-ch-jump-filter" type="text" placeholder="Filter chapters... (e.g. 1, 5, 23, 50)"
                           value="${escapeHtml(hierarchyState.filterText || '')}"
                           class="w-full text-xs font-mono px-3 py-1.5 rounded-lg border outline-none transition-all"
                           style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);" />
                </div>

                <div class="text-[10px]" style="color: var(--text-secondary);">
                    Select Chapter (${chList.length} of ${totalCh}):
                </div>

                <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 gap-2 max-h-[520px] overflow-y-auto reader-scroll pr-1">
        `;

        if (chList.length === 0) {
            html += `
                <div class="col-span-8 p-6 text-center text-xs" style="color: var(--text-muted);">
                    No chapter matching "${escapeHtml(filter)}".
                </div>
            `;
        } else {
            chList.forEach(c => {
                html += `
                    <button data-open-ch="${c}" class="py-2.5 rounded-lg border font-bold text-xs transition-all hover:scale-105"
                            style="background-color: var(--panel-bg); border-color: var(--border-subtle); color: var(--text-main);">
                        ${c}
                    </button>
                `;
            });
        }

        html += `
                </div>
            </div>
        `;

        box.innerHTML = html;

        const backBtn = document.getElementById('btn-tree-back-booklist');
        if (backBtn) {
            backBtn.addEventListener('click', () => {
                hierarchyState.level = 'document';
                hierarchyState.filterText = '';
                renderHierarchyExplorer();
            });
        }

        const filterInput = document.getElementById('input-ch-jump-filter');
        if (filterInput) {
            filterInput.addEventListener('input', (e) => {
                hierarchyState.filterText = e.target.value;
                renderHierarchyExplorer();
                const newInp = document.getElementById('input-ch-jump-filter');
                if (newInp) {
                    newInp.focus();
                    newInp.selectionStart = newInp.selectionEnd = newInp.value.length;
                }
            });
        }

        box.querySelectorAll('[data-open-ch]').forEach(btn => {
            btn.addEventListener('click', () => {
                const chNum = parseInt(btn.getAttribute('data-open-ch'), 10);
                loadScriptureBookIntoReader(doc.lang, doc.version, book.name, chNum - 1);
            });
        });
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
    
    // Quick button to open category index
    const indexBar = `
        <div class="flex items-center justify-between pb-2 mb-2 border-b font-mono text-[9px]" style="border-color: var(--border-subtle);">
            <button id="btn-reader-open-index" class="px-2 py-0.5 rounded border hover:opacity-80 flex items-center gap-1 transition-all"
                    style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-main);">
                <span class="font-bold">+</span> <strong>EXPLORE HIERARCHY / THEMES</strong>
            </button>
            <span style="color: var(--text-muted);">${escapeHtml(currentNode.category || 'DOCUMENT')}</span>
        </div>
    `;

    // Format real provenance footer
    const provFooter = currentNode.provenance 
        ? `<div class="mt-4 pt-2.5 border-t font-mono text-[9px]" style="border-color: var(--border-subtle); color: var(--text-muted);">
            <div><strong>PROVENANCE:</strong> ${escapeHtml(currentNode.provenance.publisher || currentNode.provenance.source || 'Verified Source')}</div>
            <div class="text-[8px] opacity-75 mt-0.5">License: ${escapeHtml(currentNode.provenance.license || 'public-domain')} • Retr: ${escapeHtml(currentNode.provenance.retrieved_date || '2026-09-28')}</div>
           </div>`
        : `<div class="mt-4 pt-2.5 border-t font-mono text-[9px]" style="border-color: var(--border-subtle); color: var(--text-muted);">PROVENANCE: LOCAL SOVEREIGN ARCHIVE</div>`;

    box.innerHTML = `${indexBar}<p class="text-justify font-sans leading-relaxed whitespace-pre-line">${escapeHtml(text)}</p>${provFooter}`;
    updateReaderFontSize();

    const idxBtn = document.getElementById('btn-reader-open-index');
    if (idxBtn) {
        idxBtn.addEventListener('click', () => showHierarchyRoot());
    }
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

    const hierarchyBtn = document.getElementById('tool-hierarchy');
    if (hierarchyBtn) {
        hierarchyBtn.addEventListener('click', () => {
            showHierarchyRoot();
            showToast('Knowledge Themes Opened');
        });
    }

    const mapModeBtn = document.getElementById('btn-map-mode');
    if (mapModeBtn) {
        mapModeBtn.addEventListener('click', () => {
            const nextMode = cycleMapMode();
            showToast(`Map Mode: ${nextMode === 'HEXAGON' ? 'HEXAGONAL MATRIX' : 'OFFLINE VECTOR'}`);
        });
    }

    const mapDatasetBtn = document.getElementById('btn-map-dataset');
    if (mapDatasetBtn) {
        mapDatasetBtn.addEventListener('click', () => {
            const nextDs = cycleMapDataset();
            showToast(`Map Dataset: ${nextDs}`);
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

// ⌘K Search (Nodes + BM25 + Scripture Corpus)
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

    let searchDebounceTimer = null;
    let searchSequenceId = 0;

    if (sInput) {
        sInput.addEventListener('input', () => {
            const q = sInput.value.trim();
            if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
            searchDebounceTimer = setTimeout(() => {
                renderSearchResults(q);
            }, 120);
        });

        sInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const firstHit = sResults ? sResults.querySelector('.search-result-item') : null;
                if (firstHit) {
                    firstHit.click();
                }
            }
        });
    }

    async function renderSearchResults(q) {
        if (!sResults) return;
        const currentSeq = ++searchSequenceId;
        sResults.innerHTML = '';

        if (!q) {
            sResults.innerHTML = `<div class="p-3 font-mono text-[10px] text-center" style="color: var(--text-muted);">Type to search database nodes, 66 scripture languages, and BM25 index...</div>`;
            return;
        }

        try {
            const hits = [];
            const lowQ = q.toLowerCase();

            // 1. Backend BM25 chunks
            try {
                const bm25Hits = await search(q);
                if (currentSeq !== searchSequenceId) return;
                if (bm25Hits && Array.isArray(bm25Hits)) {
                    bm25Hits.forEach(h => hits.push({
                        id: h.doc_id,
                        title: h.doc_title,
                        subtitle: h.snippet,
                        badge: 'BM25',
                        type: 'doc'
                    }));
                }
            } catch (e) {}

            // 2. Scripture Languages & Versions Matching
            const langs = cachedScriptureLangs || [];
            langs.forEach(l => {
                if (l.name.toLowerCase().includes(lowQ) || (l.native_name && l.native_name.toLowerCase().includes(lowQ)) || l.code.toLowerCase() === lowQ) {
                    hits.push({
                        id: `scripture:${l.code}`,
                        title: `Holy Scriptures in ${l.name} (${l.native_name || l.code})`,
                        subtitle: `${(l.versions || []).length} canonical translation(s) available`,
                        badge: 'LANG',
                        type: 'scripture-lang',
                        langObj: l
                    });
                }
            });

            // 3. Common canonical scripture books matching
            const commonBooks = [
                "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy", "Joshua", "Judges", "Ruth",
                "1 Samuel", "2 Samuel", "1 Kings", "2 Kings", "1 Chronicles", "2 Chronicles", "Ezra",
                "Nehemiah", "Esther", "Job", "Psalms", "Proverbs", "Ecclesiastes", "Song of Solomon",
                "Isaiah", "Jeremiah", "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel", "Amos",
                "Obadiah", "Jonah", "Micah", "Nahum", "Habakkuk", "Zephaniah", "Haggai", "Zechariah",
                "Malachi", "Matthew", "Mark", "Luke", "John", "Acts", "Romans", "1 Corinthians",
                "2 Corinthians", "Galatians", "Ephesians", "Philippians", "Colossians", "1 Thessalonians",
                "2 Thessalonians", "1 Timothy", "2 Timothy", "Titus", "Philemon", "Hebrews", "James",
                "1 Peter", "2 Peter", "1 John", "2 John", "3 John", "Jude", "Revelation"
            ];
            commonBooks.forEach(b => {
                if (b.toLowerCase().includes(lowQ)) {
                    hits.push({
                        id: `scripture:eng:kjv:${b}`,
                        title: `${b} (KJV, English)`,
                        subtitle: `Canonical Scripture Book • Holy Bible`,
                        badge: 'BOOK',
                        type: 'scripture-book',
                        lang: 'eng',
                        version: 'kjv',
                        book: b
                    });
                }
            });

            if (hits.length > 0) {
                hits.slice(0, 12).forEach(hit => {
                    const row = document.createElement('div');
                    row.className = 'p-2.5 rounded-xl border flex items-center justify-between cursor-pointer hover:opacity-80 transition-all font-mono text-xs';
                    row.style.backgroundColor = 'var(--panel-bg-subtle)';
                    row.style.borderColor = 'var(--border-subtle)';
                    row.innerHTML = `
                        <div class="pr-2 min-w-0">
                            <div class="font-bold truncate" style="color: var(--text-main);">${escapeHtml(hit.title)}</div>
                            <div class="text-[10px] truncate" style="color: var(--text-muted);">${escapeHtml(hit.subtitle).replace(/&lt;b&gt;/g, '<b>').replace(/&lt;\/b&gt;/g, '</b>')}</div>
                        </div>
                        <span class="text-[9px] px-1.5 py-0.5 rounded border flex-shrink-0" style="border-color: var(--panel-border); color: var(--text-main);">${escapeHtml(hit.badge)}</span>
                    `;
                    row.addEventListener('click', () => {
                        if (hit.type === 'scripture-lang') {
                            hierarchyState.selectedCategory = 'scripture';
                            hierarchyState.selectedLanguage = hit.langObj;
                            hierarchyState.level = 'language';
                            renderHierarchyExplorer();
                        } else if (hit.type === 'scripture-book') {
                            hierarchyState.selectedCategory = 'scripture';
                            hierarchyState.selectedLanguage = { code: hit.lang, name: 'English', versions: ['kjv'] };
                            hierarchyState.selectedDoc = { id: `scripture:${hit.lang}:${hit.version}`, lang: hit.lang, version: hit.version, title: `Holy Bible (${hit.version.toUpperCase()}, English)` };
                            hierarchyState.selectedBook = { name: hit.book };
                            loadScriptureBookIntoReader(hit.lang, hit.version, hit.book, 0);
                        } else {
                            loadNodeIntoReader({
                                id: hit.id,
                                name: hit.title,
                                doc_id: hit.id,
                                tags: ["SEARCH_RESULT"],
                                chapters: [hit.subtitle],
                                connections: ["NEXUS-0"]
                            });
                        }
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

// Add Memory Action (Truthful Toast / Data Matrix Builder in Map Mode)
function setupAddMemory() {
    const btn = document.getElementById('btn-add-memory');
    const pillAdd = document.getElementById('pill-quick-add');

    const handleAdd = async () => {
        if (activeView === 'map') {
            openDataMatrixModal();
            return;
        }

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

        if (starEl) starEl.innerText = `${(status.total_nodes || status.total_documents || 1105).toLocaleString()}`;
        if (linkEl) linkEl.innerText = `${(status.total_links || status.total_chunks || 0).toLocaleString()}`;
    } catch (e) {}
}

function handleResize() {
    if (activeView === 'galaxy') resizeGalaxy();
    if (activeView === 'constellations') resizeConstellations();
    if (activeView === 'map') resizeMap();
}

let toastTimeoutHandle = null;

export function showToast(msg) {
    const toast = document.getElementById('toast-message');
    const text = document.getElementById('toast-text');
    if (!toast || !text) return;

    if (toastTimeoutHandle) {
        clearTimeout(toastTimeoutHandle);
        toastTimeoutHandle = null;
    }

    text.innerText = msg;
    toast.classList.remove('opacity-0', 'translate-y-3');
    toast.classList.add('opacity-100', 'translate-y-0');

    toastTimeoutHandle = setTimeout(() => {
        toast.classList.remove('opacity-100', 'translate-y-0');
        toast.classList.add('opacity-0', 'translate-y-3');
        toastTimeoutHandle = null;
    }, 2400);
}

function escapeHtml(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.innerText = text;
    return div.innerHTML;
}
