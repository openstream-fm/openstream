// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type PublicPaymentMethodKind = { kind: "card" } & {
  card_type: string;
  last_4: string;
  expiration_year: string | null | undefined;
  expiration_month: string | null | undefined;
};
