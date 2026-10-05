// MAZZAROTH Sections: Constellations, Sovereign Vector Cartography Map, Librarian AI, and Sky Deck
import { askStream, getStatus, getMaps, startMapFetch, getMapFetchStatus, deleteMapRegion, getSkyProjection, getModels } from './api.js';


/* ========================================================================= */
/* 1. AUTHORITATIVE CONSTELLATIONS DATASET (Copy-Exact from Reference)       */
/* ========================================================================= */
export const CONSTELLATIONS_CATALOG = [
    // Zodiac Constellations
    {
        id: "CONST-01",
        name: "AQUARIUS",
        category: "ZODIAC",
        season: "AUTUMN",
        direction: "SOUTH",
        desc: "The Water Bearer. Represents Ganymede carrying the celestial cup, with delicate streams of stars cascading across the southern sky.",
        stars: [
            { x: -30, y: -20, name: "Sadalsuud", bright: true },
            { x: -10, y: -35, name: "Sadalmelik", bright: true },
            { x: 20, y: -25, name: "Sadachbia" },
            { x: 40, y: -10, name: "Skat" },
            { x: 15, y: 10, name: "Ancha" },
            { x: 30, y: 30, name: "Lambda Aqr" },
            { x: -10, y: 25, name: "Psi Aqr" },
            { x: -25, y: 45, name: "Omega Aqr" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [2, 4], [4, 5], [4, 6], [6, 7]]
    },
    {
        id: "CONST-02",
        name: "CAPRICORN",
        category: "ZODIAC",
        season: "AUTUMN",
        direction: "SOUTH",
        desc: "The Sea Goat. Ancient horned creature with the tail of a fish, prominent in southern autumn quadrant registers.",
        stars: [
            { x: -40, y: -20, name: "Deneb Algedi", bright: true },
            { x: -10, y: -30, name: "Dabih" },
            { x: 25, y: -20, name: "Algedi", bright: true },
            { x: 40, y: 10, name: "Nashira" },
            { x: 10, y: 30, name: "Theta Cap" },
            { x: -25, y: 20, name: "Omega Cap" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 0], [1, 5]]
    },
    {
        id: "CONST-03",
        name: "SAGITTARIUS",
        category: "ZODIAC",
        season: "SUMMER",
        direction: "SOUTH",
        desc: "The Archer / Centaur. Features the iconic 'Teapot' asterism pointing directly toward the supermassive black hole at the galactic center.",
        stars: [
            { x: -30, y: 20, name: "Kaus Media" },
            { x: -10, y: 25, name: "Kaus Australis", bright: true },
            { x: 15, y: 20, name: "Ascella" },
            { x: 35, y: 10, name: "Nunki", bright: true },
            { x: 15, y: -10, name: "Kaus Borealis" },
            { x: -10, y: -15, name: "Alnasl" },
            { x: 0, y: -35, name: "Tau Sgr" },
            { x: 30, y: -25, name: "Sigma Sgr" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [2, 4], [4, 5], [5, 1], [4, 6], [6, 7]]
    },
    {
        id: "CONST-04",
        name: "SCORPIO",
        category: "ZODIAC",
        season: "SUMMER",
        direction: "SOUTH",
        desc: "The Scorpion. Centered upon the radiant red supergiant Antares (Heart of the Scorpion) and sweeping down into the hooked stinger of Shaula.",
        stars: [
            { x: -35, y: -25, name: "Graffias" },
            { x: -20, y: -30, name: "Dschubba" },
            { x: -5, y: -15, name: "Antares", bright: true },
            { x: 5, y: 0, name: "Larawag" },
            { x: 10, y: 20, name: "Sargas", bright: true },
            { x: 25, y: 35, name: "Shaula", bright: true },
            { x: 40, y: 30, name: "Lesath" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]
    },
    {
        id: "CONST-05",
        name: "LIBRA",
        category: "ZODIAC",
        season: "SUMMER",
        direction: "SOUTH",
        desc: "The Scales of Justice. Symmetrical diamond asterism positioned between Virgo and Scorpio.",
        stars: [
            { x: 0, y: -30, name: "Zubeneschamali", bright: true },
            { x: -30, y: 0, name: "Zubenelgenubi", bright: true },
            { x: 30, y: 0, name: "Zubenelhakrabi" },
            { x: -15, y: 30, name: "Brachium" },
            { x: 15, y: 30, name: "Upsilon Lib" }
        ],
        lines: [[0, 1], [0, 2], [1, 2], [1, 3], [2, 4]]
    },
    {
        id: "CONST-06",
        name: "VIRGO",
        category: "ZODIAC",
        season: "SPRING",
        direction: "SOUTH",
        desc: "The Maiden. Anchored by the brilliant blue-white luminary Spica, this sprawling constellation contains the vast Virgo Galaxy Cluster.",
        stars: [
            { x: 0, y: -40, name: "Vindemiatrix" },
            { x: -20, y: -20, name: "Porrima", bright: true },
            { x: 15, y: -15, name: "Auva" },
            { x: -30, y: 10, name: "Zaniah" },
            { x: 0, y: 15, name: "Spica", bright: true },
            { x: 25, y: 30, name: "Heze" },
            { x: -10, y: 40, name: "Syrma" }
        ],
        lines: [[0, 1], [0, 2], [1, 3], [1, 4], [2, 4], [4, 5], [4, 6]]
    },
    {
        id: "CONST-07",
        name: "LEO",
        category: "ZODIAC",
        season: "SPRING",
        direction: "EAST",
        desc: "The Lion. Famous for its sickle asterism resembling a backwards question mark, marked by the royal blue-white star Regulus.",
        stars: [
            { x: 25, y: -30, name: "Rasalas" },
            { x: 40, y: -15, name: "Adhafera" },
            { x: 30, y: 0, name: "Algieba", bright: true },
            { x: 10, y: 0, name: "Regulus", bright: true },
            { x: 0, y: 20, name: "Chertan" },
            { x: -35, y: 20, name: "Denebola", bright: true },
            { x: -45, y: 5, name: "Zosma" },
            { x: -25, y: -10, name: "Al Jabhah" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6], [6, 7], [7, 4]]
    },
    {
        id: "CONST-08",
        name: "CANCER",
        category: "ZODIAC",
        season: "SPRING",
        direction: "NORTH",
        desc: "The Crab. Faintest of the zodiac constellations, holding the radiant Beehive open cluster (Praesepe).",
        stars: [
            { x: -20, y: -25, name: "Tegmine" },
            { x: 0, y: 0, name: "Asellus Borealis", bright: true },
            { x: 25, y: -20, name: "Acubens" },
            { x: 5, y: 30, name: "Asellus Australis" }
        ],
        lines: [[0, 1], [1, 2], [1, 3]]
    },
    {
        id: "CONST-09",
        name: "GEMINI",
        category: "ZODIAC",
        season: "WINTER",
        direction: "EAST",
        desc: "The Celestial Twins. Two parallel stick figures anchored by the twin heads Castor (sextuple star) and Pollux (giant orange).",
        stars: [
            { x: -35, y: -30, name: "Castor", bright: true },
            { x: 35, y: -30, name: "Pollux", bright: true },
            { x: -30, y: 0, name: "Mebsuta" },
            { x: 30, y: 0, name: "Wasat" },
            { x: -25, y: 30, name: "Tejat" },
            { x: 25, y: 30, name: "Alhena", bright: true }
        ],
        lines: [[0, 1], [0, 2], [1, 3], [2, 4], [3, 5], [2, 3], [4, 5]]
    },
    {
        id: "CONST-10",
        name: "TAURUS",
        category: "ZODIAC",
        season: "WINTER",
        direction: "EAST",
        desc: "The Bull. Features the V-shaped Hyades cluster ending at the blood-orange giant Aldebaran, plus the Pleiades (Seven Sisters).",
        stars: [
            { x: -35, y: -25, name: "Pleiades", bright: true },
            { x: 10, y: -10, name: "Aldebaran", bright: true },
            { x: 35, y: -25, name: "Elnath", bright: true },
            { x: 0, y: 15, name: "Tianguan" },
            { x: -20, y: 30, name: "Ain" }
        ],
        lines: [[0, 1], [2, 1], [1, 3], [3, 4]]
    },
    {
        id: "CONST-11",
        name: "ARIES",
        category: "ZODIAC",
        season: "AUTUMN",
        direction: "EAST",
        desc: "The Ram. Historic first sign of the zodiac, marked by Hamal, Sheratan, and Mesarthim in the autumn sky.",
        stars: [
            { x: -25, y: 0, name: "Mesarthim" },
            { x: 0, y: -10, name: "Sheratan", bright: true },
            { x: 25, y: 5, name: "Hamal", bright: true }
        ],
        lines: [[0, 1], [1, 2]]
    },
    {
        id: "CONST-12",
        name: "PISCES",
        category: "ZODIAC",
        season: "AUTUMN",
        direction: "WEST",
        desc: "The Fishes. Two fish swimming outward connected by a long knotted cord converging at Alrescha.",
        stars: [
            { x: -40, y: -20, name: "Gamma Psc" },
            { x: -20, y: -15, name: "Theta Psc" },
            { x: 0, y: 0, name: "Alrescha", bright: true },
            { x: 20, y: -10, name: "Eta Psc" },
            { x: 35, y: -25, name: "Kullat Nunu", bright: true },
            { x: -10, y: 25, name: "Omega Psc" },
            { x: 10, y: 25, name: "Delta Psc" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 4], [1, 5], [5, 6], [6, 3]]
    },

    // Major Deep Sky Asterisms
    {
        id: "CONST-13",
        name: "URSA MAJOR",
        category: "OTHER",
        season: "SPRING",
        direction: "NORTH",
        desc: "The Great Bear. Contains the Big Dipper pointer stars (Merak and Dubhe) that lead directly to Polaris.",
        stars: [
            { x: -45, y: -25, name: "Dubhe", bright: true },
            { x: -15, y: -28, name: "Merak", bright: true },
            { x: 0, y: -5, name: "Phecda" },
            { x: -30, y: -5, name: "Megrez" },
            { x: 15, y: 10, name: "Alioth", bright: true },
            { x: 35, y: 20, name: "Mizar" },
            { x: 50, y: 35, name: "Alkaid", bright: true }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 0], [2, 4], [4, 5], [5, 6]]
    },
    {
        id: "CONST-14",
        name: "ORION",
        category: "OTHER",
        season: "WINTER",
        direction: "SOUTH",
        desc: "The Hunter. The crown jewel of the winter sky. Belt of three stars flanked by red Betelgeuse and blue Rigel.",
        stars: [
            { x: -25, y: -35, name: "Betelgeuse", bright: true },
            { x: 25, y: -35, name: "Bellatrix", bright: true },
            { x: -8, y: 0, name: "Alnitak", bright: true },
            { x: 0, y: 0, name: "Alnilam", bright: true },
            { x: 8, y: 0, name: "Mintaka", bright: true },
            { x: -25, y: 35, name: "Saiph" },
            { x: 25, y: 35, name: "Rigel", bright: true }
        ],
        lines: [[0, 2], [1, 4], [2, 3], [3, 4], [2, 5], [4, 6], [0, 1], [5, 6]]
    },
    {
        id: "CONST-15",
        name: "CASSIOPEIA",
        category: "OTHER",
        season: "AUTUMN",
        direction: "NORTH",
        desc: "The Queen. Distinctive W-shaped crown asterism positioned opposite Ursa Major across the North Celestial Pole.",
        stars: [
            { x: -45, y: 10, name: "Caph", bright: true },
            { x: -20, y: -20, name: "Schedar", bright: true },
            { x: 0, y: 5, name: "Gamma Cas", bright: true },
            { x: 25, y: -15, name: "Ruchbah" },
            { x: 45, y: 15, name: "Segin" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 4]]
    },
    {
        id: "CONST-16",
        name: "CYGNUS",
        category: "OTHER",
        season: "SUMMER",
        direction: "NORTH",
        desc: "The Swan (Northern Cross). Flying along the Milky Way with Deneb marking the tail and vibrant Albireo at the beak.",
        stars: [
            { x: 0, y: -35, name: "Deneb", bright: true },
            { x: 0, y: -10, name: "Sadr", bright: true },
            { x: 0, y: 20, name: "Albireo" },
            { x: -35, y: -10, name: "Delta Cyg" },
            { x: 35, y: -10, name: "Gienah" }
        ],
        lines: [[0, 1], [1, 2], [3, 1], [1, 4]]
    },
    {
        id: "CONST-17",
        name: "HERCULES",
        category: "OTHER",
        season: "SUMMER",
        direction: "NORTH",
        desc: "The Hero. Anchored by the central 'Keystone' quadrilateral, home to the Great Globular Cluster M13.",
        stars: [
            { x: -20, y: -25, name: "Pi Her" },
            { x: 20, y: -25, name: "Eta Her" },
            { x: -15, y: 5, name: "Zeta Her", bright: true },
            { x: 15, y: 5, name: "Epsilon Her" },
            { x: -35, y: -35, name: "Rasalgethi", bright: true },
            { x: 35, y: -35, name: "Kornephoros" },
            { x: -25, y: 30, name: "Sarin" },
            { x: 25, y: 30, name: "Marfik" }
        ],
        lines: [[0, 1], [1, 3], [3, 2], [2, 0], [0, 4], [1, 5], [2, 6], [3, 7]]
    },
    {
        id: "CONST-18",
        name: "PEGASUS",
        category: "OTHER",
        season: "AUTUMN",
        direction: "WEST",
        desc: "The Winged Horse. Dominated by the Great Square of Pegasus connecting seamlessly with the chain of Andromeda.",
        stars: [
            { x: -30, y: -30, name: "Markab", bright: true },
            { x: 30, y: -30, name: "Scheat", bright: true },
            { x: 30, y: 30, name: "Alpheratz", bright: true },
            { x: -30, y: 30, name: "Algenib" }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 0]]
    },
    {
        id: "CONST-19",
        name: "CRUX",
        category: "OTHER",
        season: "SPRING",
        direction: "SOUTH",
        desc: "The Southern Cross. The most famous navigational marker of the southern hemisphere, pointing directly to the South Pole.",
        stars: [
            { x: 0, y: -35, name: "Gacrux", bright: true },
            { x: 0, y: 35, name: "Acrux", bright: true },
            { x: -25, y: 0, name: "Mimosa", bright: true },
            { x: 25, y: 0, name: "Imai" }
        ],
        lines: [[0, 1], [2, 3]]
    },
    {
        id: "CONST-20",
        name: "URSA MINOR",
        category: "OTHER",
        season: "ALL",
        direction: "NORTH",
        desc: "The Little Bear. Terminates at Polaris (the North Star), the pivotal rotational axis of the northern sky.",
        stars: [
            { x: -35, y: 25, name: "Kochab", bright: true },
            { x: -10, y: 28, name: "Pherkad" },
            { x: 0, y: 10, name: "Anwar" },
            { x: -25, y: 10, name: "Urodelus" },
            { x: 15, y: -5, name: "Yildun" },
            { x: 30, y: -15, name: "Polaris", bright: true }
        ],
        lines: [[0, 1], [1, 2], [2, 3], [3, 0], [2, 4], [4, 5]]
    }
];

/* ========================================================================= */
/* 2. OFFLINE GEOCACHES & CARTOGRAPHIC DATASET                               */
/* ========================================================================= */
export const GEOCACHES = [
    { id: "GEO-01", name: "Alexandria Knowledge Archive", lat: 31.2, lon: 29.9, desc: "Sovereign backup site 01. Holds historical mathematical commentaries, scriptural codices, and astronomical records." },
    { id: "GEO-02", name: "Svalbard Seedbank Vault", lat: 78.2, lon: 15.6, desc: "Arctic cold-storage cryogenic genetic vaults, resilient flora index, and agricultural registers." },
    { id: "GEO-03", name: "Atacama Celestial Sensor", lat: -23.0, lon: -67.7, desc: "High-altitude millimeter-wave interferometry ground station with direct stellar alignment." },
    { id: "GEO-04", name: "Mauna Kea Astrometry Array", lat: 19.8, lon: -155.4, desc: "Optical zenith tracking facility operating with zero external network linkages." },
    { id: "GEO-05", name: "Greenwich Prime Meridian Station", lat: 51.48, lon: 0.0, desc: "Historic 0° longitude temporal observatory and geodetic reference standard." },
    { id: "GEO-06", name: "Cape Town Southern Geodetic Hub", lat: -33.9, lon: 18.4, desc: "Southern hemisphere celestial navigation reference node and maritime log vault." },
    { id: "GEO-07", name: "Tokyo Astronomical Node", lat: 35.7, lon: 139.7, desc: "Eastern Pacific telemetry receiver and sovereign lunar cycle registry." }
];

// High-fidelity vector continent polygon paths [lon, lat]
const WORLD_CONTINENTS = [
    // North America & Central America
    [
        [-168, 65], [-160, 71], [-140, 70], [-120, 70], [-90, 70], [-80, 62], [-65, 58], [-55, 48],
        [-65, 44], [-75, 38], [-80, 26], [-82, 24], [-97, 26], [-105, 20], [-90, 16], [-84, 9],
        [-78, 8], [-85, 14], [-98, 16], [-105, 22], [-115, 30], [-120, 34], [-124, 48], [-130, 54],
        [-140, 60], [-150, 60], [-165, 60], [-168, 65]
    ],
    // South America
    [
        [-78, 8], [-72, 11], [-60, 8], [-50, 0], [-35, -5], [-35, -12], [-40, -22], [-50, -30],
        [-58, -38], [-65, -54], [-72, -55], [-75, -45], [-72, -35], [-76, -15], [-80, -2], [-78, 8]
    ],
    // Europe & Scandinavia
    [
        [-9, 36], [-9, 43], [-1, 46], [-5, 48], [2, 51], [8, 54], [10, 58], [5, 62], [14, 68],
        [28, 71], [32, 65], [24, 60], [18, 56], [28, 45], [26, 40], [23, 38], [15, 38], [15, 42],
        [8, 44], [3, 42], [-5, 36], [-9, 36]
    ],
    // Africa
    [
        [-9, 36], [10, 37], [25, 32], [33, 31], [35, 28], [43, 12], [51, 11], [42, -2], [40, -12],
        [35, -25], [28, -33], [18, -34], [12, -18], [9, -4], [3, 5], [-12, 5], [-17, 14], [-13, 28],
        [-9, 36]
    ],
    // Asia & Siberia
    [
        [32, 65], [60, 70], [80, 73], [100, 76], [140, 72], [170, 68], [180, 65], [170, 60],
        [160, 52], [142, 50], [130, 42], [122, 38], [120, 30], [110, 20], [105, 10], [100, 2],
        [98, 10], [90, 22], [80, 16], [78, 8], [72, 20], [62, 25], [55, 25], [48, 30], [35, 35],
        [35, 42], [40, 46], [50, 50], [60, 55], [60, 60], [32, 65]
    ],
    // Australia
    [
        [114, -22], [120, -14], [135, -12], [142, -11], [146, -18], [153, -28], [150, -36],
        [140, -38], [130, -32], [116, -34], [113, -26], [114, -22]
    ],
    // Great Britain & Ireland
    [
        [-5, 50], [1.5, 52], [0, 58], [-4, 58], [-5, 55], [-5, 50]
    ],
    // Greenland
    [
        [-45, 60], [-35, 66], [-20, 75], [-30, 82], [-55, 82], [-55, 70], [-45, 60]
    ],
    // Japan
    [
        [130, 31], [133, 34], [140, 36], [142, 44], [140, 44], [136, 36], [130, 31]
    ],
    // Madagascar
    [
        [44, -12], [50, -15], [47, -25], [44, -25], [44, -12]
    ],
    // Antarctica
    [
        [-180, -78], [-120, -74], [-60, -64], [-30, -72], [30, -68], [90, -66], [140, -66], [180, -78]
    ]
];

/* ========================================================================= */
/* 3. CONSTELLATIONS STATE & ENGINE                                          */
/* ========================================================================= */
/* ========================================================================= */
/* 3. UNIFIED CONSTELLATIONS & CELESTIAL DOME STATE & ENGINE                 */
/* ========================================================================= */
let cCanvas = null;
let cCtx = null;
let constLayoutMode = 'DOME'; // Default to DOME per user request!
let filterSeason = 'ALL';
let filterPosition = 'ALL';
let showConstLines = true;
let constZoom = 1.0;
let constPanX = 0;
let constPanY = 0;
let isDraggingConst = false;
let constDragStartX = 0;
let constDragStartY = 0;
let selectedConstellation = null;
let onSelectConstCallback = null;
let constEventsBound = false;

// Real-Time Astrometry & Sky Projection State
let skyProjection = null;
let skyLat = 35.6762; // Tokyo default (35.7°N, 139.7°E)
let skyLon = 139.6503;
let skyDateTime = new Date();
let hoveredSkyStar = null;
let skyDomeRadius = 260;

export function initConstellations(canvas, onSelect) {
    if (!canvas) return;
    cCanvas = canvas;
    cCtx = cCanvas.getContext('2d');
    onSelectConstCallback = onSelect;

    setupConstInteractions();
    setupConstControls();
    resizeConstellations();
    refreshSkyData();
}

function formatDateTimeLocal(d) {
    const pad = (n) => String(n).padStart(2, '0');
    const YYYY = d.getFullYear();
    const MM = pad(d.getMonth() + 1);
    const DD = pad(d.getDate());
    const hh = pad(d.getHours());
    const mm = pad(d.getMinutes());
    return `${YYYY}-${MM}-${DD}T${hh}:${mm}`;
}

export function setConstMode(mode) {
    constLayoutMode = mode;
    
    const modeSelect = document.getElementById('const-mode-select');
    if (modeSelect) modeSelect.value = mode;

    const modeBtn = document.getElementById('btn-const-mode');
    if (modeBtn) {
        modeBtn.innerText = mode === 'DOME' ? 'DOME' : (mode === 'SPHERE' ? 'SPHERE' : 'POSTER');
    }

    const domeControls = document.getElementById('const-dome-controls');
    const timeGroup = document.getElementById('tool-const-time-group');
    if (domeControls) {
        if (mode === 'DOME') {
            domeControls.classList.remove('hidden');
            domeControls.classList.add('grid');
        } else {
            domeControls.classList.add('hidden');
            domeControls.classList.remove('grid');
        }
    }
    if (timeGroup) {
        if (mode === 'DOME') {
            timeGroup.classList.remove('hidden');
            timeGroup.classList.add('flex');
        } else {
            timeGroup.classList.add('hidden');
            timeGroup.classList.remove('flex');
        }
    }

    if (cCanvas) {
        if (mode === 'DOME') {
            constPanX = 0;
            constPanY = 0;
            constZoom = 1.0;
        } else if (mode === 'POSTER') {
            constPanX = 0;
            constPanY = 100;
            constZoom = 1.0;
        } else {
            constPanX = 0;
            constPanY = 0;
            constZoom = 1.0;
        }
    }

    updateConstStatusHUD();
    if (mode === 'DOME' && !skyProjection) {
        refreshSkyData();
    } else {
        renderConstellations();
    }
}

export function cycleConstMode() {
    const modes = ['DOME', 'SPHERE', 'POSTER'];
    const idx = modes.indexOf(constLayoutMode);
    const next = modes[(idx + 1) % modes.length];
    setConstMode(next);
    return next;
}

export function cycleConstSeason() {
    const seasons = ['ALL', 'WINTER', 'SPRING', 'SUMMER', 'AUTUMN'];
    const idx = seasons.indexOf(filterSeason);
    filterSeason = seasons[(idx + 1) % seasons.length];
    const badge = document.getElementById('badge-active-season');
    if (badge) badge.innerText = filterSeason;
    const btn = document.getElementById('btn-season-cycle');
    if (btn) btn.innerText = filterSeason;
    renderConstellations();
    return filterSeason;
}

export function cycleConstPosition() {
    const positions = ['ALL', 'NORTH', 'SOUTH', 'EAST', 'WEST'];
    const idx = positions.indexOf(filterPosition);
    filterPosition = positions[(idx + 1) % positions.length];
    const badge = document.getElementById('badge-active-position');
    if (badge) badge.innerText = filterPosition;
    const btn = document.getElementById('btn-position-cycle');
    if (btn) btn.innerText = filterPosition;
    renderConstellations();
    return filterPosition;
}

export function resetConstFilter() {
    filterSeason = 'ALL';
    filterPosition = 'ALL';
    const sBadge = document.getElementById('badge-active-season');
    const pBadge = document.getElementById('badge-active-position');
    if (sBadge) sBadge.innerText = 'ALL';
    if (pBadge) pBadge.innerText = 'ALL';
    const sBtn = document.getElementById('btn-season-cycle');
    const pBtn = document.getElementById('btn-position-cycle');
    if (sBtn) sBtn.innerText = 'ALL';
    if (pBtn) pBtn.innerText = 'ALL';
    renderConstellations();
}

export function toggleConstLines() {
    showConstLines = !showConstLines;
    renderConstellations();
    return showConstLines;
}

export function zoomConstIn() {
    constZoom = Math.min(4.0, constZoom * 1.25);
    renderConstellations();
}

export function zoomConstOut() {
    constZoom = Math.max(0.4, constZoom * 0.8);
    renderConstellations();
}

export function recenterConstellations() {
    constZoom = 1.0;
    if (constLayoutMode === 'POSTER') {
        constPanX = 0;
        constPanY = 100;
    } else {
        constPanX = 0;
        constPanY = 0;
    }
    renderConstellations();
}

export function stepSkyHour(delta) {
    skyDateTime = new Date(skyDateTime.getTime() + delta * 3600 * 1000);
    const dtInput = document.getElementById('const-datetime-input');
    if (dtInput) dtInput.value = formatDateTimeLocal(skyDateTime);
    refreshSkyData();
}

let skyRefreshSequenceId = 0;

export async function refreshSkyData() {
    const seqId = ++skyRefreshSequenceId;
    try {
        const timeIso = skyDateTime.toISOString();
        const data = await getSkyProjection({
            lat: skyLat,
            lon: skyLon,
            time: timeIso,
            radius: skyDomeRadius
        });
        if (seqId !== skyRefreshSequenceId) return;
        skyProjection = data;

        const lstEl = document.getElementById('const-lst-display');
        if (lstEl && data.lst_deg !== undefined) {
            const lstHours = (data.lst_deg / 15).toFixed(2);
            lstEl.innerText = `LST: ${lstHours}h (${data.lst_deg.toFixed(1)}°)`;
        }

        updateConstStatusHUD();
        renderConstellations();
    } catch (err) {
        console.error('Failed to load sky projection:', err);
    }
}

function updateConstStatusHUD() {
    const countEl = document.getElementById('const-status-count');
    if (!countEl) return;
    if (constLayoutMode === 'DOME') {
        if (skyProjection && skyProjection.visible_stars) {
            const above = skyProjection.visible_stars.filter(s => s.alt_deg >= 0).length;
            countEl.innerText = `VISIBLE: ${above} STARS`;
        } else {
            countEl.innerText = 'CALCULATING ASTROMETRY...';
        }
    } else if (constLayoutMode === 'POSTER') {
        countEl.innerText = `${CONSTELLATIONS_CATALOG.length} CONSTELLATIONS (GRID)`;
    } else {
        countEl.innerText = `${CONSTELLATIONS_CATALOG.length} ASTERISMS (SPHERE)`;
    }
}

function setupConstControls() {
    const modeSelect = document.getElementById('const-mode-select');
    const modeBtn = document.getElementById('btn-const-mode');
    const presetSelect = document.getElementById('const-preset-select');
    const dtInput = document.getElementById('const-datetime-input');
    const prevHourBtn = document.getElementById('tool-sky-prev-hour');
    const nextHourBtn = document.getElementById('tool-sky-next-hour');
    const nowBtn = document.getElementById('tool-sky-now');
    const seasonBtn = document.getElementById('btn-season-cycle');
    const posBtn = document.getElementById('btn-position-cycle');
    const resetBtn = document.getElementById('btn-reset-const-filter');
    const linesBtn = document.getElementById('tool-const-lines');
    const zoomInBtn = document.getElementById('tool-const-zoom-in');
    const zoomOutBtn = document.getElementById('tool-const-zoom-out');
    const recenterBtn = document.getElementById('tool-const-recenter');

    if (modeSelect) {
        modeSelect.value = constLayoutMode;
        modeSelect.addEventListener('change', (e) => {
            setConstMode(e.target.value);
        });
    }

    if (modeBtn) {
        modeBtn.addEventListener('click', () => {
            cycleConstMode();
        });
    }

    if (dtInput) {
        dtInput.value = formatDateTimeLocal(skyDateTime);
        dtInput.addEventListener('change', (e) => {
            if (e.target.value) {
                skyDateTime = new Date(e.target.value);
                refreshSkyData();
            }
        });
    }

    if (presetSelect) {
        presetSelect.addEventListener('change', (e) => {
            const [latStr, lonStr] = e.target.value.split(',');
            skyLat = parseFloat(latStr);
            skyLon = parseFloat(lonStr);
            refreshSkyData();
        });
    }

    if (prevHourBtn) prevHourBtn.addEventListener('click', () => stepSkyHour(-1));
    if (nextHourBtn) nextHourBtn.addEventListener('click', () => stepSkyHour(1));
    if (nowBtn) {
        nowBtn.addEventListener('click', () => {
            skyDateTime = new Date();
            if (dtInput) dtInput.value = formatDateTimeLocal(skyDateTime);
            refreshSkyData();
        });
    }

    if (seasonBtn) seasonBtn.addEventListener('click', () => cycleConstSeason());
    if (posBtn) posBtn.addEventListener('click', () => cycleConstPosition());
    if (resetBtn) resetBtn.addEventListener('click', () => resetConstFilter());
    if (linesBtn) linesBtn.addEventListener('click', () => toggleConstLines());
    if (zoomInBtn) zoomInBtn.addEventListener('click', () => zoomConstIn());
    if (zoomOutBtn) zoomOutBtn.addEventListener('click', () => zoomConstOut());
    if (recenterBtn) recenterBtn.addEventListener('click', () => recenterConstellations());
}

export function resizeConstellations() {
    if (!cCanvas) return;
    const redZone = document.getElementById('zone-red');
    if (!redZone) return;

    const dpr = window.devicePixelRatio || 1;
    const w = redZone.clientWidth || 800;
    const h = redZone.clientHeight || 600;

    cCanvas.width = w * dpr;
    cCanvas.height = h * dpr;
    cCanvas.style.width = `${w}px`;
    cCanvas.style.height = `${h}px`;

    skyDomeRadius = Math.min(w, h) * 0.42;
    renderConstellations();
}

export function renderConstellations() {
    if (!cCanvas || !cCtx) return;

    const dpr = window.devicePixelRatio || 1;
    const w = cCanvas.width / dpr;
    const h = cCanvas.height / dpr;

    cCtx.save();
    cCtx.scale(dpr, dpr);
    cCtx.clearRect(0, 0, w, h);

    const rootStyle = getComputedStyle(document.documentElement);
    const bg = rootStyle.getPropertyValue('--bg-canvas').trim() || '#0b0d11';
    const strokeColor = rootStyle.getPropertyValue('--contrast-ink').trim() || '#ffffff';
    const textMain = rootStyle.getPropertyValue('--text-main').trim() || '#ffffff';
    const textMuted = rootStyle.getPropertyValue('--text-muted').trim() || '#71717a';
    const theme = document.documentElement.getAttribute('data-theme') || 'midnight-gold';
    const isLight = theme === 'technical-paper' || theme === 'light';
    const isDark = !isLight;

    cCtx.fillStyle = bg;
    cCtx.fillRect(0, 0, w, h);

    const cx = w / 2;
    const cy = h / 2;

    cCtx.save();
    cCtx.translate(cx + constPanX, cy + constPanY);
    cCtx.scale(constZoom, constZoom);

    if (constLayoutMode === 'DOME') {
        const r = skyDomeRadius;

        // 1. Sky Dome Outer Disc & Technical Gradient
        cCtx.beginPath();
        cCtx.arc(0, 0, r, 0, Math.PI * 2);
        if (isLight) {
            const grad = cCtx.createRadialGradient(0, 0, 10, 0, 0, r);
            grad.addColorStop(0, '#f8fafc');
            grad.addColorStop(0.7, '#edf2f7');
            grad.addColorStop(1, '#e2e8f0');
            cCtx.fillStyle = grad;
        } else if (theme === 'synth-magenta') {
            const grad = cCtx.createRadialGradient(0, 0, 10, 0, 0, r);
            grad.addColorStop(0, '#0f1224');
            grad.addColorStop(0.7, '#090b17');
            grad.addColorStop(1, '#04050a');
            cCtx.fillStyle = grad;
        } else if (theme === 'obsidian-mono') {
            const grad = cCtx.createRadialGradient(0, 0, 10, 0, 0, r);
            grad.addColorStop(0, '#141822');
            grad.addColorStop(0.7, '#0d1017');
            grad.addColorStop(1, '#06070a');
            cCtx.fillStyle = grad;
        } else {
            // default midnight-gold
            const grad = cCtx.createRadialGradient(0, 0, 10, 0, 0, r);
            grad.addColorStop(0, '#0c1a2f');
            grad.addColorStop(0.7, '#071324');
            grad.addColorStop(1, '#030a14');
            cCtx.fillStyle = grad;
        }
        cCtx.fill();
        cCtx.strokeStyle = isLight 
            ? '#0b0d11' 
            : (theme === 'synth-magenta' ? 'rgba(0, 240, 255, 0.45)' : (theme === 'midnight-gold' ? 'rgba(245, 158, 11, 0.45)' : 'rgba(255, 255, 255, 0.35)'));
        cCtx.lineWidth = 1.5;
        cCtx.stroke();

        // 2. Altitude Rings (30°, 60°) and Cardinal Coordinate Grid
        cCtx.lineWidth = 0.75;
        cCtx.strokeStyle = isDark ? 'rgba(255, 255, 255, 0.08)' : 'rgba(0, 0, 0, 0.1)';

        // 30° Alt Ring (r * 60/90)
        cCtx.beginPath();
        cCtx.arc(0, 0, r * (60 / 90), 0, Math.PI * 2);
        cCtx.stroke();

        // 60° Alt Ring (r * 30/90)
        cCtx.beginPath();
        cCtx.arc(0, 0, r * (30 / 90), 0, Math.PI * 2);
        cCtx.stroke();

        // Altitude Labels
        cCtx.font = "8px 'JetBrains Mono', monospace";
        cCtx.fillStyle = textMuted;
        cCtx.textAlign = 'left';
        cCtx.fillText("60°", 4, -r * (30 / 90) + 9);
        cCtx.fillText("30°", 4, -r * (60 / 90) + 9);
        cCtx.fillText("0° HORIZON", 4, -r + 11);

        // Zenith Crosshair (90°)
        cCtx.beginPath();
        cCtx.moveTo(-8, 0); cCtx.lineTo(8, 0);
        cCtx.moveTo(0, -8); cCtx.lineTo(0, 8);
        cCtx.strokeStyle = isDark ? 'rgba(255, 255, 255, 0.35)' : 'rgba(0, 0, 0, 0.4)';
        cCtx.stroke();

        // Cardinal Axes
        cCtx.beginPath();
        cCtx.moveTo(0, -r); cCtx.lineTo(0, r);
        cCtx.moveTo(-r, 0); cCtx.lineTo(r, 0);
        cCtx.strokeStyle = isDark ? 'rgba(255, 255, 255, 0.06)' : 'rgba(0, 0, 0, 0.07)';
        cCtx.stroke();

        // 3. Dense Background Starfield (800+ Stars)
        if (skyProjection && skyProjection.background_stars) {
            for (const bgStar of skyProjection.background_stars) {
                const distFromCenter = Math.sqrt(bgStar.x * bgStar.x + bgStar.y * bgStar.y);
                if (distFromCenter > r) continue;

                const bgRadius = Math.max(0.6, (6.5 - bgStar.magnitude) * 0.4);
                const alpha = Math.max(0.15, (6.5 - bgStar.magnitude) / 4.0);

                cCtx.beginPath();
                cCtx.arc(bgStar.x, bgStar.y, bgRadius, 0, Math.PI * 2);
                cCtx.fillStyle = isDark 
                    ? `rgba(220, 230, 255, ${alpha.toFixed(2)})` 
                    : `rgba(20, 30, 50, ${(alpha * 0.7).toFixed(2)})`;
                cCtx.fill();
            }
        }

        // 4. Constellation Vector Lines (Dashed styling)
        if (showConstLines && skyProjection && skyProjection.lines) {
            cCtx.lineWidth = 1.0;
            cCtx.strokeStyle = isDark ? 'rgba(255, 255, 255, 0.45)' : 'rgba(0, 0, 0, 0.45)';
            cCtx.setLineDash([3, 3]);

            for (const [start, end, _cName] of skyProjection.lines) {
                cCtx.beginPath();
                cCtx.moveTo(start[0], start[1]);
                cCtx.lineTo(end[0], end[1]);
                cCtx.stroke();
            }
            cCtx.setLineDash([]);
        }

        // 5. Constellation Centroid Labels
        if (showConstLines && skyProjection && skyProjection.constellation_labels) {
            cCtx.font = "bold 8.5px 'JetBrains Mono', monospace";
            cCtx.textAlign = 'center';
            cCtx.fillStyle = isDark ? 'rgba(157, 166, 181, 0.75)' : 'rgba(88, 96, 111, 0.85)';

            for (const lbl of skyProjection.constellation_labels) {
                const dist = Math.sqrt(lbl.x * lbl.x + lbl.y * lbl.y);
                if (dist < r - 15) {
                    cCtx.fillText(lbl.name.toUpperCase(), lbl.x, lbl.y);
                }
            }
        }

        // 6. Visible Primary Navigation & Asterism Stars
        if (skyProjection && skyProjection.visible_stars) {
            for (const star of skyProjection.visible_stars) {
                if (!star.is_visible && star.alt_deg < 0) continue;
                const distFromCenter = Math.sqrt(star.x * star.x + star.y * star.y);
                if (distFromCenter > r + 4) continue;

                const isHovered = hoveredSkyStar && hoveredSkyStar.name === star.name;
                const isBright = star.magnitude <= 1.8;
                const starRadius = Math.max(1.8, (4.5 - star.magnitude * 0.75) * (isHovered ? 1.5 : 1.0));

                // Star Glow
                const glow = cCtx.createRadialGradient(star.x, star.y, 0, star.x, star.y, starRadius * 2.8);
                glow.addColorStop(0, star.color || '#ffffff');
                glow.addColorStop(1, 'transparent');
                cCtx.fillStyle = glow;
                cCtx.beginPath();
                cCtx.arc(star.x, star.y, starRadius * 2.8, 0, Math.PI * 2);
                cCtx.fill();

                // Star Core
                cCtx.fillStyle = star.color || (isDark ? '#ffffff' : '#0b0d11');
                cCtx.beginPath();
                cCtx.arc(star.x, star.y, starRadius, 0, Math.PI * 2);
                cCtx.fill();

                // Halo Ring around bright stars
                if (isBright || isHovered) {
                    cCtx.beginPath();
                    cCtx.arc(star.x, star.y, starRadius + 3.5, 0, Math.PI * 2);
                    cCtx.strokeStyle = isHovered 
                        ? (isDark ? '#38bdf8' : '#0284c7') 
                        : (isDark ? 'rgba(255,255,255,0.6)' : 'rgba(0,0,0,0.55)');
                    cCtx.lineWidth = 0.6;
                    cCtx.stroke();
                }

                // Hover Reticle
                if (isHovered) {
                    cCtx.beginPath();
                    cCtx.strokeStyle = isDark ? '#38bdf8' : '#0284c7';
                    cCtx.lineWidth = 1;
                    cCtx.strokeRect(star.x - 9, star.y - 9, 18, 18);
                }

                // Star Name Labels (for prominent stars or hovered star)
                if (star.magnitude <= 2.1 || isHovered) {
                    cCtx.font = `${isHovered ? 'bold 9.5px' : '8.5px'} 'JetBrains Mono', monospace`;
                    cCtx.fillStyle = isHovered 
                        ? (isDark ? '#38bdf8' : '#0284c7') 
                        : textMain;
                    cCtx.textAlign = 'left';
                    cCtx.fillText(star.name, star.x + starRadius + 4, star.y + 3);
                }
            }
        }

        // 7. Cardinal Compass Bearings on the Horizon Rim
        if (skyProjection && skyProjection.cardinal_bearings) {
            cCtx.font = "bold 10.5px 'JetBrains Mono', monospace";
            cCtx.textAlign = 'center';
            cCtx.textBaseline = 'middle';

            for (const c of skyProjection.cardinal_bearings) {
                const rad = c.az_deg * Math.PI / 180;
                const lx = -(r + 14) * Math.sin(rad);
                const ly = -(r + 14) * Math.cos(rad);

                cCtx.fillStyle = c.label === 'N' 
                    ? (isDark ? '#f87171' : '#dc2626') 
                    : textMuted;
                cCtx.fillText(c.label, lx, ly);
            }
        }

    } else if (constLayoutMode === 'POSTER') {
        // POSTER GRID CATALOG
        cCtx.font = 'bold 12px "JetBrains Mono", monospace';
        cCtx.fillStyle = strokeColor;
        cCtx.textAlign = 'center';
        cCtx.fillText("CONSTELLATIONS", 0, -320);

        cCtx.font = '8px "JetBrains Mono", monospace';
        cCtx.fillStyle = textMuted;
        cCtx.fillText("SOVEREIGN CELESTIAL VECTOR SET", 0, -305);

        cCtx.beginPath();
        cCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.15)' : 'rgba(0,0,0,0.15)';
        cCtx.lineWidth = 1;
        cCtx.moveTo(-200, -295);
        cCtx.lineTo(200, -295);
        cCtx.stroke();

        cCtx.font = 'bold 9px "JetBrains Mono", monospace';
        cCtx.fillStyle = textMuted;
        cCtx.fillText("ZODIAC CONSTELLATIONS", 0, -280);

        const cols = 4;
        const colSpacing = 160;
        const rowSpacing = 140;

        CONSTELLATIONS_CATALOG.forEach((constell, index) => {
            let gridX, gridY;

            if (index < 12) {
                const c = index % cols;
                const r = Math.floor(index / cols);
                gridX = (c - 1.5) * colSpacing;
                gridY = -200 + r * rowSpacing;
            } else {
                const oIndex = index - 12;
                const c = oIndex % cols;
                const r = Math.floor(oIndex / cols);
                gridX = (c - 1.5) * colSpacing;
                gridY = 240 + r * rowSpacing;

                if (oIndex === 0) {
                    cCtx.font = 'bold 9px "JetBrains Mono", monospace';
                    cCtx.fillStyle = textMuted;
                    cCtx.textAlign = 'center';
                    cCtx.fillText("OTHER CONSTELLATIONS", 0, 160);

                    cCtx.beginPath();
                    cCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.15)' : 'rgba(0,0,0,0.15)';
                    cCtx.moveTo(-200, 145);
                    cCtx.lineTo(200, 145);
                    cCtx.stroke();
                }
            }

            constell._renderX = gridX;
            constell._renderY = gridY;

            const matchSeason = (filterSeason === 'ALL' || constell.season === filterSeason);
            const matchPosition = (filterPosition === 'ALL' || constell.direction === filterPosition);
            const isMatch = matchSeason && matchPosition;
            const isSelected = selectedConstellation === constell;

            cCtx.save();
            cCtx.translate(gridX, gridY);

            cCtx.globalAlpha = isMatch ? (isSelected ? 1.0 : 0.88) : 0.12;

            if (isSelected) {
                cCtx.beginPath();
                cCtx.strokeStyle = strokeColor;
                cCtx.lineWidth = 1;
                cCtx.strokeRect(-65, -55, 130, 110);
            }

            if (showConstLines && constell.lines) {
                cCtx.strokeStyle = strokeColor;
                cCtx.lineWidth = isSelected ? 1.5 : 1.0;
                cCtx.setLineDash([3, 3]);

                constell.lines.forEach(([iA, iB]) => {
                    const sA = constell.stars[iA];
                    const sB = constell.stars[iB];
                    if (sA && sB) {
                        cCtx.beginPath();
                        cCtx.moveTo(sA.x, sA.y);
                        cCtx.lineTo(sB.x, sB.y);
                        cCtx.stroke();
                    }
                });
                cCtx.setLineDash([]);
            }

            constell.stars.forEach(st => {
                cCtx.beginPath();
                cCtx.arc(st.x, st.y, st.bright ? 3.5 : 2.0, 0, Math.PI * 2);
                cCtx.fillStyle = strokeColor;
                cCtx.fill();

                if (st.bright) {
                    cCtx.beginPath();
                    cCtx.arc(st.x, st.y, 6, 0, Math.PI * 2);
                    cCtx.strokeStyle = strokeColor;
                    cCtx.lineWidth = 0.6;
                    cCtx.stroke();
                }
            });

            cCtx.font = 'bold 9px "JetBrains Mono", monospace';
            cCtx.fillStyle = textMain;
            cCtx.textAlign = 'center';
            cCtx.fillText(constell.name, 0, 48);

            cCtx.restore();
        });

    } else {
        // CELESTIAL SPHERE DOME MODE
        cCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.08)';
        cCtx.lineWidth = 1;
        [100, 200, 300, 400].forEach(r => {
            cCtx.beginPath();
            cCtx.arc(0, 0, r, 0, Math.PI * 2);
            cCtx.stroke();
        });

        cCtx.beginPath();
        cCtx.moveTo(-450, 0); cCtx.lineTo(450, 0);
        cCtx.moveTo(0, -450); cCtx.lineTo(0, 450);
        cCtx.stroke();

        CONSTELLATIONS_CATALOG.forEach((constell, index) => {
            const angle = (index / CONSTELLATIONS_CATALOG.length) * Math.PI * 2;
            const dist = 180 + (index % 3) * 80;
            const sphereX = Math.cos(angle) * dist;
            const sphereY = Math.sin(angle) * dist;

            constell._renderX = sphereX;
            constell._renderY = sphereY;

            const matchSeason = (filterSeason === 'ALL' || constell.season === filterSeason);
            const matchPosition = (filterPosition === 'ALL' || constell.direction === filterPosition);
            const isMatch = matchSeason && matchPosition;
            const isSelected = selectedConstellation === constell;

            cCtx.save();
            cCtx.translate(sphereX, sphereY);
            cCtx.globalAlpha = isMatch ? (isSelected ? 1.0 : 0.85) : 0.12;

            if (showConstLines && constell.lines) {
                cCtx.strokeStyle = strokeColor;
                cCtx.lineWidth = 1.0;
                cCtx.setLineDash([2, 3]);
                constell.lines.forEach(([iA, iB]) => {
                    const sA = constell.stars[iA];
                    const sB = constell.stars[iB];
                    if (sA && sB) {
                        cCtx.beginPath();
                        cCtx.moveTo(sA.x, sA.y);
                        cCtx.lineTo(sB.x, sB.y);
                        cCtx.stroke();
                    }
                });
                cCtx.setLineDash([]);
            }

            constell.stars.forEach(st => {
                cCtx.beginPath();
                cCtx.arc(st.x, st.y, st.bright ? 3.0 : 1.8, 0, Math.PI * 2);
                cCtx.fillStyle = strokeColor;
                cCtx.fill();
            });

            cCtx.font = 'bold 8.5px "JetBrains Mono", monospace';
            cCtx.fillStyle = textMain;
            cCtx.textAlign = 'center';
            cCtx.fillText(constell.name, 0, 44);

            cCtx.restore();
        });
    }

    cCtx.restore();
    cCtx.restore();
}

