import braintree from "braintree";
import type { PaymentsClient } from "../defs/payments/api/payments-client.js";
import { assert_never } from "../assert_never.js";
import { operation_rethrow } from "./error.js";

export const map_environment = (kind: "sandbox" | "production"): braintree.Environment => {
  if(kind === "sandbox") {
    return braintree.Environment.Sandbox;
  } else if (kind === "production") {
    return braintree.Environment.Production;
  } else {
    return assert_never(kind, "Braintree environment kind");
  }
}

export class BraintreePaymentsClient implements PaymentsClient {
  
  gateway: braintree.BraintreeGateway;

  constructor(config: {
    environment: "sandbox" | "production"
    merchant_id: string
    public_key: string
    private_key: string
  }) {
    this.gateway = new braintree.BraintreeGateway({
      environment: map_environment(config.environment),
      merchantId: config.merchant_id,
      publicKey: config.public_key,
      privateKey: config.private_key,
    })
  }

  generate_client_token: PaymentsClient["generate_client_token"] = async (query) => {
    
    const { customer_id } = query;
    const { clientToken } = await this.gateway.clientToken.generate({
      customerId: customer_id
    }).catch(operation_rethrow);

    return {
      client_token: clientToken,
    }
  }

  ensure_customer: PaymentsClient["ensure_customer"] = async (query) => {
    const { 
      customer_id,
      email,
      first_name,
      last_name,
    } = query;

    const customer = await this.gateway.customer.find(customer_id).catch((e: any) => {
      if(e?.type === "notFoundError") {
        return null;
      } else {
        throw e;
      }
    }).catch(operation_rethrow);

    if(customer) return { customer_id: customer.id };

    const res = await this.gateway.customer.create({
      id: customer_id,
      email,
      firstName: first_name,
      lastName: last_name
    }).catch(operation_rethrow)

    return {
      customer_id: res.customer.id
    }
  }

  save_payment_method: PaymentsClient["save_payment_method"] = async (query) => {
    
    const { 
      customer_id,
      payment_method_nonce,
      device_data,
    } = query;

    // options
    // failOnDuplicatePaymentMethod?: boolean | undefined;
    // makeDefault?: boolean | undefined;
    // verificationAmount?: string | undefined;
    // verificationMerchantAccountId?: string | undefined;
    // verifyCard?: boolean | undefined;
    
    const res = await this.gateway.paymentMethod.create({
      customerId: customer_id,
      paymentMethodNonce: payment_method_nonce,
      deviceData: device_data,
      options: {
        failOnDuplicatePaymentMethod: false,
        verifyCard: true,
      }
    }).catch(operation_rethrow)
    
    const creditCard = res.paymentMethod as braintree.CreditCard;

    return {
      card_type: creditCard.cardType,
      last_4: creditCard.last4,
      expiration_month: creditCard.expirationMonth || null,
      expiration_year: creditCard.expirationYear || null,
      payment_method_token: res.paymentMethod.token,
    }
  }
}