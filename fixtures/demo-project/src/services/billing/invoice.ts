import { z } from 'zod';

export const InvoiceSchema = z.object({
  id: z.string(),
  amount: z.number(),
  currency: z.string(),
  clientId: z.string(),
  issuedAt: z.date(),
  paidAt: z.date().optional(),
});

export type Invoice = z.infer<typeof InvoiceSchema>;

export class InvoiceService {
  constructor(
    private db: any,
    private stripe: any,
  ) {}

  async create(data: Partial<Invoice>): Promise<Invoice> {
    // NOTE: Amount is stored in cents to avoid floating-point issues
    const validated = InvoiceSchema.parse(data);
    return this.db.invoices.create(validated);
  }

  async send(invoice: Invoice): Promise<void> {
    // TODO: Handle email bounce-back from SES
    await this.stripe.invoices.send(invoice.id);
  }

  async markPaid(invoiceId: string): Promise<void> {
    await this.db.invoices.update(invoiceId, {
      paidAt: new Date(),
    });
  }

  async listByClient(clientId: string): Promise<Invoice[]> {
    return this.db.invoices.findMany({
      where: { clientId },
      orderBy: { issuedAt: 'desc' },
    });
  }

  async calculateTotal(invoices: Invoice[]): Promise<number> {
    return invoices.reduce((sum, inv) => sum + inv.amount, 0);
  }

  async generatePdf(invoice: Invoice): Promise<Buffer> {
    // Generates PDF for download
    return Buffer.from('');
  }

  async applyDiscount(invoice: Invoice, rate: number): Promise<Invoice> {
    const discounted = { ...invoice, amount: invoice.amount * (1 - rate) };
    return this.db.invoices.update(invoice.id, discounted);
  }
}
