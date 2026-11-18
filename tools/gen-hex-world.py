#!/usr/bin/env python3
"""Bake accurate Natural Earth 110m land hexes into web/assets/hex-world-land.json (zero deps)."""
import json
import sys
from pathlib import Path

GEO_PATH = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("/home/xander/.gemini/antigravity/brain/d480ff00-cd7a-4522-b510-0e471d9cc6ec/scratch/ne_110m_countries.geojson")
OUT_PATH = Path("web/assets/hex-world-land.json")

ME_ISOS = {"IL", "PS", "JO", "LB", "SY", "IQ", "IR", "SA", "YE", "OM", "AE", "QA", "BH", "KW", "TR", "EG", "CY"}

def point_in_ring(x, y, ring):
    inside = False
    n = len(ring)
    j = n - 1
    for i in range(n):
        xi, yi = ring[i][0], ring[i][1]
        xj, yj = ring[j][0], ring[j][1]
        if ((yi > y) != (yj > y)) and (x < (xj - xi) * (y - yi) / ((yj - yi) or 1e-12) + xi):
            inside = not inside
        j = i
    return inside

def point_in_poly(x, y, coords):
    if not coords or not point_in_ring(x, y, coords[0]):
        return False
    for hole in coords[1:]:
        if point_in_ring(x, y, hole):
            return False
    return True

def map_region(iso, continent, sub):
    if iso in ME_ISOS:
        return "ME"
    if continent == "North America":
        return "LA" if sub in ("Central America", "Caribbean") or iso == "MX" else "NA"
    if continent == "South America":
        return "LA"
    if continent == "Europe":
        return "EU"
    if continent == "Africa":
        return "AF"
    return "AP"

def main():
    data = json.loads(GEO_PATH.read_text(encoding="utf-8"))
    countries = []
    for feat in data.get("features", []):
        props = feat.get("properties", {})
        iso = props.get("ISO_A2_EH") or props.get("ISO_A2") or "-99"
        if iso == "-99":
            iso = props.get("ADM0_A3", "XX")[:2]
        name = props.get("NAME") or props.get("ADMIN") or "Unknown"
        if iso == "AQ" or name == "Antarctica":
            continue
        cont = props.get("CONTINENT", "")
        sub = props.get("SUBREGION", "")
        reg = map_region(iso, cont, sub)
        geom = feat.get("geometry", {})
        gtype = geom.get("type")
        polys = [geom["coordinates"]] if gtype == "Polygon" else (geom.get("coordinates", []) if gtype == "MultiPolygon" else [])
        # Precompute bbox for fast rejection
        bboxes = []
        for poly in polys:
            xs = [pt[0] for pt in poly[0]]
            ys = [pt[1] for pt in poly[0]]
            bboxes.append((min(xs), min(ys), max(xs), max(ys), poly))
        countries.append((iso, reg, name, bboxes))

    lon_step = 3.2
    lat_step = 2.6
    hexes = []
    row = 0
    lat = -56.0
    while lat <= 75.0:
        offset = (lon_step * 0.5) if (row % 2 == 1) else 0.0
        lon = -178.0 + offset
        while lon <= 178.0:
            matched = None
            for iso, reg, name, bboxes in countries:
                for minx, miny, maxx, maxy, poly in bboxes:
                    if minx <= lon <= maxx and miny <= lat <= maxy:
                        if point_in_poly(lon, lat, poly):
                            matched = [round(lon, 2), round(lat, 2), iso, reg, name]
                            break
                if matched:
                    break
            if matched:
                hexes.append(matched)
            lon += lon_step
        lat += lat_step
        row += 1

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUT_PATH.write_text(json.dumps({"hexes": hexes}, separators=(",", ":")), encoding="utf-8")
    print(f"Baked {len(hexes)} accurate land hexes -> {OUT_PATH} ({OUT_PATH.stat().st_size // 1024} KB)")

if __name__ == "__main__":
    main()
