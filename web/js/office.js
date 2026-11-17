// MAZZAROTH MULTI-ROOM LIBRARIAN OFFICE
// Powered by MIT-licensed Claude-Office pixel-art sprites & rooms + custom Pixel Studio overrides.
// Features: 5 interconnected rooms, 6 domain specialists, Day/Night theme sync,
// Drag-and-Drop workers, clickable room doorways/hotspots, and real-time Ollama/SLM SSE animations.

import { loadCustomSprites, loadCustomRooms } from './sprite_studio.js';

export const OFFICE_ROOMS = [
    {
        id: 'office',
        name: 'Archive Hall',
        dayImg: '/assets/office/rooms/office-day.png',
        nightImg: '/assets/office/rooms/office-night.png',
        hotspots: [
            { id: 'door_corridor', label: 'Corridor', nx: 0.16, ny: 0.42, r: 0.06, targetRoom: 'corridor' },
            { id: 'bookshelf', label: 'Scripture Stacks', nx: 0.34, ny: 0.36, r: 0.07, workerId: 'archivist', msg: '66 languages · 226 translations indexed!' },
            { id: 'coffee', label: 'Coffee Station', nx: 0.78, ny: 0.44, r: 0.06, action: 'coffee' }
        ]
    },
    {
        id: 'meeting-room',
        name: 'Study & Maps',
        dayImg: '/assets/office/rooms/meeting-room-day.webp',
        nightImg: '/assets/office/rooms/meeting-room-night.webp',
        hotspots: [
            { id: 'door_corridor', label: 'Back to Corridor', nx: 0.14, ny: 0.45, r: 0.07, targetRoom: 'corridor' },
            { id: 'orrery', label: 'Star Charts', nx: 0.52, ny: 0.48, r: 0.08, workerId: 'astronomer', msg: '88 constellations & Polaris sight reduction!' },
            { id: 'map_table', label: 'Medical & Field Table', nx: 0.68, ny: 0.56, r: 0.08, workerId: 'medic', msg: 'Clinical & FM 21-76 protocols ready!' }
        ]
    },
    {
        id: 'server-room',
        name: 'Vault Servers',
        dayImg: '/assets/office/rooms/server-room-day.webp',
        nightImg: '/assets/office/rooms/server-room-night.webp',
        hotspots: [
            { id: 'door_corridor', label: 'Back to Corridor', nx: 0.16, ny: 0.46, r: 0.07, targetRoom: 'corridor' },
            { id: 'terminal', label: 'FTS5 Index Rack', nx: 0.52, ny: 0.44, r: 0.09, workerId: 'chronicler', msg: 'SQLite FTS5 & QuoteGuard online!' }
        ]
    },
    {
        id: 'lunch-room',
        name: 'Break Lounge',
        dayImg: '/assets/office/rooms/lunch-room-day.webp',
        nightImg: '/assets/office/rooms/lunch-room-night.webp',
        hotspots: [
            { id: 'door_corridor', label: 'Back to Corridor', nx: 0.15, ny: 0.46, r: 0.07, targetRoom: 'corridor' },
            { id: 'coffee', label: 'Espresso Bar', nx: 0.66, ny: 0.44, r: 0.08, action: 'coffee' }
        ]
    },
    {
        id: 'corridor',
        name: 'Corridor',
        dayImg: '/assets/office/rooms/corridor-day.webp',
        nightImg: '/assets/office/rooms/corridor-night.webp',
        hotspots: [
            { id: 'to_office', label: 'Archive Hall', nx: 0.22, ny: 0.50, r: 0.07, targetRoom: 'office' },
            { id: 'to_meeting', label: 'Study & Maps', nx: 0.42, ny: 0.46, r: 0.07, targetRoom: 'meeting-room' },
            { id: 'to_server', label: 'Vault Servers', nx: 0.62, ny: 0.46, r: 0.07, targetRoom: 'server-room' },
            { id: 'to_lunch', label: 'Break Lounge', nx: 0.80, ny: 0.50, r: 0.07, targetRoom: 'lunch-room' }
        ]
    }
];

