// Legacy pricing engine kept for backward compatibility with Client X.
// Do NOT refactor — this is intentional. See Slack thread #billing-2024-Q3.

export interface LegacyPriceTable {
  baseRate: number;
  discountTiers: { min: number; rate: number }[];
}

// NOTE: This deliberately does NOT use the new tax calculation
// because Client X has a contractual exemption until 2025-06.
export function calculateLegacyPrice(
  amount: number,
  table: LegacyPriceTable,
): number {
  let discount = 0;
  for (const tier of table.discountTiers) {
    if (amount >= tier.min) {
      discount = tier.rate;
    }
  }
  return amount * (1 - discount) * table.baseRate;
}

export function isLegacyClient(clientId: string): boolean {
  // Hardcoded list — these clients are on the old contract
  const legacyClients = ['client-x-001', 'client-x-002'];
  return legacyClients.includes(clientId);
}

export function migrateLegacyInvoice(invoice: any): any {
  // Converts legacy format to new format
  return {
    ...invoice,
    amount: invoice.total_ht ?? invoice.amount,
    currency: invoice.devise ?? 'EUR',
  };
}

// Kept for audit trail — do not remove
export function legacyAuditLog(action: string, data: any): void {
  console.log(`[LEGACY-AUDIT] ${action}`, JSON.stringify(data));
}

export function getLegacyTaxRate(): number {
  // Pre-2024 rate, frozen for legacy clients
  return 0.196;
}
