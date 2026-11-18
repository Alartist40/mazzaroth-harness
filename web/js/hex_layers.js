// MAZZAROTH Hexagon Data-Map Layers:
// 1. Historical Empires & Colonisation Eras (100 AD -> MODERN)
// 2. Spread of World Religions Across Time Periods
// 3. 66-Language Scripture Vault Coverage
// 4. Survival Biomes & Natural Hazard Zones (FM 21-76 / Bosai)
// 5. Ancient Trade & Maritime Star-Navigation Routes

export const HEX_ERAS = ['100', '500', '1500', '1700', '1815', '1914', '1945', 'MODERN'];

export const HEX_LAYERS = [
    { id: 'empires', label: 'EMPIRES & COLONIES', ja: '帝国・植民地史' },
    { id: 'religions', label: 'WORLD RELIGIONS', ja: '宗教の伝播' },
    { id: 'plagues', label: 'PLAGUES & EPIDEMICS', ja: '疫病・パンデミック' },
    { id: 'scripture', label: '66-LANG SCRIPTURE', ja: '聖典翻訳分布' },
    { id: 'survival', label: 'SURVIVAL & HAZARDS', ja: '防災・自然環境' }
];

export const EMPIRE_META = {
    ES: { name: 'Spanish Empire', color: '#eab308', stroke: '#fef08a' },
    GB: { name: 'British Empire', color: '#e11d48', stroke: '#fda4af' },
    PT: { name: 'Portuguese Empire', color: '#10b981', stroke: '#6ee7b7' },
    FR: { name: 'French Empire', color: '#3b82f6', stroke: '#93c5fd' },
    NL: { name: 'Dutch Empire', color: '#f97316', stroke: '#fdba74' },
    OT: { name: 'Ottoman Empire', color: '#14b8a6', stroke: '#5eead4' },
    RU: { name: 'Russian Empire / USSR', color: '#8b5cf6', stroke: '#c4b5fd' },
    JP: { name: 'Japanese Sphere', color: '#ec4899', stroke: '#f9a8d4' },
    RO: { name: 'Roman / Byzantine', color: '#b91c1c', stroke: '#fca5a5' },
    CN: { name: 'Chinese Dynasties', color: '#d97706', stroke: '#fcd34d' },
    IND: { name: 'Sovereign / Indigenous', color: '#475569', stroke: '#94a3b8' }
};

export const RELIGION_META = {
    CHR: { name: 'Christianity', color: '#3b82f6', stroke: '#93c5fd' },
    ISL: { name: 'Islam', color: '#10b981', stroke: '#6ee7b7' },
    BUD: { name: 'Buddhism', color: '#f59e0b', stroke: '#fde68a' },
    HIN: { name: 'Hinduism', color: '#f97316', stroke: '#fdba74' },
    EAS: { name: 'Shinto / Confucian / Dao', color: '#ec4899', stroke: '#f9a8d4' },
    JUD: { name: 'Judaism', color: '#06b6d4', stroke: '#67e8f9' },
    TRD: { name: 'Indigenous / Traditional', color: '#64748b', stroke: '#94a3b8' }
};

export const SURVIVAL_META = {
    SEISMIC: { name: 'Ring of Fire (Seismic/Tsunami)', color: '#e11d48', stroke: '#fda4af' },
    ARID: { name: 'Arid / Water Scarcity Belt', color: '#f59e0b', stroke: '#fde68a' },
    TROPIC: { name: 'Tropical / Vector Hazard', color: '#10b981', stroke: '#6ee7b7' },
    COLD: { name: 'Subarctic / Hypothermia Zone', color: '#38bdf8', stroke: '#bae6fd' },
    TEMP: { name: 'Temperate Zone', color: '#475569', stroke: '#94a3b8' }
};

