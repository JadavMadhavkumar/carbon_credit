**FALKON FUTURE X**

Carbon Credit Formula Reference Series

**Document 5 of 6**

**Textiles & Special Streams**

Clothing, Tyres, Furniture & Specialised Waste Formulas

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

**5. Textiles & Special Stream Formulas**

Textiles, tyres, and bulky items follow different credit pathways
depending on whether items are reused, recycled, or converted to energy.
Reuse always generates the highest credits because it avoids both new
production AND waste processing.

**5.1 Textile Reuse Credit**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Textile Reuse</strong></p>
<p><strong>C_tex_reuse = W × EF_production × RR × M_reuse × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

**5.2 Textile Recycling Credit**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Textile Recycling</strong></p>
<p><strong>C_tex_rec = W × EF_fibre × η_recycle × M_recycle × V × L ×
Q</strong></p></td>
</tr>
</tbody>
</table>

**5.3 Textile Energy Recovery Credit**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Textile Energy Recovery</strong></p>
<p><strong>C_tex_energy = W × CV × EF_grid × η_plant × M_energy × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit** | **Description** |
|----|----|----|
| **EF_production** | kg CO₂e/kg | Emissions to produce new equivalent garment |
| **RR** | 0.0–1.0 | Reuse rate — % of donated items actually reused |
| **M_reuse** | 2.5–4.0 | Reuse multiplier (higher than recycling) |
| **EF_fibre** | kg CO₂e/kg | Emissions for virgin fibre production |
| **η_recycle** | 0.0–1.0 | Fibre recovery efficiency at recycling plant |
| **CV** | MJ/kg | Calorific value of textile for energy recovery |
| **EF_grid** | kg CO₂e/kWh | Grid emission factor — energy displaced |
| **η_plant** | 0.0–1.0 | Energy plant efficiency (typically 0.30–0.35) |

**5.4 Textile-Specific Factors**

| **Textile** | **EF Prod.** | **EF Fibre** | **CV (MJ/kg)** | **Best Pathway** | **M_reuse** |
|----|----|----|----|----|----|
| Cotton | 6.80 | 1.80 | 17.0 | Reuse / Compost | 3.0 |
| Wool | 90.0 | 4.20 | 22.0 | Reuse | 4.0 |
| Polyester | 9.52 | 2.85 | 32.0 | Recycle / Energy | 2.5 |
| Nylon | 7.93 | 3.10 | 30.0 | Recycle / Energy | 2.5 |
| Denim | 8.10 | 2.20 | 18.0 | Reuse / Recycle | 3.5 |
| Blended | 7.50 | 2.50 | 24.0 | Energy recovery | 2.0 |

**5.5 Tyre Credit Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Tyre Credit</strong></p>
<p><strong>C_tyre = N × W_avg × EF_tyre × P_factor × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable** | **Unit**   | **Description**                                   |
|--------------|------------|---------------------------------------------------|
| **N**        | units      | Number of tyres submitted                         |
| **W_avg**    | kg/tyre    | Average weight per tyre                           |
| **EF_tyre**  | kg CO₂e/kg | Emission factor — pathway-specific                |
| **P_factor** | 1.2–3.5    | Pathway: retreading=3.5, crumb=1.8, pyrolysis=1.2 |

| **Tyre Type** | **W_avg (kg)** | **EF_tyre** | **Retreading Credit** | **Crumb Credit** |
|----|----|----|----|----|
| Car passenger | 8 | 2.80 | 9.80 kg CO₂e | 4.03 kg CO₂e |
| SUV / 4x4 | 12 | 2.80 | 11.76 kg CO₂e | 4.84 kg CO₂e |
| Truck | 50 | 3.20 | 56.0 kg CO₂e | 23.0 kg CO₂e |
| Motorcycle | 3 | 2.60 | 2.73 kg CO₂e | 1.12 kg CO₂e |

**5.6 Bulky Item (Furniture) Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Bulky Reuse Credit</strong></p>
<p><strong>C_bulk = W × EF_material_mix × RU × M_bulk × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

> *Bulky items require facility QR or certified reuse centre scan — no
> self-report accepted due to high material composition variability.*

| **Item**        | **W (kg)** | **EF mix** | **Reuse %** | **M_bulk** | **Est. Credit** |
|-----------------|------------|------------|-------------|------------|-----------------|
| Wooden wardrobe | 40         | 2.1        | 65%         | 2.5        | 136 kg CO₂e     |
| Steel almirah   | 35         | 1.8        | 70%         | 2.0        | 88 kg CO₂e      |
| Sofa            | 25         | 3.2        | 50%         | 2.0        | 80 kg CO₂e      |
| Dining table    | 20         | 2.4        | 60%         | 2.5        | 72 kg CO₂e      |
| Mattress        | 15         | 2.8        | 40%         | 1.5        | 25 kg CO₂e      |
