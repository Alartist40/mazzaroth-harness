function cleanLabel(str) {
  if (!str) return "";
  return String(str).replace(/[\u{1F300}-\u{1FAFF}\u{2600}-\u{27BF}\u{FE00}-\u{FE0F}\u{2700}-\u{27BF}\u{1F000}-\u{1FFFF}\u{2190}-\u{21FF}🌌✨✦🪐🔍♈🌟›]/gu, "").trim();
}
let scene, camera, renderer, controls;
    let galaxyGroup;
    let starPoints, spiralDustPoints, centralCoreMesh, centralCoronaMesh, constellationLinesMesh;
    let galaxyData = { bodies: [], lines: [] };
    let currentFocusNode = null;
    let currentClusterNode = null;
    let lastRenderedCount = -1;
    let lastRenderedLinesCount = -1;
    let lastRenderedTimestamp = 0;
    let tempVec3;
    let tempTargetVec3;

    let currentSection = 'galaxy';
    const sections = {
      galaxy: {
        id: 'galaxy',
        label: 'Celestial Galaxy',
        api: '/api/memory/celestial',
        mount: () => {
          if (spiralDustPoints) spiralDustPoints.visible = true;
          if (centralCoreMesh) centralCoreMesh.visible = true;
          if (centralCoronaMesh) centralCoronaMesh.visible = true;
          document.getElementById('cluster-header-text').textContent = 'KNOWLEDGE CLUSTERS & SECTORS';
        },
        unmount: () => {
          if (spiralDustPoints) spiralDustPoints.visible = false;
          if (centralCoreMesh) centralCoreMesh.visible = false;
          if (centralCoronaMesh) centralCoronaMesh.visible = false;
        }
      },
      constellations: {
        id: 'constellations',
        label: 'Classical Constellations',
        api: '/api/sections/constellations',
        mount: () => {
          if (spiralDustPoints) spiralDustPoints.visible = false;
          if (centralCoreMesh) centralCoreMesh.visible = false;
          if (centralCoronaMesh) centralCoronaMesh.visible = false;
          document.getElementById('cluster-header-text').textContent = 'CONSTELLATION CATALOG (32)';
        },
        unmount: () => {}
      }
    };

    const container = document.getElementById('canvas-container');

    function init() {
      tempVec3 = new THREE.Vector3();
      tempTargetVec3 = new THREE.Vector3();

      scene = new THREE.Scene();
      scene.fog = new THREE.FogExp2(0x050614, 0.00018);

      camera = new THREE.PerspectiveCamera(46, window.innerWidth / window.innerHeight, 1, 25000);
      camera.position.set(0, 500, 1100);

      renderer = new THREE.WebGLRenderer({ antialias: true, powerPreference: "high-performance" });
      renderer.setSize(window.innerWidth, window.innerHeight);
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      renderer.toneMapping = THREE.ACESFilmicToneMapping;
      renderer.toneMappingExposure = 1.4;
      container.appendChild(renderer.domElement);

      controls = new THREE.OrbitControls(camera, renderer.domElement);
      controls.enableDamping = true;
      controls.dampingFactor = 0.05;
      controls.minDistance = 20;
      controls.maxDistance = 6000;
      controls.autoRotate = true;
      controls.autoRotateSpeed = 0.25;

      // Deep space ambient and core light
      scene.add(new THREE.AmbientLight(0x1e1b4b, 2.2));
      const coreLight = new THREE.PointLight(0xffcc00, 6.0, 5000);
      coreLight.position.set(0, 0, 0);
      scene.add(coreLight);

      // Rotating Galaxy Group
      galaxyGroup = new THREE.Group();
      scene.add(galaxyGroup);

      // 1. Massive Luminous Galactic Gravitational Center Core (Prominent Anchor)
      buildSupermassiveCore();

      // 2. Swirling 3D Rounded Spiral Arms connecting tightly into the Center Core
      build3DRoundedSpiralGalaxy();

      // Mouse Move & Smart Hover Detection
      window.addEventListener('mousemove', onMouseMove);

      // Smart Nearest-Node Click Lock
      window.addEventListener('click', onGalaxyClick);

      fetchGalaxy();
      setInterval(fetchGalaxy, 10000);

      window.addEventListener('resize', onWindowResize);
      animate(0);
    }

    // Prominent Supermassive Gravitational Core (Anchoring all spiral arms)
    function buildSupermassiveCore() {
      // 1. Solid radiant white/gold central sphere
      const sphereGeo = new THREE.SphereGeometry(14, 32, 32);
      const sphereMat = new THREE.MeshBasicMaterial({
        color: 0xffffff,
      });
      centralCoreMesh = new THREE.Mesh(sphereGeo, sphereMat);
      galaxyGroup.add(centralCoreMesh);

      // 2. Massive glowing sunburst corona billboard
      const coronaCanvas = document.createElement('canvas');
      coronaCanvas.width = 256;
      coronaCanvas.height = 256;
      const ctx = coronaCanvas.getContext('2d');

      const grad = ctx.createRadialGradient(128, 128, 8, 128, 128, 124);
      grad.addColorStop(0, 'rgba(255, 255, 255, 1)');
      grad.addColorStop(0.2, 'rgba(255, 204, 0, 0.95)');
      grad.addColorStop(0.5, 'rgba(251, 140, 0, 0.7)');
      grad.addColorStop(0.8, 'rgba(216, 27, 96, 0.3)');
      grad.addColorStop(1, 'rgba(46, 12, 89, 0)');
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, 256, 256);

      const coronaTex = new THREE.CanvasTexture(coronaCanvas);
      const coronaGeo = new THREE.PlaneGeometry(120, 120);
      const coronaMat = new THREE.MeshBasicMaterial({
        map: coronaTex,
        transparent: true,
        opacity: 0.95,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
        side: THREE.DoubleSide
      });
      centralCoronaMesh = new THREE.Mesh(coronaGeo, coronaMat);
      galaxyGroup.add(centralCoronaMesh);
    }

    // 3D Rounded Spiral Galaxy (Seamlessly flowing into the center core at r=0)
    function build3DRoundedSpiralGalaxy() {
      const count = 36000;
      const geo = new THREE.BufferGeometry();
      const pos = new Float32Array(count * 3);
      const col = new Float32Array(count * 3);

      const numArms = 4;
      const cCore = new THREE.Color('#ffffff');
      const cYellow = new THREE.Color('#ffcc00');
      const cOrange = new THREE.Color('#fb8c00');
      const cMagenta = new THREE.Color('#d81b60');
      const cPurple = new THREE.Color('#7b1fa2');
      const cDeepPurple = new THREE.Color('#2e0c59');

      for (let i = 0; i < count; i++) {
        const i3 = i * 3;
        const arm = i % numArms;
        const armOffset = (arm / numArms) * Math.PI * 2;

        // Start t right at 0 so spiral connects seamlessly to the centre core!
        const t = Math.pow(Math.random(), 1.25) * 16.0;
        const r = t * 62.0; // r starts right at 0
        const angle = t * 0.82 + armOffset;

        // Natural organic dispersion along spiral
        const spread = (6.0 + t * 4.0) * (Math.random() < 0.5 ? 1 : -1) * Math.pow(Math.random(), 1.8);
        const ySpread = (Math.random() - 0.5) * (8.0 + t * 2.2);

        const x = Math.cos(angle) * r + spread;
        const y = ySpread + Math.sin(r * 0.01) * 14.0;
        const z = Math.sin(angle) * r + spread;

        pos[i3] = x;
        pos[i3 + 1] = y;
        pos[i3 + 2] = z;

        // Color mapping: Core (White/Yellow) -> Orange -> Magenta -> Purple -> Deep Purple
        let c;
        const normT = t / 16.0;
        if (normT < 0.1) {
          c = cCore.clone().lerp(cYellow, normT / 0.1);
        } else if (normT < 0.28) {
          c = cYellow.clone().lerp(cOrange, (normT - 0.1) / 0.18);
        } else if (normT < 0.55) {
          c = cOrange.clone().lerp(cMagenta, (normT - 0.28) / 0.27);
        } else if (normT < 0.82) {
          c = cMagenta.clone().lerp(cPurple, (normT - 0.55) / 0.27);
        } else {
          c = cPurple.clone().lerp(cDeepPurple, (normT - 0.82) / 0.18);
        }

        col[i3] = c.r;
        col[i3 + 1] = c.g;
        col[i3 + 2] = c.b;
      }

      geo.setAttribute('position', new THREE.BufferAttribute(pos, 3));
      geo.setAttribute('color', new THREE.BufferAttribute(col, 3));

      // Ambient dust is subtle and fine (size: 2.2) so data nodes pop out clearly
      const mat = new THREE.PointsMaterial({
        size: 2.2,
        vertexColors: true,
        transparent: true,
        opacity: 0.65,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      });

      spiralDustPoints = new THREE.Points(geo, mat);
      galaxyGroup.add(spiralDustPoints);
    }

    async function fetchGalaxy() {
      try {
        const sec = sections[currentSection] || sections.galaxy;
        const res = await fetch(sec.api);
        if (!res.ok) return;
        galaxyData = await res.json();
        const linesCount = galaxyData.lines ? galaxyData.lines.length : 0;
        const bodiesChanged = galaxyData.bodies.length !== lastRenderedCount;
        const linesChanged = linesCount !== lastRenderedLinesCount;

        if (bodiesChanged || linesChanged) {
          lastRenderedCount = galaxyData.bodies.length;
          lastRenderedLinesCount = linesCount;
          lastRenderedTimestamp = galaxyData.timestamp || 0;

          // Precompute O(1) lookups for high-performance rendering & HUD
          cachedBodyMap = new Map();
          cachedAdjacencyMap = new Map();
          galaxyData.bodies.forEach(b => {
            cachedBodyMap.set(b.id, b);
            cachedAdjacencyMap.set(b.id, []);
          });
          if (galaxyData.lines) {
            galaxyData.lines.forEach(l => {
              const s = cachedBodyMap.get(l.source_id);
              const t = cachedBodyMap.get(l.target_id);
              if (s && t) {
                cachedAdjacencyMap.get(l.source_id).push(t);
                cachedAdjacencyMap.get(l.target_id).push(s);
              }
            });
          }

          renderDatabaseStars(galaxyData);
          populateClusterSidebar(galaxyData);
        } else if (galaxyData.timestamp && galaxyData.timestamp !== lastRenderedTimestamp) {
          lastRenderedTimestamp = galaxyData.timestamp;
          updateStarLuminosities(galaxyData);
        }
      } catch (e) {}
    }

    async function switchSection(sectionId) {
      if (sections[currentSection] && sections[currentSection].unmount) {
        sections[currentSection].unmount();
      }
      currentSection = sectionId;
      if (sections[currentSection] && sections[currentSection].mount) {
        sections[currentSection].mount();
      }

      document.querySelectorAll('.top-btn').forEach(btn => {
        if (btn.id === 'tab-' + sectionId) {
          btn.classList.add('active');
        } else if (btn.id && btn.id.startsWith('tab-')) {
          btn.classList.remove('active');
        }
      });

      lastRenderedCount = -1;
      lastRenderedLinesCount = -1;
      await fetchGalaxy();
      resetToGalacticView();
    }

    function getStarColor(b) {
      const isCore = b.id.includes(':core:');
      const isLang = b.id.includes(':lang:');
      const isVersion = b.id.includes(':version:');

      let baseCol;
      if (isCore) baseCol = { r: 1.0, g: 1.0, b: 0.95 };
      else if (isLang) baseCol = { r: 1.0, g: 0.88, b: 0.15 };
      else if (isVersion) baseCol = { r: 0.0, g: 0.95, b: 1.0 };
      else baseCol = { r: 0.95, g: 0.35, b: 0.95 };

      // Radial brightness falloff and recency luminosity modulation
      const r = Math.sqrt(b.x * b.x + b.z * b.z);
      const falloff = Math.max(0.0, 1.0 - r / 900.0);
      const radiusFade = 0.35 + 0.65 * falloff;
      const lum = (b.luminosity !== undefined ? b.luminosity : 1.0) * radiusFade;

      return {
        r: Math.min(1.0, baseCol.r * lum),
        g: Math.min(1.0, baseCol.g * lum),
        b: Math.min(1.0, baseCol.b * lum)
      };
    }

    function updateStarLuminosities(data) {
      if (!starPoints || !starPoints.geometry) return;
      const colAttr = starPoints.geometry.getAttribute('color');
      if (!colAttr) return;
      const count = data.bodies.length;
      for (let i = 0; i < count; i++) {
        const c = getStarColor(data.bodies[i]);
        colAttr.setXYZ(i, c.r, c.g, c.b);
      }
      colAttr.needsUpdate = true;
    }

    // Renders Database Memory Stars as Crisp, Unmistakable 4-Point Luminous Diamond Stars
    function renderDatabaseStars(data) {
      document.getElementById('db-node-count-hud').textContent = `DATABASE: ${data.bodies.length} NODES`;

      if (starPoints) galaxyGroup.remove(starPoints);
      if (constellationLinesMesh) {
        galaxyGroup.remove(constellationLinesMesh);
        constellationLinesMesh = null;
      }

      const count = data.bodies.length;
      const geo = new THREE.BufferGeometry();
      const pos = new Float32Array(count * 3);
      const col = new Float32Array(count * 3);

      for (let i = 0; i < count; i++) {
        const b = data.bodies[i];
        pos[i * 3] = b.x;
        pos[i * 3 + 1] = b.y;
        pos[i * 3 + 2] = b.z;

        const c = getStarColor(b);
        col[i * 3] = c.r;
        col[i * 3 + 1] = c.g;
        col[i * 3 + 2] = c.b;
      }

      geo.setAttribute('position', new THREE.BufferAttribute(pos, 3));
      geo.setAttribute('color', new THREE.BufferAttribute(col, 3));

      // Create crisp 4-point diamond starburst texture
      const starCanvas = document.createElement('canvas');
      starCanvas.width = 64;
      starCanvas.height = 64;
      const ctx = starCanvas.getContext('2d');

      const grad = ctx.createRadialGradient(32, 32, 0, 32, 32, 30);
      grad.addColorStop(0, 'rgba(255, 255, 255, 1)');
      grad.addColorStop(0.2, 'rgba(255, 238, 88, 0.95)');
      grad.addColorStop(0.5, 'rgba(0, 255, 255, 0.4)');
      grad.addColorStop(1, 'rgba(0, 0, 0, 0)');
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, 64, 64);

      ctx.fillStyle = 'rgba(255, 255, 255, 1)';
      ctx.beginPath();
      ctx.moveTo(0, 32); ctx.lineTo(32, 30); ctx.lineTo(64, 32); ctx.lineTo(32, 34); ctx.closePath();
      ctx.fill();

      ctx.beginPath();
      ctx.moveTo(32, 0); ctx.lineTo(30, 32); ctx.lineTo(32, 64); ctx.lineTo(34, 32); ctx.closePath();
      ctx.fill();

      const starTex = new THREE.CanvasTexture(starCanvas);

      // Stars are prominent (size: 9.0) so data nodes are unmistakable!
      const mat = new THREE.PointsMaterial({
        size: 9.0,
        map: starTex,
        vertexColors: true,
        transparent: true,
        opacity: 0.98,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      });

      starPoints = new THREE.Points(geo, mat);
      galaxyGroup.add(starPoints);

      // IN CONSTELLATIONS MODE: RENDER ALL CONSTELLATION LINES PERMANENTLY!
      if (currentSection === 'constellations' && data.lines && data.lines.length > 0) {
        const bodyMap = new Map();
        data.bodies.forEach(b => bodyMap.set(b.id, b));

        const lpos = [];
        const lcol = [];

        data.lines.forEach(l => {
          const s = bodyMap.get(l.source_id);
          const t = bodyMap.get(l.target_id);
          if (s && t) {
            lpos.push(s.x, s.y, s.z);
            lpos.push(t.x, t.y, t.z);

            lcol.push(0.0, 1.0, 1.0);
            lcol.push(0.0, 1.0, 1.0);
          }
        });

        if (lpos.length > 0) {
          const lgeo = new THREE.BufferGeometry();
          lgeo.setAttribute('position', new THREE.Float32BufferAttribute(lpos, 3));
          lgeo.setAttribute('color', new THREE.Float32BufferAttribute(lcol, 3));
          const lmat = new THREE.LineBasicMaterial({
            vertexColors: true,
            transparent: true,
            opacity: 0.85,
            blending: THREE.AdditiveBlending,
            depthWrite: false,
          });
          constellationLinesMesh = new THREE.LineSegments(lgeo, lmat);
          galaxyGroup.add(constellationLinesMesh);
        }
      }
    }

    const drilldownState = {
      step: 1,
      lang: "eng",
      langName: "English",
      version: "kjv",
      book: "Genesis",
      chapter: 1,
      totalChapters: 50,
      filterQuery: "",
      booksMeta: []
    };

    function populateClusterSidebar(data) {
      renderDrilldown();
    }

    function renderConstellationsList() {
      const list = document.getElementById("cluster-list");
      if (!list || !galaxyData) return;
      list.innerHTML = "";
      const q = (drilldownState.filterQuery || "").toLowerCase().trim();

      const zodiacSigns = ["aries", "taurus", "gemini", "cancer", "leo", "virgo", "libra", "scorpius", "sagittarius", "capricornus", "aquarius", "pisces"];
      const zodiacBodies = [];
      const asterismBodies = [];

      galaxyData.bodies.forEach(b => {
        const matchZodiac = zodiacSigns.some(z => b.id.toLowerCase().includes(z));
        if (matchZodiac) {
          zodiacBodies.push(b);
        } else {
          asterismBodies.push(b);
        }
      });

      const zFiltered = q ? zodiacBodies.filter(b => b.label.toLowerCase().includes(q)) : zodiacBodies;
      const aFiltered = q ? asterismBodies.filter(b => b.label.toLowerCase().includes(q)) : asterismBodies;

      if (zFiltered.length > 0) {
        const zTitle = document.createElement("div");
        zTitle.className = "cluster-category-title";
        zTitle.textContent = `12 Zodiac Constellations (${zFiltered.length} Stars)`;
        list.appendChild(zTitle);

        zFiltered.forEach(c => {
          const item = document.createElement("div");
          item.className = "cluster-item";
          item.innerHTML = `<span>${cleanLabel(c.label)}</span><span class="cluster-action">LOCK</span>`;
          item.onclick = () => onSelectStar(c);
          list.appendChild(item);
        });
      }

      if (aFiltered.length > 0) {
        const aTitle = document.createElement("div");
        aTitle.className = "cluster-category-title";
        aTitle.style.marginTop = "10px";
        aTitle.textContent = `Major Asterisms (${aFiltered.length} Stars)`;
        list.appendChild(aTitle);

        aFiltered.forEach(c => {
          const item = document.createElement("div");
          item.className = "cluster-item";
          item.innerHTML = `<span>${cleanLabel(c.label)}</span><span class="cluster-action">LOCK</span>`;
          item.onclick = () => onSelectStar(c);
          list.appendChild(item);
        });
      }
    }

    async function renderDrilldown() {
      const list = document.getElementById("cluster-list");
      const badge = document.getElementById("drilldown-step-label");
      const backBtn = document.getElementById("drilldown-back-btn");
      if (!list || !badge) return;

      if (currentSection === "constellations") {
        badge.textContent = "CONSTELLATIONS & ASTERISMS";
        if (backBtn) backBtn.classList.add("is-hidden");
        return renderConstellationsList();
      }

      list.innerHTML = "";
      const q = (drilldownState.filterQuery || "").toLowerCase().trim();

      if (drilldownState.step === 1) {
        badge.textContent = "STEP 1: SELECT LANGUAGE";
        if (backBtn) backBtn.classList.add("is-hidden");

        const langBodies = (galaxyData ? galaxyData.bodies : []).filter(b => b.id.includes(":lang:"));
        let filtered = langBodies;
        if (q) {
          filtered = langBodies.filter(b => b.label.toLowerCase().includes(q) || b.id.toLowerCase().includes(q));
        }

        filtered.forEach(b => {
          const item = document.createElement("div");
          item.className = "cluster-item";
          const langCode = b.id.split(":").pop();
          item.innerHTML = `<span>${cleanLabel(b.label)}</span><span class="cluster-action">SELECT →</span>`;
          item.onclick = () => selectLanguage(langCode, b);
          list.appendChild(item);
        });
      } else if (drilldownState.step === 2) {
        badge.textContent = `STEP 2: ${drilldownState.lang.toUpperCase()} VERSIONS`;
        if (backBtn) backBtn.classList.remove("is-hidden");

        let versions = [];
        try {
          const res = await fetch(`/api/scripture/versions?lang=${drilldownState.lang}`);
          if (res.ok) versions = await res.json();
        } catch (e) {}

        if (versions.length === 0) versions = ["kjv"];
        if (q) versions = versions.filter(v => v.toLowerCase().includes(q));

        versions.forEach(v => {
          const item = document.createElement("div");
          item.className = "cluster-item";
          item.innerHTML = `<span>${v.toUpperCase()}</span><span class="cluster-action">SELECT →</span>`;
          item.onclick = () => selectVersion(v);
          list.appendChild(item);
        });
      } else if (drilldownState.step === 3) {
        badge.textContent = `STEP 3: ${drilldownState.version.toUpperCase()} BOOKS`;
        if (backBtn) backBtn.classList.remove("is-hidden");

        if (!drilldownState.booksMeta || drilldownState.booksMeta.length === 0) {
          try {
            const res = await fetch(`/api/scripture/meta?lang=${drilldownState.lang}&version=${drilldownState.version}`);
            if (res.ok) {
              const meta = await res.json();
              drilldownState.booksMeta = meta.books || [];
            }
          } catch (e) {}
        }

        let books = drilldownState.booksMeta;
        if (q) books = books.filter(b => b.name.toLowerCase().includes(q));

        books.forEach(b => {
          const item = document.createElement("div");
          item.className = "cluster-item";
          item.innerHTML = `<span>${b.name} (${b.chapters} ch)</span><span class="cluster-action">READ →</span>`;
          item.onclick = () => selectBook(b.name, b.chapters);
          list.appendChild(item);
        });
      } else if (drilldownState.step === 4) {
        badge.textContent = `STEP 4: ${drilldownState.book.toUpperCase()} CHAPTERS`;
        if (backBtn) backBtn.classList.remove("is-hidden");

        for (let i = 1; i <= drilldownState.totalChapters; i++) {
          if (q && !String(i).includes(q)) continue;
          const item = document.createElement("div");
          item.className = "cluster-item";
          item.innerHTML = `<span>Chapter ${i}</span><span class="cluster-action">OPEN</span>`;
          item.onclick = () => selectChapter(i);
          list.appendChild(item);
        }
      }
    }

    function selectLanguage(langCode, body) {
      drilldownState.lang = langCode;
      drilldownState.langName = cleanLabel(body.label);
      drilldownState.step = 2;
      drilldownState.filterQuery = "";
      document.getElementById("search-query").value = "";
      onSelectStar(body);
      renderDrilldown();
    }

    function selectVersion(version) {
      drilldownState.version = version;
      drilldownState.step = 3;
      drilldownState.booksMeta = [];
      drilldownState.filterQuery = "";
      document.getElementById("search-query").value = "";

      const vBody = galaxyData ? galaxyData.bodies.find(b => b.id.includes(`:${drilldownState.lang}:`) && b.id.toLowerCase().includes(version.toLowerCase())) : null;
      if (vBody) onSelectStar(vBody);
      renderDrilldown();
    }

    function selectBook(bookName, chaptersCount) {
      drilldownState.book = bookName;
      drilldownState.totalChapters = chaptersCount;
      drilldownState.step = 4;
      drilldownState.filterQuery = "";
      document.getElementById("search-query").value = "";

      const bBody = galaxyData ? galaxyData.bodies.find(b => b.id.toLowerCase().includes(bookName.toLowerCase())) : null;
      if (bBody) onSelectStar(bBody);
      renderDrilldown();
    }

    async function selectChapter(chapterNum) {
      drilldownState.chapter = chapterNum;
      await loadScriptureChapter(drilldownState.lang, drilldownState.version, drilldownState.book, chapterNum);
    }

    function stepBackDrilldown() {
      if (drilldownState.step > 1) {
        drilldownState.step -= 1;
        drilldownState.filterQuery = "";
        document.getElementById("search-query").value = "";
        renderDrilldown();
      }
    }

    async function loadScriptureChapter(lang, version, book, chapter) {
      try {
        const res = await fetch(`/api/scripture?lang=${lang}&version=${version}&book=${encodeURIComponent(book)}&chapter=${chapter}`);
        if (!res.ok) return;
        const data = await res.json();

        drilldownState.totalChapters = data.total_chapters;
        drilldownState.chapter = data.chapter;

        document.getElementById("node-name").textContent = `${data.book.toUpperCase()} ${data.chapter}`;
        document.getElementById("reading-title").textContent = `${data.book} ${data.chapter} (${data.version.toUpperCase()})`;

        const contentDiv = document.getElementById("scripture-content");
        contentDiv.innerHTML = "";

        data.verses.forEach((v, idx) => {
          const line = document.createElement("div");
          line.className = "verse-line";
          line.innerHTML = `<span class="verse-num">${idx + 1}</span><span class="verse-text">${v}</span>`;
          contentDiv.appendChild(line);
        });

        document.getElementById("hud-metadata-view").classList.add("is-hidden");
        document.getElementById("reading-pane").classList.remove("is-hidden");
        document.getElementById("hud-panel").style.display = "flex";
      } catch (e) {
        console.error("Failed to load scripture chapter", e);
      }
    }

    function prevChapter() {
      if (drilldownState.chapter > 1) {
        selectChapter(drilldownState.chapter - 1);
      }
    }

    function nextChapter() {
      if (drilldownState.chapter < drilldownState.totalChapters) {
        selectChapter(drilldownState.chapter + 1);
      }
    }

    function onSearchInput(val) {
      drilldownState.filterQuery = val;
      renderDrilldown();
    }

    // Smart Nearest-Node Locking on Click (Fixes UX issue, guarantees easy clicking)
    function onGalaxyClick(e) {
      if (e.target.closest('.sidebar-panel') || e.target.closest('.hud-panel') || e.target.closest('.top-bar')) return;

      const mouseX = e.clientX;
      const mouseY = e.clientY;

      if (!galaxyData.bodies || galaxyData.bodies.length === 0) return;

      let closestBody = null;
      let minScreenDist = Infinity;

      for (let i = 0; i < galaxyData.bodies.length; i++) {
        const body = galaxyData.bodies[i];
        tempVec3.set(body.x, body.y, body.z);
        galaxyGroup.localToWorld(tempVec3);
        tempVec3.project(camera);

        if (tempVec3.z < 1.0) {
          const sx = (tempVec3.x * 0.5 + 0.5) * window.innerWidth;
          const sy = (-(tempVec3.y * 0.5) + 0.5) * window.innerHeight;

          const dist = Math.hypot(mouseX - sx, mouseY - sy);
          if (dist < minScreenDist) {
            minScreenDist = dist;
            closestBody = body;
          }
        }
      }

      // Check central supermassive core click (only in galaxy section)
      if (currentSection === 'galaxy') {
        tempVec3.set(0, 0, 0);
        galaxyGroup.localToWorld(tempVec3);
        tempVec3.project(camera);
        if (tempVec3.z < 1.0) {
          const csx = (tempVec3.x * 0.5 + 0.5) * window.innerWidth;
          const csy = (-(tempVec3.y * 0.5) + 0.5) * window.innerHeight;
          const cdist = Math.hypot(mouseX - csx, mouseY - csy);
          if (cdist < 40.0) {
            const coreBody = galaxyData.bodies.find(b => b.id.includes(':core:')) || {
              id: 'celestial:core:database',
              label: 'GALACTIC DATABASE CORE',
              x: 0, y: 0, z: 0,
              mass: 1.0, luminosity: 1.0,
              content: 'Supermassive second brain memory core anchoring 1,097 cognitive celestial nodes.',
              tags: ['core', 'database', 'anchor', 'memory']
            };
            onSelectStar(coreBody);
            return;
          }
        }
      }

      // Generous 65px hitbox radius: effortlessly locks onto the nearest star node!
      if (closestBody && minScreenDist < 65.0) {
        onSelectStar(closestBody);
      }
    }

    // Smart Hover Tooltip & Cursor Indicator
    function onMouseMove(e) {
      if (e.target.closest('.sidebar-panel') || e.target.closest('.hud-panel') || e.target.closest('.top-bar')) {
        document.getElementById('node-hover-tooltip').style.display = 'none';
        container.style.cursor = 'default';
        return;
      }

      const mouseX = e.clientX;
      const mouseY = e.clientY;
      const tooltip = document.getElementById('node-hover-tooltip');

      if (!galaxyData.bodies || galaxyData.bodies.length === 0) return;

      let closestBody = null;
      let minScreenDist = Infinity;
      let targetScreenPos = { x: 0, y: 0 };

      for (let i = 0; i < galaxyData.bodies.length; i++) {
        const body = galaxyData.bodies[i];
        tempVec3.set(body.x, body.y, body.z);
        galaxyGroup.localToWorld(tempVec3);
        tempVec3.project(camera);

        if (tempVec3.z < 1.0) {
          const sx = (tempVec3.x * 0.5 + 0.5) * window.innerWidth;
          const sy = (-(tempVec3.y * 0.5) + 0.5) * window.innerHeight;

          const dist = Math.hypot(mouseX - sx, mouseY - sy);
          if (dist < minScreenDist) {
            minScreenDist = dist;
            closestBody = body;
            targetScreenPos = { x: sx, y: sy };
          }
        }
      }

      if (closestBody && minScreenDist < 35.0) {
        hoveredNode = closestBody;
        tooltip.style.left = `${targetScreenPos.x}px`;
        tooltip.style.top = `${targetScreenPos.y}px`;
        tooltip.textContent = cleanLabel(closestBody.label);
        tooltip.style.display = 'block';
        container.style.cursor = 'pointer';
      } else {
        hoveredNode = null;
        tooltip.style.display = 'none';
        container.style.cursor = 'crosshair';
      }
    }

    // Warp & Inspect Selected Memory Star
    async function onSelectStar(body) {
      currentFocusNode = body;
      const isCluster = body.id.includes(':lang:');
      const isCore = body.id.includes(':core:');

      if (isCluster) {
        currentClusterNode = body;
        document.getElementById('sep-1').style.display = 'inline';
        document.getElementById('nav-lang').style.display = 'inline';
        document.getElementById('nav-lang').textContent = cleanLabel(body.label);
        document.getElementById('sep-2').style.display = 'none';
        document.getElementById('nav-version').style.display = 'none';
      } else {
        document.getElementById('sep-2').style.display = 'inline';
        document.getElementById('nav-version').style.display = 'inline';
        document.getElementById('nav-version').textContent = cleanLabel(body.label);
      }

      const worldPos = new THREE.Vector3(body.x, body.y, body.z);
      galaxyGroup.localToWorld(worldPos);
      flyCameraTo(worldPos.x, worldPos.y, worldPos.z, isCore ? 280 : (isCluster ? 150 : 90));

      openHud(body);
    }

    function openHud(body) {
      const hud = document.getElementById('hud-panel');
      const metaView = document.getElementById('hud-metadata-view');
      const readingPane = document.getElementById('reading-pane');
      if (metaView) metaView.classList.remove('is-hidden');
      if (readingPane) readingPane.classList.add('is-hidden');
      const uiName = document.getElementById('node-name');
      const uiId = document.getElementById('node-id');
      const uiClass = document.getElementById('node-class');
      const uiTime = document.getElementById('node-time');
      const uiRadius = document.getElementById('node-radius');
      const uiMass = document.getElementById('node-mass');
      const uiDesc = document.getElementById('node-desc');
      const tagsRow = document.getElementById('node-tags');
      const grid = document.getElementById('constellation-grid');

      uiName.textContent = cleanLabel(body.label).toUpperCase();
      uiId.textContent = computeHexHash(body.id);
      uiClass.textContent = body.tier ? body.tier.toUpperCase() : 'MEMORY RECORD';
      uiTime.textContent = computeFormattedUtcTime();

      const rDist = Math.sqrt(body.x * body.x + body.z * body.z);
      uiRadius.textContent = `${rDist.toFixed(1)} AU`;
      uiMass.textContent = (body.mass || 0.85).toFixed(2);

      uiDesc.textContent = body.content || `Memory record "${body.label}" active in second brain database.`;

      tagsRow.innerHTML = '';
      const tags = body.tags && body.tags.length > 0 ? body.tags : ['history', 'civilization', 'memory'];
      tags.forEach(t => {
        const pill = document.createElement('span');
        pill.className = 'tag-pill';
        pill.textContent = `#${t.toUpperCase()}`;
        tagsRow.appendChild(pill);
      });

      // O(1) adjacency lookup
      const connected = cachedAdjacencyMap.get(body.id) || [];

      grid.innerHTML = '';
      if (connected.length > 0) {
        document.getElementById('constellations-section').style.display = 'block';
        connected.slice(0, 6).forEach(m => {
          const card = document.createElement('div');
          card.className = 'constellation-card';
          card.innerHTML = `<div class="constellation-title">${cleanLabel(m.label)}</div><div class="constellation-sub">${m.id.split(':')[1] || 'node'}</div>`;
          card.onclick = () => onSelectStar(m);
          grid.appendChild(card);
        });
      } else {
        document.getElementById('constellations-section').style.display = 'none';
      }

      hud.style.display = 'flex';
    }

    function closeHud() {
      document.getElementById('hud-panel').style.display = 'none';
      document.getElementById('selection-overlay-svg').innerHTML = '';
      currentFocusNode = null;
    }

    function computeHexHash(str) {
      let hash = 0;
      for (let i = 0; i < str.length; i++) {
        hash = (hash << 5) - hash + str.charCodeAt(i);
        hash |= 0;
      }
      return '0x' + (Math.abs(hash) % 0xFFFF).toString(16).toUpperCase().padStart(4, '0');
    }

    function computeFormattedUtcTime() {
      const d = new Date();
      return `${String(d.getUTCHours()).padStart(2,'0')}:${String(d.getUTCMinutes()).padStart(2,'0')}:${String(d.getUTCSeconds()).padStart(2,'0')} UTC`;
    }

    function drawWaveform(body, time) {
      const canvas = document.getElementById('waveform-canvas');
      if (!canvas || !body) return;
      const ctx = canvas.getContext('2d');
      const w = canvas.width;
      const h = canvas.height;
      ctx.clearRect(0, 0, w, h);

      ctx.strokeStyle = 'rgba(0, 255, 255, 0.2)';
      ctx.lineWidth = 1;
      ctx.beginPath(); ctx.moveTo(0, h/2); ctx.lineTo(w, h/2); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(w/2, 0); ctx.lineTo(w/2, h); ctx.stroke();

      const t = time * 0.003;
      const lum = body.luminosity || 0.9;
      const midY = h / 2;

      ctx.beginPath();
      ctx.strokeStyle = '#00ffff';
      ctx.lineWidth = 1.5;
      for (let x = 0; x < w; x += 2) {
        const nx = x / w;
        const y = midY + Math.sin(nx * 14.0 - t * 4.0) * (14.0 * lum);
        if (x === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      }
      ctx.stroke();

      ctx.beginPath();
      ctx.strokeStyle = '#ff007f';
      ctx.lineWidth = 1.5;
      for (let x = 0; x < w; x += 2) {
        const nx = x / w;
        const y = midY + Math.cos(nx * 18.0 + t * 3.0) * (9.0 * lum);
        if (x === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      }
      ctx.stroke();
    }

    function flyCameraTo(tx, ty, tz, distance) {
      new TWEEN.Tween(controls.target)
        .to({ x: tx, y: ty, z: tz }, 1000)
        .easing(TWEEN.Easing.Cubic.Out)
        .start();

      new TWEEN.Tween(camera.position)
        .to({ x: tx + distance * 0.7, y: ty + distance * 0.5, z: tz + distance }, 1200)
        .easing(TWEEN.Easing.Cubic.Out)
        .start();
    }

    function resetToGalacticView() {
      document.getElementById('sep-1').style.display = 'none';
      document.getElementById('nav-lang').style.display = 'none';
      document.getElementById('sep-2').style.display = 'none';
      document.getElementById('nav-version').style.display = 'none';
      closeHud();

      flyCameraTo(0, 0, 0, 1100);
    }

    function zoomToCurrentCluster() {
      if (currentClusterNode) onSelectStar(currentClusterNode);
    }

    async function searchGalaxy() {
      const query = document.getElementById('search-query').value.trim();
      if (!query) return;

      const res = await fetch(`/api/memory/recall?q=${encodeURIComponent(query)}&limit=5`);
      if (res.ok) {
        const results = await res.json();
        if (results.length > 0) {
          const first = results[0];
          const foundBody = galaxyData.bodies.find(b => b.id === first.id);
          if (foundBody) onSelectStar(foundBody);
        }
      }
    }

    async function formNewMemory() {
      const label = document.getElementById('new-concept-label').value.trim();
      if (!label) return;

      await fetch('/api/memory/ingest', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          label: label,
          content: `New memory node created in second brain matrix: ${label}`,
          tier: 'semantic',
          tags: ['second-brain', 'agent-context', 'database']
        })
      });

      document.getElementById('new-concept-label').value = '';
      lastRenderedCount = -1;
      fetchGalaxy();
    }

    function toggleAutoRotate() {
      controls.autoRotate = !controls.autoRotate;
    }

    function onWindowResize() {
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);
    }

    // Updates Square Corner Brackets and Curved Linking Filaments on the Selected Star
    function updateSelectionOverlay() {
      const svg = document.getElementById('selection-overlay-svg');
      if (!currentFocusNode || document.getElementById('hud-panel').style.display === 'none') {
        svg.innerHTML = '';
        return;
      }

      tempVec3.set(currentFocusNode.x, currentFocusNode.y, currentFocusNode.z);
      galaxyGroup.localToWorld(tempVec3);
      tempVec3.project(camera);

      if (tempVec3.z >= 1.0) {
        svg.innerHTML = '';
        return;
      }

      const sx = (tempVec3.x * 0.5 + 0.5) * window.innerWidth;
      const sy = (-(tempVec3.y * 0.5) + 0.5) * window.innerHeight;

      let svgContent = '';

      // 1. Draw Square Corner Brackets around Selected Star
      const pad = 15;
      const l = 6;
      svgContent += `
        <g stroke="#00ffff" stroke-width="1.5" fill="none" filter="drop-shadow(0 0 8px #00ffff)">
          <path d="M ${sx - pad} ${sy - pad + l} L ${sx - pad} ${sy - pad} L ${sx - pad + l} ${sy - pad}" />
          <path d="M ${sx + pad} ${sy - pad + l} L ${sx + pad} ${sy - pad} L ${sx + pad - l} ${sy - pad}" />
          <path d="M ${sx - pad} ${sy + pad - l} L ${sx - pad} ${sy + pad} L ${sx - pad + l} ${sy + pad}" />
          <path d="M ${sx + pad} ${sy + pad - l} L ${sx + pad} ${sy + pad} L ${sx + pad - l} ${sy + pad}" />
        </g>
      `;

      // 2. Draw Curved Linking Lines to Connected Neighbor Nodes (O(neighbors))
      if (currentSection === 'galaxy') {
        const neighbors = cachedAdjacencyMap.get(currentFocusNode.id) || [];
        neighbors.forEach(targetBody => {
          tempTargetVec3.set(targetBody.x, targetBody.y, targetBody.z);
          galaxyGroup.localToWorld(tempTargetVec3);
          tempTargetVec3.project(camera);

          if (tempTargetVec3.z < 1.0) {
            const tx = (tempTargetVec3.x * 0.5 + 0.5) * window.innerWidth;
            const ty = (-(tempTargetVec3.y * 0.5) + 0.5) * window.innerHeight;

            const midX = (sx + tx) / 2;
            const cpY = Math.min(sy, ty) - 40;

            svgContent += `
              <path d="M ${sx} ${sy} Q ${midX} ${cpY} ${tx} ${ty}" stroke="#00ffff" stroke-width="1.5" fill="none" opacity="0.9" filter="drop-shadow(0 0 6px #00ffff)" />
              <circle cx="${tx}" cy="${ty}" r="3" fill="#00ffff" filter="drop-shadow(0 0 6px #00ffff)" />
              <circle cx="${tx}" cy="${ty}" r="8" stroke="#00ffff" stroke-width="1" fill="none" opacity="0.8" />
            `;
          }
        });
      }

      svg.innerHTML = svgContent;
    }

    function animate(time) {
      requestAnimationFrame(animate);
      TWEEN.update();

      if (galaxyGroup && controls.autoRotate) {
        galaxyGroup.rotation.y += 0.0004;
      }

      // Pulse supermassive center corona & face camera
      if (centralCoronaMesh) {
        centralCoronaMesh.lookAt(camera.position);
        const scale = 1.0 + Math.sin(time * 0.003) * 0.06;
        centralCoronaMesh.scale.set(scale, scale, 1.0);
      }

      controls.update();
      renderer.render(scene, camera);

      const hud = document.getElementById('hud-panel');
      if (hud.style.display !== 'none' && currentFocusNode) {
        drawWaveform(currentFocusNode, time);
        updateSelectionOverlay();
      } else {
        document.getElementById('selection-overlay-svg').innerHTML = '';
      }
    }

    window.onload = init;

    window.addEventListener("keydown", (e) => {
      if (e.key === "Escape") {
        const hud = document.getElementById("hud-panel");
        const reading = document.getElementById("reading-pane");
        if (reading && !reading.classList.contains("is-hidden")) {
          reading.classList.add("is-hidden");
          document.getElementById("hud-metadata-view").classList.remove("is-hidden");
        } else if (hud && hud.style.display !== "none") {
          closeHud();
        } else if (drilldownState.step > 1) {
          stepBackDrilldown();
        }
      }
    });