export const PLAGUE_META = {
    BLACK_DEATH: { name: 'Bubonic Plague (Black Death / Justinian)', color: '#be123c', stroke: '#fda4af' },
    SMALLPOX: { name: 'Smallpox & Measles (Variola)', color: '#ea580c', stroke: '#fdba74' },
    CHICKENPOX: { name: 'Chickenpox / Varicella & Viral Exanthems', color: '#eab308', stroke: '#fef08a' },
    CHOLERA: { name: 'Cholera Pandemics (Vibrio cholerae)', color: '#0284c7', stroke: '#7dd3fc' },
    INFLUENZA: { name: '1918 Influenza Pandemic (H1N1)', color: '#9333ea', stroke: '#d8b4fe' },
    TROPICAL_VECTOR: { name: 'Malaria, Yellow Fever & Typhus', color: '#16a34a', stroke: '#86efac' },
    MODERN_RESP: { name: 'Modern Respiratory Pandemics (COVID-19 / SARS)', color: '#06b6d4', stroke: '#67e8f9' },
    LOW_INCIDENCE: { name: 'Low / Sparse Epidemic Record', color: '#334155', stroke: '#64748b' }
};

export const SCRIPTURE_ISO_MAP = {
    US: { code: 'eng', label: 'English (38 versions)' }, GB: { code: 'eng', label: 'English (38 versions)' },
    AU: { code: 'eng', label: 'English (38 versions)' }, CA: { code: 'eng', label: 'English / French' },
    NZ: { code: 'mri', label: 'Maori / English' }, ZA: { code: 'zul', label: 'Zulu / Xhosa / Afrikaans' },
    DE: { code: 'deu', label: 'German (14 versions)' }, AT: { code: 'deu', label: 'German (14 versions)' },
    CH: { code: 'deu', label: 'German / French / Italian' }, NL: { code: 'nld', label: 'Dutch (8 versions)' },
    BE: { code: 'vls', label: 'Dutch / French / West Flemish' }, FR: { code: 'fra', label: 'French (5 versions)' },
    ES: { code: 'spa', label: 'Spanish (4 versions)' }, MX: { code: 'spa', label: 'Spanish (4 versions)' },
    AR: { code: 'spa', label: 'Spanish (4 versions)' }, CO: { code: 'spa', label: 'Spanish (4 versions)' },
    PE: { code: 'spa', label: 'Spanish (4 versions)' }, CL: { code: 'arn', label: 'Spanish / Mapudungun' },
    BR: { code: 'por', label: 'Portuguese (3 versions)' }, PT: { code: 'por', label: 'Portuguese (3 versions)' },
    IT: { code: 'ita', label: 'Italian (3 versions)' }, VA: { code: 'lat', label: 'Latin (3 versions)' },
    GR: { code: 'ell', label: 'Greek (3 versions)' }, IL: { code: 'heb', label: 'Hebrew (4 versions)' },
    JP: { code: 'jpn', label: 'Japanese (3 versions)' }, KR: { code: 'kor', label: 'Korean (2 versions)' },
    CN: { code: 'zho', label: 'Chinese (4 versions)' }, TW: { code: 'zho', label: 'Chinese (4 versions)' },
    RU: { code: 'rus', label: 'Russian (3 versions)' }, UA: { code: 'ukr', label: 'Ukrainian' },
    IN: { code: 'hin', label: 'Hindi / Tamil / Telugu / Bengali' }, ID: { code: 'ind', label: 'Indonesian' },
    PH: { code: 'tgl', label: 'Tagalog / Cebuano / Tausug' }, TH: { code: 'tha', label: 'Thai' },
    VN: { code: 'vie', label: 'Vietnamese' }, MM: { code: 'mya', label: 'Burmese' },
    NP: { code: 'nep', label: 'Nepali' }, MG: { code: 'mlg', label: 'Malagasy' },
    PG: { code: 'tpi', label: 'Tok Pisin' }, HT: { code: 'hat', label: 'Haitian Creole' },
    PL: { code: 'pol', label: 'Polish (2 versions)' }, SE: { code: 'swe', label: 'Swedish (3 versions)' },
    NO: { code: 'nor', label: 'Norwegian' }, DK: { code: 'dan', label: 'Danish' },
    FI: { code: 'fin', label: 'Finnish' }, CZ: { code: 'ces', label: 'Czech' },
    HU: { code: 'hun', label: 'Hungarian' }, HR: { code: 'hrv', label: 'Croatian' },
    RS: { code: 'srp', label: 'Serbian (2 versions)' }, SI: { code: 'slv', label: 'Slovenian' },
    EE: { code: 'est', label: 'Estonian' }, AL: { code: 'alb', label: 'Albanian' },
    AM: { code: 'arm', label: 'Armenian' }, EG: { code: 'cop', label: 'Coptic' },
    SY: { code: 'syr', label: 'Syriac' }, BD: { code: 'ben', label: 'Bengali' }
};