export const OFFICE_WORKERS = [
    {
        id: 'coordinator',
        name: 'Chief Librarian',
        shortName: 'Chief',
        role: 'Vault Coordinator',
        specialty: 'All archives, catalog inventory & cross-domain synthesis',
        spriteChar: 'Me-1',
        shirt: '#3b82f6',
        room: 'office',
        desk: { nx: 0.44, ny: 0.52 },
        station: { nx: 0.38, ny: 0.48 },
        facing: 'front-right',
        prompts: [
            'What languages and collections are in the vault?',
            'Summarize the Prime Core Nexus archives'
        ]
    },
    {
        id: 'archivist',
        name: 'The Archivist',
        shortName: 'Archivist',
        role: 'Scripture & Texts',
        specialty: '66 languages, 226 translations, Greek/Hebrew linguistics',
        spriteChar: 'Claude-1',
        shirt: '#f59e0b',
        room: 'office',
        desk: { nx: 0.30, ny: 0.62 },
        station: { nx: 0.34, ny: 0.46 },
        facing: 'front-right',
        prompts: [
            'Compare John 1:1 across translations in the vault',
            'What does Genesis 1:1 say in the available texts?'
        ]
    },
    {
        id: 'pathfinder',
        name: 'The Pathfinder',
        shortName: 'Pathfinder',
        role: 'Survival & Maps',
        specialty: 'FM 21-76 survival, pathfinders, knotcraft & cartography',
        spriteChar: 'explore-1',
        shirt: '#84cc16',
        room: 'office',
        desk: { nx: 0.64, ny: 0.60 },
        station: { nx: 0.56, ny: 0.56 },
        facing: 'front-left',
        prompts: [
            'What are the core water purification steps in FM 21-76?',
            'Explain wilderness shelter and firecraft priorities'
        ]
    },
    {
        id: 'astronomer',
        name: 'The Astronomer',
        shortName: 'Astronomer',
        role: 'Celestial & Stars',
        specialty: '88 constellations, Polaris alignment & star navigation',
        spriteChar: 'dev-1',
        shirt: '#6366f1',
        room: 'meeting-room',
        desk: { nx: 0.46, ny: 0.56 },
        station: { nx: 0.54, ny: 0.48 },
        facing: 'front-left',
        prompts: [
            'Explain the Orion Constellation asterism',
            'How is Polaris used for celestial latitude?'
        ]
    },
    {
        id: 'medic',
        name: 'Field Medic',
        shortName: 'Field Medic',
        role: 'Medical & Triage',
        specialty: 'Clinical medicine, anatomy, pathology & emergency triage',
        spriteChar: 'Frontend-dev-1',
        shirt: '#10b981',
        room: 'meeting-room',
        desk: { nx: 0.64, ny: 0.62 },
        station: { nx: 0.60, ny: 0.54 },
        facing: 'front-left',
        prompts: [
            'Summarize emergency airway and triage protocols',
            'What clinical conditions are indexed in the medical vault?'
        ]
    },
    {
        id: 'chronicler',
        name: 'The Chronicler',
        shortName: 'Chronicler',
        role: 'Vault & Memory',
        specialty: 'FTS5 full-vault search, memory stars & deep synthesis',
        spriteChar: 'security-audit-1',
        shirt: '#ec4899',
        room: 'server-room',
        desk: { nx: 0.50, ny: 0.58 },
        station: { nx: 0.52, ny: 0.46 },
        facing: 'front-right',
        prompts: [
            'Search the vault for navigation and star lore',
            'What medical and biological topics are indexed?'
        ]
    }
];

// Offline WebAudio synthesizer for subtle office ambience (zero external audio files)
class OfficeAudio {
    constructor() {
        this.enabled = false;
        this.ctx = null;
        this.lastTypeAt = 0;
    }

    toggle() {
        this.enabled = !this.enabled;
        if (this.enabled && !this.ctx && typeof AudioContext !== 'undefined') {
            this.ctx = new AudioContext();
        }
        if (this.enabled && this.ctx?.state === 'suspended') {
            this.ctx.resume().catch(() => {});
        }
        return this.enabled;
    }

    playTypeClick() {
        if (!this.enabled || !this.ctx) return;
        const now = performance.now();
        if (now - this.lastTypeAt < 95) return;
        this.lastTypeAt = now;
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'triangle';
        osc.frequency.setValueAtTime(680 + Math.random() * 180, this.ctx.currentTime);
        gain.gain.setValueAtTime(0.018, this.ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.0008, this.ctx.currentTime + 0.035);
        osc.connect(gain).connect(this.ctx.destination);
        osc.start();
        osc.stop(this.ctx.currentTime + 0.04);
    }

    playPageTurn() {
        if (!this.enabled || !this.ctx) return;
        const osc = this.ctx.createOscillator();
        const gain = this.ctx.createGain();
        osc.type = 'sine';
        osc.frequency.setValueAtTime(320, this.ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(520, this.ctx.currentTime + 0.09);
        gain.gain.setValueAtTime(0.025, this.ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.0008, this.ctx.currentTime + 0.11);
        osc.connect(gain).connect(this.ctx.destination);
        osc.start();
        osc.stop(this.ctx.currentTime + 0.12);
    }

    playDoneChime() {
        if (!this.enabled || !this.ctx) return;
        [523.25, 659.25].forEach((freq, idx) => {
            const osc = this.ctx.createOscillator();
            const gain = this.ctx.createGain();
            const t = this.ctx.currentTime + idx * 0.11;
            osc.type = 'sine';
            osc.frequency.setValueAtTime(freq, t);
            gain.gain.setValueAtTime(0.035, t);
            gain.gain.exponentialRampToValueAtTime(0.0008, t + 0.28);
            osc.connect(gain).connect(this.ctx.destination);
            osc.start(t);
            osc.stop(t + 0.3);
        });
    }
}

export const officeAudio = new OfficeAudio();

const CORRIDOR_DOORS = {
    office: { nx: 0.22, ny: 0.50 },
    'meeting-room': { nx: 0.42, ny: 0.46 },
    'server-room': { nx: 0.62, ny: 0.46 },
    'lunch-room': { nx: 0.80, ny: 0.50 }
};

const OFFICE_PROPS_KEY = 'mazzaroth_office_props_v1';

export const PLACEABLE_PROPS = [
    { id: 'book_stack', label: 'Book Stack' },
    { id: 'lamp', label: 'Desk Lamp' },
    { id: 'plant', label: 'Potted Plant' },
    { id: 'globe', label: 'Star Globe' }
];

const LOUNGE_SLOTS = [
    { nx: 0.44, ny: 0.56 },
    { nx: 0.54, ny: 0.64 },
    { nx: 0.62, ny: 0.52 },
    { nx: 0.48, ny: 0.68 },
    { nx: 0.68, ny: 0.62 },
    { nx: 0.56, ny: 0.50 }
];

class LibrarianOffice {
    constructor() {
        this.canvas = null;
        this.ctx = null;
        this.rafId = null;
        this.lastTime = 0;
        this.elapsed = 0;
        this.dpr = window.devicePixelRatio || 1;
        this.width = 0;
        this.height = 0;
        this.roomRect = { x: 0, y: 0, w: 0, h: 0 };

        this.activeRoomId = 'office';
        this.selectedWorkerId = 'coordinator';
        this.onSelectWorker = null;
        this.onRoomChange = null;

        this.dbStats = { docs: 63, chunks: 6825 };
        this.activeTask = null;
        this.particles = [];
        this.imageCache = new Map();
        this.bundledArtPack = { sprites: {}, rooms: {} };
        this.customSprites = {};
        this.customRooms = {};
        this.roomProps = this.loadRoomProps();
        this.dragState = null;
        this.dragPropState = null;
        this.idleWanderTimer = 2.5;

        this.workers = OFFICE_WORKERS.map((def, idx) => ({
            ...def,
            slotIdx: idx,
            homeRoom: def.room,
            nx: def.desk.nx,
            ny: def.desk.ny,
            homeNx: def.desk.nx,
            homeNy: def.desk.ny,
            waypoints: [],
            state: 'idle',
            coffeeTtl: 0,
            bubble: null,
            alertTtl: 0,
            walkFrameIdx: 0,
            walkTimer: 0,
            lookTimer: 2.0 + idx * 1.1,
            lastAction: 'Ready at station',
            screenBounds: null
        }));
    }

