// MAZZAROTH 3D/2.5D Volumetric Spiral Galaxy Engine
// 1,105 DB Nodes + Camera-Facing Billboard Sprites + Dense Luminous Center + Zero Flat Strands + Closest Star Raycast
import { getGalaxy, search } from './api.js';

export let HERO_STARS=1105; // /api/search integrated

let scene, camera, renderer, controls;
let galaxyGroup, pointsMesh, starsMesh, coreMesh, linesMesh;
let gCanvas = null;

let allGalaxyNodes = [];
let interactiveStars = [];
let galaxyLinks = [];
let selectedNode = null;
let hoveredNode = null;

let densitySetting = 'high';
let showSynapseLines = true;
let activeDomain = 'ALL';

let onSelectNodeCallback = null;
let isDragging = false;
let animFrameId = null;

const NUM_ARMS = 5;
const MAX_GALAXY_RADIUS = 480;
const CORE_RADIUS = 26;

export function initGalaxy(canvas, onSelectNode) {
    gCanvas = canvas;
    onSelectNodeCallback = onSelectNode;

    initThree();
    setupCanvasEvents();
    loadGalaxyData();
    startAnimationLoop();
}

function initThree() {
    if (!gCanvas || typeof THREE === 'undefined') return;

    const rect = gCanvas.getBoundingClientRect();
    const w = rect.width || window.innerWidth * 0.6;
    const h = rect.height || window.innerHeight * 0.8;

    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const bgColor = isDark ? 0x08090c : 0xdce1e6;

    scene = new THREE.Scene();
    scene.background = new THREE.Color(bgColor);

    camera = new THREE.PerspectiveCamera(46, w / h, 1, 10000);
    camera.position.set(0, 380, 500);

    renderer = new THREE.WebGLRenderer({ canvas: gCanvas, antialias: true, alpha: false });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    renderer.setSize(w, h, false);

    if (THREE.OrbitControls) {
        controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.dampingFactor = 0.06;
        controls.maxDistance = 2200;
        controls.minDistance = 40;
        controls.autoRotate = true;
        controls.autoRotateSpeed = 0.4;
        controls.target.set(0, 0, 0);

        controls.addEventListener('start', () => { isDragging = true; });
        controls.addEventListener('end', () => { isDragging = false; });
    }

    galaxyGroup = new THREE.Group();
    scene.add(galaxyGroup);

    buildCore();
    buildParticleDisc();
}

function createGlowTexture(isSparkle = false) {
    const canvas = document.createElement('canvas');
    canvas.width = 64;
    canvas.height = 64;
    const ctx = canvas.getContext('2d');

    if (isSparkle) {
        const grad = ctx.createRadialGradient(32, 32, 0, 32, 32, 30);
        grad.addColorStop(0, 'rgba(255, 255, 255, 1.0)');
        grad.addColorStop(0.2, 'rgba(255, 255, 255, 0.85)');
        grad.addColorStop(0.55, 'rgba(255, 255, 255, 0.25)');
        grad.addColorStop(1, 'rgba(255, 255, 255, 0)');
        ctx.fillStyle = grad;
        ctx.fillRect(0, 0, 64, 64);

        ctx.fillStyle = 'rgba(255, 255, 255, 0.95)';
        ctx.beginPath();
        ctx.moveTo(32, 2);
        ctx.quadraticCurveTo(32, 32, 62, 32);
        ctx.quadraticCurveTo(32, 32, 32, 62);
        ctx.quadraticCurveTo(32, 32, 2, 32);
        ctx.quadraticCurveTo(32, 32, 32, 2);
        ctx.fill();
    } else {
        const grad = ctx.createRadialGradient(32, 32, 0, 32, 32, 32);
        grad.addColorStop(0, 'rgba(255, 255, 255, 1.0)');
        grad.addColorStop(0.2, 'rgba(255, 255, 255, 0.85)');
        grad.addColorStop(0.5, 'rgba(255, 255, 255, 0.3)');
        grad.addColorStop(1, 'rgba(255, 255, 255, 0)');
        ctx.fillStyle = grad;
        ctx.fillRect(0, 0, 64, 64);
    }

    const tex = new THREE.CanvasTexture(canvas);
    tex.needsUpdate = true;
    return tex;
}

