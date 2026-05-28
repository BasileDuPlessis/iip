# PRD — Capability & Investment Intelligence Platform

# 1. Product Overview

## Product Name

Capability & Investment Intelligence Platform

---

## Vision

Provide enterprise leaders, architects, and transformation teams with a living map of the IT and business ecosystem in order to:

* understand operational and technical reality
* identify investment zones
* monitor capability adoption
* track engineering health
* connect business outcomes to applications
* support transformation and modernization decisions

The platform transforms governance from:

* project/application-centric

toward:

* capability- and investment-centric.

---

# 2. Product Goals

The platform must enable organizations to:

* onboard and contextualize enterprise applications
* map domains, capabilities and applications
* monitor technical delivery performance
* monitor operational adoption and diffusion
* track operational business outcomes
* identify weak or fragmented capabilities
* support modernization prioritization
* visualize ecosystem evolution over time

---

# 3. Target Users

| User                  | Goal                             |
| --------------------- | -------------------------------- |
| CIO                   | Investment prioritization        |
| Enterprise Architect  | Ecosystem visibility             |
| Domain Architect      | Capability analysis              |
| Transformation Office | Modernization roadmap            |
| Product Teams         | Delivery and adoption visibility |
| Plant IT Managers     | Local/global alignment           |
| Business Sponsors     | Operational outcome visibility   |

---

# 4. Core Concepts

## Domain

High-level business or operational area.

Examples:

* Manufacturing Operations
* Maintenance & Reliability
* Quality & Compliance

---

## Capability

Business capability supported by one or more applications.

Examples:

* Production Scheduling
* Predictive Maintenance
* Operational Reporting

---

## Application

Technical system supporting operational capabilities.

Examples:

* MES Alpha
* Plant Scheduler
* Industrial Data Platform

---

# 5. Feature 1 — Onboard a New Application

## Objective

Allow users to onboard and contextualize an application within the enterprise ecosystem.

---

# Functional Requirements

## FR-1.1 — Create Application

User can create a new application entry.

Required fields:

* application name
* type (SaaS / Custom / Legacy / Platform / COTS)
* description
* associated domain
* associated capabilities
* owner

---

## FR-1.2 — Define Story

User can describe the transformation and operational context of the application.

Examples:

* inherited from acquisition
* local tactical solution
* innovation pilot
* strategic platform
* shadow IT workaround

---

## FR-1.3 — Define Population & Scope

User can define:

* targeted population
* plants covered
* business units covered
* critical user roles

---

## FR-1.4 — Configure Technical Sources

User can connect:

* Git repositories
* CI/CD systems
* deployment systems
* observability platforms
* incident management tools

Used for:

* DORA metric calculation

---

## FR-1.5 — Configure Adoption Sources

User can connect:

* application logs
* IAM/SSO
* workflow systems
* ERP/MES event sources

Used for:

* operational adoption metrics
* diffusion metrics

---

## FR-1.6 — Configure North Star Metrics

User can define up to:

* 3 operational business metrics

Each metric must contain:

* name
* description
* associated datasource
* aggregation method
* refresh frequency

Examples:

* production throughput
* downtime reduction
* planning accuracy

---

# User Flow

```text
Create App
    ↓
Associate Domain
    ↓
Associate Capabilities
    ↓
Describe Story
    ↓
Configure Metrics Sources
    ↓
Validate Telemetry
    ↓
Application appears on ecosystem map
```

---

# 6. Feature 2 — Ecosystem Map

## Objective

Provide a visual representation of:

* domains
* capabilities
* applications
* operational indicators
* technical health
* business tendencies

The map acts as the primary governance and investment view.

---

# Functional Requirements

## FR-2.1 — Display Domain Map

Platform displays all domains.

Examples:

* Manufacturing Operations
* Quality & Compliance
* Maintenance & Reliability

---

## FR-2.2 — Display Capabilities Inside Domains

Capabilities appear inside associated domains.

Example:

```text
Manufacturing Operations
 ├── Production Scheduling
 ├── Shopfloor Execution
 └── Energy Optimization
```

---

## FR-2.3 — Display Applications per Capability

Applications supporting capabilities are visible.

Example:

```text
Production Scheduling
 ├── Plant Scheduler
 └── SAP S/4 ERP
```

---

## FR-2.4 — Display Adoption Indicators

Each application and capability displays:

* operational adoption score
* adoption trend tendency

Examples:

* increasing
* stable
* declining

Trend computed from:

* rolling monthly operational usage

---

## FR-2.5 — Display DORA Status

Applications display DORA maturity level.

Possible states:

* Low Performer
* Medium Performer
* High Performer
* Elite Performer

Status computed using:

* deployment frequency
* lead time for changes
* change failure rate
* MTTR

---

## FR-2.6 — Display DORA Trend

Platform displays DORA tendency:

