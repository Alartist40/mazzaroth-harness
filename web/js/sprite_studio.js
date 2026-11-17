// MAZZAROTH PIXEL & ENVIRONMENT STUDIO
// Godot/Aseprite-inspired 16x24 pixel character editor + custom room backdrop uploader.

const CUSTOM_SPRITES_KEY = 'mazzaroth_custom_sprites_v1';
const CUSTOM_ROOMS_KEY = 'mazzaroth_custom_rooms_v1';

const PALETTE = [
    '#0f172a', '#1e293b', '#475569', '#94a3b8', '#f8fafc',
    '#78350f', '#b45309', '#f59e0b', '#fde047',
    '#dc2626', '#ec4899', '#8b5cf6', '#3b82f6', '#06b6d4', '#10b981',
    '#ffdbac', '#f1c27d', '#c68642', '#8d5524'
];

export function loadCustomSprites() {
    try {
        return JSON.parse(localStorage.getItem(CUSTOM_SPRITES_KEY) || '{}');
    } catch {
        return {};
    }
}

export function loadCustomRooms() {
    try {
        return JSON.parse(localStorage.getItem(CUSTOM_ROOMS_KEY) || '{}');
    } catch {
        return {};
    }
}

export class PixelStudioModal {
    constructor(workers, rooms, onApply) {
        this.workers = workers;
        this.rooms = rooms;
        this.onApply = onApply;

        this.gridW = 16;
        this.gridH = 24;
        this.frames = {
            idle: this.createBlankGrid(),
            walk1: this.createBlankGrid(),
            walk2: this.createBlankGrid(),
            work: this.createBlankGrid()
        };
        this.activeFrame = 'idle';
        this.activeTool = 'pencil';
        this.activeColor = '#3b82f6';
        this.mirrorX = false;
        this.isDrawing = false;
        this.previewTick = 0;
        this.previewTimer = null;

        this.seedDefaultTemplate();
    }

    createBlankGrid() {
        return Array.from({ length: this.gridH }, () => Array(this.gridW).fill(null));
    }

    seedDefaultTemplate() {
        // Seed a clean starter silhouette in `idle` so the artist isn't staring at a blank void
        const g = this.frames.idle;
        const fillRect = (x, y, w, h, col) => {
            for (let yy = y; yy < y + h; yy++) {
                for (let xx = x; xx < x + w; xx++) {
                    if (yy >= 0 && yy < this.gridH && xx >= 0 && xx < this.gridW) g[yy][xx] = col;
                }
            }
        };
        fillRect(5, 3, 6, 2, '#78350f');   // hair
        fillRect(5, 5, 6, 5, '#f1c27d');   // head
        fillRect(6, 7, 1, 1, '#0f172a');   // eye L
        fillRect(9, 7, 1, 1, '#0f172a');   // eye R
        fillRect(4, 10, 8, 7, '#3b82f6');  // coat
        fillRect(7, 10, 2, 2, '#f8fafc');  // collar
        fillRect(5, 17, 2, 4, '#1e293b');  // leg L
        fillRect(9, 17, 2, 4, '#1e293b');  // leg R
        fillRect(5, 21, 2, 1, '#0f172a');  // boot L
        fillRect(9, 21, 2, 1, '#0f172a');  // boot R
    }