    loadRoomProps() {
        try {
            return JSON.parse(localStorage.getItem(OFFICE_PROPS_KEY) || '{}');
        } catch {
            return {};
        }
    }

    saveRoomProps() {
        localStorage.setItem(OFFICE_PROPS_KEY, JSON.stringify(this.roomProps));
    }

    addProp(kind) {
        if (!this.roomProps[this.activeRoomId]) this.roomProps[this.activeRoomId] = [];
        this.roomProps[this.activeRoomId].push({
            uid: `${kind}-${Date.now()}`,
            kind,
            nx: 0.45 + (Math.random() - 0.5) * 0.2,
            ny: 0.60 + (Math.random() - 0.5) * 0.14
        });
        this.saveRoomProps();
    }

    clearRoomProps() {
        this.roomProps[this.activeRoomId] = [];
        this.saveRoomProps();
    }

    mount(canvasEl, callbacks = {}) {
        if (!canvasEl) return;
        this.canvas = canvasEl;
        this.ctx = canvasEl.getContext('2d');
        this.onSelectWorker = callbacks.onSelectWorker || null;
        this.onRoomChange = callbacks.onRoomChange || null;

        this.reloadCustomAssets();
        this.loadBundledArtPack();
        this.preloadAssets();
        this.resize();

        window.addEventListener('resize', () => this.resize());
        if (typeof ResizeObserver !== 'undefined' && canvasEl.parentElement) {
            this.ro = new ResizeObserver(() => this.resize());
            this.ro.observe(canvasEl.parentElement);
        }

        canvasEl.addEventListener('pointerdown', (e) => this.handlePointerDown(e));
        window.addEventListener('pointermove', (e) => this.handlePointerMove(e));
        window.addEventListener('pointerup', () => this.handlePointerUp());

        if (!this.rafId) {
            this.lastTime = performance.now();
            const loop = (now) => {
                const dt = Math.min(0.05, (now - this.lastTime) / 1000);
                this.lastTime = now;
                this.update(dt);
                this.render();
                this.rafId = requestAnimationFrame(loop);
            };
            this.rafId = requestAnimationFrame(loop);
        }
    }

    async loadBundledArtPack() {
        try {
            const res = await fetch('/assets/office/art-pack.json');
            if (!res.ok) return;
            const pack = await res.json();
            if (pack && typeof pack === 'object') {
                this.bundledArtPack = {
                    sprites: pack.sprites || {},
                    rooms: pack.rooms || {}
                };
                this.reloadCustomAssets();
            }
        } catch {
            // Optional bundled pack; ignore when absent
        }
    }

    reloadCustomAssets() {
        this.customSprites = { ...(this.bundledArtPack.sprites || {}), ...loadCustomSprites() };
        this.customRooms = { ...(this.bundledArtPack.rooms || {}), ...loadCustomRooms() };
        // Clear data-URL entries so updated images reload immediately
        for (const key of this.imageCache.keys()) {
            if (key.startsWith('data:')) this.imageCache.delete(key);
        }
    }

    preloadAssets() {
        for (const r of OFFICE_ROOMS) {
            this.getImage(r.dayImg);
            this.getImage(r.nightImg);
        }
        for (const w of OFFICE_WORKERS) {
            for (const dir of ['front-left', 'front-right', 'rear-left', 'rear-right']) {
                this.getImage(`/assets/office/sprites/characters/${w.spriteChar}-${dir}.png`);
                for (let f = 1; f <= 4; f++) {
                    this.getImage(`/assets/office/sprites/characters/${w.spriteChar}-${dir}-walk${f}.webp`);
                }
            }
        }
    }

    getImage(src) {
        if (!src) return null;
        if (this.imageCache.has(src)) return this.imageCache.get(src);
        const img = new Image();
        img.src = src;
        this.imageCache.set(src, img);
        return img;
    }

    resize() {
        if (!this.canvas || !this.canvas.parentElement) return;
        const rect = this.canvas.parentElement.getBoundingClientRect();
        if (rect.width < 40 || rect.height < 40) return;
        this.dpr = Math.min(window.devicePixelRatio || 1, 2);
        this.width = Math.floor(rect.width);
        this.height = Math.floor(rect.height);
        this.canvas.width = Math.floor(this.width * this.dpr);
        this.canvas.height = Math.floor(this.height * this.dpr);
    }

    setRoom(roomId) {
        if (!OFFICE_ROOMS.some(r => r.id === roomId)) return;
        this.activeRoomId = roomId;
        if (this.onRoomChange) this.onRoomChange(roomId);
    }

    setDbStats(docs, chunks) {
        if (typeof docs === 'number') this.dbStats.docs = docs;
        if (typeof chunks === 'number') this.dbStats.chunks = chunks;
    }

