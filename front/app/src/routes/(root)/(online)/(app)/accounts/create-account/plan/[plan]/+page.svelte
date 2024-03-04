<script lang="ts">
	export let data: import("./$types").PageData;  
  import "$share/LoginDashboard/login-page.css";

  import { ripple } from "$share/ripple";
  import TextField from "$share/Form/TextField.svelte";
	import { action, _post } from "$share/net.client";
	import { mdiAccountOutline } from "@mdi/js";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
	import Formy from "$share/formy/Formy.svelte";
  import { goto } from "$app/navigation";
	import Color from "color";
  import "$share/LoginDashboard/login-page.css";
	import { invalidate_siblings } from "$lib/invalidate";
	import { lang, locale } from "$lib/locale";
	import { logical_fly } from "$share/transition";
  // TODO: payments
  // import PaymentMethodSelector from "$share/braintree/PaymentMethodSelector.svelte";
	import { display_fly_enter } from "$share/display_transitions";
	import { VALIDATE_ACCOUNT_NAME_MAX_LEN } from "$server/defs/constants";
	import { POST, unwrap } from "$lib/client";
  
  let account_name = "";
  // let sending_data = false;

  // TODO: payments
  // let selector: PaymentMethodSelector;
  // let payment_method_id: string | null = null;
    
  // let payment_nonce: string | null = null;
  // let payment_device_data: string | null = null;
  // let dropin: Dropin;

  // TODO: payments
  // let view: "data" | "pay" = "data";
  let view = "data";

  // TODO: payments
  // const send_data = action(async () => {
  //   view = "pay";
  // })
  // const back_to_data = () => {
  //   view = "data"; 
  // }
  // let sending_pay = false;
  let sending_data = false;

  // TODO: payments
  // const send_pay = action(async () => {
  const send_data = action(async () => {
    //  TODO: payments
    // if (sending_pay) return;
		// sending_pay = true;
    if (sending_data) return;
    sending_data = true;

    try {
      // TODO: payments
      // try {
      //   payment_method_id = await selector.requestMethodId();
      // } catch(e) {
      //   sending_pay = false;
      //   // we dont log a notifier message here as it automatically shows the error in the UI
      //   return;
      // }

      const { account } = unwrap(await POST("/accounts", {
        body: {
          plan_id: data.plan._id,
          // TODO: payments
          // payment_method_id: null,
          name: account_name
        }
      }));
      
      // TODO: payments
      // sending_pay = false;
      sending_data = false;

      goto(`/accounts/${account._id}`, { invalidateAll: true });
      invalidate_siblings();
    } catch (e) {
      // TODO: payments
      // sending_pay = false;
      sending_data = false;
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

  const format_price = (price: number): string => {
    return new Intl.NumberFormat($lang, {
      style: "currency",
      maximumFractionDigits: 0,
      minimumFractionDigits: 0,
      currency: "USD",
    }).format(price);
  }
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

  h2 {
    font-weight: var(--font-bold);
    font-size: 1.5rem;
    text-align: center;
    margin: 4rem 0 3rem 0;
    padding: 0 1.5rem;
  }

  /* TODO: payments
  .view-pay h2 {
    margin-bottom: 1rem;
  }
  */

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
    font-weight: var(--font-bold);
    font-size: 1.5rem;
  }

  .plan-title {
    color: var(--color);
    font-size: 1.5rem;
    font-weight: 900;
    margin-top: 1rem;
  }

  .plan-price {
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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

  /* TODO: payments 
  .back-to {
		margin-top: 1rem;
		font-size: 0.9rem;
    color: #444;
	}

  .back-to:hover {
    text-decoration: underline;
  }

  .dropin-out {
		min-height: 10rem;
		padding: 0 2.5rem;
		width: 100%;
	} */
</style>

<svelte:head>
  <title>{$locale.pages["accounts/create_account/plan"].head.title}</title>
</svelte:head>

<div class="page" in:logical_fly={{ y: -25, duration: 200 }}>
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
        {$locale.pages["accounts/create_account/plan"].plan.n_per_month.replace("@n", format_price(data.plan.price))}
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
        class:active={view === "data"}
        use:display_fly_enter={{ start: false, show: view === "data", duration: 200, x: -25 }}
        on:submit={submit}
      >
        <h2>{$locale.pages["accounts/create_account/plan"].form.title}</h2>
        
        <div class="login-page-fields">
          <div class="login-page-field">
            <TextField
              label={$locale.pages["accounts/create_account/plan"].form.fields.account_name} 
              trim
              icon={mdiAccountOutline}
              maxlength={VALIDATE_ACCOUNT_NAME_MAX_LEN}
              autocomplete="off"
              bind:value={account_name}
            />
            <div class="org-explain">
              {$locale.pages["accounts/create_account/plan"].form.fields.account_name_message}
            </div>
            <Validator value={account_name} fn={_string({
                required: true,
                maxlen: VALIDATE_ACCOUNT_NAME_MAX_LEN
              })}
            />
          </div>
        </div>
        <button type="submit" class="ripple-container login-page-button" use:ripple>
          <!--
          {$locale.pages["accounts/create_account/plan"].form.next}
          -->
          {$locale.pages["accounts/create_account/plan"].form.submit}
        </button>
      </form>
    </Formy>

  <!-- TODO: payments
   <Formy action={send_pay} let:submit>
      <form
        novalidate
        class="view view-pay"
        class:active={view === "pay"}
        use:display_fly_enter={{ start: false, show: view === "pay", duration: 200, x: -25 }}
        on:submit={submit}
      >
        <h2>{$locale.pages["accounts/create_account/plan"].form.pay.title}</h2>

        <div class="dropin-out">
          <PaymentMethodSelector
            locale={$locale.payments}
            lang={$lang}
            authorization="sandbox_d58xyrp3_xbw6cq92jcgfmzdh"
            bind:saved_methods={data.payment_methods.items}
            bind:this={selector}
          />
          <-- <Dropin authorization="sandbox_d58xyrp3_xbw6cq92jcgfmzdh" bind:this={dropin}  /> ->
        </div>

        <button class="back-to" on:click|preventDefault={() => back_to_data()}>
          {$locale.pages["accounts/create_account/plan"].form.back}
        </button>

        <button type="submit" class="ripple-container login-page-button" use:ripple>
          {$locale.pages["accounts/create_account/plan"].form.submit}
        </button>
      </form>
    </Formy> -->
  </div>
</div>