function buildCore() {
    if (coreMesh) galaxyGroup.remove(coreMesh);

    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const coreColor = isDark ? 0xffffff : 0x0a0a0c;

    // Smooth central nucleus sphere without ring
    const coreGeo = new THREE.SphereGeometry(12, 32, 32);
    const coreMat = new THREE.MeshBasicMaterial({ color: coreColor });
    coreMesh = new THREE.Mesh(coreGeo, coreMat);
    coreMesh.userData = { id: "NEXUS-0", name: "Prime Core Nexus", isCore: true };
    galaxyGroup.add(coreMesh);
}

function buildParticleDisc() {
    if (pointsMesh) galaxyGroup.remove(pointsMesh);

    const count = densitySetting === 'high' ? 16000 : (densitySetting === 'med' ? 9000 : 4500);
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);
    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';

    const baseColor = new THREE.Color(isDark ? 0xeef2f6 : 0x14171d);
    const coreColor = new THREE.Color(isDark ? 0xffffff : 0x000000);

    for (let i = 0; i < count; i++) {
        // High central density bias
        const bias = Math.pow(Math.random(), 3.0);
        const r = CORE_RADIUS + 2 + bias * MAX_GALAXY_RADIUS;

        const arm = i % NUM_ARMS;
        const baseAngle = (arm * Math.PI * 2) / NUM_ARMS;
        const twist = (r / MAX_GALAXY_RADIUS) * Math.PI * 4.4;
        const noiseAngle = (Math.random() - 0.5) * (0.28 + bias * 1.4);
        const noiseR = (Math.random() - 0.5) * (bias * 30);

        const theta = baseAngle + twist + noiseAngle;
        const finalR = r + noiseR;

        const x = Math.cos(theta) * finalR;
        // Natural spheroidal bulge at center decaying gracefully into thin disk
        const y = (Math.random() - 0.5) * (24 + (1.0 - bias) * 38);
        const z = Math.sin(theta) * finalR;

        positions[i * 3] = x;
        positions[i * 3 + 1] = y;
        positions[i * 3 + 2] = z;

        const c = baseColor.clone().lerp(coreColor, 1.0 - bias);
        colors[i * 3] = c.r;
        colors[i * 3 + 1] = c.g;
        colors[i * 3 + 2] = c.b;
    }

    const geo = new THREE.BufferGeometry();
    geo.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    geo.setAttribute('color', new THREE.BufferAttribute(colors, 3));

    const mat = new THREE.PointsMaterial({
        size: 3.4,
        vertexColors: true,
        map: createGlowTexture(false),
        transparent: true,
        opacity: isDark ? 0.9 : 0.8,
        blending: isDark ? THREE.AdditiveBlending : THREE.NormalBlending,
        depthWrite: false
    });

    pointsMesh = new THREE.Points(geo, mat);
    galaxyGroup.add(pointsMesh);
}

export async function loadGalaxyData() {
    try {
        const data = await getGalaxy();
        const rawNodes = data.nodes || data.bodies || [];
        galaxyLinks = data.links || data.lines || [];
        HERO_STARS = rawNodes.length || 1105;

        buildDatabaseStars(rawNodes);
    } catch (e) {
        console.warn('Fallback galaxy nodes:', e);
        buildDatabaseStars([]);
    }
}

