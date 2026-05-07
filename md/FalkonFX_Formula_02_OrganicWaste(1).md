**FALKON FUTURE X**

Carbon Credit Formula Reference Series

**Document 2 of 6**

**Organic Waste**

Food Waste, Composting & Biogas Carbon Credit Formulas

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

**2. Organic Waste Carbon Credit Formulas**

Organic waste is ~50–60% of Indian household waste by weight. Credits
arise from two pathways: diverting waste from landfill (avoiding
methane) and converting it to biogas or compost (displacing fossil fuel
and synthetic fertiliser).

**2.1 Composting (Landfill Diversion) Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Composting Credit</strong></p>
<p><strong>C_comp = W × EF_landfill × (1 - MC) × V × L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit** | **Description** |
|----|----|----|
| **C_comp** | kg CO₂e | Credits from composting pathway |
| **W** | kg | Total fresh weight of organic waste |
| **EF_landfill** | kg CO₂e/kg | Methane emission factor for landfill (IPCC Tier 1) |
| **MC** | 0.00–0.90 | Moisture content fraction — reduces dry weight |
| **V** | 0.50–1.00 | Verification score |
| **L** | 0.90–1.10 | Locality factor |

**2.2 Biogas / Anaerobic Digestion Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Biogas Credit</strong></p>
<p><strong>C_bio = W × VS × BMP × 0.716 × EF_grid × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit**     | **Description**                           |
|--------------|--------------|-------------------------------------------|
| **C_bio**    | kg CO₂e      | Credits from biogas energy displacement   |
| **W**        | kg           | Fresh weight of organic waste             |
| **VS**       | kg VS/kg     | Volatile Solids content fraction          |
| **BMP**      | m³ CH₄/kg VS | Biochemical Methane Potential             |
| **0.716**    | kg CH₄/m³    | Density of methane at standard conditions |
| **EF_grid**  | kg CO₂e/kWh  | Grid emission factor — energy displaced   |
| **V**        | 0.50–1.00    | Verification score                        |
| **L**        | 0.90–1.10    | Locality factor                           |

**2.3 Used Cooking Oil (UCO) — Biodiesel Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>UCO Biodiesel Credit</strong></p>
<p><strong>C_uco = V_oil × D_oil × EF_diesel × CE × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit** | **Description** |
|----|----|----|
| **C_uco** | kg CO₂e | Credits from UCO → biodiesel pathway |
| **V_oil** | litres | Volume of used cooking oil submitted |
| **D_oil** | kg/litre | Density of UCO (~0.91–0.92 kg/L) |
| **EF_diesel** | kg CO₂e/litre | Fossil diesel emission factor (2.68) |
| **CE** | 0.0–1.0 | Conversion efficiency of biodiesel plant (typ. 0.87) |
| **V** | 0.50–1.00 | Verification score |
| **L** | 0.90–1.10 | Locality factor |

**2.4 Organic Waste Emission Factors**

| **Sub-type** | **EF Landfill** | **Moisture %** | **BMP (m³CH₄/kgVS)** | **Best Pathway** |
|----|----|----|----|----|
| Vegetable scraps | 0.57 | 80% | 0.33 | Composting |
| Fruit peels | 0.53 | 85% | 0.29 | Composting |
| Cooked food waste | 1.12 | 72% | 0.48 | Biogas |
| Meat & dairy | 2.85 | 65% | 0.62 | Biogas |
| Garden / yard waste | 0.46 | 60% | 0.25 | Composting |
| Rice / grains | 0.89 | 70% | 0.41 | Biogas |
| Used cooking oil | 2.47 | 2% | 0.85 | Biodiesel |

**2.5 Worked Examples**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Example A: 5 kg Cooked Food — Biogas — AI Verified —
Delhi</strong></p>
<p>W=5.0 kg | VS=0.28 | BMP=0.48 | 0.716 density | EF_grid=0.82 | V=0.85
| L=1.02</p>
<p>C_bio = 5.0 × 0.28 × 0.48 × 0.716 × 0.82 × 0.85 × 1.02 = 0.39 kg
CO₂e</p></td>
</tr>
</tbody>
</table>

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Example B: 2 litres UCO — Facility QR —
Karnataka</strong></p>
<p>V_oil=2.0 L | D=0.915 | EF_diesel=2.68 | CE=0.87 | V=1.00 |
L=0.94</p>
<p>C_uco = 2.0 × 0.915 × 2.68 × 0.87 × 1.00 × 0.94 = 4.02 kg
CO₂e</p></td>
</tr>
</tbody>
</table>