    selectWorker(workerId, switchToWorkerRoom = true) {
        const w = this.workers.find(x => x.id === workerId);
        if (!w) return;
        this.selectedWorkerId = w.id;
        if (switchToWorkerRoom && w.room !== this.activeRoomId) {
            this.setRoom(w.room);
        }
        if (this.onSelectWorker) this.onSelectWorker(w);
    }

    getSelectedWorker() {
        return this.workers.find(w => w.id === this.selectedWorkerId) || this.workers[0];
    }

    getWorker(workerId) {
        return this.workers.find(w => w.id === workerId) || this.workers[0];
    }

    handleBackendEvent(ev) {
        if (!ev || !ev.stage) return;
        const worker = this.getWorker(ev.worker || this.selectedWorkerId);
        const stage = ev.stage;

        if (stage === 'dispatch') {
            this.activeTask = { workerId: worker.id, stage: 'dispatch', startTime: performance.now() };
            worker.coffeeTtl = 0;
            if (worker.room !== worker.homeRoom) {
                worker.room = worker.homeRoom;
                worker.nx = worker.homeNx;
                worker.ny = worker.homeNy;
                worker.waypoints = [];
            }
            if (worker.room !== this.activeRoomId) this.setRoom(worker.room);
            worker.state = 'alert';
            worker.alertTtl = 0.65;
            worker.lastAction = ev.detail || 'Query received';
            worker.bubble = { text: 'On it!', ttl: 1.6, kind: 'accent' };
        } else if (stage === 'handoff') {
            const target = ev.consult ? this.getWorker(ev.consult) : null;
            if (target) {
                worker.lastAction = `Consulting ${target.shortName}`;
                worker.bubble = { text: `Consulting @${target.shortName}...`, ttl: 2.2, kind: 'accent' };
                target.bubble = { text: 'Sharing archive notes!', ttl: 2.2, kind: 'info' };
                if (target.room === worker.room) {
                    this.moveWorkerTo(worker, target.nx + 0.06, target.ny, 'searching');
                }
            }
        } else if (stage === 'search') {
            officeAudio.playPageTurn();
            this.moveWorkerTo(worker, worker.station.nx, worker.station.ny, 'searching');
            worker.lastAction = ev.detail || 'Searching FTS5...';
            worker.bubble = { text: 'Searching FTS5...', ttl: 2.2, kind: 'info' };
        } else if (stage === 'found') {
            const hits = ev.hits ?? 0;
            const docs = ev.docs ?? 0;
            worker.lastAction = `Found ${hits} passages (${docs} docs)`;
            worker.bubble = {
                text: hits > 0 ? `Found ${hits} hits!` : 'Checking catalog...',
                ttl: 2.0,
                kind: hits > 0 ? 'success' : 'info'
            };
        } else if (stage === 'think') {
            this.moveWorkerTo(worker, worker.homeNx, worker.homeNy, 'thinking');
            worker.lastAction = ev.detail || `Evaluating on ${ev.model || 'Ollama'}`;
            worker.bubble = { text: 'Reading passages...', ttl: 3.5, kind: 'think' };
        } else if (stage === 'stream') {
            worker.waypoints = [];
            worker.nx = worker.homeNx;
            worker.ny = worker.homeNy;
            worker.state = 'typing';
            worker.lastAction = 'Drafting grounded reply...';
            worker.bubble = { text: 'Writing reply...', ttl: 2.4, kind: 'accent' };
        } else if (stage === 'verify') {
            worker.state = 'verify';
            worker.lastAction = ev.detail || 'QuoteGuard verified';
            worker.bubble = { text: '✓ Verbatim!', ttl: 2.4, kind: 'success' };
        } else if (stage === 'done') {
            officeAudio.playDoneChime();
            const elapsedMs = this.activeTask?.startTime
                ? Math.round(performance.now() - this.activeTask.startTime)
                : null;
            worker.state = 'completed';
            worker.lastAction = elapsedMs ? `Delivered in ${(elapsedMs / 1000).toFixed(1)}s` : 'Task complete';
            worker.bubble = { text: '✓ Done!', ttl: 2.2, kind: 'success' };
            setTimeout(() => {
                if (worker.state === 'completed') worker.state = 'idle';
            }, 2200);
            this.activeTask = null;
        }
        if (this.onSelectWorker && worker.id === this.selectedWorkerId) {
            this.onSelectWorker(worker);
        }
    }

    moveWorkerTo(worker, tnx, tny, arrivalState) {
        if (Math.hypot(tnx - worker.nx, tny - worker.ny) < 0.02) {
            worker.nx = tnx;
            worker.ny = tny;
            worker.state = arrivalState;
            return;
        }
        worker.waypoints = [{ nx: tnx, ny: tny, arrivalState }];
        worker.state = 'walking';
    }

    walkWorkerCrossRoom(worker, targetRoomId, tnx, tny, arrivalState = 'idle') {
        if (worker.room === targetRoomId) {
            this.moveWorkerTo(worker, tnx, tny, arrivalState);
            return;
        }
        const curRoom = OFFICE_ROOMS.find(r => r.id === worker.room);
        const curDoor = curRoom?.hotspots.find(h => h.targetRoom === 'corridor') || { nx: 0.16, ny: 0.46 };
        const corrStart = CORRIDOR_DOORS[worker.room] || { nx: 0.22, ny: 0.50 };
        const corrEnd = CORRIDOR_DOORS[targetRoomId] || { nx: 0.80, ny: 0.50 };
        const destRoom = OFFICE_ROOMS.find(r => r.id === targetRoomId);
        const destDoor = destRoom?.hotspots.find(h => h.targetRoom === 'corridor') || { nx: 0.16, ny: 0.46 };

        worker.waypoints = [];
        if (worker.room !== 'corridor') {
            worker.waypoints.push({ nx: curDoor.nx, ny: curDoor.ny, switchRoom: 'corridor', enterNx: corrStart.nx, enterNy: corrStart.ny });
        }
        if (targetRoomId !== 'corridor') {
            worker.waypoints.push({ nx: corrEnd.nx, ny: corrEnd.ny, switchRoom: targetRoomId, enterNx: destDoor.nx, enterNy: destDoor.ny });
        }
        worker.waypoints.push({ nx: tnx, ny: tny, arrivalState });
        worker.state = 'walking';
    }