function performConstHover(e) {
    const tooltip = document.getElementById('galaxy-tooltip');
    if (!cCanvas || cCanvas.offsetParent === null) {
        if (tooltip && !tooltip.classList.contains('hidden')) {
            tooltip.classList.add('hidden');
        }
        return;
    }

    const constView = document.getElementById('view-constellations');
    if (constView && constView.classList.contains('hidden')) {
        if (tooltip && !tooltip.classList.contains('hidden')) {
            tooltip.classList.add('hidden');
        }
        return;
    }

    if (constLayoutMode !== 'DOME') {
        if (tooltip && !tooltip.classList.contains('hidden')) {
            tooltip.classList.add('hidden');
        }
        return;
    }

    if (!skyProjection || !skyProjection.visible_stars) return;

    const rect = cCanvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    if (mx < 0 || mx > rect.width || my < 0 || my > rect.height) {
        if (tooltip && !tooltip.classList.contains('hidden')) {
            tooltip.classList.add('hidden');
        }
        if (hoveredSkyStar) {
            hoveredSkyStar = null;
            renderConstellations();
        }
        return;
    }

    const dpr = window.devicePixelRatio || 1;
    const cx = cCanvas.width / (2 * dpr);
    const cy = cCanvas.height / (2 * dpr);

    const invX = (mx - cx - constPanX) / constZoom;
    const invY = (my - cy - constPanY) / constZoom;

    let closest = null;
    let minDist = 18;

    for (const star of skyProjection.visible_stars) {
        if (!star.is_visible && star.alt_deg < 0) continue;
        const dx = star.x - invX;
        const dy = star.y - invY;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < minDist) {
            minDist = dist;
            closest = star;
        }
    }

    const prevStar = hoveredSkyStar;
    hoveredSkyStar = closest;

    if (closest && tooltip) {
        tooltip.classList.remove('hidden');
        tooltip.style.left = `${e.clientX + 14}px`;
        tooltip.style.top = `${e.clientY - 10}px`;
        tooltip.innerHTML = `
            <div class="font-bold">${escapeHtml(closest.name)}</div>
            <div class="text-[9px] opacity-80">${escapeHtml(closest.constellation)} • Mag ${(closest.magnitude || 0).toFixed(2)}</div>
            <div class="text-[8px] opacity-75">Alt: ${(closest.alt_deg || 0).toFixed(1)}° • Az: ${(closest.az_deg || 0).toFixed(1)}°</div>
        `;
    } else if (tooltip && !tooltip.classList.contains('hidden')) {
        tooltip.classList.add('hidden');
    }

    if (prevStar !== closest) {
        renderConstellations();
    }
}

