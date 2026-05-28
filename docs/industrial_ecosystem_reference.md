# Fictional Company — Titan Tyres Group

# Overview

Titan Tyres Group is a multinational tyre manufacturing company operating industrial production plants across Europe.

The company produces:

* passenger vehicle tyres
* truck tyres
* industrial tyres

The organization historically grew through:

* mergers & acquisitions
* local industrial initiatives
* plant-level optimization projects

As a result, the IT ecosystem is heterogeneous:

* legacy systems
* local custom applications
* SaaS platforms
* shared enterprise platforms
* shadow IT
* duplicated capabilities across plants

The company is currently transforming its governance model from:

* project/application-centric governance

toward:

* capability- and investment-centric governance.

The objective is to:

* reduce ecosystem complexity
* increase reuse
* identify strategic investment zones
* modernize critical capabilities
* improve operational performance

---

# Company Context

## Employees

Total employees: 8,400

---

# Plants

## Lille Plant (France)

* High digital maturity
* Advanced MES integration
* Strong automation

Employees: 3,200

---

## Poznan Plant (Poland)

* Medium maturity
* Hybrid digital/manual processes

Employees: 2,900

---

## Valencia Plant (Spain)

* Low maturity
* Heavy legacy usage
* Strong shadow IT presence

Employees: 2,300

---

# Headcount Split by Main Services

| Service                         | Employees |
| ------------------------------- | --------- |
| Manufacturing Operations        | 4,500     |
| Maintenance & Reliability       | 1,000     |
| Quality & Compliance            | 700       |
| Supply Chain & Logistics        | 900       |
| Engineering & Product Lifecycle | 600       |
| Data & Reporting                | 250       |
| IT & Digital                    | 450       |

---

# Strategic Objectives

The company wants to:

* reduce operational fragmentation
* standardize core industrial capabilities
* modernize legacy applications
* accelerate deployment of digital initiatives
* reduce dependence on shadow IT
* improve engineering delivery performance
* increase cross-plant capability convergence

---

# Domains, Capabilities and Applications

# 1. Manufacturing Operations

## Capabilities

* Production Scheduling
* Shopfloor Execution
* Energy Optimization

---

## Applications

### MES Alpha

Type: Off-the-shelf

Capabilities:

* Shopfloor Execution
* Traceability

Purpose:
Production execution and industrial traceability.

Story:
Originally deployed only in Lille plant during a modernization initiative in 2015. The rollout to Poznan was partially successful, but Valencia rejected deployment due to local process differences and resistance from production supervisors. MES Alpha became the de facto industrial backbone for standardized production processes.

---

### Plant Scheduler

Type: Custom

Capabilities:

* Production Scheduling

Purpose:
Local production planning and sequencing.

Story:
Created by the Poznan plant after repeated failures to adapt ERP planning logic to real production constraints. Initially developed by two local engineers as an Excel macro, later transformed into a lightweight internal web application. It is now heavily used operationally but poorly integrated with enterprise systems.

---

### Operator Portal

Type: Custom

Capabilities:

* Shopfloor Execution
* Maintenance Work Orders

Purpose:
Shopfloor operator workflows and production visibility.

Story:
Greenfield initiative launched by the Lille digital manufacturing team to simplify operator interactions with MES systems. The portal became popular because it reduced operational friction and mobile usage on the shopfloor increased significantly.

---

### Energy Monitor

Type: SaaS

Capabilities:

* Energy Optimization

Purpose:
Energy consumption optimization.

Story:
Introduced after energy price increases impacted industrial costs. Initially piloted on curing machines in Lille and rapidly showed measurable savings. Expansion to other plants is slowed by sensor standardization issues.

---

# 2. Quality & Compliance

## Capabilities

* Quality Incident Management
* Traceability
* Product Compliance

---

## Applications

### Quality Tracker

Type: Custom

Capabilities:

* Quality Incident Management
* Traceability

Purpose:
Quality incident workflows and defect management.

Story:
Built internally after multiple audits revealed inconsistent quality investigation processes between plants. The application replaced email- and Excel-based workflows but adoption remains uneven outside Lille.

