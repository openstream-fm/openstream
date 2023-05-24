<script lang="ts">
	export let data: import("./$types").PageData;  
  import "$share/LoginDashboard/login-page.css";

  import { ripple } from "$share/ripple";
  import TextField from "$lib/components/Form/TextField.svelte";
	import { action, _post } from "$share/net.client";
	import { mdiAccountOutline } from "@mdi/js";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
	import Formy from "$share/formy/Formy.svelte";
  import { goto } from "$app/navigation";
	import Color from "color";
  import "$share/LoginDashboard/login-page.css";
	import { invalidateSiblings } from "$lib/invalidate";
	import { lang, locale } from "$lib/locale";
	import { logical_fly } from "$share/transition";
	import { tick } from "svelte";
	
  let account_name = "";
  // let sending_data = false;

  let payment_nonce: string | null = null;
  let dropin: any | null = null;

  let animations = false;

  let view: "data" | "pay" = "data";
  
  const send_data = action(async () => {
    animations = false;
    view = "pay";
    tick().then(() => {
      animations = true;
    })
  })

  const back_to_data = () => {
    animations = false;
    view = "data";
    tick().then(() => {
      animations = true;
    }) 
  }
  
  let sending_pay = false;
  
  const send_pay = action(async () => {
    
    if (sending_pay) return;
		sending_pay = true;

		try {
			if (dropin == null) throw new Error('Payment datails error: collector not created');

			try {
				const payment_result = await dropin.requestPaymentMethod();
				console.log('payment result', payment_result);
				if (typeof payment_result?.nonce !== 'string') {
					throw new Error('Payment internal error: invalid response');
				} else {
					payment_nonce = payment_result.nonce;
				}
			} catch (e) {
				sending_pay = false;
				console.warn('dropin.requestPaymentMethod() error', e);
				// we dont log a notifier message here as it automatically shows the error in the UI
				return;
			}

			const payload: import("$api/accounts/POST/Payload").Payload = {
        plan_id: data.plan._id,
        name: account_name,
      };

      const { account } = await _post<import("$api/accounts/POST/Output").Output>("/api/accounts", payload);

      sending_pay = false;

      goto(`/accounts/${account._id}`, { invalidateAll: true });
      invalidateSiblings();
		} catch (e) {
			sending_pay = false;
			throw e;
		}
  })

  let color: Color;
  try {
    color = new Color(data.plan.color);
  } catch(e) {
    color = new Color("#000")
  }

  const bg_color = color.alpha(0.1).toString();

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

  $: dropin_locale = dropin_locales[$lang] || undefined;

  const braintree_hydrate = (node: HTMLElement) => {
		const fn = async () => {
			// @ts-ignore
			dropin = await window.braintree.dropin.create({
				authorization: 'sandbox_d58xyrp3_xbw6cq92jcgfmzdh',
				container: node,
				locale: dropin_locale,
				card: {
					clearFieldsAfterTokenization: false
				}
			});
		};

		// @ts-ignore
		if (window.braintree?.dropin == null) {
			const s = document.createElement('script');
			s.src = 'https://js.braintreegateway.com/web/dropin/1.37.0/js/dropin.min.js';
			document.head.appendChild(s);
			s.onload = fn;
			s.onerror = (e) => {
				console.warn('error loading braintree dropin:', e);
			};
		} else {
			fn();
		}
	};
</script>