* improving
* stable
* degrading

Computed over rolling periods.

---

## FR-2.7 — Display North Star Trend

Each application displays:

* trend of operational North Star metrics

Examples:

* throughput increasing
* downtime decreasing
* planning accuracy degrading

---

## FR-2.8 — Visual Investment Signals

Applications and capabilities are visually highlighted when:

* adoption drops
* DORA performance degrades
* North Star metrics degrade
* fragmentation exists between plants

---

# Map Visualization Principles

## Node Types

| Node        | Description            |
| ----------- | ---------------------- |
| Domain      | Business area          |
| Capability  | Operational capability |
| Application | Technical system       |

---

## Visual Signals

| Signal       | Meaning                    |
| ------------ | -------------------------- |
| Node color   | Overall health             |
| Node size    | Operational reach          |
| Border state | DORA maturity              |
| Trend arrow  | Positive/negative tendency |

---

# Example View

```text
Manufacturing Operations
 ├── Production Scheduling 🔴
 │      ├── Plant Scheduler ⚠
 │      └── SAP S/4 ERP 🟡
 │
 ├── Shopfloor Execution 🟢
 │      ├── MES Alpha 🟢
 │      └── Operator Portal 🟢
```

---

# 7. Feature 3 — Application Details View

## Objective

Provide a detailed operational, technical and business view for a single application.

---

# Functional Requirements

## FR-3.1 — Display Application Overview

Display:

* application description
* type
* associated domain
* associated capabilities
* targeted population
* plants covered
* business units covered

---

## FR-3.2 — Display Story Context

Display:

* application history
* transformation context
* strategic positioning

Examples:

* M&A legacy
* strategic modernization platform
* local tactical workaround

---

## FR-3.3 — Display DORA Metrics

Display:

* deployment frequency
* lead time for changes
* change failure rate
* MTTR

Features:

* historical trends
* rolling averages
* performer classification

---

## FR-3.4 — Display Adoption Metrics

Display:

* operational adoption score
* active users
* targeted users
* diffusion by plant
* workflow execution trends

---

## FR-3.5 — Display North Star Metrics

Display:

* configured operational metrics
* trend evolution
* historical values
* comparison between plants

---

## FR-3.6 — Display Data Sources

Display connected systems:

* Git repositories
* observability tools
* IAM systems
* operational telemetry sources

---

## FR-3.7 — Display Ecosystem Relations

Display:

* connected capabilities
* upstream/downstream applications
* dependency graph

---

# Example Application Details Layout

```text
------------------------------------------------
MES Alpha
------------------------------------------------

Type: Off-the-shelf
Domain: Manufacturing Operations

Capabilities:
- Shopfloor Execution
- Traceability

Plants:
- Lille
- Poznan

------------------------------------------------
DORA
------------------------------------------------

Deployment Frequency: Medium Performer ↑
Lead Time: Improving
CFR: Stable
MTTR: Improving

------------------------------------------------
Adoption
------------------------------------------------

Operational Adoption: 82%
Trend: ↑

Plant Coverage:
- Lille: 95%
- Poznan: 73%
- Valencia: not deployed

------------------------------------------------
North Star Metrics
------------------------------------------------

Production Throughput: +7%
Scrap Rate: -4%
Traceability Completeness: +12%

------------------------------------------------
Story
------------------------------------------------

Initially deployed in Lille during a
manufacturing modernization initiative...
```

---

# 8. Non-Functional Requirements

| Requirement                 | Target             |
| --------------------------- | ------------------ |
| Application onboarding time | < 30 min           |
| Map loading time            | < 3 sec            |
| Daily telemetry refresh     | automated          |
| Scalability                 | 1000+ applications |
| Historical metric retention | 24 months          |

---

# 9. Data Sources

## Technical Delivery

* GitHub
* GitLab
* Jenkins
* Azure DevOps
* Datadog
* PagerDuty

---

## Operational Adoption

* IAM/SSO
* Application logs
* ERP
* MES
* Workflow systems

---

## Operational Outcomes

* MES KPIs
* ERP KPIs
* Maintenance systems
* IoT telemetry
* Process mining tools

---

# 10. Success Metrics

| KPI                                           | Objective                  |
| --------------------------------------------- | -------------------------- |
| % applications onboarded                      | >80%                       |
| % telemetry automated                         | >70%                       |
| onboarding duration                           | <30 min                    |
| ecosystem map usage                           | weekly by governance teams |
| number of investment decisions using platform | increasing over time       |

---

# 11. Long-Term Vision

The platform evolves toward:

* enterprise ecosystem intelligence
* transformation sequencing support
* investment recommendation engines
* capability modernization prioritization
* AI-assisted governance

The long-term objective is to transform governance from:

* managing projects and applications

toward:

* continuously evolving enterprise capabilities and investments.