---

### Compliance Hub

Type: SaaS

Capabilities:

* Product Compliance
* Traceability

Purpose:
Regulatory compliance and audit tracking.

Story:
Corporate-driven SaaS introduced to centralize compliance evidence after regulatory pressure increased on tyre traceability and sustainability reporting.

---

# 3. Maintenance & Reliability

## Capabilities

* Maintenance Work Orders
* Predictive Maintenance
* Asset Reliability

---

## Applications

### Maintenance Hub

Type: Custom

Capabilities:

* Maintenance Work Orders
* Asset Reliability

Purpose:
Maintenance workflows and equipment management.

Story:
Legacy maintenance platform heavily customized over ten years by the Valencia plant. The application contains embedded maintenance logic specific to curing presses and older industrial equipment. Despite poor UX, maintenance teams rely on it daily.

---

### Predictive Insight

Type: SaaS

Capabilities:

* Predictive Maintenance
* Asset Reliability

Purpose:
Predictive maintenance analytics using sensor data.

Story:
Innovation pilot launched with an industrial AI startup after several costly production outages. Initially deployed on a single production line but gained executive attention after reducing unplanned downtime.

---

# 4. Supply Chain & Logistics

## Capabilities

* Inventory Management
* Warehouse Operations
* Shipment Planning

---

## Applications

### SAP S/4 ERP

Type: Off-the-shelf

Capabilities:

* Inventory Management
* Shipment Planning

Purpose:
Core ERP operations.

Story:
Enterprise-wide ERP deployed progressively after several acquisitions created fragmented finance and procurement systems. The rollout took nearly eight years and still contains plant-specific adaptations.

---

### Inventory Pro

Type: SaaS

Capabilities:

* Inventory Management
* Warehouse Operations

Purpose:
Warehouse and inventory management.

Story:
Quickly introduced during the pandemic after inventory inaccuracies disrupted raw material supply. Chosen mainly because deployment speed was more important than architectural alignment.

---

# 5. Engineering & Product Lifecycle

## Capabilities

* Engineering Change Management
* Product Specification Management

---

## Applications

### PLM Core

Type: Off-the-shelf

Capabilities:

* Engineering Change Management
* Product Specification Management

Purpose:
Engineering lifecycle and product specifications.

Story:
Inherited from a previous acquisition. Corporate leadership wants to transform it into the single enterprise PLM platform, but each plant currently maintains different engineering validation workflows and naming conventions.

---

### CAD Vault

Type: Legacy

Capabilities:

* Product Specification Management

Purpose:
Engineering drawing storage and management.

Story:
Very old engineering repository still used by senior engineers because historical product references and undocumented design rules remain stored there. Migration attempts repeatedly failed due to missing metadata consistency.

---

# 6. Data & Reporting

## Capabilities

* Operational Reporting
* Industrial Data Platform
* KPI Management

---

## Applications

### Legacy Reporting Cube

Type: Legacy

Capabilities:

* Operational Reporting
* KPI Management

Purpose:
Historical reporting and KPI extraction.

Story:
Initially considered strategic during the early BI era. Over time, reporting logic multiplied and became tightly coupled with Excel exports and local manual reporting processes. Today it is one of the most fragile systems in the company.

---

### Industrial Data Platform

Type: Platform

Capabilities:

* Industrial Data Platform
* KPI Management
* Operational Reporting

Purpose:
Shared industrial data ingestion and analytics.

Story:
Strategic cloud platform launched by the CIO organization to progressively replace fragmented reporting architectures and create shared industrial data products across plants.

---

### Excel Macros Local Ops

Type: Shadow IT

Capabilities:

* Operational Reporting

Purpose:
Local reporting and operational workarounds.

Story:
Represents dozens of local spreadsheets and macros created by operational teams to compensate for missing integrations, weak UX, or insufficient reporting flexibility in enterprise systems.

---

# Application Template

Each application onboarded into the platform must contain the following information.

---

# Story

Narrative explaining the origin and strategic position of the application.

