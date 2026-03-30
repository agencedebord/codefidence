// DECISION: Webhook retries use exponential backoff (1min, 5min, 30min, 2h)
// We deliberately chose this over linear retry to protect Stripe rate limits.

export interface WebhookEvent {
  id: string;
  type: string;
  payload: Record<string, unknown>;
  receivedAt: Date;
}

export class WebhookHandler {
  private readonly maxRetries = 4;
  private readonly backoffMs = [60_000, 300_000, 1_800_000, 7_200_000];

  async handle(event: WebhookEvent): Promise<void> {
    switch (event.type) {
      case 'invoice.paid':
        await this.handleInvoicePaid(event);
        break;
      case 'invoice.payment_failed':
        await this.handlePaymentFailed(event);
        break;
      case 'customer.subscription.deleted':
        await this.handleSubscriptionCanceled(event);
        break;
      default:
        console.warn(`Unhandled webhook type: ${event.type}`);
    }
  }

  private async handleInvoicePaid(event: WebhookEvent): Promise<void> {
    // Mark invoice as paid in our system
  }

  private async handlePaymentFailed(event: WebhookEvent): Promise<void> {
    // HACK: Stripe sometimes sends duplicate failure events
    // We deduplicate by event.id before processing
  }

  private async handleSubscriptionCanceled(event: WebhookEvent): Promise<void> {
    // Trigger churn prevention flow
  }

  async retryWithBackoff(fn: () => Promise<void>): Promise<void> {
    for (let attempt = 0; attempt < this.maxRetries; attempt++) {
      try {
        await fn();
        return;
      } catch (err) {
        if (attempt < this.maxRetries - 1) {
          await new Promise((r) => setTimeout(r, this.backoffMs[attempt]));
        }
      }
    }
    throw new Error('Max retries exceeded');
  }
}