    handlePointerDown(e) {
        if (!this.canvas) return;
        const rect = this.canvas.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;

        // 1. Check workers in active room (front-most first)
        const roomWorkers = this.workers
            .filter(w => w.room === this.activeRoomId)
            .sort((a, b) => b.ny - a.ny);

        for (const w of roomWorkers) {
            const b = w.screenBounds;
            if (b && mx >= b.x && mx <= b.x + b.w && my >= b.y && my <= b.y + b.h) {
                this.selectWorker(w.id, false);
                this.dragState = {
                    worker: w,
                    startX: mx,
                    startY: my,
                    isDragging: false
                };
                return;
            }
        }

        // 1b. Check placed props in active room
        const props = this.roomProps[this.activeRoomId] || [];
        if (this.roomRect.w > 0) {
            const pnx = (mx - this.roomRect.x) / this.roomRect.w;
            const pny = (my - this.roomRect.y) / this.roomRect.h;
            for (let i = props.length - 1; i >= 0; i--) {
                const pr = props[i];
                if (Math.hypot(pnx - pr.nx, pny - pr.ny) <= 0.04) {
                    this.dragPropState = { prop: pr };
                    return;
                }
            }
        }

        // 2. Check room hotspots / doorways
        const room = OFFICE_ROOMS.find(r => r.id === this.activeRoomId);
        if (room && this.roomRect.w > 0) {
            const nx = (mx - this.roomRect.x) / this.roomRect.w;
            const ny = (my - this.roomRect.y) / this.roomRect.h;
            for (const hs of room.hotspots) {
                if (Math.hypot(nx - hs.nx, ny - hs.ny) <= hs.r) {
                    if (hs.targetRoom) {
                        this.setRoom(hs.targetRoom);
                    } else if (hs.action === 'coffee') {
                        const idle = roomWorkers.find(w => w.state === 'idle') || roomWorkers[0];
                        if (idle) {
                            this.moveWorkerTo(idle, hs.nx, hs.ny + 0.06, 'coffee');
                            idle.bubble = { text: '☕ Espresso break!', ttl: 2.4, kind: 'info' };
                            setTimeout(() => {
                                if (idle.state === 'coffee') this.moveWorkerTo(idle, idle.homeNx, idle.homeNy, 'idle');
                            }, 3500);
                        }
                    } else if (hs.workerId) {
                        const target = this.getWorker(hs.workerId);
                        this.selectWorker(hs.workerId, false);
                        this.moveWorkerTo(target, hs.nx, hs.ny + 0.05, 'searching');
                        target.bubble = { text: hs.msg, ttl: 2.6, kind: 'info' };
                    }
                    return;
                }
            }
        }
    }

    handlePointerMove(e) {
        if (!this.canvas) return;
        const rect = this.canvas.getBoundingClientRect();
        const mx = e.clientX - rect.left;
        const my = e.clientY - rect.top;

        if (this.dragPropState && this.roomRect.w > 0) {
            this.dragPropState.prop.nx = Math.max(0.08, Math.min(0.92, (mx - this.roomRect.x) / this.roomRect.w));
            this.dragPropState.prop.ny = Math.max(0.22, Math.min(0.90, (my - this.roomRect.y) / this.roomRect.h));
            this.canvas.style.cursor = 'grabbing';
            return;
        }

        if (this.dragState) {
            const dist = Math.hypot(mx - this.dragState.startX, my - this.dragState.startY);
            if (dist > 5 && this.roomRect.w > 0) {
                this.dragState.isDragging = true;
                this.dragState.worker.waypoints = [];
                this.dragState.worker.state = 'dragged';
                const nx = Math.max(0.08, Math.min(0.92, (mx - this.roomRect.x) / this.roomRect.w));
                const ny = Math.max(0.22, Math.min(0.90, (my - this.roomRect.y) / this.roomRect.h));
                this.dragState.worker.nx = nx;
                this.dragState.worker.ny = ny;
                this.canvas.style.cursor = 'grabbing';
            }
            return;
        }

        const roomWorkers = this.workers.filter(w => w.room === this.activeRoomId);
        let cursor = 'default';
        for (const w of roomWorkers) {
            const b = w.screenBounds;
            if (b && mx >= b.x && mx <= b.x + b.w && my >= b.y && my <= b.y + b.h) {
                cursor = 'grab';
                break;
            }
        }
        if (cursor === 'default' && this.roomRect.w > 0) {
            const room = OFFICE_ROOMS.find(r => r.id === this.activeRoomId);
            const nx = (mx - this.roomRect.x) / this.roomRect.w;
            const ny = (my - this.roomRect.y) / this.roomRect.h;
            if (room?.hotspots.some(hs => Math.hypot(nx - hs.nx, ny - hs.ny) <= hs.r)) {
                cursor = 'pointer';
            }
        }
        this.canvas.style.cursor = cursor;
    }