export const TRADE_ROUTES = [
    { name: 'Silk Road', color: '#f59e0b', pts: [[109, 34], [87, 43], [67, 40], [51, 35], [36, 36], [29, 41]] },
    { name: 'Maritime Spice Route', color: '#10b981', pts: [[113, 23], [104, 1], [80, 7], [73, 19], [45, 13], [32, 31]] },
    { name: 'Manila–Acapulco Galleon', color: '#eab308', pts: [[121, 14], [145, 20], [-155, 22], [-100, 17]] },
    { name: 'Trans-Saharan Gold & Salt', color: '#f97316', pts: [[-6, 34], [-3, 17], [9, 12], [31, 30]] },
    { name: 'Polynesian & Polaris Nav', color: '#38bdf8', pts: [[139, 35], [-156, 20], [-149, -17], [175, -37]] }
];

const SPANISH_AM = new Set(['MX', 'GT', 'HN', 'SV', 'NI', 'CR', 'PA', 'CU', 'DO', 'PR', 'CO', 'VE', 'EC', 'PE', 'BO', 'CL', 'AR', 'PY', 'UY', 'PH']);
const PORTUGUESE_EMP = new Set(['PT', 'BR', 'AO', 'MZ', 'GW', 'TL']);
const BRITISH_EMP = new Set(['GB', 'IE', 'CA', 'IN', 'PK', 'BD', 'MM', 'MY', 'AU', 'NZ', 'ZA', 'ZW', 'ZM', 'KE', 'UG', 'TZ', 'NG', 'GH', 'SL', 'EG', 'SD', 'YE', 'LK', 'FJ']);
const FRENCH_EMP = new Set(['FR', 'DZ', 'MA', 'TN', 'ML', 'NE', 'TD', 'SN', 'GN', 'CI', 'BF', 'BJ', 'TG', 'CM', 'GA', 'CG', 'CF', 'MG', 'VN', 'LA', 'KH', 'SY', 'LB']);
const DUTCH_EMP = new Set(['NL', 'ID', 'SR']);
const OTTOMAN_EMP = new Set(['TR', 'GR', 'BG', 'RS', 'BA', 'AL', 'MK', 'SY', 'IQ', 'JO', 'IL', 'PS', 'LB', 'EG', 'LY', 'TN', 'SA']);
const RUSSIAN_EMP = new Set(['RU', 'UA', 'BY', 'KZ', 'UZ', 'TM', 'KG', 'TJ', 'GE', 'AM', 'AZ', 'EE', 'LV', 'LT', 'MD', 'FI']);
const ROMAN_EMP = new Set(['IT', 'ES', 'PT', 'FR', 'GB', 'BE', 'CH', 'AT', 'GR', 'TR', 'SY', 'IL', 'PS', 'JO', 'EG', 'LY', 'TN', 'DZ', 'MA', 'RO', 'BG', 'HR', 'RS']);
const JAPAN_SPHERE = new Set(['JP', 'KR', 'KP', 'TW']);

