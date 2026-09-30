#!/usr/bin/env python3
"""Generate maps/world.pmtiles from Natural Earth 110m countries (offline basemap)."""
import json, math, sqlite3, os, sys, time
import numpy as np
from shapely.geometry import shape, box, mapping, Polygon, MultiPolygon, GeometryCollection
from shapely.ops import transform as shp_transform
from shapely.strtree import STRtree
import mapbox_vector_tile

EXTENT = 4096
MAXZ = int(sys.argv[1]) if len(sys.argv) > 1 else 8
GEO = "/tmp/opencode/ne_50m.geojson"
MB = "/tmp/opencode/world.mbtiles"
OUT = "/home/xander/Documents/portfolio/mazzaroth/maps/world.pmtiles"

def merc_y(lat_deg):
    r = math.radians(max(min(lat_deg, 85.05112878), -85.05112878))
    return math.log(math.tan(math.pi / 4 + r / 2))

def lat_of_merc(y):
    return math.degrees(math.atan(math.sinh(y)))

def tile_bounds(x, y, z):
    n = 2 ** z
    w = x / n * 360.0 - 180.0
    e = (x + 1) / n * 360.0 - 180.0
    north = lat_of_merc(math.pi * (1 - 2 * y / n))
    south = lat_of_merc(math.pi * (1 - 2 * (y + 1) / n))
    return w, south, e, north

def polys_of(g):
    if g.is_empty:
        return []
    if isinstance(g, Polygon):
        return [g] if g.area > 0 else []
    if isinstance(g, (MultiPolygon, GeometryCollection)):
        out = []
        for part in g.geoms:
            out.extend(polys_of(part))
        return out
    return []

def to_px_geom(g, w, s, e, north):
    my_n = merc_y(north)
    my_s = merc_y(south)  # noqa: F821 - set below
    return None

def make_transform(w, south, e, north):
    my_n = merc_y(north)
    my_s = merc_y(south)
    span_x = (e - w) or 1e-9
    span_y = (my_n - my_s) or 1e-9

    def fn(xs, ys):
        xs = np.asarray(xs, dtype=float)
        ys = np.asarray(ys, dtype=float)
        ys_clipped = np.clip(ys, -85.05112878, 85.05112878)
        px = (xs - w) / span_x * EXTENT
        py = (my_n - np.log(np.tan(np.pi / 4 + np.radians(ys_clipped) / 2))) / span_y * EXTENT
        return px, py

    return fn

def main():
    t0 = time.time()
    geo = json.load(open(GEO))
    names, geoms = [], []
    for f in geo["features"]:
        g = shape(f["geometry"])
        if not g.is_valid:
            g = g.buffer(0)
        if not g.is_empty:
            geoms.append(g)
            names.append(f["properties"].get("NAME") or "")
    reps = [g.representative_point() for g in geoms]
    tree = STRtree(geoms)
    print(f"loaded {len(geoms)} country geoms", flush=True)

    if os.path.exists(MB):
        os.remove(MB)
    con = sqlite3.connect(MB)
    con.execute("CREATE TABLE metadata (name TEXT, value TEXT)")
    con.execute("CREATE TABLE tiles (zoom_level INTEGER, tile_column INTEGER, tile_row INTEGER, tile_data BLOB)")
    meta = {
        "name": "Mazzaroth Sovereign Basemap",
        "type": "baselayer",
        "version": "1.1.0",
        "description": "Natural Earth 50m countries, offline vector basemap",
        "format": "pbf",
        "bounds": "-180,-85.051129,180,85.051129",
        "center": "0,20,2",
        "minzoom": "0",
        "maxzoom": str(MAXZ),
        "json": json.dumps({"vector_layers": [
            {"id": "countries", "fields": {"admin": "String"}},
            {"id": "labels", "fields": {"name": "String"}},
        ]}),
    }
    con.executemany("INSERT INTO metadata VALUES (?,?)", meta.items())

    total = 0
    for z in range(0, MAXZ + 1):
        n = 2 ** z
        zc = 0
        for tx in range(n):
            for ty in range(n):
                w, s, e, nt = tile_bounds(tx, ty, z)
                bbox = box(w, s, e, nt)
                idxs = tree.query(bbox)
                feats = []
                label_feats = []
                tf = make_transform(w, s, e, nt)
                for i in idxs:
                    cand = geoms[int(i)]
                    rp = reps[int(i)]
                    if w <= rp.x <= e and s <= rp.y <= nt:
                        label_feats.append({
                            "geometry": mapping(shp_transform(tf, rp)),
                            "properties": {"name": names[int(i)]},
                        })
                    if not cand.intersects(bbox):
                        continue
                    clipped = cand.intersection(bbox)
                    for poly in polys_of(clipped):
                        px = shp_transform(tf, poly)
                        if px.area < 0.4:
                            continue
                        px = px.simplify(0.4)
                        if px.is_empty:
                            continue
                        feats.append({
                            "geometry": mapping(px),
                            "properties": {"admin": names[int(i)]},
                        })
                layers = [{"name": "countries", "features": feats}]
                if label_feats:
                    layers.append({"name": "labels", "features": label_feats})
                if not feats and not label_feats:
                    continue
                buf = mapbox_vector_tile.encode(layers, default_options={"y_coord_down": True})
                tms_y = n - 1 - ty
                con.execute("INSERT INTO tiles VALUES (?,?,?,?)", (z, tx, tms_y, buf))
                zc += 1
                total += 1
        con.commit()
        print(f"z{z}: {zc} tiles ({time.time()-t0:.1f}s)", flush=True)

    con.commit()
    con.close()
    from pmtiles.convert import mbtiles_to_pmtiles
    mbtiles_to_pmtiles(MB, OUT, MAXZ)
    print(f"done: {OUT} {os.path.getsize(OUT)} bytes, {total} tiles, {time.time()-t0:.1f}s")

if __name__ == "__main__":
    main()
