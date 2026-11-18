// MAZZAROTH Bilingual i18n Layer (Complete System-Wide EN / JA)

const LANG_KEY = 'mazzaroth_lang_v1';

export function getActiveLang() {
    return localStorage.getItem(LANG_KEY) === 'ja' ? 'ja' : 'en';
}

export function setActiveLang(lang) {
    const next = lang === 'ja' ? 'ja' : 'en';
    localStorage.setItem(LANG_KEY, next);
    return next;
}

export const I18N = {
    en: {
        ui: {
            appTitle: 'MAZZAROTH // SOVEREIGN CELESTIAL GRAPH & KNOWLEDGE ENGINE',
            offlineBadge: '100% OFFLINE-FIRST',
            audioPrefix: 'AUDIO:',
            searchLabel: 'SEARCH',
            newLabel: 'NEW',
            sectionsHeader: 'SYSTEM SECTIONS',
            modulesCount: '4 MODULES',
            navGalaxy: 'GALAXY',
            navGalaxySub: 'Logarithmic Spiral Stipple',
            navConstellations: 'CONSTELLATIONS',
            navConstellationsSub: 'Zodiac, Asterisms & Sky Dome',
            navLibrarian: 'LIBRARIAN',
            navLibrarianSub: 'Local Ollama AI Engine',
            navMap: 'MAP',
            navMapSub: 'Offline Vector Cartography',
            activeBadge: 'ACTIVE',
            viewBadge: 'VIEW',
            toolLayout: 'LAYOUT',
            toolTime: 'TIME',
            toolNow: 'NOW',
            toolSeason: 'SEASON',
            toolCoord: 'COORD',
            toolModel: 'MODEL',
            toolLocal: 'LOCAL',
            toolLayer: 'LAYER',
            toolLlmHost: 'HOST',
            locPresetLabel: 'LOCATION PRESET',
            utcTimeLabel: 'UTC DATE / TIME',
            resetBtn: 'RESET',
            domeOpt: '01 // CELESTIAL DOME (LIVE)',
            sphereOpt: '02 // SPHERE DOME (3D)',
            posterOpt: '03 // POSTER CATALOG (GRID)',
            synopticBtn: 'SYNOPTIC',
            synapsesLabel: 'SYNAPSES',
            clickStarHint: 'CLICK STAR TO TRAVERSE',
            prevBtn: '‹ PREV',
            nextBtn: 'NEXT ›',
            targetLockBtn: 'TARGET LOCK',
            systemSpecs: 'SYSTEM SPECS',
            activeStars: 'ACTIVE STARS',
            synapseLinks: 'SYNAPSE LINKS',
            sovereignRepo: 'SOVEREIGN DATA REPO',
            langsVersions: '66 LANGS / 226 VERSIONS',
            dbIndexTag: 'DATABASE_INDEX',
            themeExplorer: 'THEME EXPLORER',
            vaultLabel: 'Mazzaroth Sovereign Vault',
            filterPlaceholder: 'Filter knowledge themes... (e.g. scripture, astronomy, survival, medical)',
            matchedSuffix: 'MATCHED',
            themesHeader: 'KNOWLEDGE THEMES',
            mapLegend: 'DRAG: PAN • SCROLL: ZOOM • CLICK FLOATING BADGE / CELL TO READ RECORD'
        },
        categories: {
            scripture: { label: 'SCRIPTURE & SACRED TEXTS', desc: '66 Languages, 226 Translations & Thousands of Canonical Books' },
            astronomy: { label: 'ASTRONOMY & ASTROMETRY', desc: 'Star navigation handbook, celestial lore & ephemeris data' },
            survival: { label: 'SURVIVAL & EXPEDITION', desc: 'Field manual, water purification, shelter & wilderness tactics' },
            medical: { label: 'EMERGENCY MEDICAL', desc: 'First responder protocols, triage & wound intervention' },
            cognitive: { label: 'COGNITIVE MEMORY', desc: 'Sovereign neural memory stars & contextual linkages' },
            youth: { label: 'YOUTH MINISTRIES (TED)', desc: 'Adventurers, Pathfinders, Ambassadors, SYL, PCM, manuals, laws, pledges & songs' },
            pathfinders: { label: 'PATHFINDER HONORS & AWARDS', desc: 'Complete requirements, answers & field study manuals across 9 disciplines' }
        },
        officeTitle: 'LIBRARIAN // SOVEREIGN OFFICE',
        officeSubtitle: 'Click any worker to assign & query your local Ollama vault',
        studioBtn: 'PIXEL STUDIO',
        hideOffice: 'HIDE OFFICE',
        showOffice: 'SHOW OFFICE',
        soundOff: 'SOUND: OFF',
        soundOn: 'SOUND: ON',
        queryBtn: 'QUERY',
        stopBtn: 'STOP',
        askPlaceholder: (name) => `Ask ${name} about the local vault...`,
        rooms: {
            office: 'Archive Hall',
            'meeting-room': 'Study & Maps',
            'server-room': 'Vault Servers',
            'lunch-room': 'Break Lounge',
            corridor: 'Corridor'
        },
        workers: {}
    },
    ja: {
        ui: {
            appTitle: 'マザロス // 自律型天体ナレッジ＆オフライン知識庫',
            offlineBadge: '100% オフライン駆動',
            audioPrefix: '音響:',
            searchLabel: '検索',
            newLabel: '新規',
            sectionsHeader: 'システム機能',
            modulesCount: '4 モジュール',
            navGalaxy: '銀河',
            navGalaxySub: '対数螺旋ナレッジ星図',
            navConstellations: '星座',
            navConstellationsSub: '黄道十二宮・全天ドーム',
            navLibrarian: '司書室',
            navLibrarianSub: 'ローカルOllama AI知識庫',
            navMap: '地図',
            navMapSub: 'オフライン・ベクトル地図',
            activeBadge: '稼働中',
            viewBadge: '表示',
            toolLayout: '表示',
            toolTime: '時刻',
            toolNow: '現在',
            toolSeason: '季節',
            toolCoord: '方位',
            toolModel: 'モデル',
            toolLocal: '接続',
            toolLayer: '切替',
            toolLlmHost: 'ホスト',
            locPresetLabel: '観測地プリセット',
            utcTimeLabel: 'UTC 日時',
            resetBtn: 'リセット',
            domeOpt: '01 // 全天ドーム（リアルタイム）',
            sphereOpt: '02 // 3D天球儀モード',
            posterOpt: '03 // 星座カタログ（一覧）',
            synopticBtn: '対照表示',
            synapsesLabel: 'シナプス結合',
            clickStarHint: '星をクリックして移動',
            prevBtn: '‹ 前へ',
            nextBtn: '次へ ›',
            targetLockBtn: '対象固定',
            systemSpecs: 'システム仕様',
            activeStars: '稼働スター数',
            synapseLinks: 'シナプス結合数',
            sovereignRepo: '知識リポジトリ',
            langsVersions: '66言語 / 226翻訳版',
            dbIndexTag: 'データベース索引',
            themeExplorer: 'テーマ探索',
            vaultLabel: 'マザロス自律知識庫',
            filterPlaceholder: '知識テーマを絞り込む...（例: 聖書、天文、防災、医療）',
            matchedSuffix: '件一致',
            themesHeader: '知識テーマ一覧',
            mapLegend: 'ドラッグ: 移動 • スクロール: 拡大縮小 • ピンをクリックして記録を表示'
        },
        categories: {
            scripture: { label: '聖典・宗教文献', desc: '66言語・226翻訳版の聖典アーカイブと原語対照' },
            astronomy: { label: '天文学・天文航法', desc: '北極星・主要星座・天体位置計算ハンドブック' },
            survival: { label: '防災・サバイバル技術', desc: '飲料水浄化・シェルター構築・災害避難教範' },
            medical: { label: '救急医療・臨床知識', desc: '気道確保・止血法・START式トリアージ・臨床プロトコル' },
            cognitive: { label: '記憶ノード・ノート', desc: '自律型ナレッジ星図と個人メモリンク' },
            youth: { label: 'ユース・ミニストリー', desc: '指導者・青少年活動マニュアル・規律・資料集' },
            pathfinders: { label: 'パスファインダー技能章', desc: '9分野の実地学習マニュアル・履修要件アーカイブ' }
        },
        officeTitle: '司書室 // SOVEREIGN OFFICE',
        officeSubtitle: '担当者をクリックしてローカルOllama知識庫へ質問できます',
        studioBtn: 'アートスタジオ',
        hideOffice: 'オフィス隠す',
        showOffice: 'オフィス表示',
        soundOff: '音声: 切',
        soundOn: '音声: 入',
        queryBtn: '質問',
        stopBtn: '停止',
        askPlaceholder: (name) => `${name}にローカル知識庫について質問する...`,
        rooms: {
            office: '書庫ホール',
            'meeting-room': '研究室・地図',
            'server-room': 'サーバー室',
            'lunch-room': '休憩室',
            corridor: '回廊'
        },
        workers: {
            coordinator: {
                name: '司書長',
                shortName: '司書長',
                role: '総合案内',
                specialty: '全アーカイブ統括・分野横断リサーチ',
                prompts: ['知識庫に収録されている分野を教えて', '北極星と水質浄化の要点をまとめて']
            },
            archivist: {
                name: '古文書官',
                shortName: '古文書官',
                role: '聖典・文献',
                specialty: '66言語・226訳の聖典比較と原語解説',
                prompts: ['ヨハネによる福音書1章1節を比較して', '創世記1章1節の翻訳の違いを教えて']
            },
            pathfinder: {
                name: '開拓者',
                shortName: '開拓者',
                role: '防災・生存',
                specialty: '飲料水の浄化・シェルター・地震津波避難',
                prompts: ['飲料水の煮沸消毒と濾過の方法は？', '低体温症を防ぐシェルターの作り方は？']
            },
            astronomer: {
                name: '天文学者',
                shortName: '天文学者',
                role: '天文・星座',
                specialty: '88星座・北極星による緯度測定・天文航法',
                prompts: ['北斗七星から北極星と緯度を求める方法は？', 'オリオン座と夏の大三角の特徴を教えて']
            },
            medic: {
                name: '救護官',
                shortName: '救護官',
                role: '医療・救急',
                specialty: '気道確保・直接圧迫止血・START式トリアージ',
                prompts: ['直接圧迫止血法とショック対応の手順は？', 'START式トリアージの判定基準を教えて']
            },
            chronicler: {
                name: '記録官',
                shortName: '記録官',
                role: '検索・記憶',
                specialty: 'FTS5全文検索・記憶ノード・横断統合',
                prompts: ['防災と医療の重要プロトコルを検索して', '天文航法に関する記録をまとめて']
            }
        }
    }
};
