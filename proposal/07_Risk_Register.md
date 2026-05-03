# Phase I Risk Register

| Risk | Why It Matters | Mitigation During Phase I |
| --- | --- | --- |
| Synthetic scenario bias | Deterministic traces are useful for feasibility but not a substitute for platform data. | Keep claims scoped to feasibility, document assumptions, and define a Phase II ingestion path for representative autonomy logs. |
| Integration boundary drift | Mission software interfaces may differ from the snapshot contract used in this prototype. | Preserve a narrow C ABI, publish the snapshot schema, and validate assumptions early with integration partners. |
| Reviewer confusion about product scope | The proposal can be misread as “another autonomy stack” instead of an assurance layer. | Repeat the opening angle in the README, technical volume, and claim matrix. |
| Trust score overtuning | A trust accumulator can look arbitrary if unsupported by evidence. | Keep the math simple, deterministic, and directly tied to scenario artifacts. |
| Topic-specific transition risk | Each topic has different transition stakeholders and mission workflows. | Keep the Phase I work product software-centric and pair it with a topic-specific Phase II integration path. |