export function getEmpireForHex(iso, era) {
    if (era === 'MODERN') return 'IND';
    if (era === '100' || era === '500') {
        if (ROMAN_EMP.has(iso)) return 'RO';
        if (iso === 'CN') return 'CN';
        if (iso === 'JP') return 'JP';
        return 'IND';
    }
    if (era === '1500') {
        if (iso === 'ES' || iso === 'CU' || iso === 'DO' || iso === 'MX') return 'ES';
        if (iso === 'PT' || iso === 'BR') return 'PT';
        if (OTTOMAN_EMP.has(iso)) return 'OT';
        if (iso === 'CN') return 'CN';
        if (iso === 'RU') return 'RU';
        if (iso === 'JP') return 'JP';
        return 'IND';
    }
    if (era === '1700') {
        if (SPANISH_AM.has(iso) || iso === 'ES') return 'ES';
        if (PORTUGUESE_EMP.has(iso)) return 'PT';
        if (iso === 'US' || iso === 'CA' || iso === 'GB' || iso === 'IE' || iso === 'IN') return 'GB';
        if (iso === 'FR' || iso === 'HT') return 'FR';
        if (DUTCH_EMP.has(iso)) return 'NL';
        if (OTTOMAN_EMP.has(iso)) return 'OT';
        if (RUSSIAN_EMP.has(iso)) return 'RU';
        if (iso === 'CN') return 'CN';
        if (iso === 'JP') return 'JP';
        return 'IND';
    }
    if (era === '1815') {
        if (SPANISH_AM.has(iso) || iso === 'ES') return 'ES';
        if (BRITISH_EMP.has(iso)) return 'GB';
        if (PORTUGUESE_EMP.has(iso)) return 'PT';
        if (iso === 'FR' || iso === 'DZ') return 'FR';
        if (DUTCH_EMP.has(iso)) return 'NL';
        if (OTTOMAN_EMP.has(iso)) return 'OT';
        if (RUSSIAN_EMP.has(iso)) return 'RU';
        if (iso === 'CN') return 'CN';
        if (iso === 'JP') return 'JP';
        return 'IND';
    }
    if (era === '1914' || era === '1945') {
        if (BRITISH_EMP.has(iso)) return 'GB';
        if (FRENCH_EMP.has(iso)) return 'FR';
        if (JAPAN_SPHERE.has(iso) || (era === '1945' && (iso === 'PH' || iso === 'ID' || iso === 'MY'))) return 'JP';
        if (DUTCH_EMP.has(iso)) return 'NL';
        if (PORTUGUESE_EMP.has(iso)) return 'PT';
        if (RUSSIAN_EMP.has(iso)) return 'RU';
        if (era === '1914' && OTTOMAN_EMP.has(iso)) return 'OT';
        if (iso === 'ES') return 'ES';
        return 'IND';
    }
    return 'IND';
}

const ISLAM_CORE = new Set(['SA', 'YE', 'OM', 'AE', 'QA', 'KW', 'BH', 'IQ', 'IR', 'SY', 'JO', 'PS', 'LB', 'TR', 'EG', 'LY', 'TN', 'DZ', 'MA', 'SD', 'SO', 'ML', 'NE', 'TD', 'SN', 'MR', 'AF', 'PK', 'UZ', 'KZ', 'TM', 'KG', 'TJ', 'AZ', 'ID', 'MY', 'BD']);
const BUDDHIST_CORE = new Set(['TH', 'MM', 'KH', 'LA', 'LK', 'MN', 'BT', 'VN']);
const HINDU_CORE = new Set(['IN', 'NP']);
const EAST_ASIAN = new Set(['CN', 'JP', 'KR', 'KP', 'TW']);