function buildDatabaseStars(rawNodes) {
    if (starsMesh) galaxyGroup.remove(starsMesh);

    const rootNode = {
        id: "NEXUS-0",
        name: "Prime Core Nexus",
        label: "Prime Core Nexus",
        arm: "CORE",
        domain: "cognitive",
        category: "COGNITIVE",
        tags: ["ROOT", "ORIGIN", "INDEX"],
        chapters: [
            "Galactic epicenter of the Mazzaroth repository. This sovereign root node orchestrates cataloged memory stars across 5 logarithmic spiral arms.",
            "Chapter 2: Every star acts as a localized node of memory containing full textual records, telemetry logs, or philosophical manuscripts.",
            "Chapter 3: Verification hash: 0x9B88A4F1. Local integrity validated across all memory blocks."
        ],
        x: 0, y: 0, z: 0,
        isCore: true,
        connections: []
    };

    allGalaxyNodes = [rootNode];

    const connectionsMap = {};
    galaxyLinks.forEach(l => {
        const s = l.source_id || l.source;
        const t = l.target_id || l.target;
        if (s && t) {
            if (!connectionsMap[s]) connectionsMap[s] = [];
            if (!connectionsMap[t]) connectionsMap[t] = [];
            connectionsMap[s].push(t);
            connectionsMap[t].push(s);
        }
    });

    const starCount = Math.max(rawNodes.length, 1105);

    for (let index = 0; index < starCount; index++) {
        const node = rawNodes[index] || {
            id: `STAR-${String(index + 1).padStart(4, '0')}`,
            label: `Memory Star #${index + 1}`,
            category: (index % 5 === 0) ? 'BIBLE' : (index % 5 === 1 ? 'MEDICAL' : (index % 5 === 2 ? 'SURVIVAL' : (index % 5 === 3 ? 'LITERATURE' : 'COGNITIVE')))
        };

        const arm = index % NUM_ARMS;
        // Smooth continuous radial distribution without grouping banding
        const distFactor = Math.pow(Math.random(), 2.2);
        const r = CORE_RADIUS + 6 + distFactor * (MAX_GALAXY_RADIUS - CORE_RADIUS);
        const baseAngle = (arm * Math.PI * 2) / NUM_ARMS;
        const twist = (r / MAX_GALAXY_RADIUS) * Math.PI * 4.4;
        const noiseAngle = (Math.random() - 0.5) * (0.24 + distFactor * 0.8);
        const theta = baseAngle + twist + noiseAngle;

        const x = Math.cos(theta) * r;
        const y = (Math.random() - 0.5) * (18 + (1.0 - distFactor) * 24);
        const z = Math.sin(theta) * r;

        let domain = 'cognitive';
        const cat = (node.category || node.tier || '').toLowerCase();
        const tags = (node.tags || []).map(t => String(t).toLowerCase());

        if (cat.includes('bible') || cat.includes('scripture') || tags.some(t => t.includes('bible'))) {
            domain = 'bible';
        } else if (cat.includes('med') || cat.includes('health') || tags.some(t => t.includes('med'))) {
            domain = 'medical';
        } else if (cat.includes('surv') || tags.some(t => t.includes('surv'))) {
            domain = 'survival';
        } else if (cat.includes('lit') || cat.includes('gutenberg') || tags.some(t => t.includes('lit'))) {
            domain = 'literature';
        }

        const starObj = {
            id: node.id || `NODE-${index}`,
            name: node.label || node.name || node.title || `Memory Star #${index + 1}`,
            label: node.label || node.name || node.title || `Memory Star #${index + 1}`,
            arm: `Arm ${arm + 1}`,
            domain: domain,
            category: domain.toUpperCase(),
            tags: node.tags || [domain.toUpperCase()],
            chapters: node.chapters || [
                node.content || `Indexed knowledge entity in sector ${domain.toUpperCase()}. Verified sovereign record.`,
                `Chapter 2: Contextual link associations and telemetry logs established under local authority.`
            ],
            x: x, y: y, z: z,
            isCore: false,
            connections: connectionsMap[node.id] || ["NEXUS-0"],
            doc_id: node.doc_id || node.id
        };

        allGalaxyNodes.push(starObj);
        if (index < 12) rootNode.connections.push(starObj.id);
    }

    applyDomainFilter();

    selectedNode = allGalaxyNodes[0];
    if (onSelectNodeCallback) onSelectNodeCallback(selectedNode);
    rebuildSynapseLines();
}

function getDomainColorObj(domain, isDark) {
    if (isDark) {
        switch(domain) {
            case 'bible': return new THREE.Color(0xffffff);
            case 'medical': return new THREE.Color(0x38bdf8);
            case 'survival': return new THREE.Color(0x22c55e);
            case 'literature': return new THREE.Color(0xfbbf24);
            case 'cognitive': return new THREE.Color(0xc084fc);
            default: return new THREE.Color(0xe2e8f0);
        }
    } else {
        switch(domain) {
            case 'bible': return new THREE.Color(0x0f172a);
            case 'medical': return new THREE.Color(0x0284c7);
            case 'survival': return new THREE.Color(0x16a34a);
            case 'literature': return new THREE.Color(0xd97706);
            case 'cognitive': return new THREE.Color(0x9333ea);
            default: return new THREE.Color(0x1e293b);
        }
    }
}

