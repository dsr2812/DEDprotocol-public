# Comparative Oncology — Context and Boundaries

**Status:** context note, not evidence. **Date:** 2026-11-02.

**Rule.** No comparative-oncology mechanism is implemented as a payload mechanism in
`ded_ark_engine`, and no animal result validates the RBDC carrier protocol. The engine's
elephant and shark "anchors" are algebraic encodings, not biological mechanisms. This note
records what the comparative-oncology literature supports, what it does not, and which
sources passed verification.

---

## 1. Why this note exists

A comparative-oncology deep dive and reading list were supplied as background. The material
is genuinely useful for framing and limitations, but the list mixed verified papers with
at least one wrong venue and two items that could not be verified at all. Importing it
wholesale would have put unverifiable citations into the technical report draft and blurred the line
between evolutionary biology and the framework's own claims. This note is the audit trail.

The short version: comparative oncology is **context**, not validation. It can explain why a
tissue-scale, multi-layer view of cancer is not unreasonable. It cannot supply an
electromagnetic carrier, a frequency, or a clinical prediction.

---

## 2. The six strategies, verified

| Group | Verified strategy | Best source | Relation to DED | What it does **not** support |
|---|---|---|---|---|
| Elephants | At least 20 *TP53* copies (40 alleles), including 19 retrogenes; higher p53-dependent apoptosis after DNA damage; evolutionary interpretation contested | Abegglen et al. 2015; Moore 2025 | A contrasting "eliminate the damaged cell" strategy. DED models reversion, not annihilation. | That elephants generate or use a 33 THz signal; that the engine's "elephant ratchet" is an elephant property. |
| Naked mole rats | High-molecular-mass hyaluronan (>5× human/mouse); CD44/p16^INK4a signalling; early contact inhibition | Tian et al. 2013 | Tissue-level (ECM) growth restraint; supports the general point that tissue-scale constraints matter. | Any electromagnetic mechanism; HMM-HA is not modelled in the engine. |
| Bowhead whales | Fewer oncogenic hits needed in vitro, but enhanced NHEJ/HR repair, higher fidelity, lower mutation rates; CIRBP sufficient to improve repair in human cells and extend *Drosophila* lifespan | Firsanov et al. 2025 | The closest conceptual analogy to DED's "repair/reversion rather than kill" language. It is molecular, not electromagnetic. | That DNA-repair biology validates a THz or sub-kHz carrier; that CIRBP is a DED payload. |
| Bats | *M. lucifugus* has a *TP53* duplication and elevated p53-dependent apoptosis, but bat fibroblasts are readily transformed by two oncogenic hits; cell-autonomous resistance is limited, pointing to immune/niche mechanisms | Athar et al. 2025 | A caution against cell-intrinsic single-mechanism stories; loosely consistent with a tissue-level, non-cell-autonomous framing. | The popular claim that bats have "enhanced p53 without the drawbacks"; that bat immunity is a validated cancer therapy model. |
| Xenarthrans | Independent tumour-suppressor gene duplications; extremely slow sloth proliferation; hypersensitivity to low-level DNA damage; cancer nearly absent in armadillos | Vazquez et al. 2022 | Parallel evolution and redundancy; another example that resistance can arise through different tissue strategies. | Any carrier or bioelectric mechanism. |
| Blind mole rats | Interferon-β1–mediated "concerted cell death" (massive necrosis) after abnormal proliferation | Gorbunova et al. 2012 | A tissue-level coordinated response — but destructive. It is the **opposite** of reversion. | A "kill" strategy for DED; DED explicitly does not model annihilation. |

### The elephant nuance

The textbook story — extra *TP53* copies make elephant cells hypersensitive to DNA damage and
therefore cancer-resistant — is well supported at the cellular level (Abegglen et al. 2015).
Its **evolutionary interpretation** is now contested. Moore (2025) argues that the expansion
may be explained by selection on reproduction, with cancer resistance as a by-product; the
"hot testicle" hypothesis proposes germline protection under elevated testicular temperature
as the selective pressure. The honest position for DED is:

- the *TP53* expansion and enhanced apoptotic response are real, cited observations;
- the claim that cancer prevention was the sole or primary driver is a hypothesis under debate;
- DED's "elephant anchor" is a heuristic encoding of the *observation*, not a mechanistic
  claim about *TP53*, and certainly not a claim that elephants emit 33 THz.