    handlePointerUp() {
        if (this.dragPropState) {
            this.dragPropState = null;
            this.saveRoomProps();
            if (this.canvas) this.canvas.style.cursor = 'default';
            return;
        }
        if (!this.dragState) return;
        const { worker, isDragging } = this.dragState;
        this.dragState = null;

        if (isDragging) {
            worker.homeNx = worker.nx;
            worker.homeNy = worker.ny;
            worker.state = 'idle';
            worker.lastAction = 'Relocated in room';
            worker.bubble = { text: 'Nice spot!', ttl: 1.8, kind: 'accent' };
            if (this.onSelectWorker) this.onSelectWorker(worker);
        } else {
            worker.bubble = { text: 'Ask me anything!', ttl: 1.8, kind: 'accent' };
        }
        if (this.canvas) this.canvas.style.cursor = 'default';
    }

    update(dt) {
        this.elapsed += dt;

        // Autonomous living office: idle workers stroll around or walk cross-room for coffee
        this.idleWanderTimer -= dt;
        if (this.idleWanderTimer <= 0) {
            this.idleWanderTimer = 4.0 + Math.random() * 4.5;
            const idleWorkers = this.workers.filter(w => w.state === 'idle' && w.waypoints.length === 0);
            const loungeCount = this.workers.filter(w =>
                w.room === 'lunch-room' || w.waypoints.some(wp => wp.switchRoom === 'lunch-room')
            ).length;

            if (idleWorkers.length > 0 && !this.dragState && !this.dragPropState) {
                const w = idleWorkers[Math.floor(Math.random() * idleWorkers.length)];
                if (w.room !== w.homeRoom) {
                    this.walkWorkerCrossRoom(w, w.homeRoom, w.homeNx, w.homeNy, 'idle');
                    w.lastAction = 'Returning to desk';
                } else if (loungeCount < 2 && Math.random() < 0.25) {
                    const slot = LOUNGE_SLOTS[w.slotIdx % LOUNGE_SLOTS.length];
                    this.walkWorkerCrossRoom(w, 'lunch-room', slot.nx, slot.ny, 'coffee');
                    w.lastAction = 'Walking to Break Lounge';
                    w.bubble = { text: '☕ Coffee break', ttl: 2.2, kind: 'info' };
                } else {
                    // Stroll to a nearby spot in their own room
                    const jx = Math.max(0.22, Math.min(0.78, w.homeNx + (Math.random() - 0.5) * 0.20));
                    const jy = Math.max(0.38, Math.min(0.74, w.homeNy + (Math.random() - 0.5) * 0.14));
                    this.moveWorkerTo(w, jx, jy, 'idle');
                }
            }
        }

        for (const w of this.workers) {
            if (w.alertTtl > 0) {
                w.alertTtl -= dt;
                if (w.alertTtl <= 0 && w.state === 'alert') w.state = 'thinking';
            }
            if (w.bubble) {
                w.bubble.ttl -= dt;
                if (w.bubble.ttl <= 0) w.bubble = null;
            }
            if (w.state === 'coffee' && w.waypoints.length === 0) {
                w.coffeeTtl -= dt;
                if (w.coffeeTtl <= 0) {
                    this.walkWorkerCrossRoom(w, w.homeRoom, w.homeNx, w.homeNy, 'idle');
                    w.lastAction = 'Returning to desk';
                }
            }

            // Subtle idle looking around
            if (w.state === 'idle' && w.waypoints.length === 0) {
                w.lookTimer -= dt;
                if (w.lookTimer <= 0) {
                    w.lookTimer = 3.0 + Math.random() * 4.5;
                    w.facing = w.facing === 'front-left' ? 'front-right' : 'front-left';
                }
            }

            if (w.waypoints.length > 0 && w.state !== 'dragged') {
                const wp = w.waypoints[0];
                const dx = wp.nx - w.nx;
                const dy = wp.ny - w.ny;
                const dist = Math.hypot(dx, dy);
                const speed = 0.22; // normalized room units per sec
                if (dist <= speed * dt) {
                    w.nx = wp.nx;
                    w.ny = wp.ny;
                    w.waypoints.shift();
                    if (wp.switchRoom) {
                        w.room = wp.switchRoom;
                        if (typeof wp.enterNx === 'number') w.nx = wp.enterNx;
                        if (typeof wp.enterNy === 'number') w.ny = wp.enterNy;
                    }
                    if (w.waypoints.length === 0) {
                        w.state = wp.arrivalState || 'idle';
                        if (w.state === 'coffee') {
                            w.coffeeTtl = 4.5;
                            w.lastAction = 'Having coffee';
                        }
                    }
                } else {
                    w.nx += (dx / dist) * speed * dt;
                    w.ny += (dy / dist) * speed * dt;
                    w.walkTimer += dt;
                    if (w.walkTimer >= 0.12) {
                        w.walkTimer = 0;
                        w.walkFrameIdx = (w.walkFrameIdx + 1) % 4;
                    }
                    if (dy < 0 && dx >= 0) w.facing = 'front-left';
                    else if (dy < 0 && dx < 0) w.facing = 'front-right';
                    else if (dy >= 0 && dx >= 0) w.facing = 'rear-left';
                    else w.facing = 'front-left';
                }
            }

            if (w.state === 'typing') {
                officeAudio.playTypeClick();
                if (Math.random() < dt * 4 && this.roomRect.w > 0) {
                    const sx = this.roomRect.x + w.nx * this.roomRect.w;
                    const sy = this.roomRect.y + w.ny * this.roomRect.h - 46;
                    this.particles.push({
                        x: sx + (Math.random() - 0.5) * 16,
                        y: sy,
                        vx: (Math.random() - 0.5) * 14,
                        vy: -18 - Math.random() * 10,
                        life: 0.7,
                        size: 3,
                        color: w.shirt
                    });
                }
            }
        }

        for (let i = this.particles.length - 1; i >= 0; i--) {
            const p = this.particles[i];
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
            if (p.life <= 0) this.particles.splice(i, 1);
        }
    }