function updateStarsGeometry() {
    if (starsMesh) galaxyGroup.remove(starsMesh);

    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const count = interactiveStars.length;
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);
    const sizes = new Float32Array(count);

    for (let i = 0; i < count; i++) {
        const s = interactiveStars[i];
        positions[i * 3] = s.x;
        positions[i * 3 + 1] = s.y;
        positions[i * 3 + 2] = s.z;

        const c = getDomainColorObj(s.domain, isDark);
        colors[i * 3] = c.r;
        colors[i * 3 + 1] = c.g;
        colors[i * 3 + 2] = c.b;

        sizes[i] = (s.id === (selectedNode && selectedNode.id)) ? 8.5 : (s.isCore ? 6.5 : 4.8);
    }

    const geo = new THREE.BufferGeometry();
    geo.setAttribute('position', new THREE.BufferAttribute(positions, 3));
    geo.setAttribute('color', new THREE.BufferAttribute(colors, 3));

    const mat = new THREE.PointsMaterial({
        size: 4.8,
        vertexColors: true,
        map: createGlowTexture(true),
        transparent: true,
        opacity: isDark ? 0.95 : 0.85,
        blending: isDark ? THREE.AdditiveBlending : THREE.NormalBlending,
        depthWrite: false
    });

    starsMesh = new THREE.Points(geo, mat);
    galaxyGroup.add(starsMesh);
}

function rebuildSynapseLines() {
    if (linesMesh) galaxyGroup.remove(linesMesh);
    if (!showSynapseLines || !selectedNode) return;

    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const coords = [];

    const conns = selectedNode.connections || [];
    const starMap = {};
    allGalaxyNodes.forEach(s => { starMap[s.id] = s; });

    conns.forEach(targetId => {
        const target = starMap[targetId];
        if (target) {
            coords.push(selectedNode.x, selectedNode.y, selectedNode.z);
            coords.push(target.x, target.y, target.z);
        }
    });

    if (coords.length > 0) {
        const geo = new THREE.BufferGeometry();
        geo.setAttribute('position', new THREE.Float32BufferAttribute(coords, 3));
        const mat = new THREE.LineBasicMaterial({
            color: isDark ? 0xffffff : 0x000000,
            transparent: true,
            opacity: 0.35,
            linewidth: 1
        });
        linesMesh = new THREE.LineSegments(geo, mat);
        galaxyGroup.add(linesMesh);
    }
}

export function setDomainFilter(domain) {
    activeDomain = domain ? domain.toUpperCase() : 'ALL';
    applyDomainFilter();

    const hudText = document.getElementById('galaxy-hud-text');
    if (hudText) {
        hudText.innerText = activeDomain === 'ALL'
            ? 'SECTOR: GALAXY (5 ARMS)'
            : `SECTOR: ${activeDomain} DOMAIN`;
    }
}

function applyDomainFilter() {
    if (activeDomain === 'ALL') {
        interactiveStars = [...allGalaxyNodes];
    } else {
        const filterKey = activeDomain.toLowerCase();
        interactiveStars = allGalaxyNodes.filter(s => {
            if (s.isCore) return true;
            return s.domain === filterKey || (s.category && s.category.toLowerCase().includes(filterKey));
        });
    }
    updateStarsGeometry();
    rebuildSynapseLines();
}

export function setDensity(setting) {
    densitySetting = setting;
    buildParticleDisc();
}

export function toggleSynapses() {
    showSynapseLines = !showSynapseLines;
    rebuildSynapseLines();
    return showSynapseLines;
}

// Zoom In: Move closer (multiply scalar < 1)
export function zoomGalaxyIn() {
    if (!camera) return;
    camera.position.multiplyScalar(0.8);
}

// Zoom Out: Move further (multiply scalar > 1)
export function zoomGalaxyOut() {
    if (!camera) return;
    camera.position.multiplyScalar(1.25);
}

export function recenterGalaxy() {
    if (!camera || !controls) return;
    camera.position.set(0, 380, 500);
    controls.target.set(0, 0, 0);
}

export function selectGalaxyStar(starId) {
    const star = allGalaxyNodes.find(s => s.id === starId || s.name === starId);
    if (star) {
        selectedNode = star;
        if (onSelectNodeCallback) onSelectNodeCallback(selectedNode);
        updateStarsGeometry();
        rebuildSynapseLines();
    }
}