export function getReligionForHex(iso, region, era) {
    if (iso === 'IL') return 'JUD';
    if (era === '100') {
        if (iso === 'IL' || iso === 'PS') return 'JUD';
        if (iso === 'IN' || iso === 'NP' || iso === 'LK') return 'HIN';
        if (EAST_ASIAN.has(iso)) return 'EAS';
        if (region === 'EU' || region === 'ME') return 'TRD'; // early spread
        return 'TRD';
    }
    if (era === '500') {
        if (region === 'EU' || iso === 'ET' || iso === 'EG' || iso === 'SY' || iso === 'TR') return 'CHR';
        if (HINDU_CORE.has(iso)) return 'HIN';
        if (BUDDHIST_CORE.has(iso) || iso === 'CN') return 'BUD';
        if (iso === 'JP' || iso === 'KR') return 'EAS';
        return 'TRD';
    }
    if (era === '1500') {
        if (region === 'EU' || iso === 'ET' || iso === 'RU') return 'CHR';
        if (ISLAM_CORE.has(iso)) return 'ISL';
        if (HINDU_CORE.has(iso)) return 'HIN';
        if (BUDDHIST_CORE.has(iso)) return 'BUD';
        if (EAST_ASIAN.has(iso)) return 'EAS';
        return 'TRD';
    }
    // 1700, 1815, 1914, 1945, MODERN
    if (region === 'EU' || region === 'NA' || region === 'LA' || iso === 'AU' || iso === 'NZ' || iso === 'PH' || iso === 'ZA' || iso === 'KE' || iso === 'UG' || iso === 'CD' || iso === 'ET' || iso === 'RU') return 'CHR';
    if (ISLAM_CORE.has(iso) || iso === 'NG' || iso === 'BA' || iso === 'AL') return 'ISL';
    if (HINDU_CORE.has(iso)) return 'HIN';
    if (BUDDHIST_CORE.has(iso)) return 'BUD';
    if (EAST_ASIAN.has(iso)) return 'EAS';
    return 'TRD';
}

const SEISMIC_ISOS = new Set(['JP', 'ID', 'PH', 'CL', 'PE', 'MX', 'NZ', 'PG', 'TW', 'TR', 'IT', 'GR', 'NP']);
const ARID_ISOS = new Set(['SA', 'YE', 'OM', 'AE', 'EG', 'LY', 'DZ', 'MA', 'TN', 'SD', 'ML', 'NE', 'TD', 'MR', 'IQ', 'IR', 'AU', 'KZ', 'UZ', 'TM', 'NA', 'BW']);
const TROPIC_ISOS = new Set(['BR', 'CO', 'VE', 'EC', 'CD', 'CG', 'CM', 'NG', 'GH', 'CI', 'KE', 'TZ', 'MG', 'IN', 'TH', 'MM', 'VN', 'KH', 'LA', 'MY', 'BD']);
const COLD_ISOS = new Set(['RU', 'CA', 'NO', 'SE', 'FI', 'IS', 'GL', 'MN']);

export function getSurvivalForHex(iso, lat) {
    if (SEISMIC_ISOS.has(iso)) return 'SEISMIC';
    if (ARID_ISOS.has(iso)) return 'ARID';
    if (TROPIC_ISOS.has(iso)) return 'TROPIC';
    if (COLD_ISOS.has(iso) || lat >= 58 || lat <= -48) return 'COLD';
    return 'TEMP';
}

const MED_PLAGUE_CORE = new Set(['IT', 'GR', 'TR', 'EG', 'SY', 'IL', 'PS', 'LB', 'IQ', 'IR', 'FR', 'ES', 'GB', 'DE', 'NL', 'BE', 'PL', 'RU', 'CN', 'MN', 'KZ', 'UZ']);
const AMERICAS_ISOS = new Set(['US', 'CA', 'MX', 'GT', 'HN', 'SV', 'NI', 'CR', 'PA', 'CU', 'DO', 'HT', 'CO', 'VE', 'EC', 'PE', 'BO', 'CL', 'AR', 'PY', 'UY', 'BR']);
const CHOLERA_BELT = new Set(['IN', 'BD', 'MM', 'TH', 'ID', 'PK', 'IR', 'IQ', 'EG', 'TR', 'RU', 'GB', 'FR', 'DE', 'IT', 'ES', 'US']);
const H1N1_CORE = new Set(['US', 'CA', 'GB', 'FR', 'DE', 'ES', 'IT', 'RU', 'IN', 'CN', 'JP', 'KR', 'BR', 'ZA', 'AU', 'NZ', 'MX', 'AR']);
const MODERN_HUBS = new Set(['US', 'CA', 'GB', 'FR', 'DE', 'IT', 'ES', 'BR', 'IN', 'CN', 'JP', 'KR', 'AU', 'ZA', 'RU', 'MX', 'TR', 'SA', 'ID', 'TH', 'VN']);

