# Architecture — Investment Intelligence Platform (Rust + WASM)

## 1. Architecture Goals

- Rust-first implementation across frontend, backend, and analytics services.
- WASM-based client for interactive graph/heatmap experiences.
- Capability-centric domain model (not project/application-centric).
- Explicit indicator separation: technical, business, adoption.
- Incremental rollout: start with synthetic data + manual onboarding, then phase in external integrations.

---

## 2. High-Level Topology

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Browser (WASM)                                                       │
│  - Rust UI (Leptos/Yew + wasm-bindgen/web-sys)                      │
│  - Graph/heatmap rendering                                           │
│  - Initiative challenge views                                        │
└───────────────────────────────▲──────────────────────────────────────┘
                                │ HTTPS (JSON/REST; optional GraphQL)
┌───────────────────────────────┴──────────────────────────────────────┐
│ Rust API Layer (Axum)                                                │
│  - AuthN/AuthZ                                                       │
│  - Query APIs (ecosystem, indicators, dashboard)                     │
│  - Command APIs (onboarding, initiative analysis requests)           │
└──────────────▲───────────────────────▲───────────────────────────────┘
               │                       │
               │                       │
┌──────────────┴──────────────┐  ┌─────┴───────────────────────────────┐
│ Domain & Application Core   │  │ Analytics & Recommendation Engine    │
│ (pure Rust crates)          │  │ (pure Rust crate + jobs)             │
│ - entities/value objects    │  │ - scoring                            │
│ - policies/use-cases        │  │ - investment zone detection          │
│ - validation                │  │ - initiative challenge insights      │
└──────────────▲──────────────┘  └─────▲───────────────────────────────┘
               │                       │
               └───────────────┬───────┘
                               │
                    ┌──────────┴──────────┐
                    │ Persistence + Ingest │
                    │ - PostgreSQL         │
                    │ - Redis (cache/queue)│
                    │ - Connectors/jobs    │
                    └──────────────────────┘
```

---

## 3. Cargo Workspace Layout

```text
iip/
  Cargo.toml                # workspace root
  crates/
    domain/                 # core entities, value objects, invariants
    application/            # use-cases (onboarding, mapping, analysis)
    analytics/              # scoring, zone detection, recommendation logic
    infra-postgres/         # repositories, migrations, SQLx/SeaORM adapters
    infra-connectors/       # CMDB/backlog/cloud/IAM + phase-2 adapters
    api/                    # Axum HTTP API + auth middleware
    webapp/                 # WASM frontend (Leptos/Yew)
    dataset-sim/            # synthetic monthly data generator + seed loader
```

Design rule: `domain` and `application` stay framework-agnostic and free of transport/database dependencies.

---

## 4. Core Domain Model

Primary aggregates:

- `BusinessCapability`
- `Application`
- `Plant`
- `Dependency` (application↔application/application↔platform/data asset)
- `Indicator` (`TechnicalIndicator`, `BusinessIndicator`, `AdoptionIndicator`)
- `Initiative`
- `InvestmentZone`
- `CommonCapabilityLayerCandidate`

Key relationships:

- Applications support one or more capabilities.
- Capability and application indicators are stored by period (monthly).
- Initiatives are evaluated against dependency graph + indicator history.
- Investment zones are derived from indicator thresholds and risk rules.

---

## 5. Data & Storage Design

## Primary store
- PostgreSQL for transactional/domain data and historical monthly indicators.

## Suggested schema groups
- `core_*`: applications, capabilities, plants, mappings
- `graph_*`: dependencies, integration styles, criticality
- `ind_*`: monthly technical/business/adoption indicators
- `init_*`: initiatives, challenge findings, recommendation steps
- `gov_*`: investment zones, prioritization runs, audit trail

## Optional secondary stores
- Redis for caching hot dashboard queries and async job coordination.
- Object storage for connector snapshots and import artifacts.

---

## 6. Runtime Components

## API service (`crates/api`)
- Serves onboarding, ecosystem graph, dashboards, initiative intelligence.
- Enforces role-based access (`CIO`, `EA`, `DomainArchitect`, `PlantIT`, etc.).

## Analytics worker (`crates/analytics`)
- Periodic jobs:
  - recompute monthly indicator rollups
  - recalculate investment zones
  - refresh initiative challenge scores/recommendation pathways

## Connector workers (`crates/infra-connectors`)
- Phase 1: CMDB, backlog tools, cloud inventory, IAM.
- Phase 2: observability, CI/CD, Git, usage analytics.

---

## 7. WASM Frontend Architecture

## UI modules
- Ecosystem map (graph exploration).
- Capability heatmap and investment matrix.
- Application onboarding wizard (MVP required fields).
- Initiative challenge workspace.

## State strategy
- Read models cached in browser state (query slices by domain/capability/plant).
- Command/query separation in frontend API client.
- Optimistic updates only for low-risk forms; server-confirmed updates for scoring-dependent views.

## Visualization
- Graph rendering with WebGL/SVG-capable Rust-friendly integration.
- Heavy transforms precomputed server-side to keep WASM client responsive.

---

## 8. Decision Intelligence Flow

1. Ingest/update ecosystem + indicator data (monthly cadence).
2. Build/refresh dependency and capability projection models.
3. Run scoring:
   - technical health and debt pressure
   - business value and cross-plant impact
   - adoption maturity/variability
4. Detect investment zones.
5. Challenge initiatives with measurable ecosystem evidence.
6. Produce incremental transformation pathway recommendations.

---

## 9. Dataset & Environments

- `dataset-sim` generates deterministic synthetic data for 24 monthly periods.
- Seed profiles:
  - `demo-small` (single domain, faster local iteration)
  - `demo-full` (all plants/domains/capabilities from DATASET.md)
- Local dev stack: API + PostgreSQL + Redis + WASM webapp.

---

## 10. Delivery Phases

## Phase A — Foundational MVP
- Workspace setup, core domain, onboarding, capability mapping, synthetic seed.
- Basic graph and dashboard views with monthly indicators.

## Phase B — Initiative Intelligence
- Challenge engine, risk insights, incremental recommendation paths.
- Investment zone detection and prioritization APIs.

## Phase C — Integration Expansion
- Phase 1 connectors first, then Phase 2.
- Data freshness automation and governance workflow hardening.

---

## 11. Quality & Operational Constraints

- Strong type safety across shared DTOs (Rust-to-WASM shared crates where useful).
- Versioned APIs and migration discipline for indicator models.
- Observability: tracing/metrics on API latency, recomputation duration, job failures.
- Security: enterprise SSO integration path, role-based authorization, auditable decisions.
- Scalability target aligned with PRD baseline (1000+ applications).