let constHoverRafId = null;
function handleConstHover(e) {
    if (constHoverRafId) return;
    constHoverRafId = requestAnimationFrame(() => {
        constHoverRafId = null;
        performConstHover(e);
    });
}

function setupConstInteractions() {
    if (constEventsBound || !cCanvas) return;
    constEventsBound = true;

    let constPointerDown = { x: 0, y: 0 };

    cCanvas.addEventListener('mouseleave', () => {
        hoveredSkyStar = null;
        const tooltip = document.getElementById('galaxy-tooltip');
        if (tooltip) tooltip.classList.add('hidden');
        renderConstellations();
    });

    cCanvas.addEventListener('mousedown', (e) => {
        if (e.button === 0) {
            constPointerDown = { x: e.clientX, y: e.clientY };
            isDraggingConst = true;
            constDragStartX = e.clientX - constPanX;
            constDragStartY = e.clientY - constPanY;
        }
    });

    cCanvas.addEventListener('click', (e) => {
        const dragDist = Math.hypot(e.clientX - constPointerDown.x, e.clientY - constPointerDown.y);
        if (dragDist > 6) return;

        const rect = cCanvas.getBoundingClientRect();
        const dpr = window.devicePixelRatio || 1;
        const cx = cCanvas.width / (2 * dpr);
        const cy = cCanvas.height / (2 * dpr);

        if (constLayoutMode === 'DOME') {
            if (hoveredSkyStar && onSelectConstCallback) {
                const starDoc = {
                    id: `STAR-${hoveredSkyStar.name.toUpperCase().replace(/\s+/g, '-')}`,
                    doc_id: 'star-navigation-handbook',
                    name: `${hoveredSkyStar.name} (${hoveredSkyStar.constellation})`,
                    category: 'ASTRONOMY',
                    tags: ['CELESTIAL', 'NAV-STAR', hoveredSkyStar.constellation.toUpperCase()],
                    chapters: [
                        `Celestial Navigational Star: ${hoveredSkyStar.name}\nConstellation: ${hoveredSkyStar.constellation}\nApparent Magnitude: ${(hoveredSkyStar.magnitude || 0).toFixed(2)}\nAltitude: ${(hoveredSkyStar.alt_deg || 0).toFixed(1)}°\nAzimuth: ${(hoveredSkyStar.az_deg || 0).toFixed(1)}°\n\nPart of the sovereign Star Navigation & Celestial Lore Handbook. This bright beacon provides azimuth alignment and celestial wayfinding across navigational corridors.`,
                        `Astrometry & Equatorial Coordinates:\nRight Ascension / Declination converted via local sidereal time (LST) and terrestrial latitude.\nObserved from Lat ${skyLat.toFixed(2)}°, Lon ${skyLon.toFixed(2)}°.`
                    ],
                    provenance: {
                        source: "Mazzaroth Astrometry Engine",
                        publisher: "Public Domain Astronomy Collective",
                        license: "public-domain",
                        retrieved_date: "2026-09-30"
                    }
                };
                onSelectConstCallback(starDoc);
            }
        } else {
            const mx = (e.clientX - rect.left - cx - constPanX) / constZoom;
            const my = (e.clientY - rect.top - cy - constPanY) / constZoom;

            let hit = null;
            for (let c of CONSTELLATIONS_CATALOG) {
                const rx = c._renderX || 0;
                const ry = c._renderY || 0;
                if (Math.abs(mx - rx) < 60 && Math.abs(my - ry) < 50) {
                    hit = c;
                    break;
                }
            }

            if (hit) {
                selectedConstellation = hit;
                if (onSelectConstCallback) {
                    onSelectConstCallback({
                        id: hit.id,
                        name: `${hit.name} (${hit.category})`,
                        tags: [hit.season, hit.direction, "CONSTELLATION"],
                        chapters: [
                            hit.desc,
                            `Chapter 2: Celestial Astrometry catalog registers ${hit.stars.length} principal stars anchored in celestial quadrant ${hit.direction}. Optimal observation zenith aligns with the ${hit.season} sky.`,
                            `Chapter 3: Star catalog identifiers: ${hit.stars.map(s => s.name).join(', ')}.`
                        ],
                        connections: ["NEXUS-0"]
                    });
                }
                renderConstellations();
            }
        }
    });

    window.addEventListener('mousemove', (e) => {
        if (isDraggingConst) {
            constPanX = e.clientX - constDragStartX;
            constPanY = e.clientY - constDragStartY;
            renderConstellations();
        } else {
            handleConstHover(e);
        }
    });

    window.addEventListener('mouseup', () => {
        isDraggingConst = false;
    });

    cCanvas.addEventListener('wheel', (e) => {
        e.preventDefault();
        const factor = e.deltaY < 0 ? 1.12 : 0.89;
        constZoom = Math.max(0.3, Math.min(4.0, constZoom * factor));
        renderConstellations();
    }, { passive: false });
}

/* ========================================================================= */
/* 4. SOVEREIGN CARTOGRAPHY MAP MODULE                                       */
/* ========================================================================= */
let mCanvas = null;
let mCtx = null;
let mapZoom = 1.0;
let mapPanX = 0;
let mapPanY = 0;
let isDraggingMap = false;
let mapDragStartX = 0;
let mapDragStartY = 0;
let showGraticule = true;
let onSelectGeocacheCallback = null;
let mapEventsBound = false;

// Dual-Mode Map Operating State
let mapOperatingMode = 'HEXAGON'; // 'HEXAGON' | 'OFFLINE'
let activeMapDatasetKey = 'TOTAL';
let hoveredHex = null;
let hoveredCountry = null;
let selectedMapRegion = null;
let selectedCountry = null;
let hexWorldGrid = [];

export const MAP_DATASETS = {
    "TOTAL": {
        id: "TOTAL",
        title: "TOTAL CHRISTIAN POPULATION (~2.42B)",
        metricLabel: "Adherents Worldwide",
        regions: {
            "NA": { name: "North America", count: "260,000,000", pct: "70.2%", intensity: 0.78, chapters: ["North America contains approximately 260M Christian adherents across the United States and Canada, representing an influential history of Protestant, Evangelical, and Catholic settlement.", "Chapter 2: Major manuscript holding centers include the Smithsonian collections, Princeton Theological Seminary archives, and Harvard Houghton codices."] },
            "LA": { name: "Latin America", count: "592,000,000", pct: "83.4%", intensity: 0.95, chapters: ["Latin America holds the largest contiguous concentration of Christian believers on Earth, exceeding 592M adherents across Brazil, Mexico, Colombia, and Argentina.", "Chapter 2: Primarily Roman Catholic (approx 70%) with a surging Evangelical and Pentecostal demographic expanding across urban coastal corridors."] },
            "EU": { name: "Europe", count: "540,000,000", pct: "72.1%", intensity: 0.82, chapters: ["Europe represents the historic cradle of both Western Catholic and Eastern Orthodox traditions, accounting for approximately 540M adherents.", "Chapter 2: Contains foundational archival centers: the Vatican Apostolic Library (Rome), Mt. Athos monasteries (Greece), and the British Library (London)."] },
            "AF": { name: "Sub-Saharan Africa", count: "685,000,000", pct: "62.8%", intensity: 0.98, chapters: ["Sub-Saharan Africa is currently the global epicentre of numerical Christian growth, with over 685M believers across Nigeria, DRC, Ethiopia, Kenya, and South Africa.", "Chapter 2: Features ancient continuous Christian heritage in the Ethiopian Orthodox Tewahedo Church dating back to the 4th century Axumite Kingdom."] },
            "ME": { name: "Middle East & Levant", count: "18,500,000", pct: "3.2%", intensity: 0.40, chapters: ["The geographic birthplace of the faith, retaining ancient Apostolic communities: Syriac, Coptic, Armenian, Maronite, and Melkite churches.", "Chapter 2: Anchors of early Christian literature: St. Catherine's Monastery (Sinai Codex Sinaiticus), the Patriarchate of Jerusalem, and Antioch."] },
            "AP": { name: "Asia-Pacific", count: "380,000,000", pct: "8.9%", intensity: 0.55, chapters: ["Asia-Pacific hosts approximately 380M Christians, with massive concentrations in the Philippines (~90M, 85%), China (est. 60-80M), South Korea (~14M, 28%), and India (~32M).", "Chapter 2: Dynamic diversity between ancient St. Thomas Christian communities in Kerala and vibrant house church networks across East Asia."] }
        }
    },
    "CATHOLIC": {
        id: "CATHOLIC",
        title: "ROMAN CATHOLIC CHURCH (~1.38B)",
        metricLabel: "Catholic Baptized Population",
        regions: {
            "NA": { name: "North America", count: "82,000,000", pct: "22.1%", intensity: 0.60, chapters: ["North American Catholicism comprises approximately 72M in the US and 10M in Canada, concentrated in historic urban dioceses and Hispanic populations."] },
            "LA": { name: "Latin America", count: "480,000,000", pct: "67.5%", intensity: 0.96, chapters: ["Home to nearly 39% of the world's Catholic population. Brazil is the largest single Catholic nation with over 120M baptized adherents."] },
            "EU": { name: "Europe", count: "260,000,000", pct: "34.8%", intensity: 0.85, chapters: ["Centered at the Holy See in Vatican City, with high historical density in Italy, Spain, France, Poland, Portugal, and Ireland."] },
            "AF": { name: "Sub-Saharan Africa", count: "240,000,000", pct: "22.0%", intensity: 0.88, chapters: ["Rapidly expanding dioceses across the Democratic Republic of Congo, Nigeria, Uganda, and Tanzania, supplying global missionary vocations."] },
            "ME": { name: "Middle East & Levant", count: "3,800,000", pct: "0.8%", intensity: 0.35, chapters: ["Eastern Rite Catholic jurisdictions including the Maronite Church of Lebanon, the Melkite Greek Catholic Church, and the Chaldean Catholic Church."] },
            "AP": { name: "Asia-Pacific", count: "145,000,000", pct: "3.4%", intensity: 0.65, chapters: ["Predominant in the Philippines (85M, third largest in world), Timor-Leste (98%), with significant communities in Vietnam and South Korea."] }
        }
    },
    "PROTESTANT": {
        id: "PROTESTANT",
        title: "PROTESTANT & EVANGELICAL (~900M)",
        metricLabel: "Protestant & Independent Adherents",
        regions: {
            "NA": { name: "North America", count: "165,000,000", pct: "44.6%", intensity: 0.90, chapters: ["Broad historic spectrum of Reformed, Baptist, Methodist, Lutheran, and modern non-denominational evangelical movements."] },
            "LA": { name: "Latin America", count: "105,000,000", pct: "14.8%", intensity: 0.70, chapters: ["Remarkable demographic expansion over the last four decades, particularly strong in Brazil, Guatemala, and urban hubs."] },
            "EU": { name: "Europe", count: "95,000,000", pct: "12.7%", intensity: 0.65, chapters: ["Historic cradle of the Protestant Reformation: Lutheran Nordic nations, Anglican British Isles, and Reformed Germanic cantons."] },
            "AF": { name: "Sub-Saharan Africa", count: "390,000,000", pct: "35.7%", intensity: 0.98, chapters: ["The largest continental population of Protestants globally, characterized by dynamic Anglican, Methodist, and Pentecostal assemblies."] },
            "ME": { name: "Middle East & Levant", count: "1,200,000", pct: "0.2%", intensity: 0.20, chapters: ["Small historical missionary communities and modern expatriate fellowships in the Persian Gulf."] },
            "AP": { name: "Asia-Pacific", count: "144,000,000", pct: "3.4%", intensity: 0.60, chapters: ["Prominent Presbyterian and Methodist heritages in South Korea, alongside massive independent and house church networks in mainland China."] }
        }
    },
    "ORTHODOX": {
        id: "ORTHODOX",
        title: "EASTERN & ORIENTAL ORTHODOXY (~300M)",
        metricLabel: "Orthodox Communion Adherents",
        regions: {
            "NA": { name: "North America", count: "6,500,000", pct: "1.7%", intensity: 0.35, chapters: ["Immigrant diaspora dioceses (Greek, Russian, Antiochian, Coptic) preserving ancient Byzantine and Oriental liturgies."] },
            "LA": { name: "Latin America", count: "1,500,000", pct: "0.2%", intensity: 0.20, chapters: ["Orthodox parish communities established primarily in São Paulo, Buenos Aires, and Mexico City."] },
            "EU": { name: "Europe", count: "185,000,000", pct: "24.7%", intensity: 0.95, chapters: ["Heartland of Eastern Orthodoxy: Russia, Greece, Romania, Serbia, Bulgaria, Ukraine, and Georgia. Governed by autocephalous synods."] },
            "AF": { name: "Sub-Saharan Africa", count: "55,000,000", pct: "5.0%", intensity: 0.85, chapters: ["Dominated by the ancient Ethiopian Orthodox Tewahedo Church (approx 45M) and Eritrean Church, preserving Ge'ez liturgical manuscripts."] },
            "ME": { name: "Middle East & Levant", count: "13,500,000", pct: "2.3%", intensity: 0.75, chapters: ["The historic Coptic Orthodox Church of Egypt (approx 10M), the Greek Orthodox Patriarchate of Antioch, and the Armenian Apostolic Church."] },
            "AP": { name: "Asia-Pacific", count: "38,500,000", pct: "0.9%", intensity: 0.40, chapters: ["The ancient St. Thomas Christians of the Malankara Orthodox Syrian Church in Kerala, India (dating traditionally to 52 AD)."] }
        }
    },
    "MANUSCRIPTS": {
        id: "MANUSCRIPTS",
        title: "ANCIENT BIBLICAL CODICES & REPOSITORIES",
        metricLabel: "Primary Manuscript Hubs",
        regions: {
            "ME": { name: "Sinai & Levant Hubs", count: "St. Catherine's & Jerusalem", pct: "4th-10th Cent.", intensity: 1.0, chapters: ["Sinai holds Codex Sinaiticus (c. 330-360 AD), the Syriac Sinaiticus, and thousands of intact uncial fragments. The Dead Sea Scrolls are preserved in Jerusalem."] },
            "EU": { name: "Vatican, London & Athos", count: "Codex Vaticanus & Alexandrinus", pct: "Primary Uncials", intensity: 0.98, chapters: ["The Vatican Apostolic Library houses Codex Vaticanus B (c. 325 AD). The British Library preserves Codex Alexandrinus and Sinaiticus leaves."] },
            "AF": { name: "Alexandria & Lake Tana", count: "Coptic & Ge'ez Codices", pct: "3rd-14th Cent.", intensity: 0.90, chapters: ["The Coptic Museum and monastery archives of Wadi El Natrun preserve Sahidic and Bohairic papyri. Lake Tana island monasteries house illuminated Ge'ez Gospels."] },
            "NA": { name: "Smithsonian & Chester Beatty", count: "Papyrus 45, 46, 47", pct: "2nd-3rd Cent.", intensity: 0.70, chapters: ["North American and Dublin collections host the earliest known Pauline epistles and Gospel papyri fragments, demonstrating extraordinary textual reliability."] },
            "LA": { name: "Colonial Cathedrals", count: "Polyglot Bibles", pct: "16th Cent.", intensity: 0.40, chapters: ["Puebla and Lima archives maintain Antwerp and London Polyglot editions utilized for missionary translation work."] },
            "AP": { name: "Kerala Nasrani Archives", count: "Peshitta Papyri", pct: "Early Syriac", intensity: 0.55, chapters: ["Palm-leaf manuscripts and copper plate charters preserving the East Syriac Peshitta tradition in south India."] }
        }
    }
};