Examples:

* inherited through acquisition
* local tactical solution
* innovation pilot
* strategic enterprise platform
* obsolete but business-critical
* shadow IT workaround
* temporary greenfield experiment

Purpose:
Provide transformation and governance context.

---

# Population & Scope

| Field                  | Description                                    |
| ---------------------- | ---------------------------------------------- |
| Targeted Population    | Expected user population                       |
| Actual Active Users    | Operational users                              |
| Plants Covered         | Number of sites using the app                  |
| Business Units Covered | Scope across organization                      |
| Critical User Types    | Operators / planners / engineers / maintenance |

---

# Metrics Model

The platform evaluates applications through three measurement pillars.

---

# 1. Technical Delivery Metrics (DORA)

## Purpose

Measure engineering delivery performance and operational reliability.

---

## Metrics

### Deployment Frequency (DF)

Frequency of production-impacting deployments.

---

### Lead Time for Changes (LTC)

Time between code commit and successful production deployment.

---

### Change Failure Rate (CFR)

Percentage of deployments causing incidents or requiring remediation.

---

### Mean Time to Restore (MTTR)

Average duration required to restore service after incident.

---

# DORA Data Sources

## CI/CD Platforms

Examples:

* GitHub Actions
* GitLab CI
* Jenkins
* Azure DevOps

---

## Source Control

Examples:

* GitHub
* GitLab
* Bitbucket

---

## Deployment Systems

Examples:

* ArgoCD
* Kubernetes
* Spinnaker

---

## Incident Management

Examples:

* PagerDuty
* ServiceNow
* OpsGenie

---

## Observability Platforms

Examples:

* Datadog
* Grafana
* Dynatrace
* New Relic

---

# Notes

DORA metrics apply mainly to:

* custom applications
* internal platforms
* APIs
* cloud-native services

For SaaS/COTS systems, technical health is approximated using:

* incidents
* availability
* upgrade cadence
* integration stability

---

# 2. Adoption & Diffusion Metrics

## Purpose

Measure operational usage and organizational penetration.

---

# Adoption Metrics

## Nominal Operational Usage

Definition:
Percentage of targeted users executing expected operational workflows through the application.

Examples:

* production orders processed digitally
* maintenance work orders completed in workflow
* quality incidents logged correctly

---

# Diffusion Metrics

## Site Deployment Coverage

Definition:
Number of plants/business units actively using the application.

---

# Adoption Data Sources

## Application Logs

Examples:

* login events
* workflow execution logs
* feature usage logs

---

## IAM / SSO

Examples:

* Okta
* Azure AD
* Keycloak

---

## Workflow Engines

Examples:

* Camunda
* ServiceNow workflows
* BPM platforms

---

## ERP / MES Events

Examples:

* production orders
* maintenance tickets
* quality records

---

# 3. North Star Metrics

## Purpose

Measure direct operational business outcomes associated with the application.

Each application contains a maximum of:

* 3 North Star metrics

Principle:
If the metric improves, business impact improves.

Metrics must be:

* measurable
* operational
* automatically collected

---

# Example North Star Metrics by Application

## MES Alpha

* Production throughput
* Scrap rate reduction
* Production traceability completeness

---

## Plant Scheduler

* Planning accuracy
* Changeover reduction
* Production delay reduction

---

## Maintenance Hub

* Unplanned downtime reduction
* Mean Time To Repair reduction
* Preventive maintenance ratio

---

## Quality Tracker

* Defect resolution time
* Quality incident recurrence reduction
* First-pass quality rate

---

## Industrial Data Platform

* KPI availability latency
* Reduction of Excel-generated reports
* Trusted KPI usage rate

---

# North Star Data Sources

## Operational Systems

Examples:

* MES
* ERP
* Maintenance systems
* Quality systems

---

## Industrial IoT Platforms

Examples:

* machine telemetry
* sensors
* OPC-UA streams

---

## Reporting Platforms

Examples:

* Power BI
* Tableau
* Data warehouse metrics

---

## Process Mining Platforms

Examples:

* Celonis
* Signavio