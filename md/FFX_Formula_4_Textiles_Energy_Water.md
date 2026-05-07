**FALKON FUTURE X**

Carbon Intelligence Platform

**Formula Reference — Doc 4 of 5**

**Textiles, Water & Energy**

Carbon Credit Calculation Formulas

<table>
<colgroup>
<col style="width: 50%" />
<col style="width: 50%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Organization</strong></p>
<p>Falkon Future X</p></td>
<td><p><strong>Version</strong></p>
<p>v1.0 — April 2026</p></td>
</tr>
</tbody>
</table>

**Textiles, Water Conservation & Household Energy Saving Formulas**

Beyond physical waste, Falkon Future X credits three additional
household behaviour categories: textile diversion (clothing, fabric),
water conservation (reduced hot water use), and household energy
reduction (behavioural changes tracked via smart meter or utility bill
upload). These formulas extend the platform beyond traditional waste
management into full household carbon accounting.

**1. Textiles Formulas**

The global fashion industry produces 10% of global CO₂ emissions.
Clothing has high embodied carbon — cotton requires ~10,000 litres of
water per kg and synthetic fabrics are derived from fossil fuels.
Donating, repairing, or textile-recycling garments generates credits by
avoiding landfill and displacing new garment production.

**1.1 Master Textile Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) = W ×
EF_textile × TM × QF × VS × LF</strong></p>
<p>W = Weight of textile (kg)</p>
<p>EF_textile = Embodied carbon of fibre type (kg CO₂e/kg)</p>
<p>TM = Treatment multiplier (donation &gt; repair &gt; recycle &gt;
landfill divert)</p>
<p>QF = Quality factor (1.00 wearable | 0.70 worn | 0.40 damaged)</p>
<p>VS = Verification score (0.50 | 0.85 | 1.00)</p>
<p>LF = Locality factor</p></td>
</tr>
</tbody>
</table>

**1.2 Cotton & Natural Fibre Clothing — Donation / Reuse**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits = W × 6.80 × 3.00 ×
QF × VS × LF</strong></p>
<p>EF = 6.80 kg CO₂e/kg (cotton lifecycle: farming, ginning, weaving,
dye)</p>
<p>TM = 3.00 (donation/reuse avoids entire new garment production)</p>
<p>QF = 1.00 wearable, 0.70 worn but intact, 0.40 heavily worn</p>
<p>Example: 3 kg cotton clothes donated, AI-verified, wearable:</p>
<p>3 × 6.80 × 3.00 × 1.00 × 0.85 × 1.00 = 52.02 kg CO₂e</p></td>
</tr>
</tbody>
</table>

**1.3 Synthetic Fabrics (Polyester, Nylon, Acrylic) — Textile
Recycling**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits = W × 9.50 × 2.50 ×
QF × VS × LF</strong></p>
<p>EF = 9.50 kg CO₂e/kg (fossil-fuel derived fibres, higher embodied
carbon)</p>
<p>TM = 2.50 (mechanical recycling into fibrefill or new fibre)</p>
<p>QF = 1.00 clean | 0.60 blended/difficult to separate</p>
<p>Note: Blended fabrics (cotton/polyester mix) use lower of the two EF
values</p></td>
</tr>
</tbody>
</table>

**1.4 Leather & Leather Goods**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits = W × 17.0 × 2.00 ×
QF × VS × LF</strong></p>
<p>EF = 17.0 kg CO₂e/kg (cattle raising + tanning process, highest
textile EF)</p>
<p>TM = 2.00 (reuse/repair only; leather is difficult to recycle)</p>
<p>QF = 1.00 intact, 0.50 cracked/delaminated</p>
<p>Applies to: shoes, bags, belts, jackets, wallets</p></td>
</tr>
</tbody>
</table>

**1.5 Textile Repair Credit (Bonus)**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Repair Credit = W_extended
× EF × 0.50 × VS</strong></p>
<p>W_extended = Estimated weight of garment whose life was extended by
repair</p>
<p>EF = Fibre type emission factor</p>
<p>0.50 = Repair multiplier (half the avoidance credit vs full
donation)</p>
<p>Note: Claimed via receipt from tailor/cobbler — photo upload
accepted</p>
<p>Example: Shoe repair extends 0.8 kg leather shoes:</p>
<p>0.8 × 17.0 × 0.50 × 0.85 = 5.78 kg CO₂e</p></td>
</tr>
</tbody>
</table>

**2. Water Conservation Formulas**

Hot water production is a significant household energy expense. In
India, water heating accounts for 15–25% of residential electricity
consumption. Reducing hot water use reduces electricity (or gas)
consumption, which reduces CO₂ emissions from the grid.