### The bat correction

The supplied deep dive said bats have "two copies" of p53 and "elevated p53 activity" while
avoiding the elephant's drawbacks. The verified picture is more interesting and less
convenient: *Myotis lucifugus* does show a *TP53* duplication and elevated p53-dependent
apoptosis, but bat fibroblasts do **not** show robust cell-autonomous anticancer defences —
they are transformed by two oncogenic hits (p53 or pRb inactivation plus HRAS^G12V). The
authors conclude that bat cancer resistance is likely to depend substantially on
non-cell-autonomous factors such as the immune system or tissue microenvironment (Athar et
al. 2025). That is a caution against over-reading any single cell-intrinsic mechanism, and it
is a better citation than the unverified comparative review in the supplied list.

---

## 3. What this means for DEDprotocol

### Legitimate uses

1. **Background for Peto's paradox.** Neoplastic incidence does not scale simply with body
   mass or cell number; large, long-lived species have evolved resistance. DED already cites
   this as motivation, not proof.
2. **Plausibility of a tissue-scale framing.** Comparative oncology shows that resistance can
   be non-cell-autonomous (ECM, immune, niche) and multi-layered. That makes a tissue-scale
   model *less unreasonable*; it does not make this particular model true.
3. **A caution about single-mechanism claims.** Evolved resistance uses redundant, staged
   systems. DED's single-axis carrier hypothesis is far simpler than evolved biology and
   should be presented as a pilot-scale specificity hypothesis, not as a match for natural
   resistance.
4. **Named heuristic anchors, clearly labelled.** The elephant and shark polynomials may stay
   as named algebraic encodings, provided the paper says they are not mechanisms, not measured
   resonances, and not therapeutic targets.

### Prohibited uses

- "Elephants use a 33 THz ratchet." No source supports this.
- "Whale DNA repair validates entropic-reversion carriers." No; the analogy is conceptual and
  molecular, not electromagnetic.
- "Naked mole-rat hyaluronan supports ECM impedance matching." No; HMM-HA is not modelled.
- "Bats prove that boosting p53 is safe." No; Athar et al. show limited cell-autonomous
  resistance and ready transformation in vitro.
- "Blind mole-rat interferon supports a kill strategy." No; DED explicitly models reversion,
  not annihilation, and the blind mole-rat strategy is necrotic.
- Any implication that animal cancer resistance predicts human clinical response to the
  candidate payload.

### If a collaborator asks

> "Does the animal work support your carrier?"

No. It supports the general premise that cancer resistance is a tissue-level, evolved
phenomenon worth studying. The carrier is the framework's own hypothesis and has not been
tested in any animal or human system. The correct next step is the pre-specified in vitro
specificity test, not a translational claim.

---

## 4. Reading-list audit

| Supplied citation | Audit result | Action |
|---|---|---|
| Abegglen et al. (2015), *JAMA* | Verified. *JAMA* 314(17), 1850–1860. Supplied title was a paraphrase; the actual title is "Potential mechanisms for cancer resistance in elephants and comparative cellular response to DNA damage in humans." | Cite as [9]. Correct the technical report draft's title. |
| Moore, T. (2025), *Reproduction* | Verified. *Reproduction* 170(2), e250254. doi:10.1530/REP-25-0254. | Cite as context/nuance. |
| Tian et al. (2013), *Nature* | Verified. *Nature* 499, 346–349. doi:10.1038/nature12234. | Context only. |
| Firsanov et al. (2025), *Nature* | Verified. *Nature* 648, 717–725. doi:10.1038/s41586-025-09694-5. Open access; published 29 Oct 2025. | Cite as context. |
| Paniagua & Joyce (2026), *FEBS Open Bio* | **Wrong venue.** The commentary is in *Molecular Oncology* (FEBS Press), doi:10.1002/1878-0261.70250, titled "Insights into a long life without cancer: the case of the bowhead whale." It is a commentary, not a primary study. | Do not cite in the technical report draft; list here only. |
| Zhao & Chen (2025), *Biotic Resources* | **Unverified.** No matching author/venue record found. A bats-in-biomedical-research review exists, but authorship and venue could not be confirmed. | **Exclude.** |
| "p53-Mediated Cancer Resistance in Elephants and Bats: A Comparative Review" | **Unverified.** A ResearchGate record exists, but no journal, year, or DOI could be confirmed. | **Exclude;** use Athar et al. 2025 instead. |
| Vazquez et al. (2022), *eLife* | Verified. *eLife* 11, e82558. doi:10.7554/eLife.82558. A bioRxiv preprint also exists. An earlier review note calling it "bioRxiv, not eLife" was **wrong**; the eLife version is the version of record. | Cite as context. |
| Gorbunova et al. (2012), *PNAS* | Verified. *PNAS* 109(47), 19392–19396. doi:10.1073/pnas.1217211109. | Context only. |
| "Mechanisms of cancer resistance in long-lived mammals" | Verified as Seluanov, Gladyshev, Vijg & Gorbunova (2018), *Nature Reviews Cancer* 18(7), 433–441; PMID 29622806. | Cite as review context. |