export const COUNTRY_DATA = {
    "US": { id: "US", name: "United States", flag: "🇺🇸", region: "NA", lon: -95, lat: 38, count: "230,000,000", pct: "70.6%", centers: "Smithsonian, Princeton Seminary, Houghton Library", chapters: ["The United States holds one of the most diverse concentrations of Christian traditions globally, spanning Protestant, Evangelical, Roman Catholic, and Eastern Orthodox communities.", "Chapter 2: Major archival collections include the Chester Beatty papyri facsimiles, the Morgan Library Gutenberg Bibles, and Princeton Theological Seminary cuneiform and uncial archives."] },
    "CA": { id: "CA", name: "Canada", flag: "🇨🇦", region: "NA", lon: -105, lat: 55, count: "22,000,000", pct: "57.5%", centers: "Toronto Pontifical Institute, McGill, Laval", chapters: ["Canada retains historic Catholic foundations in Quebec dating to 1608 alongside Anglican and Presbyterian settlements in Ontario and Maritime provinces.", "Chapter 2: Contains prominent rare book holdings at the Pontifical Institute of Mediaeval Studies (Toronto) and McGill University Rare Books."] },
    "MX": { id: "MX", name: "Mexico", flag: "🇲🇽", region: "NA", lon: -102, lat: 23, count: "115,000,000", pct: "89.3%", centers: "Basilica of Guadalupe, Puebla Cathedral Library", chapters: ["Mexico has the second-largest Catholic population worldwide, anchored historically in 16th-century Franciscan and Dominican missionary foundations.", "Chapter 2: Houses the historic Biblioteca Palafoxiana in Puebla (founded 1646), the first public library in the Americas preserving polyglot scripture codices."] },
    "BR": { id: "BR", name: "Brazil", flag: "🇧🇷", region: "LA", lon: -52, lat: -14, count: "180,000,000", pct: "86.8%", centers: "Aparecida National Sanctuary, São Paulo Archdiocesan Archives", chapters: ["Brazil is home to the largest total Roman Catholic community on Earth (approx 120M) alongside a dynamic Evangelical and Pentecostal demographic exceeding 50M.", "Chapter 2: Major centers include the National Sanctuary of Aparecida and Rio de Janeiro Benedictine Monastery collections."] },
    "CO": { id: "CO", name: "Colombia", flag: "🇨🇴", region: "LA", lon: -73, lat: 4, count: "45,000,000", pct: "92.1%", centers: "Bogotá Archdiocesan Archive, Las Lajas Sanctuary", chapters: ["Colombia has maintained a continuous Catholic cultural heritage since the 16th century, preserving early missionary catechetical manuscripts."] },
    "AR": { id: "AR", name: "Argentina", flag: "🇦🇷", region: "LA", lon: -64, lat: -34, count: "36,000,000", pct: "80.0%", centers: "Buenos Aires Metropolitan Cathedral, Jesuit Reductions", chapters: ["Argentina features deep Jesuit historical roots across the Rio de la Plata, with historic archival repositories in Córdoba and Buenos Aires."] },
    "GB": { id: "GB", name: "United Kingdom", flag: "🇬🇧", region: "EU", lon: -2, lat: 54, count: "33,000,000", pct: "48.0%", centers: "British Library, Oxford Bodleian, Cambridge", chapters: ["The United Kingdom preserves some of the world's most critical biblical codices: Codex Alexandrinus (5th c.) and major leaves of Codex Sinaiticus (4th c.) at the British Library.", "Chapter 2: The Bodleian Library at Oxford houses extensive Greek, Latin, and Ge'ez scripture manuscripts."] },
    "DE": { id: "DE", name: "Germany", flag: "🇩🇪", region: "EU", lon: 10, lat: 51, count: "43,000,000", pct: "52.0%", centers: "Wittenberg Luther House, Gutenberg Museum Mainz", chapters: ["Germany is the historic cradle of the Protestant Reformation (1517). Mainz produced the Gutenberg 42-Line Bible (1455), the first major book printed using movable metal type."] },
    "FR": { id: "FR", name: "France", flag: "🇫🇷", region: "EU", lon: 2, lat: 47, count: "36,000,000", pct: "54.0%", centers: "Bibliothèque Nationale de France, Cluny, Avignon", chapters: ["The Bibliothèque Nationale de France in Paris preserves Codex Ephraemi Rescriptus (5th c.) and irreplaceable Carolingian illuminated Gospel books."] },
    "IT": { id: "IT", name: "Italy & Vatican", flag: "🇮🇹", region: "EU", lon: 12.5, lat: 42, count: "48,000,000", pct: "80.5%", centers: "Vatican Apostolic Library, St. Peter's, Roman Catacombs", chapters: ["The Vatican Apostolic Library houses Codex Vaticanus Graecus 1209 (c. 325 AD), one of the oldest and most authoritative Greek manuscripts of the Bible."] },
    "ES": { id: "ES", name: "Spain", flag: "🇪🇸", region: "EU", lon: -4, lat: 40, count: "34,000,000", pct: "72.0%", centers: "El Escorial Royal Library, Complutensian Polyglot", chapters: ["Spain was the birthplace of the Complutensian Polyglot Bible (1514-1517), the first printed multi-language edition of the entire Bible (Hebrew, Greek, Aramaic, Latin)."] },
    "PL": { id: "PL", name: "Poland", flag: "🇵🇱", region: "EU", lon: 19, lat: 52, count: "33,000,000", pct: "88.0%", centers: "Jasna Góra Monastery, Jagiellonian Library", chapters: ["Poland has one of the highest densities of Catholic adherence in Europe, preserving historic Latin and Polish scripture translations from the 15th century."] },
    "GR": { id: "GR", name: "Greece", flag: "🇬🇷", region: "EU", lon: 22, lat: 39, count: "10,000,000", pct: "93.0%", centers: "Mt. Athos Monasteries, Patmos Apocalypse Grotto", chapters: ["Mt. Athos (Holy Mountain) autonomous monastic republic preserves over 15,000 Byzantine and post-Byzantine Greek biblical manuscripts."] },
    "UA": { id: "UA", name: "Ukraine", flag: "🇺🇦", region: "EU", lon: 31, lat: 49, count: "30,000,000", pct: "78.0%", centers: "Kyiv Pechersk Lavra, Ostrog Bible Archives", chapters: ["Home to the Ostrog Bible (1581), the first complete printed edition of the Old and New Testaments in Church Slavonic."] },
    "NG": { id: "NG", name: "Nigeria", flag: "🇳🇬", region: "AF", lon: 8, lat: 9, count: "95,000,000", pct: "46.5%", centers: "Lagos Theological Hubs, Jos Archival Center", chapters: ["Nigeria possesses the largest Christian population on the African continent, featuring massive Anglican, Catholic, and Pentecostal assemblies."] },
    "CD": { id: "CD", name: "DR Congo", flag: "🇨🇩", region: "AF", lon: 23, lat: -2, count: "85,000,000", pct: "92.0%", centers: "Kinshasa Archdiocesan Archive, Kisangani", chapters: ["The Democratic Republic of the Congo has the largest Catholic population in Africa and a rapidly growing theological faculty network."] },
    "ET": { id: "ET", name: "Ethiopia", flag: "🇪🇹", region: "AF", lon: 39, lat: 9, count: "65,000,000", pct: "62.0%", centers: "Axum St. Mary of Zion, Lake Tana Monasteries, Lalibela", chapters: ["The Ethiopian Orthodox Tewahedo Church preserves an ancient continuous tradition dating back to King Ezana of Axum (c. 330 AD), with thousands of illuminated Ge'ez Gospel codices."] },
    "KE": { id: "KE", name: "Kenya", flag: "🇰🇪", region: "AF", lon: 37, lat: 0, count: "43,000,000", pct: "85.0%", centers: "Nairobi Theological Colleges", chapters: ["Kenya serves as a central publishing and translation crossroads for over 60 East African languages."] },
    "ZA": { id: "ZA", name: "South Africa", flag: "🇿🇦", region: "AF", lon: 25, lat: -29, count: "44,000,000", pct: "78.0%", centers: "Cape Town Archives, Genadendal Mission", chapters: ["South Africa maintains deep archival collections from early Dutch Reformed, Anglican, and Moravian missionary translation initiatives."] },
    "EG": { id: "EG", name: "Egypt", flag: "🇪🇬", region: "ME", lon: 30, lat: 27, count: "11,000,000", pct: "10.0%", centers: "Coptic Orthodox Patriarchate, Wadi El Natrun, St. Catherine (Sinai)", chapters: ["Egypt is home to the ancient Coptic Orthodox Church founded by St. Mark the Evangelist. The desert monasteries of Wadi El Natrun and Sinai preserve 3rd-century Sahidic and Bohairic papyri."] },
    "IL": { id: "IL", name: "Israel & Palestine", flag: "🇮🇱", region: "ME", lon: 35.2, lat: 31.8, count: "300,000", pct: "2.5%", centers: "Jerusalem Holy Sepulchre, Shrine of the Book (Dead Sea Scrolls)", chapters: ["The historic geographic ground zero of both the Old and New Testaments. Jerusalem holds the Dead Sea Scrolls (dating from 3rd c. BC to 1st c. AD)."] },
    "LB": { id: "LB", name: "Lebanon & Syria", flag: "🇱🇧", region: "ME", lon: 35.8, lat: 33.8, count: "2,500,000", pct: "36.0%", centers: "Maronite Bkerké Patriarchate, Antiochian Patriarchate", chapters: ["Preserves ancient Eastern Rite Catholic and Orthodox heritage in Syriac, Aramaic, and Arabic liturgical traditions."] },
    "TR": { id: "TR", name: "Turkey", flag: "🇹🇷", region: "ME", lon: 35, lat: 39, count: "300,000", pct: "0.4%", centers: "Ecumenical Patriarchate of Constantinople, Seven Churches of Asia", chapters: ["Asia Minor was the setting for the missionary journeys of Paul and the Seven Churches of Revelation (Ephesus, Smyrna, Pergamum, etc.)."] },
    "SA": { id: "SA", name: "Saudi Arabia & Gulf", flag: "🇸🇦", region: "ME", lon: 45, lat: 24, count: "2,200,000", pct: "4.0%", centers: "Historic Najran Inscriptions, Expatriate Fellowships", chapters: ["Features pre-Islamic Christian inscriptions in Najran alongside international expatriate church communities."] },
    "PH": { id: "PH", name: "Philippines", flag: "🇵🇭", region: "AP", lon: 122, lat: 13, count: "92,000,000", pct: "85.0%", centers: "San Agustin Church Manila, Santo Niño de Cebu", chapters: ["The Philippines is the largest Christian nation in Asia (approx 85M Catholics, 10M Protestants), rooted in 1521 Spanish foundations."] },
    "CN": { id: "CN", name: "China", flag: "🇨🇳", region: "AP", lon: 104, lat: 35, count: "70,000,000", pct: "5.0%", centers: "Amity Printing Company Nanjing, Xi'an Nestorian Stele", chapters: ["China hosts the Amity Printing Company in Nanjing (over 200 million Bibles printed in 100+ languages). The Xi'an Stele records Christian presence dating back to 635 AD (Tang Dynasty)."] },
    "IN": { id: "IN", name: "India", flag: "🇮🇳", region: "AP", lon: 79, lat: 21, count: "32,000,000", pct: "2.4%", centers: "St. Thomas Mount (Chennai), Malankara Archives (Kerala)", chapters: ["The St. Thomas Christians of Kerala trace their tradition directly to the Apostle Thomas arriving in Muziris (52 AD), preserving ancient East Syriac and Malayalam manuscripts."] },
    "KR": { id: "KR", name: "South Korea", flag: "🇰🇷", region: "AP", lon: 128, lat: 36, count: "14,000,000", pct: "28.0%", centers: "Seoul Theological Centers, Yanghwajin Martyr Archives", chapters: ["South Korea has an extraordinary history of self-initiated Christian learning in the late 18th century, becoming one of the largest missionary-sending nations in the world."] },
    "JP": { id: "JP", name: "Japan", flag: "🇯🇵", region: "AP", lon: 138, lat: 37, count: "2,000,000", pct: "1.5%", centers: "Nagasaki Kakure Kirishitan Sites, Oura Cathedral", chapters: ["Features the remarkable history of the Hidden Christians (Kakure Kirishitan) who preserved their faith and Latin/Portuguese prayers underground for over 250 years."] },
    "ID": { id: "ID", name: "Indonesia", flag: "🇮🇩", region: "AP", lon: 118, lat: -2, count: "29,000,000", pct: "10.5%", centers: "North Sumatra Batak Church (HKBP), Moluccas Dioceses", chapters: ["Indonesia has a vibrant Christian population exceeding 29 million, with major concentrations in North Sumatra, North Sulawesi, and Papua."] },
    "AU": { id: "AU", name: "Australia", flag: "🇦🇺", region: "AP", lon: 134, lat: -25, count: "11,000,000", pct: "44.0%", centers: "St. Mary's Cathedral Sydney, Australian Bible Society", chapters: ["Australia maintains extensive Pacific Bible translation archives and historical missionary records across Indigenous and diaspora communities."] },
    "CU": { id: "CU", name: "Cuba & Caribbean", flag: "🇨🇺", region: "NA", lon: -79, lat: 22, count: "6,800,000", pct: "60.0%", centers: "Havana Cathedral Archives, Santiago de Cuba", chapters: ["Cuba preserves early 16th-century Spanish colonial missions and diocesan registries dating back to the bishopric of Baracoa (1518)."] },
    "GT": { id: "GT", name: "Central America", flag: "🇬🇹", region: "NA", lon: -89, lat: 14.5, count: "42,000,000", pct: "86.0%", centers: "Antigua Guatemala, San Salvador Cathedral", chapters: ["Central America spans historic missionary hubs across Guatemala, Honduras, El Salvador, Nicaragua, Costa Rica, and Panama, retaining extensive colonial manuscripts."] }
};

export const MAP_HOTSPOTS = {
    "TOTAL": [
        { lon: -95.0, lat: 38.0, code: "US", weight: 1.0 },
        { lon: -105.0, lat: 55.0, code: "CA", weight: 0.85 },
        { lon: -102.0, lat: 23.0, code: "MX", weight: 0.90 },
        { lon: -52.0, lat: -14.0, code: "BR", weight: 1.0 },
        { lon: -73.0, lat: 4.0, code: "CO", weight: 0.88 },
        { lon: -64.0, lat: -34.0, code: "AR", weight: 0.88 },
        { lon: 12.5, lat: 42.0, code: "IT", weight: 1.0 },
        { lon: 10.0, lat: 51.0, code: "DE", weight: 0.95 },
        { lon: -2.0, lat: 54.0, code: "GB", weight: 0.92 },
        { lon: 2.0, lat: 47.0, code: "FR", weight: 0.90 },
        { lon: 8.0, lat: 9.0, code: "NG", weight: 1.0 },
        { lon: 23.0, lat: -2.0, code: "CD", weight: 0.98 },
        { lon: 39.0, lat: 9.0, code: "ET", weight: 0.95 },
        { lon: 30.0, lat: 27.0, code: "EG", weight: 0.92 },
        { lon: 35.2, lat: 31.8, code: "IL", weight: 1.0 },
        { lon: 122.0, lat: 13.0, code: "PH", weight: 1.0 },
        { lon: 104.0, lat: 35.0, code: "CN", weight: 0.90 },
        { lon: 79.0, lat: 21.0, code: "IN", weight: 0.88 },
        { lon: 128.0, lat: 36.0, code: "KR", weight: 0.92 },
        { lon: 138.0, lat: 37.0, code: "JP", weight: 0.80 },
        { lon: 134.0, lat: -25.0, code: "AU", weight: 0.85 }
    ],
    "CATHOLIC": [
        { lon: 12.5, lat: 42.0, code: "IT", weight: 1.0 },
        { lon: -4.0, lat: 40.0, code: "ES", weight: 0.95 },
        { lon: -52.0, lat: -14.0, code: "BR", weight: 1.0 },
        { lon: -102.0, lat: 23.0, code: "MX", weight: 0.98 },
        { lon: 122.0, lat: 13.0, code: "PH", weight: 1.0 },
        { lon: 23.0, lat: -2.0, code: "CD", weight: 0.95 },
        { lon: 19.0, lat: 52.0, code: "PL", weight: 0.92 }
    ],
    "PROTESTANT": [
        { lon: -95.0, lat: 38.0, code: "US", weight: 1.0 },
        { lon: 10.0, lat: 51.0, code: "DE", weight: 0.95 },
        { lon: -2.0, lat: 54.0, code: "GB", weight: 0.92 },
        { lon: 8.0, lat: 9.0, code: "NG", weight: 1.0 },
        { lon: 37.0, lat: 0.0, code: "KE", weight: 0.92 },
        { lon: 128.0, lat: 36.0, code: "KR", weight: 0.95 }
    ],
    "ORTHODOX": [
        { lon: 22.0, lat: 39.0, code: "GR", weight: 1.0 },
        { lon: 39.0, lat: 9.0, code: "ET", weight: 1.0 },
        { lon: 30.0, lat: 27.0, code: "EG", weight: 0.95 },
        { lon: 31.0, lat: 49.0, code: "UA", weight: 0.92 },
        { lon: 35.8, lat: 33.8, code: "LB", weight: 0.90 }
    ],
    "MANUSCRIPTS": [
        { lon: 30.0, lat: 27.0, code: "EG", weight: 1.0 },
        { lon: 35.2, lat: 31.8, code: "IL", weight: 1.0 },
        { lon: 12.5, lat: 42.0, code: "IT", weight: 1.0 },
        { lon: -2.0, lat: 54.0, code: "GB", weight: 0.95 },
        { lon: 22.0, lat: 39.0, code: "GR", weight: 0.95 },
        { lon: 39.0, lat: 9.0, code: "ET", weight: 0.92 }
    ]
};

export function registerMapDataset(key, data) {
    if (!key || !data) return;
    MAP_DATASETS[key] = data;
}

export function getMapOperatingMode() {
    return mapOperatingMode;
}

export function getActiveMapDataset() {
    return MAP_DATASETS[activeMapDatasetKey];
}