**2.1 Hot Water Reduction Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) = ΔV ×
ρ_water × Cp × ΔT × (1/η) × EF_grid / 3,600,000</strong></p>
<p>ΔV = Volume of hot water saved (litres)</p>
<p>ρ_water = Density of water (1.0 kg/L)</p>
<p>Cp = Specific heat capacity of water (4,186 J/kg·°C)</p>
<p>ΔT = Temperature rise (target temp − inlet temp, typically 35°C)</p>
<p>η = Heater efficiency (0.85 electric | 0.70 gas geyser)</p>
<p>EF_grid = Grid emission factor (kg CO₂e/kWh) — state specific</p>
<p>3,600,000 = Joules per kWh (conversion constant)</p>
<p>Simplified (India, electric geyser, 40°C rise, avg grid EF=0.82):</p>
<p><strong>► Credits (kg CO₂e) = ΔV × 0.0565</strong></p>
<p>Example: Save 50 L/month shower water: 50 × 0.0565 = 2.83 kg
CO₂e/month</p></td>
</tr>
</tbody>
</table>

**2.2 Rainwater Harvesting Credit**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) =
V_harvested × EF_treatment</strong></p>
<p>V_harvested = Volume collected and used (kL = kilolitres)</p>
<p>EF_treatment = 0.35 kg CO₂e/kL (avoided municipal water treatment
energy)</p>
<p>Verified via: smart meter photo upload or municipality
confirmation</p>
<p>Example: 1,000 L (1 kL) harvested monthly: 1 × 0.35 = 0.35 kg
CO₂e</p></td>
</tr>
</tbody>
</table>

**3. Household Energy Saving Formulas**

Households that demonstrate reduction in electricity or LPG consumption
compared to their 12-month baseline earn energy-saving credits. Baseline
is established at onboarding from 12 months of utility bill uploads.

**3.1 Electricity Reduction Formula**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) = ΔkWh ×
EF_state_grid</strong></p>
<p>ΔkWh = kWh_baseline_monthly − kWh_actual_monthly</p>
<p>EF_state_grid = State-specific grid emission factor (kg CO₂e/kWh)</p>
<p>State Grid Emission Factors (MoEFCC 2023):</p>
<p>IN-MH: 0.82 | IN-DL: 0.87 | IN-KA: 0.74 | IN-TN: 0.78</p>
<p>IN-GJ: 0.88 | IN-RJ: 0.92 | IN-UP: 0.96 | IN-WB: 0.99</p>
<p>Example: Delhi household saves 40 kWh/month:</p>
<p>40 × 0.87 = 34.8 kg CO₂e/month = 417.6 kg CO₂e/year</p></td>
</tr>
</tbody>
</table>

**3.2 LPG / CNG Cooking Gas Reduction**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) =
ΔV_cylinders × W_cylinder × EF_LPG × CF_combustion</strong></p>
<p>ΔV_cylinders = Cylinders saved per month vs baseline</p>
<p>W_cylinder = Net weight of gas per cylinder (14.2 kg standard
India)</p>
<p>EF_LPG = 3.015 kg CO₂e/kg LPG (combustion, IPCC)</p>
<p>CF_combustion = 0.99 (combustion efficiency factor)</p>
<p>Simplified: Credits = ΔV_cylinders × 42.33 kg CO₂e per cylinder
saved</p>
<p>Example: 1 cylinder saved/month = 42.33 kg CO₂e/month</p></td>
</tr>
</tbody>
</table>

**3.3 Solar Panel Energy Generation (Household Rooftop)**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td style="text-align: center;"><p><strong>► Credits (kg CO₂e) =
kWh_generated × EF_state_grid × SF</strong></p>
<p>kWh_generated = kWh produced by rooftop solar (from inverter
data/upload)</p>
<p>EF_state_grid = State grid displacement factor</p>
<p>SF = Solar displacement factor (0.85 — accounts for curtailment)</p>
<p>Example: 300 kWh/month solar in Maharashtra:</p>
<p>300 × 0.82 × 0.85 = 209.1 kg CO₂e/month = 2,509 kg CO₂e/year</p>
<p>= 2.51 tonnes CO₂e/year</p></td>
</tr>
</tbody>
</table>

**4. Textile Emission Factor Reference**

| **Textile Type** | **EF (kg CO₂e/kg)** | **Treatment Mult.** | **Accepted Method** |
|----|----|----|----|
| Cotton (woven) | 6.80 | 3.00 | Donation / reuse |
| Organic cotton | 4.20 | 3.00 | Donation / reuse |
| Polyester | 9.52 | 2.50 | Mechanical recycling |
| Nylon / Polyamide | 9.80 | 2.50 | Mechanical recycling |
| Acrylic | 7.80 | 2.00 | Downcycling / fibrefill |
| Wool | 36.0 | 3.50 | Donation / reuse (highest EF!) |
| Silk | 35.0 | 3.50 | Donation / reuse |
| Linen (flax) | 3.50 | 2.50 | Donation / natural fibre recycling |
| Leather (cattle hide) | 17.0 | 2.00 | Repair / reuse only |
| Cotton/Poly blend | 6.80 | 2.00 | Blended recycle (lower rate) |

*— End of Document 4 of 5 —*
