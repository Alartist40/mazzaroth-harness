# SOURCING.md — Legal Knowledge Sourcing Guide

This document outlines legal, copyright-clean sources for content ingested into the Librarian Box.

## 1. Non-Negotiable Provenance Policy

Ingestion will reject any document missing source, publisher, license, or retrieval date.
Valid license tags:
- `public-domain`: Works where copyright has expired (pre-1929 in the US) or US Federal Government works (17 U.S.C. § 105).
- `cc0`: Creative Commons zero dedicated works.
- `cc-by` / `cc-by-sa`: Permissive Creative Commons requiring attribution.
- `gutenberg`: Works from Project Gutenberg covered by their public domain terms.
- `personal-use-only`: Documents for private use; never bundled in redistributable releases.

## 2. Approved Public Domain Repositories

| Domain | Content Description | License | Source / Reference |
|---|---|---|---|
| **Survival & Preparedness** | US Army FM 21-76 / FM 3-05.70, FM 4-25.11 (First Aid), FEMA Emergency Handbooks | Public Domain (US Gov) | [govinfo.gov](https://www.govinfo.gov), [archive.org](https://archive.org) |
| **Agriculture & Flora** | USDA Plant Identification & Foraging Manuals, Forest Service Guides | Public Domain (US Gov) | [usda.gov](https://www.usda.gov) |
| **Health & Medical** | MedlinePlus Health Topics, Military Medical Field Manuals | Public Domain (NLM/USG) | [medlineplus.gov](https://medlineplus.gov) |
| **Classical Literature & Science** | Project Gutenberg Texts (Pre-1929 works: Shakespeare, Darwin, Newton, Faraday, Plato) | Public Domain | [gutenberg.org](https://www.gutenberg.org) |
| **Scripture & Philosophy** | King James Version (KJV), World English Bible (WEB), ASV 1901, Young's Literal Translation (YLT), Brenton Septuagint | Public Domain | Verified PD Bible text sources |
| **Cartography & Maps** | OpenStreetMap regional map extractions in `.pmtiles` vector format | ODbL / Open Data | [protomaps.com](https://protomaps.com), [overturemaps.org](https://overturemaps.org) |

## 3. Copyright Caution List

- **Modern Bible Translations** (NIV, ESV, NKJV, NLT) are strictly proprietary and copyrighted. Do NOT ingest unless licensed for personal use (`personal-use-only`).
- **Where There Is No Doctor / Hesperian**: Free for personal non-commercial use, but NOT redistributable. Keep in personal storage only.