const REGION_POLYGONS = {
    "NA": [
        // North America Main (USA, Canada, Alaska, Northern Mexico)
        [[-168, 65], [-160, 71], [-140, 70], [-120, 70], [-90, 70], [-80, 62], [-65, 58], [-55, 48], [-65, 44], [-75, 38], [-80, 26], [-82, 24], [-97, 26], [-105, 20], [-90, 16], [-98, 16], [-105, 22], [-115, 30], [-120, 34], [-124, 48], [-130, 54], [-140, 60], [-150, 60], [-165, 60], [-168, 65]],
        // Central America isthmus
        [[-92, 16], [-88, 14], [-84, 10], [-77, 8], [-78, 7], [-83, 8], [-87, 13], [-92, 16]],
        // Cuba
        [[-85, 21.5], [-74, 20], [-74, 23.5], [-85, 23.5], [-85, 21.5]],
        // Hispaniola & Puerto Rico
        [[-74.5, 17.5], [-65, 17.5], [-65, 20.2], [-74.5, 20.2], [-74.5, 17.5]],
        // Greenland
        [[-55, 60], [-45, 60], [-35, 66], [-20, 75], [-20, 82], [-30, 83], [-55, 83], [-65, 78], [-55, 70], [-55, 60]],
        // Arctic Archipelago
        [[-125, 70], [-100, 70], [-80, 73], [-70, 78], [-85, 82], [-110, 78], [-125, 74], [-125, 70]]
    ],
    "LA": [
        // South America Main
        [[-78, 8], [-72, 11], [-60, 10], [-50, 0], [-35, -5], [-35, -10], [-38, -18], [-42, -23], [-48, -28], [-53, -33], [-58, -38], [-65, -54], [-70, -55], [-75, -52], [-75, -45], [-72, -35], [-76, -18], [-81, -5], [-80, 2], [-78, 8]],
        // Tierra del Fuego
        [[-70, -52], [-65, -52], [-65, -55], [-70, -55], [-70, -52]]
    ],
    "EU": [
        // Western / Central / Eastern Europe
        [[-9, 36], [-9, 43], [-1, 46], [-5, 48], [2, 51], [8, 54], [10, 55], [14, 54], [22, 55], [28, 55], [30, 46], [28, 41], [23, 38], [15, 38], [15, 42], [8, 44], [3, 42], [-5, 36], [-9, 36]],
        // Scandinavia & Finland
        [[5, 58], [10, 58], [14, 68], [28, 71], [34, 68], [30, 60], [24, 60], [18, 56], [10, 56], [5, 58]],
        // Great Britain & Ireland
        [[-10, 51], [-6, 51], [-5, 55], [1.5, 52], [0, 58], [-4, 59], [-6, 56], [-10, 54], [-10, 51]],
        // Iceland
        [[-24, 63], [-14, 63], [-14, 66], [-24, 66], [-24, 63]],
        // Italy & Sicily
        [[8, 45], [13, 46], [18, 41], [16, 38], [14, 37], [12, 38], [10, 44], [8, 45]],
        [[12, 36.5], [15.5, 36.5], [15.5, 38.5], [12, 38.5], [12, 36.5]],
        // Greece & Balkans
        [[19, 42], [28, 42], [28, 36], [23, 36], [20, 39], [19, 42]]
    ],
    "AF": [
        // Continental Africa
        [[-17, 15], [-12, 28], [-9, 36], [10, 37], [25, 32], [33, 31], [35, 28], [43, 12], [51, 11], [42, -2], [40, -12], [35, -25], [28, -34], [18, -34], [12, -18], [9, -4], [3, 5], [-12, 5], [-17, 15]],
        // Madagascar
        [[43, -12], [50.5, -12], [50.5, -25.5], [43, -25.5], [43, -12]]
    ],
    "ME": [
        // Arabian Peninsula & Levant
        [[26, 42], [42, 42], [48, 30], [56, 26], [60, 22], [55, 16], [45, 12], [36, 15], [34, 28], [35, 36], [26, 40], [26, 42]],
        // Persia & Anatolia / Caucasus
        [[45, 38], [60, 42], [75, 42], [75, 30], [60, 25], [50, 30], [45, 38]]
    ],
    "AP": [
        // Siberia & East Asia
        [[30, 60], [60, 70], [80, 73], [100, 76], [130, 74], [170, 68], [180, 65], [170, 60], [160, 52], [142, 50], [130, 42], [100, 50], [70, 52], [50, 55], [30, 60]],
        // India & subcontinent
        [[68, 24], [72, 32], [80, 32], [90, 25], [92, 21], [85, 16], [80, 8], [77, 8], [72, 18], [68, 24]],
        // Sri Lanka
        [[79.5, 6], [82, 6], [82, 10], [79.5, 10], [79.5, 6]],
        // China / Central & Eastern Asia
        [[75, 30], [90, 40], [105, 45], [122, 42], [122, 30], [118, 22], [108, 20], [98, 24], [85, 28], [75, 30]],
        // Korean Peninsula
        [[124, 34], [130, 35], [130, 42], [124, 40], [124, 34]],
        // Japan Archipelago (Honshu, Hokkaido, Kyushu, Shikoku)
        [[128, 31], [131, 33], [137, 34], [141, 40], [145.5, 44.5], [141, 45.5], [139, 41.5], [136, 36.5], [132, 34.5], [128, 31]],
        // Taiwan
        [[120, 21.8], [122.2, 21.8], [122.2, 25.4], [120, 25.4], [120, 21.8]],
        // Indochina & Southeast Asia
        [[93, 22], [108, 22], [109, 10], [104, 1], [100, 4], [98, 10], [93, 16], [93, 22]],
        // Indonesia (Sumatra, Java, Borneo, Sulawesi)
        [[95, 5.5], [106, -6], [116, -8.5], [125, -8.5], [141, -2], [150, -5], [141, -9], [120, -10], [100, 0], [95, 5.5]],
        [[108, 4], [119, 4], [119, -4.5], [108, -2], [108, 4]],
        // Philippines
        [[119, 18.5], [126.5, 18.5], [126.5, 5.5], [121, 5.5], [119, 12], [119, 18.5]],
        // Australia
        [[113, -22], [120, -14], [135, -12], [142, -11], [146, -18], [153, -28], [150, -38], [140, -38], [130, -32], [116, -35], [113, -26], [113, -22]],
        // Tasmania
        [[144, -41], [148, -41], [148, -44], [144, -44], [144, -41]],
        // New Zealand
        [[166, -46], [172, -41], [178, -37], [174, -35], [172, -40], [168, -45], [166, -46]]
    ]
};