export function getPlagueForHex(iso, region, lat, era) {
    const isTropic = TROPIC_ISOS.has(iso) || (Math.abs(lat) <= 22 && (region === 'AF' || region === 'LA' || region === 'AP'));
    if (era === '100') {
        if (ROMAN_EMP.has(iso) || iso === 'IR' || iso === 'IQ') return 'SMALLPOX'; // Antonine Plague
        if (iso === 'CN' || iso === 'IN') return 'CHICKENPOX';
        if (isTropic) return 'TROPICAL_VECTOR';
        return 'LOW_INCIDENCE';
    }
    if (era === '500') {
        if (ROMAN_EMP.has(iso) || iso === 'IQ' || iso === 'IR' || iso === 'YE') return 'BLACK_DEATH'; // Plague of Justinian
        if (iso === 'CN' || iso === 'JP' || iso === 'KR') return 'SMALLPOX';
        if (isTropic) return 'TROPICAL_VECTOR';
        return 'LOW_INCIDENCE';
    }
    if (era === '1500') {
        if (AMERICAS_ISOS.has(iso)) return 'SMALLPOX'; // Columbian Exchange Smallpox & Measles
        if (MED_PLAGUE_CORE.has(iso) || region === 'EU' || region === 'ME') return 'BLACK_DEATH';
        if (iso === 'JP' || iso === 'KR' || iso === 'IN') return 'CHICKENPOX';
        if (isTropic) return 'TROPICAL_VECTOR';
        return 'LOW_INCIDENCE';
    }
    if (era === '1700') {
        if (AMERICAS_ISOS.has(iso) || iso === 'GB' || iso === 'FR' || iso === 'ES' || iso === 'CN' || iso === 'JP') return 'SMALLPOX';
        if (iso === 'RU' || iso === 'TR' || iso === 'EG' || iso === 'IR') return 'BLACK_DEATH';
        if (iso === 'DE' || iso === 'IT' || iso === 'NL' || iso === 'SE') return 'CHICKENPOX';
        if (isTropic) return 'TROPICAL_VECTOR';
        return 'LOW_INCIDENCE';
    }
    if (era === '1815') {
        if (CHOLERA_BELT.has(iso)) return 'CHOLERA'; // 1st–3rd Cholera Pandemics
        if (iso === 'BR' || iso === 'CU' || iso === 'CO' || iso === 'NG' || iso === 'GH' || isTropic) return 'TROPICAL_VECTOR';
        if (iso === 'MX' || iso === 'PE' || iso === 'ZA' || iso === 'AU') return 'SMALLPOX';
        if (region === 'EU' || iso === 'US' || iso === 'CA' || iso === 'JP') return 'CHICKENPOX';
        return 'LOW_INCIDENCE';
    }
    if (era === '1914') {
        if (H1N1_CORE.has(iso)) return 'INFLUENZA'; // 1918 Spanish Flu
        if (iso === 'MN' || iso === 'KZ' || iso === 'MG') return 'BLACK_DEATH';
        if (isTropic) return 'TROPICAL_VECTOR';
        return 'CHICKENPOX';
    }
    if (era === '1945') {
        if (iso === 'PL' || iso === 'UA' || iso === 'DE' || iso === 'RU' || isTropic) return 'TROPICAL_VECTOR'; // Wartime Typhus & Malaria
        if (iso === 'IN' || iso === 'BD' || iso === 'CN' || iso === 'EG') return 'CHOLERA';
        if (region === 'EU' || region === 'NA' || iso === 'JP' || iso === 'AU') return 'CHICKENPOX'; // Pre-vaccine childhood varicella
        return 'SMALLPOX';
    }
    // MODERN
    if (MODERN_HUBS.has(iso)) return 'MODERN_RESP';
    if (isTropic) return 'TROPICAL_VECTOR';
    if (iso === 'YE' || iso === 'HT' || iso === 'SO' || iso === 'SD') return 'CHOLERA';
    return 'CHICKENPOX';
}