<style>

  .page {
    display: flex;
    flex-direction: column;
    padding: 4rem 0 6rem 0;
  }

  .view {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .view:not(.active) {
		display: none;
	}

	.animations {
		animation-name: view-enter;
		animation-duration: 200ms;
		animation-timing-function: ease;
		animation-fill-mode: forwards;
	}

	@keyframes view-enter {
		0% {
			opacity: 0;
			transform: translateX(-25px);
		}

		100% {
			opacity: 1;
			transform: none;
		}
	}

  h2 {
    font-weight: 600;
    font-size: 1.5rem;
    text-align: center;
    margin: 4rem 0 3rem 0;
    padding: 0 1.5rem;
  }

  .view-pay h2 {
    margin-bottom: 1rem;
  }

  .org-explain {
    color: #999;
    font-size: 0.8rem;
    margin: 0.5rem 0.25rem;
  }

  .plan {
    align-self: stretch;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: -1rem 0 0 0;
    background: var(--bg-color);
    padding: 2rem 0;
  }

  .plan-pretitle {
    font-weight: 600;
    font-size: 1.5rem;
  }

  .plan-title {
    color: var(--color);
    font-size: 1.5rem;
    font-weight: 900;
    margin-top: 1rem;
  }

  .plan-price {
    font-weight: 700;
    font-size: 1.1rem;
    margin-top: 0.75rem;
  }

  .plan-features {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 0.5rem;
  }

  .plan-feature > b {
    font-weight: 700;
  }

  .plan-back {
    margin-top: 1rem;
    font-size: 0.9rem;
    border-radius: 0.25rem;
    padding: 0.5rem 1rem;
    align-self: center;
    transition: background-color 200ms ease;
  }

  .login-page-button {
		margin: 2rem 3rem 0 0;
	}

  .plan-back:hover {
    background: rgba(0,0,0,0.05); 
  }

  .back-to {
		margin-top: 1rem;
		font-size: 0.95rem;
    color: #444;
	}

  .back-to:hover {
    text-decoration: underline;
  }

  .braintree-dropin-out {
		min-height: 10rem;
		padding: 0 2.5rem;
		width: 100%;
	}
	.braintree-dropin-out :global(*) {
		font-family: inherit !important;
	}

	.braintree-dropin :global(.braintree-sheet__header-label) {
		display: none !important;
	}

	.braintree-dropin :global(.braintree-sheet__content--form) {
		padding: 1.5rem 1rem;
	}

	.braintree-dropin :global(.braintree-form__flexible-fields),
	.braintree-dropin :global(.braintree-form__field-group:not(:first-child)) {
		margin-block-start: 1.5rem !important;
	}

	.braintree-dropin :global(.braintree-form__field-group),
	.braintree-dropin :global(.braintree-form__flexible-fields) {
		margin-block-end: 0;
	}
</style>

<svelte:head>
  <title>{$locale.pages["accounts/create_account/plan"].head.title}</title>
</svelte:head>

<div class="page" in:logical_fly|local={{ y: -25, duration: 200 }}>
  <div class="login-page-box">
    <div class="login-page-title">
      {$locale.pages["accounts/create_account/plan"].title}
    </div>

    <div class="plan" style:--bg-color={bg_color} style:--color={color.toString()}>
      <div class="plan-pretitle">
        {$locale.pages["accounts/create_account/plan"].plan.title}
      </div>
      <div class="plan-title">
        {data.plan.display_name}
      </div>
      <div class="plan-price">
        {$locale.pages["accounts/create_account/plan"].plan.$_n_per_month.replace("@n", String(data.plan.price))}
      </div>
      <div class="plan-features">
        <div class="plan-feature">
          <b>{data.plan.limits.stations}</b>
          {
            data.plan.limits.stations === 1 ? 
            $locale.pages["accounts/create_account/plan"].plan.station :
            $locale.pages["accounts/create_account/plan"].plan.stations
          }
        </div>
        <div class="plan-feature">
          <b>
            {new Intl.NumberFormat().format(data.plan.limits.listeners)}
          </b>
          {$locale.pages["accounts/create_account/plan"].plan.listeners}
        </div>
        <div class="plan-feature">
          <b>
            {data.plan.limits.transfer / 1_000_000_000_000} TB
          </b>
          {$locale.pages["accounts/create_account/plan"].plan.transfer}
        </div>
        <div class="plan-feature">
          <b>
            {data.plan.limits.storage / 1_000_000_000} GB
          </b>
          {$locale.pages["accounts/create_account/plan"].plan.storage}
        </div>
      </div>

      <a href="/accounts/create-account" class="na plan-back ripple-container" use:ripple>
        {$locale.pages["accounts/create_account/plan"].plan.back}
      </a>
    </div>

    <Formy action={send_data} let:submit>
      <form
        class="view view-data"
        class:animations
        class:active={view === "data"}
        on:submit={submit}
      >
        <h2>{$locale.pages["accounts/create_account/plan"].form.title}</h2>
        
        <div class="login-page-fields">
          <div class="login-page-field">
            <TextField
              label={$locale.pages["accounts/create_account/plan"].form.fields.account_name} 
              trim
              icon={mdiAccountOutline}
              autocomplete="off"
              bind:value={account_name}
            />
            <div class="org-explain">
              {$locale.pages["accounts/create_account/plan"].form.fields.account_name_message}
            </div>
            <Validator value={account_name} fn={_string({ required: true, maxlen: 50 })} />
          </div>
        </div>
        <button type="submit" class="ripple-container login-page-button" use:ripple>
          {$locale.pages["accounts/create_account/plan"].form.next}
        </button>
      </form>
    </Formy>

    <Formy action={send_pay} let:submit>
      <form
        novalidate
        class="view view-pay"
        class:animations
        class:active={view === "pay"}
        on:submit={submit}
      >
        <h2>{$locale.pages["accounts/create_account/plan"].form.pay.title}</h2>

        <div class="braintree-dropin-out">
          <div class="braintree-dropin" use:braintree_hydrate />
        </div>

        <button class="back-to" on:click|preventDefault={() => back_to_data()}>
          {$locale.pages["accounts/create_account/plan"].form.back}
        </button>

        <button type="submit" class="ripple-container login-page-button" use:ripple>
          {$locale.pages["accounts/create_account/plan"].form.submit}
        </button>
      </form>
    </Formy>
  </div>
</div>