function pointInPolygon(pt, poly) {
    const [x, y] = pt;
    let inside = false;
    for (let i = 0, j = poly.length - 1; i < poly.length; j = i++) {
        const [xi, yi] = poly[i];
        const [xj, yj] = poly[j];
        const intersect = ((yi > y) !== (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if (intersect) inside = !inside;
    }
    return inside;
}

function generateHexWorldGrid() {
    hexWorldGrid = [];
    const scaleX = 2.4;
    const scaleY = 2.4;
    const dLon = 2.1;
    const dLat = 1.8;
    const hexRadius = 2.65;

    let rowIdx = 0;
    for (let lat = 78; lat >= -56; lat -= dLat) {
        rowIdx++;
        const offset = (rowIdx % 2) * (dLon / 2);
        for (let lon = -175 + offset; lon <= 180; lon += dLon) {
            let matchedRegion = null;
            for (let [regId, polys] of Object.entries(REGION_POLYGONS)) {
                for (let p of polys) {
                    if (pointInPolygon([lon, lat], p)) {
                        matchedRegion = regId;
                        break;
                    }
                }
                if (matchedRegion) break;
            }

            if (matchedRegion) {
                // Associate each cell with its nearest country within the region
                let matchedCountry = null;
                let minCDist = 9999;
                for (const c of Object.values(COUNTRY_DATA)) {
                    if (c.region === matchedRegion) {
                        const d = Math.hypot(lon - c.lon, (lat - c.lat) * 1.15);
                        if (d < minCDist) {
                            minCDist = d;
                            matchedCountry = c;
                        }
                    }
                }

                const x = lon * scaleX;
                const y = -lat * scaleY;
                hexWorldGrid.push({
                    lon: lon,
                    lat: lat,
                    x: x,
                    y: y,
                    radius: hexRadius,
                    region: matchedRegion,
                    country: matchedCountry ? matchedCountry.id : null
                });
            }
        }
    }
    recomputeHexIntensities();
}

function recomputeHexIntensities() {
    const currDataset = MAP_DATASETS[activeMapDatasetKey] || MAP_DATASETS["TOTAL"];
    const hotspots = MAP_HOTSPOTS[activeMapDatasetKey] || MAP_HOTSPOTS["TOTAL"] || [];

    for (let i = 0; i < hexWorldGrid.length; i++) {
        const hex = hexWorldGrid[i];
        const regData = currDataset.regions[hex.region];
        let minDist = 999;
        for (let j = 0; j < hotspots.length; j++) {
            const spot = hotspots[j];
            const d = Math.hypot(hex.lon - spot.lon, (hex.lat - spot.lat) * 1.15);
            if (d < minDist) minDist = d;
        }
        const proximityBoost = Math.max(0, 1 - minDist / 22);
        const baseIntensity = regData ? regData.intensity : 0.25;
        hex.intensity = Math.min(1.0, baseIntensity * 0.3 + proximityBoost * 0.7);
    }
}

export function initMap(canvas, onSelectGeocache) {
    mCanvas = canvas;
    mCtx = mCanvas.getContext('2d', { alpha: false });
    onSelectGeocacheCallback = onSelectGeocache;

    generateHexWorldGrid();
    setupMapInteractions();
    resizeMap();
    bootMapLibre();
    setMapMode('HEXAGON');
}

let maplibreActive = false;
let maplibreProtocolRegistered = false;
let maplibreInstance = null;
let mapRegionList = [];
let activeRegion = null;
let mapMarkers = [];

function mapThemeColors() {
    const rootStyle = getComputedStyle(document.documentElement);
    const theme = document.documentElement.getAttribute('data-theme') || 'midnight-gold';
    const light = theme === 'technical-paper' || theme === 'light';
    return {
        bg: rootStyle.getPropertyValue('--bg-canvas').trim() || (light ? '#f4f6f8' : '#07111e'),
        ink: rootStyle.getPropertyValue('--contrast-ink').trim() || (light ? '#0b0d17' : '#f59e0b'),
        land: light ? '#dfe3e8' : '#10243d',
        accentPrimary: rootStyle.getPropertyValue('--accent-primary').trim() || '#f59e0b',
        accentSecondary: rootStyle.getPropertyValue('--accent-secondary').trim() || '#38bdf8',
        hexBase: rootStyle.getPropertyValue('--hex-base').trim() || '#10243d',
        hexStroke: rootStyle.getPropertyValue('--hex-stroke').trim() || '#1a3960'
    };
}

function buildGraticuleGeoJSON() {
    const features = [];
    for (let lon = -180; lon <= 180; lon += 30) {
        const line = [];
        for (let lat = -85; lat <= 85; lat += 5) line.push([lon, lat]);
        features.push({ type: 'Feature', properties: {}, geometry: { type: 'LineString', coordinates: line } });
    }
    for (let lat = -75; lat <= 75; lat += 15) {
        const line = [];
        for (let lon = -180; lon <= 180; lon += 5) line.push([lon, lat]);
        features.push({ type: 'Feature', properties: {}, geometry: { type: 'LineString', coordinates: line } });
    }
    return { type: 'FeatureCollection', features };
}

function buildMapStyle(archiveUrl) {
    const colors = mapThemeColors();
    return {
        version: 8,
        glyphs: `${window.location.origin}/fonts/{fontstack}/{range}.pbf`,
        sources: {
            basemap: { type: 'vector', url: archiveUrl },
            graticule: { type: 'geojson', data: buildGraticuleGeoJSON() }
        },
        layers: [
            { id: 'bg', type: 'background', paint: { 'background-color': colors.bg } },
            {
                id: 'land-fill', type: 'fill', source: 'basemap', 'source-layer': 'countries',
                paint: { 'fill-color': colors.land, 'fill-opacity': 1 }
            },
            {
                id: 'land-line', type: 'line', source: 'basemap', 'source-layer': 'countries',
                paint: {
                    'line-color': colors.ink,
                    'line-width': ['interpolate', ['linear'], ['zoom'], 1, 0.4, 5, 0.8, 8, 1.1],
                    'line-opacity': 0.55
                }
            },
            {
                id: 'graticule', type: 'line', source: 'graticule',
                layout: { visibility: showGraticule ? 'visible' : 'none' },
                paint: { 'line-color': colors.ink, 'line-width': 0.5, 'line-opacity': 0.15 }
            },
            {
                id: 'country-labels', type: 'symbol', source: 'basemap', 'source-layer': 'labels',
                minzoom: 1.5,
                layout: {
                    'text-field': ['get', 'name'],
                    'text-font': ['Noto Sans Regular'],
                    'text-size': ['interpolate', ['linear'], ['zoom'], 2, 10, 5, 13, 8, 15],
                    'text-max-width': 9,
                    'text-padding': 14,
                    'text-letter-spacing': 0.06
                },
                paint: {
                    'text-color': colors.ink,
                    'text-halo-color': colors.bg,
                    'text-halo-width': 1.6,
                    'text-opacity': 0.92
                }
            }
        ]
    };
}

function buildStreetStyle(archiveUrl) {
    const colors = mapThemeColors();
    const ink = colors.ink;
    const roadWidth = ['interpolate', ['linear'], ['zoom'],
        5, ['match', ['get', 'kind_detail'],
            ['motorway', 'motorway_link', 'trunk', 'trunk_link'], 1.1, 0.5],
        9, ['match', ['get', 'kind_detail'],
            ['motorway', 'motorway_link', 'trunk', 'trunk_link'], 1.9,
            ['primary', 'primary_link'], 1.5, 0.7],
        12, ['match', ['get', 'kind_detail'],
            ['motorway', 'motorway_link', 'trunk', 'trunk_link'], 2.9,
            ['primary', 'primary_link'], 2.3,
            ['secondary', 'secondary_link'], 1.7, 1.1],
        15, ['match', ['get', 'kind_detail'],
            ['motorway', 'motorway_link', 'trunk', 'trunk_link'], 5.5,
            ['primary', 'primary_link'], 4.2,
            ['secondary', 'secondary_link'], 3.2,
            ['tertiary', 'tertiary_link', 'living_street'], 2.5, 1.8]];
    const placeSize = ['interpolate', ['linear'], ['zoom'],
        4, ['match', ['get', 'kind_detail'], 'city', 12, 'town', 10, 9],
        9, ['match', ['get', 'kind_detail'], 'city', 16, 'town', 14, 'suburb', 12, 11],
        13, ['match', ['get', 'kind_detail'], 'city', 20, 'town', 17, 'suburb', 14, 12]];
    return {
        version: 8,
        glyphs: `${window.location.origin}/fonts/{fontstack}/{range}.pbf`,
        sources: {
            basemap: { type: 'vector', url: archiveUrl },
            graticule: { type: 'geojson', data: buildGraticuleGeoJSON() }
        },
        layers: [
            { id: 'bg', type: 'background', paint: { 'background-color': colors.bg } },
            {
                id: 'earth-fill', type: 'fill', source: 'basemap', 'source-layer': 'earth',
                paint: { 'fill-color': colors.land }
            },
            {
                id: 'water-fill', type: 'fill', source: 'basemap', 'source-layer': 'water',
                paint: { 'fill-color': ink, 'fill-opacity': 0.13 }
            },
            {
                id: 'water-line', type: 'line', source: 'basemap', 'source-layer': 'water',
                minzoom: 11,
                paint: {
                    'line-color': ink, 'line-opacity': 0.5,
                    'line-width': ['interpolate', ['linear'], ['zoom'], 11, 0.6, 14, 1.4, 15, 2]
                }
            },
            {
                id: 'landcover-fill', type: 'fill', source: 'basemap', 'source-layer': 'landcover',
                minzoom: 4,
                paint: { 'fill-color': ink, 'fill-opacity': 0.05 }
            },
            {
                id: 'landuse-fill', type: 'fill', source: 'basemap', 'source-layer': 'landuse',
                minzoom: 9,
                paint: { 'fill-color': ink, 'fill-opacity': 0.06 }
            },
            {
                id: 'buildings-fill', type: 'fill', source: 'basemap', 'source-layer': 'buildings',
                minzoom: 12,
                paint: { 'fill-color': ink, 'fill-opacity': 0.17 }
            },
            {
                id: 'boundaries-line', type: 'line', source: 'basemap', 'source-layer': 'boundaries',
                minzoom: 4,
                paint: {
                    'line-color': ink, 'line-width': 0.8, 'line-opacity': 0.4,
                    'line-dasharray': [2.5, 2.5]
                }
            },
            {
                id: 'roads-line', type: 'line', source: 'basemap', 'source-layer': 'roads',
                minzoom: 3,
                filter: ['in', ['get', 'kind'], ['literal', ['highway', 'major_road', 'minor_road', 'other', 'aeroway']]],
                layout: { 'line-cap': 'round', 'line-join': 'round' },
                paint: { 'line-color': ink, 'line-width': roadWidth, 'line-opacity': 0.9 }
            },
            {
                id: 'roads-rail', type: 'line', source: 'basemap', 'source-layer': 'roads',
                minzoom: 9,
                filter: ['==', ['get', 'kind'], 'rail'],
                paint: {
                    'line-color': ink, 'line-opacity': 0.65,
                    'line-width': ['interpolate', ['linear'], ['zoom'], 9, 0.8, 14, 1.5],
                    'line-dasharray': [4, 3]
                }
            },
            {
                id: 'roads-ferry', type: 'line', source: 'basemap', 'source-layer': 'roads',
                minzoom: 6,
                filter: ['in', ['get', 'kind'], ['literal', ['ferry', 'aerialway']]],
                paint: {
                    'line-color': ink, 'line-opacity': 0.55, 'line-width': 1,
                    'line-dasharray': [2, 3]
                }
            },
            {
                id: 'roads-path', type: 'line', source: 'basemap', 'source-layer': 'roads',
                minzoom: 14,
                filter: ['==', ['get', 'kind'], 'path'],
                paint: {
                    'line-color': ink, 'line-opacity': 0.6, 'line-width': 0.8,
                    'line-dasharray': [2, 2]
                }
            },
            {
                id: 'road-labels', type: 'symbol', source: 'basemap', 'source-layer': 'roads',
                minzoom: 13,
                filter: ['any', ['has', 'name'], ['has', 'ref']],
                layout: {
                    'symbol-placement': 'line',
                    'text-field': ['coalesce', ['get', 'name'], ['get', 'ref']],
                    'text-font': ['Noto Sans Regular'],
                    'text-size': 10,
                    'text-padding': 3,
                    'text-max-angle': 30,
                    'text-letter-spacing': 0.02
                },
                paint: {
                    'text-color': ink,
                    'text-halo-color': colors.bg,
                    'text-halo-width': 1.5,
                    'text-opacity': 0.95
                }
            },
            {
                id: 'place-labels', type: 'symbol', source: 'basemap', 'source-layer': 'places',
                minzoom: 4,
                filter: ['all',
                    ['==', ['get', 'kind'], 'locality'],
                    ['in', ['get', 'kind_detail'], ['literal', ['city', 'town', 'village', 'hamlet', 'suburb', 'locality', 'farm']]],
                ],
                layout: {
                    'text-field': ['coalesce', ['get', 'name'], ['get', 'name:en']],
                    'text-font': ['Noto Sans Regular'],
                    'text-size': placeSize,
                    'text-padding': 6,
                    'text-max-width': 8
                },
                paint: {
                    'text-color': ink,
                    'text-halo-color': colors.bg,
                    'text-halo-width': 1.6,
                    'text-opacity': 0.95
                }
            },
            {
                id: 'hood-labels', type: 'symbol', source: 'basemap', 'source-layer': 'places',
                minzoom: 13,
                filter: ['==', ['get', 'kind'], 'neighbourhood'],
                layout: {
                    'text-field': ['coalesce', ['get', 'name'], ['get', 'name:en']],
                    'text-font': ['Noto Sans Regular'],
                    'text-size': ['interpolate', ['linear'], ['zoom'], 13, 10, 15, 12],
                    'text-padding': 4,
                    'text-max-width': 8,
                    'text-transform': 'uppercase',
                    'text-letter-spacing': 0.08
                },
                paint: {
                    'text-color': ink,
                    'text-halo-color': colors.bg,
                    'text-halo-width': 1.4,
                    'text-opacity': 0.8
                }
            }
        ]
    };
}

async function detectRegionMeta(file) {
    const fallbackSchema = file.replace(/\.pmtiles$/, '') === 'world' ? 'overview' : 'street';
    try {
        const pm = new pmtiles.PMTiles(`${window.location.origin}/maps/${file}`);
        const header = await pm.getHeader();
        const meta = await pm.getMetadata().catch(() => null);
        let ids = [];
        if (meta) {
            if (Array.isArray(meta.vector_layers)) {
                ids = meta.vector_layers.map((l) => l.id);
            } else if (typeof meta.json === 'string') {
                try {
                    const parsed = JSON.parse(meta.json);
                    if (Array.isArray(parsed.vector_layers)) ids = parsed.vector_layers.map((l) => l.id);
                } catch (e) {}
            }
        }
        let schema;
        if (ids.length) schema = ids.includes('roads') ? 'street' : 'overview';
        else schema = fallbackSchema;
        const b = [header.minLon, header.minLat, header.maxLon, header.maxLat];
        const bounds = b.every((v) => typeof v === 'number' && isFinite(v)) ? b : null;
        return {
            schema,
            bounds,
            maxzoom: header.maxZoom || (schema === 'street' ? 15 : 8),
            center: (isFinite(header.centerLon) && isFinite(header.centerLat))
                ? [header.centerLon, header.centerLat] : null
        };
    } catch (e) {
        return { schema: fallbackSchema, bounds: null, maxzoom: fallbackSchema === 'street' ? 15 : 8, center: null };
    }
}

function formatRegionSize(bytes) {
    if (bytes >= 1073741824) return (bytes / 1073741824).toFixed(2) + ' GB';
    if (bytes >= 1048576) return (bytes / 1048576).toFixed(1) + ' MB';
    if (bytes >= 1024) return (bytes / 1024).toFixed(0) + ' KB';
    return bytes + ' B';
}

function renderRegionList() {
    const host = document.getElementById('map-regions');
    if (!host) return;
    host.innerHTML = '';
    if (mapRegionList.length === 0) return;

    const cap = document.createElement('div');
    cap.className = 'pointer-events-none font-mono text-[9px] px-2 py-0.5 rounded border tracking-widest';
    cap.style.cssText = 'background-color: var(--panel-bg); border-color: var(--border-subtle); color: var(--text-muted);';
    cap.textContent = 'INSTALLED REGIONS';
    host.appendChild(cap);

    mapRegionList.forEach((r) => {
        const active = activeRegion && activeRegion.filename === r.filename;
        const row = document.createElement('div');
        row.className = 'flex items-center gap-1';

        const btn = document.createElement('button');
        btn.className = 'text-left px-2.5 py-1 rounded border transition-all';
        btn.style.cssText = active
            ? 'background-color: var(--panel-bg); border-color: var(--contrast-ink); color: var(--text-main);'
            : 'background-color: var(--panel-bg); border-color: var(--border-subtle); color: var(--text-secondary); opacity: 0.85;';
        btn.innerHTML = `<span style="font-weight:600">${escapeHtml(r.name)}</span> <span style="color:var(--text-muted)">${escapeHtml(formatRegionSize(r.size_bytes))}</span>`;
        btn.title = `Switch map to ${r.filename}`;
        btn.addEventListener('click', () => switchMapRegion(r.filename));
        row.appendChild(btn);

        if (r.name !== 'world') {
            const del = document.createElement('button');
            del.className = 'px-1.5 py-1 rounded border transition-all opacity-70 hover:opacity-100';
            del.style.cssText = 'background-color: var(--panel-bg); border-color: var(--border-subtle); color: var(--text-muted);';
            del.textContent = '✕';
            del.title = `Delete the ${r.name} region pack`;
            del.addEventListener('click', () => deleteRegionPack(r.filename));
            row.appendChild(del);
        }
        host.appendChild(row);
    });

    const hint = document.createElement('div');
    hint.className = 'pointer-events-none font-mono text-[9px] px-2 py-0.5 rounded border';
    hint.style.cssText = 'background-color: var(--panel-bg); border-color: var(--border-subtle); color: var(--text-muted);';
    hint.textContent = '↓ TOOL → DOWNLOAD MORE REGIONS';
    host.appendChild(hint);
}

async function deleteRegionPack(filename) {
    const label = filename.replace(/\.pmtiles$/, '');
    if (!window.confirm(`Delete the "${label}" region pack?`)) return;
    try {
        await deleteMapRegion(filename);
    } catch (e) {
        setMapStatus(`DELETE: ${e.message}`, true);
        return;
    }
    const wasActive = activeRegion && activeRegion.filename === filename;
    try { mapRegionList = (await getMaps()) || []; } catch (e) { mapRegionList = []; }
    renderRegionList();
    if (wasActive) {
        const fallback = mapRegionList.find((r) => r.name === 'world') || mapRegionList[0];
        if (fallback) {
            try { localStorage.setItem('mazzaroth:mapRegion', fallback.filename); } catch (e) {}
            await mountMapLibre(fallback.filename);
        } else {
            teardownMapLibre();
        }
    }
}

async function switchMapRegion(file) {
    if (typeof maplibregl === 'undefined') return;
    if (!mapRegionList.some((r) => r.filename === file)) return;
    if (activeRegion && activeRegion.filename === file && maplibreActive) return;
    setMapStatus(`MAPLIBRE: loading ${file}...`, false);
    teardownMapLibre();
    try { localStorage.setItem('mazzaroth:mapRegion', file); } catch (e) {}
    await mountMapLibre(file);
}

// ---------------------------------------------------------------------------
// Browser downloader: curated country packs + live fetch status (no-code UX)
// ---------------------------------------------------------------------------

const COUNTRY_PACKS = [
    // Asia
    { n: 'Japan', b: [122.9, 24.0, 145.9, 45.6], z: 14 },
    { n: 'South Korea', b: [126.0, 33.1, 131.9, 38.7], z: 14 },
    { n: 'North Korea', b: [124.3, 37.6, 130.7, 43.1], z: 12 },
    { n: 'China', b: [73.4, 18.0, 135.1, 53.6], z: 10 },
    { n: 'Taiwan', b: [119.9, 21.8, 122.1, 25.4], z: 14 },
    { n: 'Hong Kong', b: [113.8, 22.1, 114.5, 22.6], z: 15 },
    { n: 'Singapore', b: [103.6, 1.15, 104.1, 1.48], z: 15 },
    { n: 'Philippines', b: [116.9, 4.6, 126.6, 21.2], z: 12 },
    { n: 'Vietnam', b: [102.1, 8.4, 109.6, 23.4], z: 12 },
    { n: 'Thailand', b: [97.2, 5.6, 105.7, 20.5], z: 13 },
    { n: 'Malaysia', b: [99.6, 0.7, 119.5, 7.4], z: 13 },
    { n: 'Indonesia', b: [95.0, -11.1, 141.1, 6.0], z: 11 },
    { n: 'Myanmar', b: [92.2, 9.8, 101.2, 28.6], z: 12 },
    { n: 'India', b: [68.1, 6.7, 97.4, 35.5], z: 11 },
    { n: 'Nepal', b: [80.0, 26.3, 88.3, 30.5], z: 13 },
    { n: 'Sri Lanka', b: [79.7, 5.9, 81.9, 9.9], z: 14 },
    { n: 'Bangladesh', b: [88.0, 20.6, 92.7, 26.6], z: 13 },
    { n: 'Pakistan', b: [60.8, 23.7, 77.0, 36.9], z: 11 },
    { n: 'Mongolia', b: [87.7, 41.6, 119.9, 52.2], z: 11 },
    // Oceania
    { n: 'Australia', b: [112.9, -43.7, 153.7, -10.0], z: 11 },
    { n: 'New Zealand', b: [166.4, -47.3, 178.7, -34.4], z: 13 },
    { n: 'Fiji', b: [173.5, -20.7, 180.0, -12.4], z: 14 },
    // Americas
    { n: 'United States', b: [-125.0, 24.4, -66.9, 49.4], z: 10 },
    { n: 'Canada', b: [-141.0, 41.7, -52.6, 83.1], z: 10 },
    { n: 'Mexico', b: [-118.5, 14.5, -86.7, 32.7], z: 12 },
    { n: 'Guatemala', b: [-92.3, 13.7, -88.2, 17.9], z: 13 },
    { n: 'Cuba', b: [-85.0, 19.8, -74.1, 23.3], z: 13 },
    { n: 'Brazil', b: [-74.0, -33.8, -34.7, 5.3], z: 10 },
    { n: 'Argentina', b: [-73.6, -55.1, -53.6, -21.8], z: 11 },
    { n: 'Chile', b: [-75.7, -56.0, -66.4, -17.5], z: 11 },
    { n: 'Peru', b: [-81.4, -18.4, -68.6, -0.0], z: 11 },
    { n: 'Colombia', b: [-79.1, -4.3, -66.9, 13.4], z: 11 },
    // Europe
    { n: 'United Kingdom', b: [-8.8, 49.9, 1.8, 60.9], z: 13 },
    { n: 'Ireland', b: [-10.6, 51.4, -5.9, 55.4], z: 14 },
    { n: 'France', b: [-5.5, 41.3, 9.7, 51.2], z: 12 },
    { n: 'Spain', b: [-9.5, 36.0, 3.4, 43.9], z: 13 },
    { n: 'Portugal', b: [-9.6, 36.9, -6.1, 42.2], z: 13 },
    { n: 'Germany', b: [5.8, 47.2, 15.1, 55.1], z: 12 },
    { n: 'Italy', b: [6.6, 36.6, 18.6, 47.1], z: 13 },
    { n: 'Netherlands', b: [3.3, 50.7, 7.3, 53.6], z: 14 },
    { n: 'Belgium', b: [2.5, 49.5, 6.4, 51.6], z: 14 },
    { n: 'Switzerland', b: [5.9, 45.8, 10.5, 47.9], z: 14 },
    { n: 'Austria', b: [9.5, 46.3, 17.2, 49.1], z: 13 },
    { n: 'Denmark', b: [8.0, 54.5, 15.2, 57.8], z: 14 },
    { n: 'Norway', b: [4.6, 58.0, 31.1, 71.2], z: 12 },
    { n: 'Sweden', b: [10.9, 55.3, 24.2, 69.1], z: 12 },
    { n: 'Finland', b: [19.4, 59.7, 31.6, 70.2], z: 12 },
    { n: 'Poland', b: [14.1, 49.0, 24.2, 54.9], z: 12 },
    { n: 'Czechia', b: [12.0, 48.5, 18.9, 51.1], z: 13 },
    { n: 'Greece', b: [19.3, 34.7, 29.7, 41.8], z: 13 },
    { n: 'Turkey', b: [25.6, 35.8, 44.9, 42.2], z: 11 },
    { n: 'Ukraine', b: [22.1, 44.3, 40.3, 52.4], z: 11 },
    { n: 'Russia', b: [19.2, 41.1, 180.0, 77.7], z: 9 },
    // Africa & Middle East
    { n: 'Egypt', b: [24.7, 22.0, 36.9, 31.7], z: 11 },
    { n: 'Morocco', b: [-13.2, 27.7, -1.0, 35.9], z: 12 },
    { n: 'Nigeria', b: [2.6, 4.2, 14.7, 13.9], z: 11 },
    { n: 'Kenya', b: [33.9, -4.7, 41.9, 5.1], z: 12 },
    { n: 'Ethiopia', b: [33.0, 3.4, 48.0, 14.9], z: 11 },
    { n: 'South Africa', b: [16.4, -34.9, 32.9, -22.1], z: 11 },
    { n: 'Saudi Arabia', b: [34.5, 16.3, 55.7, 32.2], z: 11 },
    { n: 'United Arab Emirates', b: [51.5, 22.6, 56.5, 26.1], z: 13 },
    { n: 'Israel', b: [34.2, 29.4, 35.9, 33.3], z: 14 }
];

let fetchPollTimer = null;
let lastFetchState = null;
let downloadPanelReady = false;

function slugifyRegion(name) {
    return name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, '');
}

function setDownloadUi(state, text) {
    const status = document.getElementById('map-download-status');
    const go = document.getElementById('map-download-go');
    if (status) {
        status.textContent = text || '';
        status.dataset.state = state || 'idle';
        status.style.color = state === 'error' ? 'var(--text-main)' : 'var(--text-secondary)';
    }
    if (go) go.disabled = (state === 'running');
    if (go) go.style.opacity = (state === 'running') ? '0.5' : '1';
}

export function toggleDownloadPanel() {
    const panel = document.getElementById('map-download-panel');
    if (!panel) return;
    if (panel.classList.contains('hidden')) {
        openDownloadPanel();
    } else {
        panel.classList.add('hidden');
        stopFetchPoll();
    }
}

function openDownloadPanel() {
    const panel = document.getElementById('map-download-panel');
    const sel = document.getElementById('map-download-country');
    if (!panel || !sel) return;
    if (!downloadPanelReady || sel.options.length === 0) {
        sel.innerHTML = '';
        COUNTRY_PACKS.forEach((c) => {
            const opt = document.createElement('option');
            opt.value = c.n;
            opt.textContent = `${c.n} · z${c.z}`;
            sel.appendChild(opt);
        });
        sel.value = 'Japan';
        downloadPanelReady = true;
    }
    panel.classList.remove('hidden');
    // resume a download that may be running (started earlier / survives reload)
    startFetchPoll();
    pollFetchStatus();
}

function startFetchPoll() {
    if (fetchPollTimer) return;
    fetchPollTimer = setInterval(pollFetchStatus, 1500);
}

function stopFetchPoll() {
    if (fetchPollTimer) {
        clearInterval(fetchPollTimer);
        fetchPollTimer = null;
    }
}

async function pollFetchStatus() {
    let st;
    try {
        st = await getMapFetchStatus();
    } catch (e) {
        return;
    }
    if (!st) return;
    const last = (st.log && st.log.length) ? st.log[st.log.length - 1] : '';
    if (st.state === 'running') {
        lastFetchState = 'running';
        setDownloadUi('running', `● DOWNLOADING ${st.region} — ${last}`);
        return;
    }
    if (st.state === 'done') {
        const wasRunning = lastFetchState === 'running';
        lastFetchState = 'done';
        setDownloadUi('done', `✓ ${st.region} ready`);
        stopFetchPoll();
        if (wasRunning) await openFetchedRegion(st.region);
        return;
    }
    if (st.state === 'error') {
        lastFetchState = 'error';
        setDownloadUi('error', `✗ ${st.message || 'download failed'}`);
        stopFetchPoll();
        return;
    }
    // idle
    if (lastFetchState !== 'done') setDownloadUi('idle', 'PICK A COUNTRY BELOW');
    lastFetchState = 'idle';
}

async function openFetchedRegion(region) {
    try { mapRegionList = (await getMaps()) || []; } catch (e) { mapRegionList = []; }
    renderRegionList();
    const file = `${region}.pmtiles`;
    if (mapRegionList.some((r) => r.filename === file)) {
        setDownloadUi('done', `✓ ${region} ready — opening it now`);
        await switchMapRegion(file);
    }
}

export async function startSelectedDownload() {
    const sel = document.getElementById('map-download-country');
    if (!sel) return;
    const pack = COUNTRY_PACKS.find((c) => c.n === sel.value);
    if (!pack) return;
    const name = slugifyRegion(pack.n);
    setDownloadUi('running', `● REQUESTING ${pack.n} (z${pack.z})…`);
    try {
        await startMapFetch(pack.b, name, pack.z);
    } catch (e) {
        setDownloadUi('error', `✗ ${e.message}`);
        return;
    }
    lastFetchState = 'running';
    startFetchPoll();
}

function teardownMapLibre() {
    mapMarkers.forEach((m) => { try { m.remove(); } catch (e) {} });
    mapMarkers = [];
    if (maplibreInstance) {
        try { maplibreInstance.remove(); } catch (e) {}
    }
    maplibreInstance = null;
    maplibreActive = false;
    window.__maplibre = null;
    const cont = document.getElementById('maplibre-container');
    if (cont) cont.classList.add('hidden');
    if (mCanvas) mCanvas.classList.remove('hidden');
    const attr = document.getElementById('map-attribution');
    if (attr) attr.classList.add('hidden');
}

function applyMapLibreTheme() {
    if (!maplibreActive || !maplibreInstance) return;
    const colors = mapThemeColors();
    const set = (id, prop, val) => {
        try { maplibreInstance.setPaintProperty(id, prop, val); } catch (e) {}
    };
    if (activeRegion && activeRegion.schema === 'street') {
        set('bg', 'background-color', colors.bg);
        set('earth-fill', 'fill-color', colors.land);
        set('water-fill', 'fill-color', colors.ink);
        set('water-line', 'line-color', colors.ink);
        set('landcover-fill', 'fill-color', colors.ink);
        set('landuse-fill', 'fill-color', colors.ink);
        set('buildings-fill', 'fill-color', colors.ink);
        set('boundaries-line', 'line-color', colors.ink);
        set('roads-line', 'line-color', colors.ink);
        set('roads-rail', 'line-color', colors.ink);
        set('roads-ferry', 'line-color', colors.ink);
        set('roads-path', 'line-color', colors.ink);
        set('road-labels', 'text-color', colors.ink);
        set('road-labels', 'text-halo-color', colors.bg);
        set('place-labels', 'text-color', colors.ink);
        set('place-labels', 'text-halo-color', colors.bg);
        set('hood-labels', 'text-color', colors.ink);
        set('hood-labels', 'text-halo-color', colors.bg);
        return;
    }
    set('bg', 'background-color', colors.bg);
    set('land-fill', 'fill-color', colors.land);
    set('land-line', 'line-color', colors.ink);
    set('graticule', 'line-color', colors.ink);
    set('country-labels', 'text-color', colors.ink);
    set('country-labels', 'text-halo-color', colors.bg);
}

function updateCoordsHud(lon, lat) {
    const coordsEl = document.getElementById('map-cursor-coords');
    if (!coordsEl) return;
    if (Math.abs(lat) <= 90 && Math.abs(lon) <= 180) {
        coordsEl.innerText = `${Math.abs(lat).toFixed(2)}° ${lat >= 0 ? 'N' : 'S'}, ${Math.abs(lon).toFixed(2)}° ${lon >= 0 ? 'E' : 'W'}`;
    }
}

function addGeocacheMarkers() {
    if (!maplibreInstance) return;
    GEOCACHES.forEach((geo) => {
        const el = document.createElement('div');
        el.className = 'geocache-marker';
        el.setAttribute('role', 'button');
        el.setAttribute('title', geo.name);

        const ring = document.createElement('span');
        ring.className = 'geocache-ring';
        const label = document.createElement('span');
        label.className = 'geocache-label';
        label.textContent = geo.name;

        el.appendChild(ring);
        el.appendChild(label);
        el.addEventListener('click', (ev) => {
            ev.stopPropagation();
            if (onSelectGeocacheCallback) {
                onSelectGeocacheCallback({
                    id: geo.id,
                    name: geo.name,
                    tags: ['GEOCACHE', 'OFFLINE_VAULT'],
                    chapters: [
                        geo.desc,
                        `Chapter 2: Ground site coordinates confirmed at ${geo.lat.toFixed(2)}° Lat, ${geo.lon.toFixed(2)}° Lon. Vector cartography verified with zero network dependencies.`,
                        'Chapter 3: Regional communications status: Offline local storage mirror intact.'
                    ],
                    connections: ['NEXUS-0']
                });
            }
        });

        const marker = new maplibregl.Marker({ element: el, anchor: 'center' })
            .setLngLat([geo.lon, geo.lat])
            .addTo(maplibreInstance);
        mapMarkers.push(marker);
    });
}

function setMapStatus(msg, isError) {
    const el = document.getElementById('maplibre-status');
    if (!el) return;
    if (!msg) {
        el.classList.add('hidden');
        return;
    }
    el.innerText = msg;
    el.classList.remove('hidden');
    el.style.color = isError ? '#ef4444' : 'var(--text-secondary)';
    el.style.borderColor = isError ? '#ef4444' : 'var(--panel-border)';
}

async function bootMapLibre() {
    if (typeof maplibregl === 'undefined' || typeof pmtiles === 'undefined') {
        setMapStatus('MAPLIBRE: vendor lib missing — canvas fallback (hard refresh Ctrl+Shift+R)', true);
        return;
    }

    if (!maplibreProtocolRegistered) {
        try {
            const protocol = new pmtiles.Protocol();
            maplibregl.addProtocol('pmtiles', protocol.tile);
            maplibreProtocolRegistered = true;
        } catch (e) {
            setMapStatus('MAPLIBRE: protocol registration failed — canvas fallback', true);
            return;
        }
    }

    try {
        mapRegionList = (await getMaps()) || [];
    } catch (e) {
        setMapStatus('MAPLIBRE: /api/maps unreachable — canvas fallback', true);
        return;
    }
    if (mapRegionList.length === 0) {
        setMapStatus('MAPLIBRE: maps/*.pmtiles missing — canvas fallback', false);
        return;
    }

    const container = document.getElementById('maplibre-container');
    if (!container) {
        setMapStatus('MAPLIBRE: #maplibre-container missing — hard refresh (Ctrl+Shift+R)', true);
        return;
    }

    renderRegionList();

    let file = null;
    try { file = localStorage.getItem('mazzaroth:mapRegion'); } catch (e) {}
    if (!file || !mapRegionList.some((r) => r.filename === file)) {
        const world = mapRegionList.find((r) => r.name === 'world');
        file = (world || mapRegionList[0]).filename;
    }
    await mountMapLibre(file);
}

async function mountMapLibre(file) {
    const info = await detectRegionMeta(file);
    activeRegion = { filename: file, ...info };
    const isStreet = info.schema === 'street';
    const container = document.getElementById('maplibre-container');
    if (!container) {
        setMapStatus('MAPLIBRE: #maplibre-container missing — hard refresh (Ctrl+Shift+R)', true);
        return;
    }

    setMapStatus(`MAPLIBRE: booting ${file}...`, false);

    try {
        const archiveUrl = `pmtiles://${window.location.origin}/maps/${file}`;
        maplibreInstance = new maplibregl.Map({
            container: 'maplibre-container',
            style: isStreet ? buildStreetStyle(archiveUrl) : buildMapStyle(archiveUrl),
            center: [10, 25],
            zoom: 1.5,
            minZoom: 1,
            maxZoom: isStreet ? (info.maxzoom || 15) : 8,
            attributionControl: false,
            dragRotate: false,
            pitchWithRotate: false
        });
        window.__maplibre = maplibreInstance;

        maplibreInstance.on('error', (e) => {
            const msg = e && e.error ? e.error.message : String(e);
            console.warn('MapLibre:', msg);
            setMapStatus(`MAPLIBRE: ${msg}`, true);
        });

        await new Promise((resolve, reject) => {
            const timer = setTimeout(() => reject(new Error('style load timeout (8s)')), 8000);
            maplibreInstance.once('load', () => {
                clearTimeout(timer);
                resolve();
            });
        });

        const attr = document.getElementById('map-attribution');
        if (attr) attr.classList.toggle('hidden', !isStreet);

        const applyRegionView = () => {
            if (!maplibreInstance) return;
            if (isStreet && info.bounds) {
                maplibreInstance.fitBounds(info.bounds, { padding: 36, duration: 0 });
                const z = maplibreInstance.getZoom();
                maplibreInstance.setMinZoom(Math.max(3, z - 1));
            } else if (!isStreet) {
                maplibreInstance.fitBounds([[-170, -58], [170, 78]], { padding: 26, duration: 0 });
            }
            updateCoordsHud(maplibreInstance.getCenter().lng, maplibreInstance.getCenter().lat);
        };
        if (isStreet && info.bounds) {
            try { maplibreInstance.maxBounds(info.bounds); } catch (e) {}
        }
        // The map view may still be display:none at boot — fit once it becomes visible
        if (container.clientWidth > 0) {
            applyRegionView();
        } else {
            maplibreInstance.once('resize', applyRegionView);
        }

        maplibreActive = true;
        if (mapOperatingMode === 'OFFLINE') {
            container.classList.remove('hidden');
            if (mCanvas) mCanvas.classList.add('hidden');
        } else {
            container.classList.add('hidden');
            if (mCanvas) mCanvas.classList.remove('hidden');
        }
        addGeocacheMarkers();
        applyMapLibreTheme();
        renderRegionList();
        setMapStatus(`MAPLIBRE: ${file} ACTIVE (${isStreet ? 'street-level offline map' : 'offline vector basemap'})`, false);
        setTimeout(() => {
            if (maplibreActive) setMapStatus(null);
        }, 6000);

        maplibreInstance.on('mousemove', (e) => {
            if (e && e.lngLat) updateCoordsHud(e.lngLat.lng, e.lngLat.lat);
        });
    } catch (e) {
        console.warn('MapLibre unavailable, vector canvas fallback active:', e && e.message ? e.message : e);
        setMapStatus(`MAPLIBRE: ${e && e.message ? e.message : 'boot failed'} — canvas fallback`, true);
        if (maplibreInstance) {
            try { maplibreInstance.remove(); } catch (err) {}
        }
        maplibreInstance = null;
        window.__maplibre = null;
        maplibreActive = false;
        activeRegion = null;
        const cont = document.getElementById('maplibre-container');
        if (cont) cont.classList.add('hidden');
        if (mCanvas) mCanvas.classList.remove('hidden');
        renderRegionList();
        renderMap();
    }
}

export function toggleGraticule() {
    showGraticule = !showGraticule;
    if (maplibreActive && maplibreInstance) {
        try {
            maplibreInstance.setLayoutProperty('graticule', 'visibility', showGraticule ? 'visible' : 'none');
        } catch (e) {}
    }
    renderMap();
    return showGraticule;
}

export function recenterMap() {
    if (mapOperatingMode === 'OFFLINE' && maplibreActive && maplibreInstance) {
        if (activeRegion && activeRegion.schema === 'street' && activeRegion.bounds) {
            maplibreInstance.fitBounds(activeRegion.bounds, { padding: 36, duration: 600 });
        } else {
            maplibreInstance.easeTo({ center: [10, 25], zoom: 1.5, duration: 600 });
        }
        return;
    }
    if (!mCanvas) return;
    const redZone = document.getElementById('zone-red');
    const w = redZone ? redZone.clientWidth : mCanvas.width;
    const h = redZone ? redZone.clientHeight : mCanvas.height;
    mapPanX = w / 2;
    mapPanY = h / 2;
    mapZoom = 1.0;
    renderMap();
}

export function zoomMapIn() {
    if (mapOperatingMode === 'OFFLINE' && maplibreActive && maplibreInstance) {
        try { maplibreInstance.zoomIn({ duration: 220 }); } catch (e) {}
        return;
    }
    if (!mCanvas) return;
    mapZoom = Math.max(0.4, Math.min(5.0, mapZoom * 1.3));
    renderMap();
}

export function zoomMapOut() {
    if (mapOperatingMode === 'OFFLINE' && maplibreActive && maplibreInstance) {
        try { maplibreInstance.zoomOut({ duration: 220 }); } catch (e) {}
        return;
    }
    if (!mCanvas) return;
    mapZoom = Math.max(0.4, Math.min(5.0, mapZoom / 1.3));
    renderMap();
}

export function resizeMap() {
    if (maplibreActive && maplibreInstance) {
        try { maplibreInstance.resize(); } catch (e) {}
    }
    if (!mCanvas) return;
    const redZone = document.getElementById('zone-red');
    if (redZone) {
        mCanvas.width = redZone.clientWidth;
        mCanvas.height = redZone.clientHeight;
        mapPanX = mCanvas.width / 2;
        mapPanY = mCanvas.height / 2;
        renderMap();
    }
}

export function renderMap() {
    if (mapOperatingMode === 'HEXAGON') {
        renderHexDataMap();
        return;
    }

    if (maplibreActive) {
        applyMapLibreTheme();
        return;
    }
    if (!mCanvas || !mCtx) return;

    const rootStyle = getComputedStyle(document.documentElement);
    const bg = rootStyle.getPropertyValue('--bg-canvas').trim() || '#0b0d11';
    const strokeColor = rootStyle.getPropertyValue('--contrast-ink').trim() || '#ffffff';
    const textMain = rootStyle.getPropertyValue('--text-main').trim() || '#ffffff';
    const textMuted = rootStyle.getPropertyValue('--text-muted').trim() || '#71717a';
    const isDark = document.documentElement.getAttribute('data-theme') !== 'technical-paper' && document.documentElement.getAttribute('data-theme') !== 'light';

    mCtx.fillStyle = bg;
    mCtx.fillRect(0, 0, mCanvas.width, mCanvas.height);

    mCtx.save();
    mCtx.translate(mapPanX, mapPanY);
    mCtx.scale(mapZoom, mapZoom);

    // Map Projection Scale (Equirectangular: 1 deg lon = 2.4px, 1 deg lat = 2.4px)
    const scaleX = 2.4;
    const scaleY = 2.4;

    // 1. Graticule Lines (Parallels & Meridians)
    if (showGraticule) {
        mCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.07)';
        mCtx.lineWidth = 1;

        // Longitude Meridians (Every 30°)
        for (let lon = -180; lon <= 180; lon += 30) {
            const x = lon * scaleX;
            mCtx.beginPath();
            mCtx.moveTo(x, -90 * scaleY);
            mCtx.lineTo(x, 90 * scaleY);
            mCtx.stroke();

            // Meridian labels
            if (lon % 60 === 0 && Math.abs(lon) < 180) {
                mCtx.font = '7.5px "JetBrains Mono", monospace';
                mCtx.fillStyle = textMuted;
                mCtx.textAlign = 'center';
                mCtx.fillText(`${Math.abs(lon)}°${lon > 0 ? 'E' : (lon < 0 ? 'W' : '')}`, x, 90 * scaleY + 12);
            }
        }

        // Latitude Parallels (Every 15°)
        for (let lat = -75; lat <= 75; lat += 15) {
            const y = -lat * scaleY;
            mCtx.beginPath();
            mCtx.moveTo(-180 * scaleX, y);
            mCtx.lineTo(180 * scaleX, y);
            mCtx.stroke();

            // Parallel labels
            if (lat !== 0 && lat % 30 === 0) {
                mCtx.font = '7.5px "JetBrains Mono", monospace';
                mCtx.fillStyle = textMuted;
                mCtx.textAlign = 'left';
                mCtx.fillText(`${Math.abs(lat)}°${lat > 0 ? 'N' : 'S'}`, 180 * scaleX + 6, y + 3);
            }
        }

        // Equator & Prime Meridian Highlight Lines
        mCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.22)' : 'rgba(0,0,0,0.22)';
        mCtx.lineWidth = 1.2;

        // Equator (0° Lat)
        mCtx.beginPath();
        mCtx.moveTo(-180 * scaleX, 0);
        mCtx.lineTo(180 * scaleX, 0);
        mCtx.stroke();

        // Prime Meridian (0° Lon)
        mCtx.beginPath();
        mCtx.moveTo(0, -90 * scaleY);
        mCtx.lineTo(0, 90 * scaleY);
        mCtx.stroke();

        // Tropics (23.5° N/S) & Polar Circles (66.5° N/S)
        mCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.12)' : 'rgba(0,0,0,0.12)';
        mCtx.setLineDash([4, 4]);
        [-66.5, -23.5, 23.5, 66.5].forEach(lat => {
            const y = -lat * scaleY;
            mCtx.beginPath();
            mCtx.moveTo(-180 * scaleX, y);
            mCtx.lineTo(180 * scaleX, y);
            mCtx.stroke();
        });
        mCtx.setLineDash([]);
    }

    // 2. World Outer Boundary Frame
    mCtx.strokeStyle = isDark ? 'rgba(255,255,255,0.25)' : 'rgba(0,0,0,0.25)';
    mCtx.lineWidth = 1;
    mCtx.strokeRect(-180 * scaleX, -90 * scaleY, 360 * scaleX, 180 * scaleY);

    // 3. Render High-Fidelity Vector Continents
    mCtx.strokeStyle = strokeColor;
    mCtx.lineWidth = 1.4;
    mCtx.fillStyle = isDark ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.05)';

    WORLD_CONTINENTS.forEach(polygon => {
        if (!polygon || polygon.length === 0) return;
        mCtx.beginPath();
        polygon.forEach(([lon, lat], i) => {
            const px = lon * scaleX;
            const py = -lat * scaleY;
            if (i === 0) mCtx.moveTo(px, py);
            else mCtx.lineTo(px, py);
        });
        mCtx.closePath();
        mCtx.fill();
        mCtx.stroke();
    });

    // 4. Ocean Label Markers
    mCtx.font = 'bold 8px "JetBrains Mono", monospace';
    mCtx.fillStyle = isDark ? 'rgba(255,255,255,0.22)' : 'rgba(0,0,0,0.25)';
    mCtx.textAlign = 'center';
    mCtx.fillText("PACIFIC OCEAN", -120 * scaleX, -10 * scaleY);
    mCtx.fillText("PACIFIC OCEAN", 150 * scaleX, 5 * scaleY);
    mCtx.fillText("ATLANTIC OCEAN", -30 * scaleX, 15 * scaleY);
    mCtx.fillText("INDIAN OCEAN", 75 * scaleX, -15 * scaleY);
    mCtx.fillText("ARCTIC OCEAN", 0, -80 * scaleY);

    // 5. Render Geocache Site Pins
    GEOCACHES.forEach(geo => {
        const px = geo.lon * scaleX;
        const py = -geo.lat * scaleY;

        // Outer Target Ring
        mCtx.beginPath();
        mCtx.arc(px, py, 9, 0, Math.PI * 2);
        mCtx.strokeStyle = strokeColor;
        mCtx.lineWidth = 1;
        mCtx.stroke();

        // Inner Solid Core
        mCtx.beginPath();
        mCtx.arc(px, py, 3.5, 0, Math.PI * 2);
        mCtx.fillStyle = strokeColor;
        mCtx.fill();

        // Crosshair ticks
        mCtx.beginPath();
        mCtx.moveTo(px - 13, py); mCtx.lineTo(px - 9, py);
        mCtx.moveTo(px + 9, py); mCtx.lineTo(px + 13, py);
        mCtx.moveTo(px, py - 13); mCtx.lineTo(px, py - 9);
        mCtx.moveTo(px, py + 9); mCtx.lineTo(px, py + 13);
        mCtx.strokeStyle = strokeColor;
        mCtx.lineWidth = 0.8;
        mCtx.stroke();

        // Label
        mCtx.font = 'bold 8.5px "JetBrains Mono", monospace';
        mCtx.fillStyle = textMain;
        mCtx.textAlign = 'left';
        mCtx.fillText(geo.name, px + 15, py + 3);

        // Lat/Lon Subtext
        mCtx.font = '7px "JetBrains Mono", monospace';
        mCtx.fillStyle = textMuted;
        mCtx.fillText(`${Math.abs(geo.lat).toFixed(1)}°${geo.lat >= 0 ? 'N' : 'S'}, ${Math.abs(geo.lon).toFixed(1)}°${geo.lon >= 0 ? 'E' : 'W'}`, px + 15, py + 12);
    });

    mCtx.restore();
}

