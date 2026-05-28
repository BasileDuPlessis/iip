# Copilot Instructions for this Repository

## Repository state

This repository currently contains product documentation only (`docs/PRD.md`) and no implementation code yet.

## Build, test, and lint

No build, test, or lint commands are defined in this repository at this time.

## High-level architecture (from PRD)

The product is an **Investment Intelligence Platform** for enterprise IT transformation governance.  
The MVP is organized into four connected capabilities:

1. **Ecosystem Mapping**: model both IT ecosystem (applications, dependencies, integrations, platforms, data flows) and business ecosystem (domains, capabilities, processes, plant usage), with indicators on each side.
2. **Application Onboarding**: lightweight onboarding flow to place applications quickly in the ecosystem, then enrich progressively.
3. **Initiative Intelligence**: evaluate initiatives against ecosystem reality (variability, coupling, adoption, dependencies), produce challenge insights, and recommend incremental transformation paths.
4. **Investment Dashboard**: visualize capability heatmaps, technical health, adoption, duplication, and investment zones for prioritization.

Core domain concepts used across features:

- **Business Capability** (business-level, app-independent capability)
- **Application** (technical implementation mapped to capabilities)
- **Investment Zone** (priority area inferred from indicators)
- **Common Capability Layer** (reusable shared services/APIs/workflows/data products/platforms)

## Key conventions to preserve

- Keep governance and prioritization centered on **capabilities and investment zones**, not project/application-first framing.
- Treat indicators as first-class and explicitly separated into **technical**, **business**, and **adoption** signals.
- Keep initiative analysis tied to measurable ecosystem evidence; avoid recommendation flows that skip dependency/coupling/readiness checks.
- Preserve the PRD’s **incremental transformation** approach (evolutionary sequencing before full harmonization).
- For onboarding-related work, preserve required MVP fields: application name, owner, type, plants, supported capabilities, connected systems, criticality, and basic technical health indicators.
- Keep integrations phased as described in the PRD (Phase 1: CMDB/backlog/cloud inventory/IAM; Phase 2: observability/CI-CD/Git/usage analytics).