    isNightTheme() {
        const theme = document.documentElement.getAttribute('data-theme') || 'midnight-gold';
        return theme !== 'technical-paper' && theme !== 'light';
    }

    render() {
        if (!this.ctx || this.width === 0) return;
        const ctx = this.ctx;
        const isNight = this.isNightTheme();
        const room = OFFICE_ROOMS.find(r => r.id === this.activeRoomId) || OFFICE_ROOMS[0];

        ctx.save();
        ctx.scale(this.dpr, this.dpr);
        ctx.imageSmoothingEnabled = false;

        ctx.fillStyle = isNight ? '#090c12' : '#ebe5d9';
        ctx.fillRect(0, 0, this.width, this.height);

        // Resolve room backdrop (custom uploaded art takes priority)
        const customRoom = this.customRooms[room.id];
        const todKey = isNight ? 'night' : 'day';
        const roomSrc = (customRoom && (customRoom[todKey] || customRoom.night || customRoom.day))
            || (isNight ? room.nightImg : room.dayImg);
        const roomImg = this.getImage(roomSrc);

        if (roomImg && roomImg.complete && roomImg.naturalWidth > 0) {
            const scale = Math.min(this.width / roomImg.naturalWidth, this.height / roomImg.naturalHeight);
            const dw = Math.floor(roomImg.naturalWidth * scale);
            const dh = Math.floor(roomImg.naturalHeight * scale);
            const dx = Math.floor((this.width - dw) * 0.5);
            const dy = Math.floor((this.height - dh) * 0.5);
            this.roomRect = { x: dx, y: dy, w: dw, h: dh };
            ctx.drawImage(roomImg, dx, dy, dw, dh);
        } else {
            this.roomRect = { x: 0, y: 0, w: this.width, h: this.height };
        }

        // Subtle doorway / hotspot indicators
        for (const hs of room.hotspots) {
            const hx = this.roomRect.x + hs.nx * this.roomRect.w;
            const hy = this.roomRect.y + hs.ny * this.roomRect.h;
            if (hs.targetRoom) {
                ctx.fillStyle = isNight ? 'rgba(15, 23, 42, 0.78)' : 'rgba(255, 255, 255, 0.85)';
                ctx.strokeStyle = '#f59e0b';
                ctx.lineWidth = 1;
                ctx.font = 'bold 9px ui-monospace, monospace';
                const tw = ctx.measureText(`→ ${hs.label}`).width + 10;
                ctx.beginPath();
                ctx.roundRect(hx - tw * 0.5, hy - 9, tw, 16, 4);
                ctx.fill();
                ctx.stroke();
                ctx.fillStyle = isNight ? '#fde68a' : '#0f172a';
                ctx.textAlign = 'center';
                ctx.fillText(`→ ${hs.label}`, hx, hy + 2);
            }
        }

        // Render placed custom props (depth-sorted)
        const roomProps = [...(this.roomProps[this.activeRoomId] || [])].sort((a, b) => a.ny - b.ny);
        for (const pr of roomProps) {
            this.drawRoomProp(ctx, pr, isNight);
        }

        // Render workers in active room (depth-sorted by ny)
        const roomWorkers = this.workers
            .filter(w => w.room === this.activeRoomId)
            .sort((a, b) => (a.state === 'dragged' ? 999 : a.ny) - (b.state === 'dragged' ? 999 : b.ny));

        for (const w of roomWorkers) {
            this.drawSpriteWorker(ctx, w, isNight);
        }

        // Particles
        for (const p of this.particles) {
            ctx.globalAlpha = Math.max(0, p.life);
            ctx.fillStyle = p.color;
            ctx.fillRect(Math.round(p.x), Math.round(p.y), p.size, p.size);
        }
        ctx.globalAlpha = 1;

        // Overlays & Bubbles
        for (const w of roomWorkers) {
            this.drawWorkerOverlay(ctx, w, isNight);
        }

        ctx.restore();
    }

    drawRoomProp(ctx, pr, isNight) {
        const px = Math.round(this.roomRect.x + pr.nx * this.roomRect.w);
        const py = Math.round(this.roomRect.y + pr.ny * this.roomRect.h);
        ctx.fillStyle = 'rgba(0,0,0,0.28)';
        ctx.beginPath();
        ctx.ellipse(px, py, 11, 5, 0, 0, Math.PI * 2);
        ctx.fill();

        if (pr.kind === 'book_stack') {
            ctx.fillStyle = '#92400e';
            ctx.fillRect(px - 8, py - 6, 16, 5);
            ctx.fillStyle = '#1d4ed8';
            ctx.fillRect(px - 7, py - 11, 14, 5);
            ctx.fillStyle = '#dc2626';
            ctx.fillRect(px - 6, py - 15, 12, 4);
        } else if (pr.kind === 'lamp') {
            if (isNight) {
                ctx.fillStyle = 'rgba(251, 191, 36, 0.25)';
                ctx.beginPath();
                ctx.arc(px, py - 18, 18, 0, Math.PI * 2);
                ctx.fill();
            }
            ctx.fillStyle = '#ca8a04';
            ctx.fillRect(px - 2, py - 14, 4, 14);
            ctx.fillStyle = '#16a34a';
            ctx.fillRect(px - 7, py - 20, 14, 7);
        } else if (pr.kind === 'plant') {
            ctx.fillStyle = '#9a3412';
            ctx.fillRect(px - 5, py - 8, 10, 8);
            ctx.fillStyle = '#15803d';
            ctx.fillRect(px - 8, py - 18, 16, 10);
            ctx.fillStyle = '#22c55e';
            ctx.fillRect(px - 5, py - 23, 10, 6);
        } else if (pr.kind === 'globe') {
            ctx.fillStyle = '#78350f';
            ctx.fillRect(px - 4, py - 5, 8, 5);
            ctx.fillStyle = '#0284c7';
            ctx.beginPath();
            ctx.arc(px, py - 14, 8, 0, Math.PI * 2);
            ctx.fill();
            ctx.strokeStyle = '#f59e0b';
            ctx.lineWidth = 1.5;
            ctx.beginPath();
            ctx.ellipse(px, py - 14, 10, 4, -0.35, 0, Math.PI * 2);
            ctx.stroke();
        }
    }