function drawHexagon(ctx, x, y, r, fill, stroke) {
    ctx.beginPath();
    for (let i = 0; i < 6; i++) {
        const angle = (Math.PI / 3) * i;
        const hx = x + r * Math.cos(angle);
        const hy = y + r * Math.sin(angle);
        if (i === 0) ctx.moveTo(hx, hy);
        else ctx.lineTo(hx, hy);
    }
    ctx.closePath();
    ctx.fillStyle = fill;
    ctx.fill();
    if (stroke) {
        ctx.lineWidth = 0.55;
        ctx.strokeStyle = stroke;
        ctx.stroke();
    }
}

function renderHexDataMap() {
    if (!mCanvas || !mCtx) return;

    const colors = mapThemeColors();
    const bg = colors.bg;
    const hexBase = colors.hexBase;
    const hexStroke = colors.hexStroke;
    const accentCyan = colors.accentSecondary;
    const theme = document.documentElement.getAttribute('data-theme') || 'midnight-gold';
    const isLight = theme === 'technical-paper' || theme === 'light';

    mCtx.fillStyle = bg;
    mCtx.fillRect(0, 0, mCanvas.width, mCanvas.height);

    mCtx.save();
    mCtx.translate(mapPanX, mapPanY);
    mCtx.scale(mapZoom, mapZoom);

    const scaleX = 2.4;
    const scaleY = 2.4;

    // 1. Subtle Oceanic Radar Grid Dots (Background Matrix)
    mCtx.fillStyle = isLight ? 'rgba(0, 0, 0, 0.05)' : 'rgba(255, 255, 255, 0.04)';
    const dLon = 8.4;
    const dLat = 7.2;
    for (let lat = 72; lat >= -54; lat -= dLat) {
        for (let lon = -170; lon <= 170; lon += dLon) {
            mCtx.beginPath();
            mCtx.arc(lon * scaleX, -lat * scaleY, 0.8, 0, Math.PI * 2);
            mCtx.fill();
        }
    }

    // 2. Ultra-Fast High-Density Hexagons (Read precomputed intensity, zero expensive shadowBlur)
    for (let i = 0; i < hexWorldGrid.length; i++) {
        const hex = hexWorldGrid[i];
        const cellIntensity = hex.intensity || 0.2;
        const isHovered = hoveredHex && Math.abs(hoveredHex.x - hex.x) < 1 && Math.abs(hoveredHex.y - hex.y) < 1;
        
        let isSelected = false;
        if (selectedCountry) {
            isSelected = (hex.country === selectedCountry.id);
        } else if (selectedMapRegion) {
            isSelected = (hex.region === selectedMapRegion.id);
        }

        let fillColor = hexBase;
        let strokeColor = hexStroke;

        // Color Gradient: Red (Hot) -> Yellow (Warm) -> Light Blue (Cool) -> Dark Blue / Slate (Base)
        if (isLight) {
            if (cellIntensity >= 0.75) {
                fillColor = '#b91c1c';
                strokeColor = '#991b1b';
            } else if (cellIntensity >= 0.52) {
                fillColor = '#d97706';
                strokeColor = '#b45309';
            } else if (cellIntensity >= 0.30) {
                fillColor = '#0284c7';
                strokeColor = '#0369a1';
            } else {
                fillColor = '#e2e8f0';
                strokeColor = '#cbd5e1';
            }
        } else if (theme === 'synth-magenta') {
            if (cellIntensity >= 0.75) {
                fillColor = '#f43f5e';
                strokeColor = '#fda4af';
            } else if (cellIntensity >= 0.52) {
                fillColor = '#fde047';
                strokeColor = '#fef08a';
            } else if (cellIntensity >= 0.30) {
                fillColor = '#0284c7';
                strokeColor = '#00f0ff';
            } else {
                fillColor = '#121424';
                strokeColor = '#1e2238';
            }
        } else {
            // Default Midnight Gold & Obsidian Mono
            if (cellIntensity >= 0.75) {
                fillColor = '#e11d48';
                strokeColor = '#fb7185';
            } else if (cellIntensity >= 0.52) {
                fillColor = '#f59e0b';
                strokeColor = '#fef08a';
            } else if (cellIntensity >= 0.30) {
                fillColor = '#0284c7';
                strokeColor = '#38bdf8';
            } else {
                fillColor = '#0f172a';
                strokeColor = '#1e293b';
            }
        }

        if (isSelected) {
            fillColor = '#38bdf8';
            strokeColor = '#ffffff';
        } else if (isHovered) {
            strokeColor = '#ffffff';
        }

        drawHexagon(mCtx, hex.x, hex.y, hex.radius, fillColor, strokeColor);
    }

    // 3. Country Beacons & Regional Nodes
    for (const c of Object.values(COUNTRY_DATA)) {
        const cx = c.lon * scaleX;
        const cy = -c.lat * scaleY;
        const isSel = selectedCountry && selectedCountry.id === c.id;
        const isHov = hoveredCountry && hoveredCountry.id === c.id;

        mCtx.beginPath();
        mCtx.arc(cx, cy, isSel ? 7 : (isHov ? 5 : 3.2), 0, Math.PI * 2);
        if (isSel) {
            mCtx.fillStyle = '#38bdf8';
            mCtx.fill();
            mCtx.lineWidth = 1.8;
            mCtx.strokeStyle = '#ffffff';
            mCtx.stroke();
        } else if (isHov) {
            mCtx.fillStyle = accentCyan;
            mCtx.fill();
            mCtx.lineWidth = 1.4;
            mCtx.strokeStyle = '#ffffff';
            mCtx.stroke();
        } else {
            mCtx.fillStyle = isLight ? '#0284c7' : '#ffffff';
            mCtx.fill();
            mCtx.lineWidth = 0.7;
            mCtx.strokeStyle = isLight ? '#0369a1' : 'rgba(255, 255, 255, 0.5)';
            mCtx.stroke();
        }

        mCtx.font = isSel ? 'bold 8.5px "JetBrains Mono", monospace' : 'bold 7px "JetBrains Mono", monospace';
        mCtx.fillStyle = isSel ? '#38bdf8' : (isHov ? accentCyan : (isLight ? '#334155' : 'rgba(255, 255, 255, 0.75)'));
        mCtx.textAlign = 'center';
        mCtx.fillText(c.id, cx, cy - (isSel ? 9 : 6));
    }

    mCtx.restore();
    updateMapFloatingBadges();
}

function updateMapFloatingBadges() {
    const container = document.getElementById('map-floating-badges');
    if (!container) return;
    if (mapOperatingMode !== 'HEXAGON') {
        container.innerHTML = '';
        container.classList.add('hidden');
        return;
    }

    const currDataset = MAP_DATASETS[activeMapDatasetKey] || MAP_DATASETS["TOTAL"];
    if (!currDataset) {
        container.innerHTML = '';
        return;
    }

    container.classList.remove('hidden');
    container.innerHTML = '';

    const scaleX = 2.4;
    const scaleY = 2.4;

    // 1. Single on-demand detail card if a Country is selected
    if (selectedCountry) {
        const c = selectedCountry;
        const screenX = mapPanX + c.lon * scaleX * mapZoom;
        const screenY = mapPanY - c.lat * scaleY * mapZoom;

        const card = document.createElement('div');
        card.className = "absolute pointer-events-auto rounded-2xl p-4 shadow-2xl transition-all duration-200 text-xs font-mono select-none flex flex-col gap-2 backdrop-blur-xl animate-fade-in";
        card.style.left = `${Math.max(16, Math.min(window.innerWidth - 320, screenX + 16))}px`;
        card.style.top = `${Math.max(60, Math.min(window.innerHeight - 340, screenY - 60))}px`;
        card.style.width = '300px';
        card.style.backgroundColor = 'var(--panel-bg)';
        card.style.border = '1.5px solid var(--accent-primary)';
        card.style.boxShadow = '0 12px 36px rgba(0, 0, 0, 0.45), 0 0 20px rgba(245, 158, 11, 0.2)';

        card.innerHTML = `
            <div class="flex items-center justify-between border-b pb-2" style="border-color: var(--border-subtle);">
                <div class="flex items-center space-x-2">
                    <span class="text-base leading-none">${c.flag || '📍'}</span>
                    <div class="flex flex-col">
                        <span class="font-bold text-sm tracking-wide" style="color: var(--text-main);">${c.name}</span>
                        <span class="text-[9px] uppercase tracking-widest" style="color: var(--accent-secondary);">${c.region} REGION • ${c.id}</span>
                    </div>
                </div>
                <button id="btn-close-detail-card" class="w-6 h-6 rounded-full flex items-center justify-center opacity-70 hover:opacity-100 hover:bg-white/10 transition-colors" style="color: var(--text-muted);" title="Close detail card">✕</button>
            </div>
            
            <div class="flex items-baseline justify-between py-1 px-2.5 rounded-lg" style="background-color: var(--panel-bg-subtle);">
                <div class="flex flex-col">
                    <span class="text-[9px] uppercase tracking-wider" style="color: var(--text-muted);">${currDataset.metricLabel}</span>
                    <span class="font-extrabold text-base tracking-tight" style="color: var(--accent-primary);">${c.count}</span>
                </div>
                <span class="text-[11px] font-bold px-1.5 py-0.5 rounded" style="background-color: var(--panel-border); color: var(--accent-secondary);">${c.pct}</span>
            </div>

            ${c.centers ? `
            <div class="text-[10px] leading-relaxed">
                <span class="font-bold uppercase tracking-wider" style="color: var(--text-secondary);">Centers: </span>
                <span style="color: var(--text-muted);">${c.centers}</span>
            </div>
            ` : ''}

            ${c.chapters && c.chapters.length > 0 ? `
            <div class="text-[10.5px] leading-relaxed line-clamp-3 p-2 rounded border" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-secondary);">
                ${c.chapters[0]}
            </div>
            ` : ''}

            <button id="btn-inspect-country-librarian" class="w-full mt-1 py-1.5 rounded-lg border text-[10px] font-bold tracking-wider uppercase transition-all flex items-center justify-center gap-1.5 hover:scale-[1.02] active:scale-[0.98]" style="background-color: var(--panel-bg-subtle); border-color: var(--accent-primary); color: var(--accent-primary);">
                <span>📖</span> <span>LOAD TO READING DECK</span>
            </button>
        `;

        const closeBtn = card.querySelector('#btn-close-detail-card');
        if (closeBtn) {
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                selectedCountry = null;
                selectedMapRegion = null;
                renderMap();
            });
        }

        const inspectBtn = card.querySelector('#btn-inspect-country-librarian');
        if (inspectBtn) {
            inspectBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                selectCountry(c.id);
            });
        }

        container.appendChild(card);
        return;
    }

    // 2. Single on-demand detail card if a Region is selected
    if (selectedMapRegion) {
        const reg = selectedMapRegion;
        const regData = currDataset.regions[reg.id] || reg;
        
        const card = document.createElement('div');
        card.className = "absolute pointer-events-auto rounded-2xl p-4 shadow-2xl transition-all duration-200 text-xs font-mono select-none flex flex-col gap-2 backdrop-blur-xl animate-fade-in";
        card.style.right = '24px';
        card.style.top = '72px';
        card.style.width = '290px';
        card.style.backgroundColor = 'var(--panel-bg)';
        card.style.border = '1.5px solid var(--accent-primary)';
        card.style.boxShadow = '0 12px 36px rgba(0, 0, 0, 0.45), 0 0 20px rgba(245, 158, 11, 0.2)';

        card.innerHTML = `
            <div class="flex items-center justify-between border-b pb-2" style="border-color: var(--border-subtle);">
                <div class="flex items-center space-x-2">
                    <span class="font-bold text-sm tracking-wide" style="color: var(--text-main);">${regData.name || reg.name}</span>
                    <span class="text-[9px] uppercase tracking-widest px-1.5 py-0.5 rounded" style="background-color: var(--panel-bg-subtle); color: var(--accent-secondary);">${reg.id}</span>
                </div>
                <button id="btn-close-region-card" class="w-6 h-6 rounded-full flex items-center justify-center opacity-70 hover:opacity-100 hover:bg-white/10 transition-colors" style="color: var(--text-muted);" title="Close detail card">✕</button>
            </div>
            
            <div class="flex items-baseline justify-between py-1 px-2.5 rounded-lg" style="background-color: var(--panel-bg-subtle);">
                <div class="flex flex-col">
                    <span class="text-[9px] uppercase tracking-wider" style="color: var(--text-muted);">${currDataset.metricLabel}</span>
                    <span class="font-extrabold text-base tracking-tight" style="color: var(--accent-primary);">${regData.count || '--'}</span>
                </div>
                <span class="text-[11px] font-bold px-1.5 py-0.5 rounded" style="background-color: var(--panel-border); color: var(--accent-secondary);">${regData.pct || '--'}</span>
            </div>

            ${regData.chapters && regData.chapters.length > 0 ? `
            <div class="text-[10.5px] leading-relaxed line-clamp-4 p-2 rounded border" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle); color: var(--text-secondary);">
                ${regData.chapters[0]}
            </div>
            ` : ''}

            <button id="btn-inspect-region-librarian" class="w-full mt-1 py-1.5 rounded-lg border text-[10px] font-bold tracking-wider uppercase transition-all flex items-center justify-center gap-1.5 hover:scale-[1.02] active:scale-[0.98]" style="background-color: var(--panel-bg-subtle); border-color: var(--accent-primary); color: var(--accent-primary);">
                <span>📖</span> <span>LOAD REGION TO READING DECK</span>
            </button>
        `;

        const closeBtn = card.querySelector('#btn-close-region-card');
        if (closeBtn) {
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                selectedMapRegion = null;
                renderMap();
            });
        }

        const inspectBtn = card.querySelector('#btn-inspect-region-librarian');
        if (inspectBtn) {
            inspectBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                selectDataMapRegion(reg.id);
            });
        }

        container.appendChild(card);
    }
}

