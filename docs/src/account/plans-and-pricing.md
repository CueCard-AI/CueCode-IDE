---
title: Plans & Pricing
description: Compare CueCode's Free, Pro, and Business plans, and understand token-based usage metering, spend limits, and trial details.
---

# Plans & Pricing

For costs and more information on pricing, visit [CueCode's pricing page](https://cuecode.dev/pricing).

CueCode works without AI features or a subscription. No [authentication](../authentication.md) is required for the editor itself.

## Plans {#plans}

|                                                | Free    | Pro       | Student   | Business  |
| ---------------------------------------------- | ------- | --------- | --------- | --------- |
| CueCode-hosted AI models                           | —       | ✓         | ✓         | ✓         |
| [AI via own API keys](../ai/use-api-access.md) | ✓       | ✓         | ✓         | ✓         |
| [External Agents](../ai/external-agents.md)    | ✓       | ✓         | ✓         | ✓         |
| Edit Predictions                               | Limited | Unlimited | Unlimited | Unlimited |
| Org-wide admin controls                        | —       | —         | —         | ✓         |
| Roles & permissions                            | —       | —         | —         | ✓         |
| Consolidated billing                           | —       | —         | —         | ✓         |

### CueCode Free {#free}

CueCode is free to use. You can configure AI agents with your own API keys via [Use API Access](../ai/use-api-access.md). [Edit Predictions](../ai/edit-prediction.md) are available on a limited basis. CueCode's hosted models require a Pro subscription.

### CueCode Pro {#pro}

CueCode Pro includes access to all hosted AI models and Edit Predictions. The plan includes $5 of monthly token credit; usage beyond that is billed at the rates listed on [CueCode-Hosted Models](./zed-hosted-models.md). A trial of CueCode Pro includes $20 of credit, usable for 14 days.

For details on billing and payment, see [Individual Billing](./billing.md).

### CueCode Business {#business}

CueCode Business gives members with a paid Business seat access to all of CueCode's hosted AI models, unlimited Edit Predictions, plus org-wide controls for administrators: which AI features are available, what data leaves your organization, and how AI spend is tracked. Paid seats and AI usage are consolidated into a single invoice.

For a full feature overview, see [CueCode Business](../business/overview.md). For billing details, see [Billing](./billing.md#organization).

### Student Plan {#student}

The [CueCode Student plan](https://cuecode.dev/education) includes all CueCode Pro features: unlimited [Edit Predictions](../ai/edit-prediction.md), all [hosted AI models](./zed-hosted-models.md) except Claude Opus, and $10/month in token credits. Available free for one year to verified university students.

## Usage {#usage}

Usage of CueCode's hosted models is measured on a token basis, converted to dollars at the rates listed on [CueCode-Hosted Models](./zed-hosted-models.md) (list price from the provider, +10%).

Monthly included credit resets on your monthly billing date. To view your current usage, navigate to the Billing page at [cuecode.dev](https://cuecode.dev). Usage data from our metering provider, Orb, is embedded on that page.

## Spend Limits {#usage-spend-limits}

### CueCode Pro {#pro-spend-limits}

On your Billing page you'll find an input for `Monthly Spend Limit`. For CueCode Pro, the dollar amount here specifies your pre-tax _monthly_ limit for spend on tokens, _not counting_ the $5/month included with your Pro subscription.

The default value for Pro users is $10, for a total monthly spend with CueCode of $20 ($10 for your Pro subscription, $10 in incremental token spend). This can be set to $0 to limit your spend with CueCode to exactly $10/month. If you adjust this limit _higher_ than $10 and consume more than $10 of incremental token spend, that usage may be billed during the month via [CueCode Pro threshold billing](./billing.md#threshold-billing).

Once the spend limit is hit, we'll stop any further usage until your token spend limit resets.

### CueCode Business {#business-spend-limits}

On CueCode Business, administrators set a pre-tax org-wide spend limit from the Data & Privacy page in the organization dashboard. Seats and AI usage are consolidated into [Organization billing](./billing.md#organization). Once the org-wide spend limit is reached, we'll stop hosted model usage for members until the limit resets or an administrator raises it.

> **Note:** Spend limits are a CueCode Pro and Business feature. Student plan users cannot configure spend limits; usage is capped at the $10/month included credit.

### Trials {#trials}

Trials automatically convert to CueCode Free when they end. Trials do not include access to Anthropic's Opus models. No cancellation is needed to prevent conversion to CueCode Pro.
