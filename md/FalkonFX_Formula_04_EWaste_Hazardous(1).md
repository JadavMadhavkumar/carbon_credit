**FALKON FUTURE X**

Carbon Credit Formula Reference Series

**Document 4 of 6**

**E-Waste & Hazardous**

Electronic Waste & Hazardous Material Carbon Credit Formulas

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

**4. E-Waste & Hazardous Waste Formulas**

E-waste credits combine two value streams: (1) avoided CO₂e from
recovering precious/rare-earth metals that would otherwise require
virgin mining, and (2) avoided harm credits from safe containment of
toxics (mercury, lead, cadmium).

**4.1 E-Waste Composite Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>E-Waste Credit</strong></p>
<p><strong>C_ew = W × (EF_metal + EF_hazard) × R × M × V ×
L</strong></p></td>
</tr>
</tbody>
</table>

| **Variable**  | **Unit**   | **Description**                             |
|---------------|------------|---------------------------------------------|
| **C_ew**      | kg CO₂e    | Total e-waste credits                       |
| **W**         | kg         | Weight of device submitted                  |
| **EF_metal**  | kg CO₂e/kg | Avoided emissions from recovered metals     |
| **EF_hazard** | kg CO₂e/kg | Equivalenced hazard avoidance value         |
| **R**         | 0.0–1.0    | Recovery efficiency of certified e-recycler |
| **M**         | 4.0–10.0   | Credit multiplier (device-specific)         |
| **V**         | 0.85–1.00  | Minimum AI verification required            |
| **L**         | 0.90–1.10  | Locality factor                             |

**4.2 Device-Specific Emission Factors**

| **Device** | **EF_metal** | **EF_hazard** | **Total EF** | **Recov. R** | **Mult. M** |
|----|----|----|----|----|----|
| Smartphone | 32.0 | 13.0 | 45.0 | 0.85 | 8.0 |
| Laptop | 220.0 | 100.0 | 320.0 | 0.88 | 10.0 |
| Desktop PC | 180.0 | 80.0 | 260.0 | 0.85 | 9.0 |
| Tablet | 85.0 | 35.0 | 120.0 | 0.85 | 8.5 |
| CRT Monitor | 30.0 | 40.0 | 70.0 | 0.80 | 5.0 |
| LCD Monitor | 45.0 | 25.0 | 70.0 | 0.82 | 5.5 |
| Li-ion Battery | 8.0 | 4.6 | 12.6 | 0.90 | 7.0 |
| CFL Bulb | 1.5 | 3.8 | 5.3 | 0.95 | 4.0 |
| Refrigerator | 90.0 | 40.0 | 130.0 | 0.75 | 6.0 |
| Air Conditioner | 120.0 | 60.0 | 180.0 | 0.78 | 7.0 |

**4.3 Precious Metal Recovery Sub-Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Metal Recovery Sub-Credit</strong></p>
<p><strong>C_metal = Σ (mass_i × EF_primary_i × η_i) for each metal
i</strong></p></td>
</tr>
</tbody>
</table>

| **Metal** | **Mass/smartphone** | **EF primary (kg CO₂e/kg)** | **Recovery η** |
|----|----|----|----|
| Gold (Au) | 30 mg | 35,000 | 0.97 |
| Silver (Ag) | 160 mg | 120 | 0.95 |
| Copper (Cu) | 15,000 mg | 4.1 | 0.93 |
| Palladium (Pd) | 5 mg | 75,000 | 0.96 |
| Cobalt (Co) | 5,000 mg | 9.0 | 0.88 |

**4.4 Hazardous Waste — Safe Disposal Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Hazardous Credit</strong></p>
<p><strong>C_haz = W × EF_haz × D × V × L</strong></p></td>
</tr>
</tbody>
</table>

> *Only facility QR scan (V = 1.00) is accepted for hazardous waste.
> Self-reporting is not permitted.*

| **Hazardous Type** | **EF_haz (kg CO₂e/kg)** | **Destruction η** | **Notes** |
|----|----|----|----|
| Paints & solvents | 2.2 | 0.95 | VOC emissions avoided |
| Medicines / pharma | 3.1 | 0.99 | Water contamination avoided |
| Pesticides | 4.8 | 0.98 | High toxicity harm factor |
| Motor oil | 3.3 | 0.96 | Soil contamination avoided |
| Non-Li-ion batteries | 5.2 | 0.97 | Lead/cadmium containment |
| Fluorescent tubes | 5.3 | 0.99 | Mercury containment |