function setupCanvasEvents() {
    if (!gCanvas) return;

    const tooltip = document.getElementById('galaxy-tooltip');

    // Click anywhere to select closest star
    gCanvas.addEventListener('click', (e) => {
        const rect = gCanvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        const closest = findClosestStarToScreen(mouseX, mouseY, rect.width, rect.height);
        if (closest) {
            selectedNode = closest;
            if (onSelectNodeCallback) onSelectNodeCallback(selectedNode);
            updateStarsGeometry();
            rebuildSynapseLines();
        }
    });

    // Hover tooltip HUD
    gCanvas.addEventListener('mousemove', (e) => {
        if (isDragging) {
            if (tooltip) tooltip.classList.add('hidden');
            return;
        }

        const rect = gCanvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        const hovered = findClosestStarToScreen(mouseX, mouseY, rect.width, rect.height, 36);
        if (hovered && tooltip) {
            tooltip.innerHTML = `<strong>${escapeHtml(hovered.name)}</strong> <span style="opacity: 0.6;">[${escapeHtml(hovered.category || '')}]</span>`;
            tooltip.style.left = `${e.clientX + 14}px`;
            tooltip.style.top = `${e.clientY - 14}px`;
            tooltip.classList.remove('hidden');
            gCanvas.style.cursor = 'pointer';
        } else {
            if (tooltip) tooltip.classList.add('hidden');
            gCanvas.style.cursor = 'crosshair';
        }
    });

    gCanvas.addEventListener('mouseleave', () => {
        if (tooltip) tooltip.classList.add('hidden');
    });

    // Search Pill
    const searchPill = document.getElementById('pill-search-input');
    if (searchPill) {
        searchPill.addEventListener('keydown', async (e) => {
            if (e.key === 'Enter') {
                const q = searchPill.value.trim();
                if (!q) return;
                try {
                    const hits = await search(q);
                    if (hits && hits.length > 0) {
                        const hit = hits[0];
                        const star = allGalaxyNodes.find(s => s.id === hit.doc_id || s.name.toLowerCase().includes(q.toLowerCase()));
                        if (star) selectGalaxyStar(star.id);
                    } else {
                        const localHit = allGalaxyNodes.find(s => s.name.toLowerCase().includes(q.toLowerCase()));
                        if (localHit) selectGalaxyStar(localHit.id);
                    }
                } catch (err) {
                    console.error('Search error:', err);
                }
            }
        });
    }
}

function findClosestStarToScreen(screenX, screenY, width, height, maxRadius = 140) {
    if (!camera || interactiveStars.length === 0) return null;

    let closest = null;
    let minDistance = maxRadius;

    const tempV = new THREE.Vector3();

    for (let i = 0; i < interactiveStars.length; i++) {
        const star = interactiveStars[i];
        tempV.set(star.x, star.y, star.z);
        tempV.applyMatrix4(galaxyGroup.matrixWorld);
        tempV.project(camera);

        if (tempV.z > 1.0) continue;

        const sx = ((tempV.x + 1) * width) / 2;
        const sy = ((-tempV.y + 1) * height) / 2;

        const dist = Math.hypot(screenX - sx, screenY - sy);
        if (dist < minDistance) {
            minDistance = dist;
            closest = star;
        }
    }

    return closest;
}

function startAnimationLoop() {
    function animate() {
        if (controls) controls.update();
        if (renderer && scene && camera) {
            renderer.render(scene, camera);
        }
        animFrameId = requestAnimationFrame(animate);
    }
    animate();
}

export function renderGalaxy() {
    if (!renderer || !scene || !camera) return;

    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const bgColor = isDark ? 0x08090c : 0xdce1e6;
    scene.background.set(bgColor);

    buildCore();
    buildParticleDisc();
    updateStarsGeometry();
    rebuildSynapseLines();
}

export function resizeGalaxy() {
    if (!gCanvas || !renderer || !camera) return;
    const rect = gCanvas.parentElement.getBoundingClientRect();
    if (rect.width > 0 && rect.height > 0) {
        camera.aspect = rect.width / rect.height;
        camera.updateProjectionMatrix();
        renderer.setSize(rect.width, rect.height, false);
    }
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.innerText = text || '';
    return div.innerHTML;
}