export function selectCountry(code) {
    const country = COUNTRY_DATA[code];
    if (!country) return;
    selectedCountry = country;
    selectedMapRegion = { id: country.region, name: country.name };
    renderMap();

    if (onSelectGeocacheCallback) {
        const dataset = MAP_DATASETS[activeMapDatasetKey] || MAP_DATASETS["TOTAL"];
        onSelectGeocacheCallback({
            id: `GEO-${country.id}`,
            name: `${country.flag || '📍'} ${country.name} — ${dataset.title}`,
            tags: ["NATION_DATA", country.region, country.id, activeMapDatasetKey],
            chapters: [
                `Metric: ${country.count} (${country.pct} demographic presence) — ${dataset.metricLabel}`,
                ...(country.centers ? [`Key Centers: ${country.centers}`] : []),
                ...(country.chapters || [
                    `${country.name} represents a foundational repository of sovereign data and demographic distribution in the offline index.`
                ])
            ],
            connections: ["NEXUS-0"]
        });
    }
}

export function selectDataMapRegion(regionId) {
    const dataset = MAP_DATASETS[activeMapDatasetKey];
    if (!dataset) return;
    const regData = dataset.regions[regionId];
    if (!regData) return;

    selectedCountry = null;
    selectedMapRegion = { id: regionId, ...regData };
    renderMap();

    if (onSelectGeocacheCallback) {
        onSelectGeocacheCallback({
            id: `GEO-DATA-${regionId}`,
            name: `${regData.name} — ${dataset.title}`,
            tags: ["GEODEMOGRAPHIC", regionId, activeMapDatasetKey],
            chapters: [
                `Metric: ${regData.count} (${regData.pct} regional share) — ${dataset.metricLabel}`,
                ...(regData.chapters || [
                    `${regData.name} demonstrates a vital presence within the ${dataset.title} global matrix.`,
                    `Regional distributions are preserved locally in the offline knowledge archive.`
                ])
            ],
            connections: ["NEXUS-0"]
        });
    }
}

export function setMapMode(mode) {
    mapOperatingMode = mode;
    const isHex = mapOperatingMode === 'HEXAGON';
    const container = document.getElementById('maplibre-container');
    const badgeContainer = document.getElementById('map-floating-badges');
    const modeLabel = document.getElementById('map-active-mode-label');
    const datasetHud = document.getElementById('map-dataset-hud');
    const datasetLabel = document.getElementById('map-dataset-label');
    const datasetWrap = document.getElementById('deck-map-dataset-wrap');
    const modeBtn = document.getElementById('btn-map-mode');
    const legendText = document.getElementById('map-legend-text');
    const regionsEl = document.getElementById('map-regions');
    const dlPanel = document.getElementById('map-download-panel');
    const dlBtn = document.getElementById('tool-map-download');
    const gratBtn = document.getElementById('tool-map-graticule');

    if (modeBtn) {
        modeBtn.innerText = isHex ? 'HEXAGON' : 'OFFLINE';
    }
    if (datasetWrap) {
        datasetWrap.classList.toggle('hidden', !isHex);
    }
    if (dlBtn) {
        dlBtn.classList.toggle('hidden', isHex);
    }
    if (gratBtn) {
        gratBtn.classList.toggle('hidden', isHex);
    }
    if (regionsEl) {
        regionsEl.classList.toggle('hidden', isHex);
    }
    if (dlPanel && isHex) {
        dlPanel.classList.add('hidden');
    }
    if (modeLabel) {
        modeLabel.innerText = isHex ? 'HEXAGONAL DATA MATRIX' : 'OFFLINE VECTOR BASEMAP';
    }
    if (datasetHud) {
        datasetHud.classList.toggle('hidden', !isHex);
    }
    if (datasetLabel) {
        const currDataset = MAP_DATASETS[activeMapDatasetKey];
        if (currDataset) datasetLabel.innerText = currDataset.title;
    }
    if (legendText) {
        legendText.innerText = isHex 
            ? 'DRAG: PAN • SCROLL: ZOOM • CLICK COUNTRY PIN OR HEX CELL TO INSPECT'
            : 'DRAG: PAN • SCROLL OR +/−: ZOOM • CLICK GEOCACHE PIN TO READ';
    }

    if (isHex) {
        if (container) container.classList.add('hidden');
        if (mCanvas) mCanvas.classList.remove('hidden');
        if (badgeContainer) badgeContainer.classList.remove('hidden');
        renderMap();
    } else {
        if (badgeContainer) {
            badgeContainer.innerHTML = '';
            badgeContainer.classList.add('hidden');
        }
        if (maplibreActive && container) {
            container.classList.remove('hidden');
            if (mCanvas) mCanvas.classList.add('hidden');
            applyMapLibreTheme();
        } else {
            if (container) container.classList.add('hidden');
            if (mCanvas) mCanvas.classList.remove('hidden');
            renderMap();
        }
    }
}

export function cycleMapMode() {
    const next = mapOperatingMode === 'HEXAGON' ? 'OFFLINE' : 'HEXAGON';
    setMapMode(next);
    return next;
}

export function cycleMapDataset() {
    const keys = Object.keys(MAP_DATASETS);
    if (keys.length === 0) return;
    const currIdx = keys.indexOf(activeMapDatasetKey);
    const nextIdx = (currIdx + 1) % keys.length;
    activeMapDatasetKey = keys[nextIdx];
    
    const dataset = MAP_DATASETS[activeMapDatasetKey];
    const datasetLabel = document.getElementById('map-dataset-label');
    const datasetBtn = document.getElementById('btn-map-dataset');
    if (datasetLabel) datasetLabel.innerText = dataset.title;
    if (datasetBtn) datasetBtn.innerText = dataset.id;

    recomputeHexIntensities();
    renderMap();
    return activeMapDatasetKey;
}

function setupMapInteractions() {
    if (mapEventsBound || !mCanvas) return;
    mapEventsBound = true;

    const scaleX = 2.4;
    const scaleY = 2.4;
    let pointerDownPos = { x: 0, y: 0 };
    let mouseMovePending = false;
    let lastMouseMoveEv = null;

    mCanvas.addEventListener('mousedown', (e) => {
        if (e.button === 0) {
            pointerDownPos = { x: e.clientX, y: e.clientY };
            isDraggingMap = true;
            mapDragStartX = e.clientX - mapPanX;
            mapDragStartY = e.clientY - mapPanY;
        }
    });

    mCanvas.addEventListener('click', (e) => {
        const dragDist = Math.hypot(e.clientX - pointerDownPos.x, e.clientY - pointerDownPos.y);
        if (dragDist > 6) return;

        const rect = mCanvas.getBoundingClientRect();
        const mx = (e.clientX - rect.left - mapPanX) / mapZoom;
        const my = (e.clientY - rect.top - mapPanY) / mapZoom;

        if (mapOperatingMode === 'HEXAGON') {
            // Check country beacon click first (radius 16px)
            let hitCountry = null;
            for (const c of Object.values(COUNTRY_DATA)) {
                const cx = c.lon * scaleX;
                const cy = -c.lat * scaleY;
                if (Math.hypot(mx - cx, my - cy) <= 16) {
                    hitCountry = c;
                    break;
                }
            }

            if (hitCountry) {
                selectCountry(hitCountry.id);
                return;
            }

            // Check hex cell click
            let clickedHex = null;
            for (let i = 0; i < hexWorldGrid.length; i++) {
                const hex = hexWorldGrid[i];
                if (Math.hypot(mx - hex.x, my - hex.y) <= hex.radius * 1.3) {
                    clickedHex = hex;
                    break;
                }
            }

            if (clickedHex && clickedHex.country) {
                selectCountry(clickedHex.country);
            } else if (clickedHex && clickedHex.region) {
                selectedCountry = null;
                selectDataMapRegion(clickedHex.region);
            } else {
                selectedCountry = null;
                selectedMapRegion = null;
                renderMap();
            }
        } else {
            let hitGeo = null;
            for (let g of GEOCACHES) {
                const gx = g.lon * scaleX;
                const gy = -g.lat * scaleY;
                if (Math.hypot(mx - gx, my - gy) <= 16) {
                    hitGeo = g;
                    break;
                }
            }

            if (hitGeo && onSelectGeocacheCallback) {
                onSelectGeocacheCallback({
                    id: hitGeo.id,
                    name: hitGeo.name,
                    tags: ["GEOCACHE", "OFFLINE_VAULT"],
                    chapters: [
                        hitGeo.desc,
                        `Chapter 2: Ground site coordinates confirmed at ${hitGeo.lat.toFixed(2)}° Lat, ${hitGeo.lon.toFixed(2)}° Lon. Vector cartography verified with zero network dependencies.`,
                        `Chapter 3: Regional communications status: Offline local storage mirror intact.`
                    ],
                    connections: ["NEXUS-0"]
                });
            }
        }
    });

    window.addEventListener('mousemove', (e) => {
        if (mapOperatingMode === 'OFFLINE' && maplibreActive) return;
        lastMouseMoveEv = e;
        if (!mouseMovePending) {
            mouseMovePending = true;
            requestAnimationFrame(() => {
                mouseMovePending = false;
                if (!lastMouseMoveEv) return;
                const ev = lastMouseMoveEv;

                if (isDraggingMap) {
                    mapPanX = ev.clientX - mapDragStartX;
                    mapPanY = ev.clientY - mapDragStartY;
                    renderMap();
                    return;
                }
                
                if (!mCanvas || mCanvas.offsetParent === null) return;
                const rect = mCanvas.getBoundingClientRect();
                if (ev.clientX < rect.left || ev.clientX > rect.right || ev.clientY < rect.top || ev.clientY > rect.bottom) {
                    if (hoveredHex || hoveredCountry) {
                        hoveredHex = null;
                        hoveredCountry = null;
                        if (mapOperatingMode === 'HEXAGON') renderMap();
                    }
                    return;
                }
                const mx = (ev.clientX - rect.left - mapPanX) / mapZoom;
                const my = (ev.clientY - rect.top - mapPanY) / mapZoom;

                if (mapOperatingMode === 'HEXAGON') {
                    let foundHex = null;
                    for (let i = 0; i < hexWorldGrid.length; i++) {
                        const hex = hexWorldGrid[i];
                        if (Math.abs(mx - hex.x) <= hex.radius * 1.2 && Math.abs(my - hex.y) <= hex.radius * 1.2) {
                            foundHex = hex;
                            break;
                        }
                    }

                    let foundCountry = null;
                    for (const c of Object.values(COUNTRY_DATA)) {
                        const cx = c.lon * scaleX;
                        const cy = -c.lat * scaleY;
                        if (Math.abs(mx - cx) <= 8 && Math.abs(my - cy) <= 8) {
                            foundCountry = c;
                            break;
                        }
                    }

                    if (foundHex !== hoveredHex || foundCountry !== hoveredCountry) {
                        hoveredHex = foundHex;
                        hoveredCountry = foundCountry;
                        renderMap();
                    }
                } else {
                    const lon = (mx / scaleX).toFixed(2);
                    const lat = (-my / scaleY).toFixed(2);
                    const coordsEl = document.getElementById('map-cursor-coords');
                    if (coordsEl) {
                        const latNum = parseFloat(lat);
                        const lonNum = parseFloat(lon);
                        if (Math.abs(latNum) <= 90 && Math.abs(lonNum) <= 180) {
                            coordsEl.innerText = `${Math.abs(latNum).toFixed(2)}° ${latNum >= 0 ? 'N' : 'S'}, ${Math.abs(lonNum).toFixed(2)}° ${lonNum >= 0 ? 'E' : 'W'}`;
                        }
                    }
                }
            });
        }
    });

    window.addEventListener('mouseup', () => {
        isDraggingMap = false;
    });

    mCanvas.addEventListener('wheel', (e) => {
        e.preventDefault();
        const factor = e.deltaY < 0 ? 1.15 : 0.85;
        mapZoom = Math.max(0.4, Math.min(4.5, mapZoom * factor));
        renderMap();
    }, { passive: false });
}

/* ========================================================================= */
/* 5. LIBRARIAN AI CHAT MODULE                                               */
/* ========================================================================= */
let currentLlmModel = null;
let installedLlmModels = [];
let librarianConversationHistory = [];

export function initLibrarian() {
    const input = document.getElementById('librarian-input');
    const sendBtn = document.getElementById('btn-librarian-send');
    const chatLog = document.getElementById('librarian-chat-log');
    const modelBtn = document.getElementById('btn-ollama-model');
    const statusDot = document.getElementById('ollama-status-dot');

    getStatus().then(status => {
        if (status.llm_online) {
            if (statusDot) statusDot.style.backgroundColor = '#22c55e';
            if (modelBtn && status.llm_model) {
                if (!currentLlmModel) currentLlmModel = status.llm_model;
                modelBtn.innerText = currentLlmModel.toUpperCase().slice(0, 8);
                modelBtn.title = `Active Model: ${currentLlmModel}`;
            }
        } else {
            if (statusDot) statusDot.style.backgroundColor = '#eab308';
        }
    }).catch(() => {});

    getModels().then(models => {
        if (models && models.length > 0) {
            installedLlmModels = models.map(m => m.name || m.model);
            if (!currentLlmModel && installedLlmModels.length > 0) {
                currentLlmModel = installedLlmModels[0];
            }
            if (modelBtn && currentLlmModel) {
                modelBtn.innerText = currentLlmModel.toUpperCase().slice(0, 8);
                modelBtn.title = `Active Model: ${currentLlmModel} (click to cycle)`;
            }
        }
    }).catch(() => {});

    if (modelBtn) {
        modelBtn.addEventListener('click', () => {
            if (installedLlmModels.length > 0) {
                const idx = installedLlmModels.indexOf(currentLlmModel);
                const nextIdx = (idx + 1) % installedLlmModels.length;
                currentLlmModel = installedLlmModels[nextIdx];
                modelBtn.innerText = currentLlmModel.toUpperCase().slice(0, 8);
                modelBtn.title = `Active Model: ${currentLlmModel} (click to cycle)`;
            }
        });
    }

    document.querySelectorAll('.prompt-chip').forEach(chip => {
        chip.addEventListener('click', () => {
            const prompt = chip.innerText.replace(/^"|"$/g, '');
            if (input) input.value = prompt;
            submitLibrarianQuery(prompt);
        });
    });

    if (sendBtn && input) {
        sendBtn.addEventListener('click', () => {
            const q = input.value.trim();
            if (q) submitLibrarianQuery(q);
        });

        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                const q = input.value.trim();
                if (q) submitLibrarianQuery(q);
            }
        });
    }

    const clearBtn = document.getElementById('btn-clear-chat');
    if (clearBtn && chatLog) {
        clearBtn.addEventListener('click', () => {
            librarianConversationHistory = [];
            chatLog.innerHTML = `
                <div class="p-4 rounded-xl border flex gap-3" style="background-color: var(--panel-bg-subtle); border-color: var(--border-subtle);">
                    <div class="w-6 h-6 rounded-md border flex items-center justify-center font-bold text-[10px]"
                         style="border-color: var(--panel-border); background-color: var(--panel-bg); color: var(--text-main);">AI</div>
                    <div class="space-y-2 flex-1 leading-relaxed">
                        <p class="font-bold" style="color: var(--text-main);">Mazzaroth Librarian Online.</p>
                        <p style="color: var(--text-secondary);">Conversation buffer reset. Query the local knowledge vault with citations.</p>
                    </div>
                </div>
            `;
        });
    }
}

export function submitLibrarianQuery(question) {
    const chatLog = document.getElementById('librarian-chat-log');
    const input = document.getElementById('librarian-input');
    if (!chatLog) return;
    if (input) input.value = '';

    const userMsg = document.createElement('div');
    userMsg.className = 'p-3 rounded-xl border flex gap-2.5 font-mono text-xs min-w-0 max-w-full';
    userMsg.style.backgroundColor = 'var(--panel-bg)';
    userMsg.style.borderColor = 'var(--panel-border)';
    userMsg.innerHTML = `
        <div class="w-6 h-6 rounded-md border flex items-center justify-center font-bold text-[10px] shrink-0" style="background-color: var(--contrast-ink); color: var(--contrast-paper);">YOU</div>
        <div class="flex-1 min-w-0 break-words whitespace-pre-wrap" style="color: var(--text-main); overflow-wrap: anywhere; word-break: break-word;">${escapeHtml(question)}</div>
    `;
    chatLog.appendChild(userMsg);

    const aiMsg = document.createElement('div');
    aiMsg.className = 'p-4 rounded-xl border flex gap-3 font-mono text-xs min-w-0 max-w-full';
    aiMsg.style.backgroundColor = 'var(--panel-bg-subtle)';
    aiMsg.style.borderColor = 'var(--border-subtle)';

    const tokenContainerId = `ai-tokens-${Date.now()}`;
    const citationContainerId = `ai-cits-${Date.now()}`;

    aiMsg.innerHTML = `
        <div class="w-6 h-6 rounded-md border flex items-center justify-center font-bold text-[10px] shrink-0" style="border-color: var(--panel-border); background-color: var(--panel-bg); color: var(--text-main);">AI</div>
        <div class="space-y-2 flex-1 min-w-0 leading-relaxed overflow-hidden">
            <div id="${tokenContainerId}" class="break-words whitespace-pre-wrap min-w-0" style="color: var(--text-main); overflow-wrap: anywhere; word-break: break-word; line-height: 1.6;"></div>
            <div id="${citationContainerId}" class="flex flex-wrap gap-1.5 pt-2 hidden"></div>
        </div>
    `;
    chatLog.appendChild(aiMsg);
    chatLog.scrollTop = chatLog.scrollHeight;

    const tokenSpan = document.getElementById(tokenContainerId);
    const citSpan = document.getElementById(citationContainerId);

    let accumulatedResponse = '';
    const outgoingHistory = [...librarianConversationHistory];
    librarianConversationHistory.push({ role: 'user', content: question });

    askStream(
        question,
        (token) => {
            if (tokenSpan) {
                tokenSpan.innerText += token;
                accumulatedResponse += token;
                chatLog.scrollTop = chatLog.scrollHeight;
            }
        },
        (citations) => {
            if (citSpan && citations && citations.length > 0) {
                citSpan.classList.remove('hidden');
                citSpan.innerHTML = `<span class="text-[9px] font-bold uppercase mr-1" style="color: var(--text-muted);">CITATIONS:</span>` +
                    citations.map(c => {
                        const title = typeof c === 'string' ? c : (c.doc_title || c.title || 'Citation');
                        return `<span class="px-2 py-0.5 rounded border text-[9px]" style="background-color: var(--panel-bg); border-color: var(--panel-border); color: var(--text-main);">${escapeHtml(title)}</span>`;
                    }).join('');
            }
        },
        () => {
            if (accumulatedResponse.trim()) {
                librarianConversationHistory.push({ role: 'assistant', content: accumulatedResponse.trim() });
            }
        },
        (err) => {
            if (tokenSpan) {
                tokenSpan.innerText += `\n[Librarian Offline: ${err.message}]`;
            }
        },
        currentLlmModel,
        outgoingHistory
    );
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.innerText = text || '';
    return div.innerHTML;
}



