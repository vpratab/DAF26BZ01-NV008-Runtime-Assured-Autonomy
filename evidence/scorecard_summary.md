# Evidence Scorecard Summary

- Topic: `DAF26BZ01-NV008 Runtime Assured Autonomy`
- Generated: `2026-05-03T04:54:52Z`
- Git Head: `d709bcaf70054af777d40c229584227a5e50a167`
- Scenario Pass Rate: `3/3 (100.0%)`
- Evidence Type: `deterministic synthetic autonomy traces for submission-stage feasibility review`

| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |
| --- | --- | --- | --- | --- | --- |
| Nominal Sortie | `nominal` | `PASS` | `1.000` | `None` | [scenario_01_nominal_sortie](scenario_01_nominal_sortie/trust_scorecard.json) |
| Degraded Contingency Handling | `degraded` | `PASS` | `0.523` | `None` | [scenario_02_degraded_contingency](scenario_02_degraded_contingency/trust_scorecard.json) |
| Faulty Autonomy Chain | `fault` | `PASS` | `0.000` | `20` | [scenario_03_faulty_autonomy_chain](scenario_03_faulty_autonomy_chain/trust_scorecard.json) |

## Notes

- Nominal scenarios are expected to remain fully accepted.
- Degraded scenarios are expected to produce concern signals without hard reject behavior.
- Fault scenarios are expected to produce deterministic reject behavior.
- This summary is generated automatically from the underlying per-scenario scorecards.
