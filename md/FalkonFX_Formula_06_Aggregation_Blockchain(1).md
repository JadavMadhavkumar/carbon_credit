**FALKON FUTURE X**

Carbon Credit Formula Reference Series

**Document 6 of 6**

**Aggregation & Blockchain**

Household → Municipal → National Credit Aggregation & On-Chain Formulas

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Organisation</strong></p>
<p>Falkon Future X</p></td>
<td><p><strong>Version</strong></p>
<p>1.0 — April 2026</p></td>
</tr>
</tbody>
</table>

**6. Aggregation, Normalisation & Blockchain Formulas**

This document defines how household credits aggregate to municipal and
national levels, how credits are normalised for fair comparison, and how
they convert to on-chain token units for blockchain recording.

**6.1 Household Monthly Total**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Household Monthly Aggregate</strong></p>
<p><strong>C_hh = Σ C_i for all verified submissions i in the
period</strong></p></td>
</tr>
</tbody>
</table>

**6.2 Municipal Aggregation Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Municipality Monthly Total</strong></p>
<p><strong>C_mun = Σ (C_hh_j × A_j) for all households j in
municipality</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit** | **Description** |
|----|----|----|
| **C_mun** | kg CO₂e | Total municipal carbon credits for reporting period |
| **C_hh_j** | kg CO₂e | Credits from household j |
| **A_j** | 0.80–1.00 | Audit compliance factor (penalises flagged submissions) |

**6.3 National Aggregation**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>National Monthly Total</strong></p>
<p><strong>C_nat = Σ C_mun_k for all municipalities k in the
country</strong></p></td>
</tr>
</tbody>
</table>

**6.4 Per-Capita Normalisation**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Per-Capita Credit Index (PCI)</strong></p>
<p><strong>PCI = (C_mun / Pop_mun) × 1000 [kg CO₂e per 1,000
residents]</strong></p></td>
</tr>
</tbody>
</table>

**6.5 Participation Rate Bonus Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Participation-Adjusted Score</strong></p>
<p><strong>C_adj = C_mun × (1 + 0.20 × PR) [PR = active_hh /
total_hh]</strong></p></td>
</tr>
</tbody>
</table>

> *PR = active_households ÷ total_registered_households. An 'active'
> household is one with at least 1 verified submission in the reporting
> period. Maximum 20% bonus at PR = 1.0.*

**6.6 Blockchain Token Scaling Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>On-Chain Token Amount</strong></p>
<p><strong>token_units = round(C_kg × 1000) [1 token_unit = 1 gram
CO₂e]</strong></p></td>
</tr>
</tbody>
</table>

| **Description** | **kg CO₂e** | **token_units (on-chain)** | **Human Display** |
|----|----|----|----|
| Tiny submission | 0.009 | 9 | 9 g CO₂e |
| Typical submission | 9.31 | 9,310 | 9.31 kg CO₂e |
| Large e-waste | 320.0 | 320,000 | 320.0 kg CO₂e |
| 1 full Carbon Credit | 1,000.0 | 1,000,000 | 1.000 CC token |
| Monthly household | ~150.0 | ~150,000 | ~0.150 CC tokens |
| Monthly municipality | ~50,000 | 50,000,000 | ~50 CC tokens |

**6.7 Fraud Penalty Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Credit with Fraud Penalty</strong></p>
<p><strong>C_final = C_calculated × (1 - P) [P = fraud penalty
fraction]</strong></p></td>
</tr>
</tbody>
</table>

| **Fraud Signal** | **Penalty P** | **Action** |
|----|----|----|
| AI category mismatch | 0.70 (discard) | Submission rejected, 30% admin fee |
| Weight outlier (\>3σ) | 0.40 | Credits reduced 40%, flagged for review |
| Duplicate photo detected | 1.00 | Full rejection, account warning |
| 3rd violation in 30 days | 1.00 | Account suspended, credits frozen |
| Fake facility QR | 1.00 | Suspended, legal referral |

**6.8 Complete Formula Reference — All 6 Documents**

| **Formula** | **Doc** | **Application** |
|----|----|----|
| C = W × EF × M × V × L × Q | 1 — Master | Universal — all waste types |
| C_comp = W × EF_landfill × (1-MC) × V × L | 2 — Organic | Composting / landfill diversion |
| C_bio = W × VS × BMP × 0.716 × EF_grid × V × L | 2 — Organic | Biogas / anaerobic digestion |
| C_uco = V_oil × D × EF_diesel × CE × V × L | 2 — Organic | Used cooking oil → biodiesel |
| C_rec = W × (EF_v - EF_r) × η × M × V × L × Q | 3 — Recyclables | Paper, plastic, glass, metals |
| C_ew = W × (EF_metal + EF_hazard) × R × M × V × L | 4 — E-Waste | Electronic devices |
| C_metal = Σ (mass_i × EF_i × η_i) | 4 — E-Waste | Precious metal recovery sub-credit |
| C_haz = W × EF_haz × D × V × L | 4 — Hazardous | Safe hazardous disposal |
| C_tex = W × EF_prod × RR × M × V × L | 5 — Textiles | Textile reuse / recycling |
| C_tyre = N × W_avg × EF_tyre × P × V × L | 5 — Tyres | Tyre retreading / crumbing |
| C_bulk = W × EF_mix × RU × M_bulk × V × L | 5 — Bulky | Furniture & bulky items |
| C_hh = Σ C_i | 6 — Aggregation | Household monthly total |
| C_mun = Σ (C_hh_j × A_j) | 6 — Aggregation | Municipal monthly total |
| C_nat = Σ C_mun_k | 6 — Aggregation | National total |
| PCI = (C_mun / Pop) × 1000 | 6 — Aggregation | Per-capita credit index |
| C_adj = C_mun × (1 + 0.20 × PR) | 6 — Aggregation | Participation bonus adjustment |
| token_units = round(C_kg × 1000) | 6 — Blockchain | On-chain token integer scaling |
| C_final = C_calc × (1 - P) | 6 — Fraud | Fraud penalty deduction |

*— End of Falkon Future X Carbon Credit Formula Reference Series —*