    drawSpriteWorker(ctx, w, isNight) {
        const sx = Math.round(this.roomRect.x + w.nx * this.roomRect.w);
        const sy = Math.round(this.roomRect.y + w.ny * this.roomRect.h);
        const isSelected = w.id === this.selectedWorkerId;
        const isDragged = w.state === 'dragged';
        const lift = isDragged ? 14 : 0;
        const spriteH = Math.max(48, Math.min(82, Math.round(this.roomRect.h * 0.19)));

        // Selection ring
        if (isSelected) {
            ctx.strokeStyle = w.shirt;
            ctx.lineWidth = 2;
            ctx.beginPath();
            ctx.ellipse(sx, sy, 18, 8, 0, 0, Math.PI * 2);
            ctx.stroke();
        }

        // Shadow
        ctx.fillStyle = 'rgba(0,0,0,0.35)';
        ctx.beginPath();
        ctx.ellipse(sx, sy, isDragged ? 10 : 14, isDragged ? 4 : 6, 0, 0, Math.PI * 2);
        ctx.fill();

        // Check custom Pixel Studio sprite first
        const custom = this.customSprites[w.id];
        let spriteSrc = null;
        if (custom) {
            if (w.state === 'walking') {
                spriteSrc = (w.walkFrameIdx % 2 === 0 ? custom.walk1 : custom.walk2) || custom.idle;
            } else if (w.state === 'typing' || w.state === 'thinking') {
                spriteSrc = custom.work || custom.idle;
            } else {
                spriteSrc = custom.idle;
            }
        } else {
            let dir = w.facing || 'front-right';
            if (w.state === 'walking') {
                if (dir === 'rear-right') dir = 'front-right';
                spriteSrc = `/assets/office/sprites/characters/${w.spriteChar}-${dir}-walk${w.walkFrameIdx + 1}.webp`;
            } else {
                spriteSrc = `/assets/office/sprites/characters/${w.spriteChar}-${dir}.png`;
            }
        }

        const img = this.getImage(spriteSrc);
        if (img && img.complete && img.naturalWidth > 0) {
            const aspect = img.naturalWidth / img.naturalHeight;
            const dw = Math.round(spriteH * aspect);
            const dh = spriteH;
            const bob = w.state === 'typing'
                ? Math.sin(this.elapsed * 20) * 1.5
                : (w.state === 'idle' ? Math.sin(this.elapsed * 2.4 + w.homeNx * 10) * 1.0 : 0);
            const drawX = Math.round(sx - dw * 0.5);
            const drawY = Math.round(sy - dh - lift + bob);
            ctx.drawImage(img, drawX, drawY, dw, dh);
            w.screenBounds = { x: drawX, y: drawY, w: dw, h: dh };
        } else {
            w.screenBounds = { x: sx - 16, y: sy - spriteH, w: 32, h: spriteH };
        }
    }

    drawWorkerOverlay(ctx, w, isNight) {
        const sx = Math.round(this.roomRect.x + w.nx * this.roomRect.w);
        const sy = Math.round(this.roomRect.y + w.ny * this.roomRect.h);
        const spriteH = Math.max(48, Math.min(82, Math.round(this.roomRect.h * 0.19)));

        // Nameplate
        ctx.font = 'bold 9px ui-monospace, monospace';
        ctx.textAlign = 'center';
        ctx.fillStyle = w.id === this.selectedWorkerId ? w.shirt : (isNight ? '#cbd5e1' : '#334155');
        ctx.fillText(w.shortName.toUpperCase(), sx, sy + 13);

        if (w.state === 'alert' || w.alertTtl > 0) {
            ctx.fillStyle = '#f59e0b';
            ctx.fillRect(sx - 7, sy - spriteH - 18, 14, 14);
            ctx.fillStyle = '#0f172a';
            ctx.font = 'bold 10px monospace';
            ctx.fillText('!', sx, sy - spriteH - 8);
            return;
        }

        let bubbleText = w.bubble?.text || null;
        let bubbleKind = w.bubble?.kind || 'info';
        if (w.state === 'thinking' && this.activeTask?.startTime) {
            const secs = ((performance.now() - this.activeTask.startTime) / 1000).toFixed(1);
            bubbleText = `Thinking ${secs}s...`;
            bubbleKind = 'think';
        }
        if (!bubbleText) return;

        ctx.font = '600 9px ui-monospace, monospace';
        const tw = ctx.measureText(bubbleText).width;
        const bw = tw + 12;
        const bh = 16;
        const bx = Math.round(sx - bw * 0.5);
        const by = Math.round(sy - spriteH - 22);

        ctx.fillStyle = isNight ? 'rgba(15, 23, 42, 0.92)' : 'rgba(255, 255, 255, 0.95)';
        ctx.strokeStyle = bubbleKind === 'success' ? '#22c55e' : (bubbleKind === 'accent' ? w.shirt : '#94a3b8');
        ctx.lineWidth = 1.2;
        ctx.beginPath();
        ctx.roundRect(bx, by, bw, bh, 4);
        ctx.fill();
        ctx.stroke();

        ctx.fillStyle = isNight ? '#f8fafc' : '#0f172a';
        ctx.fillText(bubbleText, sx, by + 11);
    }
}

export const librarianOffice = new LibrarianOffice();
