# Solicitation Alignment

## Topic Basis

- **Track posture:** Phase I
- **Source basis:** DAF 26.BZ Release 1 Phase I topic language, pp. 14-17.
- **Objective summary:** Develop runtime assured autonomy functions that monitor black-box autonomy outputs, determine when COAs are infeasible, incorrect, or non-optimal, and activate mitigation or reversionary behaviors.

## What This Repository Intentionally Covers

- black-box fault detection and isolation for autonomy-generated COAs
- platform and fleet safety checks such as corridor compliance and path feasibility
- performance monitoring for correct and optimal mission execution under contingencies
- mitigation hooks for loiter, reversionary autonomy, return-to-base, or ditch logic

## How The Repository Maps To The Topic

| Solicitation Need | Repository Response |
| --- | --- |
| Topic-specific runtime checks | `core/src/profile.rs` encodes five topic-shaped trust properties tied to this mission area. |
| Repeatable proof and replay | `tooling/replay`, `tooling/eval`, `evidence/`, and `package_manifest.json` provide deterministic reproduction. |
| Integration path | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` define the C ABI boundary for autonomy-stack insertion. |
| Reviewer-verifiable evidence | `evidence/scorecard_summary.md`, `proof_log.txt`, `timeline.json`, and `trace.svg` make the behavior inspectable. |
| Clear scope discipline | This repository is scoped as: This repository deliberately focuses on runtime COA trust assessment and mitigation triggers rather than replacing the underlying flight or mission autonomy stack. |

## What The Package Is Not Claiming

- it is not a replacement for the underlying autonomy stack
- it is not a certification package
- it is not based on classified program data
- it is not claiming operational fielding approval

## Why The Current Shape Is Credible

The strongest near-term value of RTVLAS is the ability to make autonomy behavior observable,
explainable, and rejectable when it drifts outside mission or safety expectations. That is the
thread this repository follows for this specific topic.
