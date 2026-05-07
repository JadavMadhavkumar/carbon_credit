**FALKON FUTURE X**

Carbon Credit Formula Reference Series

**Document 1 of 6**

**Master Formula**

Universal Carbon Credit Calculation for All Waste Types

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

**1. The Universal Carbon Credit Formula**

This is the master formula underpinning all carbon credit calculations
on the Falkon Future X platform. All six category-specific formulas are
derived from this single equation.

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>MASTER FORMULA — All Waste Categories</strong></p>
<p><strong>C = W × EF × M × V × L × Q</strong></p></td>
</tr>
</tbody>
</table>

**1.1 Variable Definitions**

| **Variable** | **Unit** | **Description** |
|----|----|----|
| **C** | kg CO₂e | Carbon credits earned (kg CO₂ equivalent) |
| **W** | kg | Dry weight of waste submitted by household |
| **EF** | kg CO₂e/kg | Emission Factor — CO₂e avoided per kg of this waste type |
| **M** | 1.0 – 10.0 | Credit Multiplier — ecosystem value adjustment |
| **V** | 0.50 – 1.00 | Verification Score — confidence in reported data |
| **L** | 0.90 – 1.10 | Locality Factor — state/city grid emission adjustment |
| **Q** | 0.60 – 1.00 | Quality Factor — condition/purity of waste submitted |

**1.2 Converting to Tradeable Carbon Credits**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Unit Conversion</strong></p>
<p><strong>1 Carbon Credit (CC) = 1 tonne CO₂e = 1,000 kg
CO₂e</strong></p></td>
</tr>
</tbody>
</table>

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>Tradeable Tokens</strong></p>
<p><strong>CC_tokens = C ÷ 1000 (C is in kg CO₂e)</strong></p></td>
</tr>
</tbody>
</table>

**1.3 Verification Score (V)**

| **Verification Mode** | **Score (V)** | **Description** |
|----|----|----|
| Self-reported (no photo) | 0.50 | User declares weight only |
| Photo — AI inconclusive | 0.65 | Photo submitted but unclear |
| Photo — AI verified | 0.85 | AI confirms category & approx. weight |
| Facility QR scan | 1.00 | Weighed & scanned at certified facility |
| Govt-certified auditor | 1.00 | Third-party MRV confirmation |

**1.4 Quality Factor (Q)**

| **Condition** | **Q** | **Examples** |
|----|----|----|
| Clean / source-separated | 1.00 | Washed PET, dry cardboard, sorted e-waste |
| Lightly contaminated | 0.92 | Paper with minor moisture, plastic with labels |
| Mixed / unsorted | 0.80 | Mixed plastics, unsorted paper |
| Severely contaminated | 0.60 | Landfill divert credit only |

**1.5 Locality Factor (L) — India State Grid Data**

| **State**   | **Code** | **L** | **State**      | **Code** | **L** |
|-------------|----------|-------|----------------|----------|-------|
| Maharashtra | IN-MH    | 0.98  | Tamil Nadu     | IN-TN    | 0.97  |
| Delhi NCR   | IN-DL    | 1.02  | Rajasthan      | IN-RJ    | 1.05  |
| Karnataka   | IN-KA    | 0.94  | Uttar Pradesh  | IN-UP    | 1.08  |
| Gujarat     | IN-GJ    | 1.01  | West Bengal    | IN-WB    | 1.03  |
| Telangana   | IN-TS    | 0.99  | Punjab         | IN-PB    | 0.96  |
| Kerala      | IN-KL    | 0.91  | Madhya Pradesh | IN-MP    | 1.06  |

**1.6 Worked Example**

<table>
<colgroup>
<col style="width: 100%" />
</colgroup>
<tbody>
<tr>
<td><p><strong>3 kg PET Bottles — AI Verified — Maharashtra —
Clean</strong></p>
<p>W = 3.0 kg | EF = 1.78 | M = 2.1 | V = 0.85 | L = 0.98 | Q = 1.00</p>
<p>C = 3.0 × 1.78 × 2.1 × 0.85 × 0.98 × 1.00 = 9.31 kg CO₂e</p>
<p>Carbon Credits = 9.31 ÷ 1000 = 0.00931 CC tokens</p></td>
</tr>
</tbody>
</table>

*— See Documents 2–6 for category-specific formulas —*