    bind() {
        const modal = document.getElementById('pixel-studio-modal');
        const openBtn = document.getElementById('btn-open-studio');
        const closeBtn = document.getElementById('btn-close-studio');
        if (!modal || !openBtn) return;

        openBtn.addEventListener('click', () => {
            modal.classList.remove('hidden');
            modal.classList.add('flex');
            this.renderGrid();
            this.startPreviewLoop();
        });

        if (closeBtn) {
            closeBtn.addEventListener('click', () => {
                modal.classList.add('hidden');
                modal.classList.remove('flex');
                this.stopPreviewLoop();
            });
        }

        // Populate target worker & room selects
        const workerSel = document.getElementById('studio-worker-select');
        if (workerSel) {
            workerSel.innerHTML = this.workers
                .map(w => `<option value="${w.id}">${w.name} (${w.role})</option>`)
                .join('');
        }
        const roomSel = document.getElementById('studio-room-select');
        if (roomSel) {
            roomSel.innerHTML = this.rooms
                .map(r => `<option value="${r.id}">${r.name}</option>`)
                .join('');
        }

        // Palette swatches
        const palEl = document.getElementById('studio-palette');
        const customColor = document.getElementById('studio-custom-color');
        if (palEl) {
            palEl.innerHTML = '';
            PALETTE.forEach(col => {
                const sw = document.createElement('button');
                sw.className = 'w-5 h-5 rounded border transition-transform hover:scale-110';
                sw.style.backgroundColor = col;
                sw.style.borderColor = col === this.activeColor ? '#ffffff' : 'rgba(148,163,184,0.35)';
                sw.addEventListener('click', () => {
                    this.activeColor = col;
                    if (customColor) customColor.value = col;
                    this.updatePaletteHighlight();
                });
                palEl.appendChild(sw);
            });
        }
        if (customColor) {
            customColor.addEventListener('input', (e) => {
                this.activeColor = e.target.value;
                this.updatePaletteHighlight();
            });
        }

        // Tools
        document.querySelectorAll('.studio-tool-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.activeTool = btn.getAttribute('data-tool');
                document.querySelectorAll('.studio-tool-btn').forEach(b => {
                    b.style.borderColor = b === btn ? 'var(--accent-primary)' : 'var(--border-subtle)';
                });
            });
        });

        const mirrorBtn = document.getElementById('studio-mirror-btn');
        if (mirrorBtn) {
            mirrorBtn.addEventListener('click', () => {
                this.mirrorX = !this.mirrorX;
                mirrorBtn.style.borderColor = this.mirrorX ? 'var(--accent-primary)' : 'var(--border-subtle)';
                mirrorBtn.innerText = this.mirrorX ? 'MIRROR X: ON' : 'MIRROR X: OFF';
            });
        }

        // Frame tabs
        document.querySelectorAll('.studio-frame-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.activeFrame = btn.getAttribute('data-frame');
                document.querySelectorAll('.studio-frame-btn').forEach(b => {
                    b.style.borderColor = b === btn ? 'var(--accent-primary)' : 'var(--border-subtle)';
                });
                this.renderGrid();
            });
        });

        const copyIdleBtn = document.getElementById('studio-copy-idle');
        if (copyIdleBtn) {
            copyIdleBtn.addEventListener('click', () => {
                if (this.activeFrame !== 'idle') {
                    this.frames[this.activeFrame] = this.frames.idle.map(row => [...row]);
                    this.renderGrid();
                }
            });
        }

        const clearBtn = document.getElementById('studio-clear-frame');
        if (clearBtn) {
            clearBtn.addEventListener('click', () => {
                this.frames[this.activeFrame] = this.createBlankGrid();
                this.renderGrid();
            });
        }

        // Drawing canvas events
        const canvas = document.getElementById('studio-grid-canvas');
        if (canvas) {
            canvas.addEventListener('pointerdown', (e) => {
                this.isDrawing = true;
                this.applyCanvasTool(e, canvas);
            });
            window.addEventListener('pointermove', (e) => {
                if (this.isDrawing) this.applyCanvasTool(e, canvas);
            });
            window.addEventListener('pointerup', () => {
                this.isDrawing = false;
            });
        }

        // Apply pixel art to selected worker
        const applyBtn = document.getElementById('studio-apply-worker');
        if (applyBtn) {
            applyBtn.addEventListener('click', () => {
                const workerId = workerSel?.value || this.workers[0].id;
                const sprites = loadCustomSprites();
                sprites[workerId] = {
                    idle: this.gridToDataUrl(this.frames.idle),
                    walk1: this.hasPixels(this.frames.walk1) ? this.gridToDataUrl(this.frames.walk1) : null,
                    walk2: this.hasPixels(this.frames.walk2) ? this.gridToDataUrl(this.frames.walk2) : null,
                    work: this.hasPixels(this.frames.work) ? this.gridToDataUrl(this.frames.work) : null
                };
                localStorage.setItem(CUSTOM_SPRITES_KEY, JSON.stringify(sprites));
                if (this.onApply) this.onApply('sprite', workerId);
                applyBtn.innerText = 'SAVED!';
                setTimeout(() => { applyBtn.innerText = 'APPLY TO WORKER'; }, 1200);
            });
        }

        // Import PNG sprite directly for selected worker
        const importSpriteInput = document.getElementById('studio-import-sprite');
        if (importSpriteInput) {
            importSpriteInput.addEventListener('change', (e) => {
                const file = e.target.files?.[0];
                if (!file) return;
                const reader = new FileReader();
                reader.onload = () => {
                    const workerId = workerSel?.value || this.workers[0].id;
                    const sprites = loadCustomSprites();
                    sprites[workerId] = { idle: reader.result };
                    localStorage.setItem(CUSTOM_SPRITES_KEY, JSON.stringify(sprites));
                    if (this.onApply) this.onApply('sprite', workerId);
                };
                reader.readAsDataURL(file);
            });
        }

        // Reset worker sprite
        const resetWorkerBtn = document.getElementById('studio-reset-worker');
        if (resetWorkerBtn) {
            resetWorkerBtn.addEventListener('click', () => {
                const workerId = workerSel?.value || this.workers[0].id;
                const sprites = loadCustomSprites();
                delete sprites[workerId];
                localStorage.setItem(CUSTOM_SPRITES_KEY, JSON.stringify(sprites));
                if (this.onApply) this.onApply('sprite', workerId);
            });
        }

        // Export PNG
        const exportBtn = document.getElementById('studio-export-png');
        if (exportBtn) {
            exportBtn.addEventListener('click', () => {
                const a = document.createElement('a');
                a.href = this.gridToDataUrl(this.frames[this.activeFrame], 4);
                a.download = `mazzaroth-${workerSel?.value || 'worker'}-${this.activeFrame}.png`;
                a.click();
            });
        }

        // Custom Room Backdrop Upload
        const roomUpload = document.getElementById('studio-room-upload');
        if (roomUpload) {
            roomUpload.addEventListener('change', (e) => {
                const file = e.target.files?.[0];
                if (!file) return;
                const roomId = roomSel?.value || 'office';
                const tod = document.getElementById('studio-room-tod')?.value || 'night';
                const reader = new FileReader();
                reader.onload = () => {
                    const rooms = loadCustomRooms();
                    if (!rooms[roomId]) rooms[roomId] = {};
                    rooms[roomId][tod] = reader.result;
                    localStorage.setItem(CUSTOM_ROOMS_KEY, JSON.stringify(rooms));
                    if (this.onApply) this.onApply('room', roomId);
                };
                reader.readAsDataURL(file);
            });
        }

        const resetRoomBtn = document.getElementById('studio-reset-room');
        if (resetRoomBtn) {
            resetRoomBtn.addEventListener('click', () => {
                const roomId = roomSel?.value || 'office';
                const rooms = loadCustomRooms();
                delete rooms[roomId];
                localStorage.setItem(CUSTOM_ROOMS_KEY, JSON.stringify(rooms));
                if (this.onApply) this.onApply('room', roomId);
            });
        }

        const exportPackBtn = document.getElementById('studio-export-pack');
        if (exportPackBtn) {
            exportPackBtn.addEventListener('click', () => {
                const pack = {
                    version: 1,
                    sprites: loadCustomSprites(),
                    rooms: loadCustomRooms()
                };
                const blob = new Blob([JSON.stringify(pack, null, 2)], { type: 'application/json' });
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'mazzaroth-office-art-pack.json';
                a.click();
                URL.revokeObjectURL(url);
            });
        }

        const importPackInput = document.getElementById('studio-import-pack');
        if (importPackInput) {
            importPackInput.addEventListener('change', (e) => {
                const file = e.target.files?.[0];
                if (!file) return;
                const reader = new FileReader();
                reader.onload = () => {
                    try {
                        const pack = JSON.parse(reader.result);
                        if (pack.sprites) localStorage.setItem(CUSTOM_SPRITES_KEY, JSON.stringify(pack.sprites));
                        if (pack.rooms) localStorage.setItem(CUSTOM_ROOMS_KEY, JSON.stringify(pack.rooms));
                        if (this.onApply) this.onApply('pack', 'all');
                    } catch (err) {
                        console.warn('Invalid art pack JSON', err);
                    }
                };
                reader.readAsText(file);
            });
        }
    }

    updatePaletteHighlight() {
        const palEl = document.getElementById('studio-palette');
        if (!palEl) return;
        Array.from(palEl.children).forEach(sw => {
            sw.style.borderColor = sw.style.backgroundColor === this.activeColor ? '#ffffff' : 'rgba(148,163,184,0.35)';
        });
    }

    hasPixels(grid) {
        return grid.some(row => row.some(c => c !== null));
    }

    applyCanvasTool(e, canvas) {
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor(((e.clientX - rect.left) / rect.width) * this.gridW);
        const y = Math.floor(((e.clientY - rect.top) / rect.height) * this.gridH);
        if (x < 0 || x >= this.gridW || y < 0 || y >= this.gridH) return;

        const grid = this.frames[this.activeFrame];
        if (this.activeTool === 'eyedropper') {
            if (grid[y][x]) {
                this.activeColor = grid[y][x];
                this.updatePaletteHighlight();
            }
            this.isDrawing = false;
            return;
        }
        if (this.activeTool === 'fill') {
            const target = grid[y][x];
            if (target !== this.activeColor) this.floodFill(grid, x, y, target, this.activeColor);
            this.isDrawing = false;
            this.renderGrid();
            return;
        }
        const val = this.activeTool === 'eraser' ? null : this.activeColor;
        grid[y][x] = val;
        if (this.mirrorX) {
            const mx = this.gridW - 1 - x;
            grid[y][mx] = val;
        }
        this.renderGrid();
    }

    floodFill(grid, sx, sy, target, replacement) {
        const stack = [[sx, sy]];
        while (stack.length) {
            const [cx, cy] = stack.pop();
            if (cx < 0 || cx >= this.gridW || cy < 0 || cy >= this.gridH) continue;
            if (grid[cy][cx] !== target) continue;
            grid[cy][cx] = replacement;
            stack.push([cx + 1, cy], [cx - 1, cy], [cx, cy + 1], [cx, cy - 1]);
        }
    }

    renderGrid() {
        const canvas = document.getElementById('studio-grid-canvas');
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        const cellW = canvas.width / this.gridW;
        const cellH = canvas.height / this.gridH;
        const grid = this.frames[this.activeFrame];

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (let y = 0; y < this.gridH; y++) {
            for (let x = 0; x < this.gridW; x++) {
                ctx.fillStyle = (x + y) % 2 === 0 ? '#1e293b' : '#0f172a';
                ctx.fillRect(x * cellW, y * cellH, cellW, cellH);
                if (grid[y][x]) {
                    ctx.fillStyle = grid[y][x];
                    ctx.fillRect(x * cellW, y * cellH, cellW, cellH);
                }
                ctx.strokeStyle = 'rgba(148, 163, 184, 0.14)';
                ctx.strokeRect(x * cellW, y * cellH, cellW, cellH);
            }
        }
    }

    gridToDataUrl(grid, scale = 3) {
        const c = document.createElement('canvas');
        c.width = this.gridW * scale;
        c.height = this.gridH * scale;
        const ctx = c.getContext('2d');
        ctx.imageSmoothingEnabled = false;
        for (let y = 0; y < this.gridH; y++) {
            for (let x = 0; x < this.gridW; x++) {
                if (grid[y][x]) {
                    ctx.fillStyle = grid[y][x];
                    ctx.fillRect(x * scale, y * scale, scale, scale);
                }
            }
        }
        return c.toDataURL('image/png');
    }

    startPreviewLoop() {
        this.stopPreviewLoop();
        this.previewTimer = setInterval(() => {
            const pCanvas = document.getElementById('studio-preview-canvas');
            if (!pCanvas) return;
            const ctx = pCanvas.getContext('2d');
            ctx.imageSmoothingEnabled = false;
            ctx.clearRect(0, 0, pCanvas.width, pCanvas.height);

            const seq = ['idle', 'walk1', 'idle', 'walk2'];
            const key = seq[this.previewTick % seq.length];
            this.previewTick++;
            const grid = this.hasPixels(this.frames[key]) ? this.frames[key] : this.frames.idle;
            const scale = Math.floor(Math.min(pCanvas.width / this.gridW, pCanvas.height / this.gridH));
            const ox = Math.floor((pCanvas.width - this.gridW * scale) * 0.5);
            const oy = Math.floor((pCanvas.height - this.gridH * scale) * 0.5);

            for (let y = 0; y < this.gridH; y++) {
                for (let x = 0; x < this.gridW; x++) {
                    if (grid[y][x]) {
                        ctx.fillStyle = grid[y][x];
                        ctx.fillRect(ox + x * scale, oy + y * scale, scale, scale);
                    }
                }
            }
        }, 240);
    }

    stopPreviewLoop() {
        if (this.previewTimer) {
            clearInterval(this.previewTimer);
            this.previewTimer = null;
        }
    }
}