**Lessons recorded.** (i) A plausible title is not a citation; venue and year must be checked.
(ii) A preprint and its journal version can both exist; the journal version is the record of
version. (iii) A review that cannot be located should be dropped, not "cleaned up" by guessing.
(iv) A commentary is not a primary study and should not be cited as one.

---

## 5. Verified references

1. Abegglen, L.M., Caulin, A.F., Chan, A., Lee, K., Robinson, R., Campbell, M.S., Kiso, W.K.,
   Schmitt, D.L., Waddell, P.J., Bhaskara, S., Jensen, S.T., Maley, C.C. & Schiffman, J.D.
   Potential mechanisms for cancer resistance in elephants and comparative cellular response
   to DNA damage in humans. *JAMA* 314(17), 1850–1860 (2015). doi:10.1001/jama.2015.13134.
2. Moore, T. Peto's paradox and elephant TP53 retrogenes: cancer or reproduction?
   *Reproduction* 170(2), e250254 (2025). doi:10.1530/REP-25-0254.
3. Tian, X., Azpurua, J., Hine, C., Vaidya, A., Myakishev-Rempel, M., Ablaeva, J., Mao, Z.,
   Nevo, E., Gorbunova, V. & Seluanov, A. High-molecular-mass hyaluronan mediates the cancer
   resistance of the naked mole rat. *Nature* 499, 346–349 (2013). doi:10.1038/nature12234.
4. Firsanov, D., Zacher, M., Tian, X., et al. Evidence for improved DNA repair in the
   long-lived bowhead whale. *Nature* 648, 717–725 (2025). doi:10.1038/s41586-025-09694-5.
5. Athar, F., Zheng, Z., Riquier, S., et al. Limited cell-autonomous anticancer mechanisms in
   long-lived bats. *Nature Communications* 16, 4125 (2025). doi:10.1038/s41467-025-59403-z.
6. Vazquez, J.M., Pena, M.T., Muhammad, B., Kraft, M., Adams, L.B. & Lynch, V.J. Parallel
   evolution of reduced cancer risk and tumor suppressor duplications in Xenarthra. *eLife*
   11, e82558 (2022). doi:10.7554/eLife.82558.
7. Gorbunova, V., Hine, C., Tian, X., Ablaeva, J., Gudkov, A.V., Nevo, E. & Seluanov, A.
   Cancer resistance in the blind mole rat is mediated by concerted necrotic cell death
   mechanism. *PNAS* 109(47), 19392–19396 (2012). doi:10.1073/pnas.1217211109.
8. Seluanov, A., Gladyshev, V.N., Vijg, J. & Gorbunova, V. Mechanisms of cancer resistance in
   long-lived mammals. *Nature Reviews Cancer* 18(7), 433–441 (2018). PMID 29622806.
9. Paniagua, I. & Joyce, J.A. Insights into a long life without cancer: the case of the
   bowhead whale. *Molecular Oncology* (2026). doi:10.1002/1878-0261.70250. [Commentary;
   context only, not cited in the technical report draft.]

---

## 6. The paragraph used in the technical report draft

The technical report draft's §6.6 uses only the verified sources above and makes the boundary explicit:
these mechanisms are context and caution, not validation; none is implemented in the engine;
and the single-axis carrier hypothesis is simpler than evolved resistance. See
`manuscript/technical_report_draft.md` §6.6.
