<script lang="ts">
  export let lang: string;
  export let authorization: string | (() => Promise<string>);

  let state: "loading" | "ok" | "error" = "loading"; 
  let error: Error | null = null;

  import type { Dropin } from "braintree-web-drop-in";
  let dropin: Dropin | null;
  // let payment_nonce: string | null = null;

  let loading_resolve: (instance: Dropin) => void;
  let loading_reject: (e: Error) => void;
  const loading_promise = new Promise<Dropin>((resolve, reject) => {
    loading_resolve = resolve;
    loading_reject = reject;
  });

  const logger = default_logger.scoped("bt");
  
  const dropin_locales: Partial<Record<string, string>> = {
		en: 'en_US',
		'es-AR': 'es_ES',
		es: 'es_ES',
		pt: 'pt_BR',
		fr: 'fr_FR',
		it: 'it_IT',
		de: 'de_DE',
		ar: 'ar_EG',
		zh: 'zh_CN'
	};

	$: dropin_locale = dropin_locales[lang] || undefined;

  import { default_logger } from "$share/logger";

  const hydrate = (node: HTMLElement) => {
    const fn = async () => {
      
      const Dropin = await import("braintree-web-drop-in");
      logger.info("dropin loaded");
      
      let resolved_authorization: string;
      if(typeof authorization === "string") {
        resolved_authorization = authorization;
      } else {
        try {
          resolved_authorization = await authorization();
        } catch(e: any) {
          state = "error";
          error = e;
          loading_reject(e);
          logger.warn("error getting authorization");
          logger.error(e);
          throw e;
        }
      }

      try {
        dropin = await Dropin.create({
          dataCollector: true,
          preselectVaultedPaymentMethod: true,
          paymentOptionPriority: ["card"],
          // authorization: "sandbox_d58xyrp3_xbw6cq92jcgfmzdh",
          authorization: resolved_authorization,
          container: node,
          locale: dropin_locale,
          card: { clearFieldsAfterTokenization: false },
        });
        state = "ok"
        loading_resolve(dropin);
      } catch(e: any) {
        state = "error";
        error = e;
        loading_reject(e);
        logger.warn("error creating dropin");
        logger.error(e);
        throw e;
      }
    };

    fn();
  };

  export const requestPaymentMethod = async () => {
    const instance = await loading_promise;
    return await instance.requestPaymentMethod()
  }

  export const clearSelectedPaymentMethod = async () => {
    const instance = await loading_promise;
    return instance.clearSelectedPaymentMethod();
  }
</script>

<style>
	.dropin, .dropin :global(*) {
		font-family: inherit !important;
	}

	.dropin :global(.braintree-sheet__header-label) {
		display: none !important;
	}

	.dropin :global(.braintree-sheet__content--form) {
		padding: 1.5rem 1rem;
	}

	.dropin :global(.braintree-form__flexible-fields),
	.dropin :global(.braintree-form__field-group:not(:first-child)) {
		margin-block-start: 1.5rem !important;
	}

	.dropin :global(.braintree-form__field-group),
	.dropin :global(.braintree-form__flexible-fields) {
		margin-block-end: 0;
	}
</style>

<div class="dropin" use:hydrate />
