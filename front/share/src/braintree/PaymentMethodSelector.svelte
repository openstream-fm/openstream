<script lang="ts">

  export let lang: string;
  export let authorization: string | (() => Promise<string>);
  export let saved_methods: PublicPaymentMethod[];
  export let selected_method: PublicPaymentMethod | null = null;
  export let user_id: string | null = null;
  export let locale: import("$server/locale/share/payments/payments.locale").PaymentsLocale;

  let stage: "saved" | "new" = saved_methods.length === 0 ? "new" : "saved"; 

  $: if(saved_methods.length === 0) {
    stage = "new";
  }

  import Dropin from "./Dropin.svelte";
  import { display_fly_enter } from "$share/display_transitions";
  import { _post } from "$share/net.client";
  import { _error } from "$share/notify";
  import Card from "./Card.svelte";

  let dropin: Dropin;

  type PublicPaymentMethod = import("$server/defs/PublicPaymentMethod").PublicPaymentMethod;

  export const requestMethodId = async (): Promise<string> => {
    
    if(stage === "saved") {
     
      if(selected_method) {
        return selected_method._id;
      } else {
        _error(locale.no_method_error_messaage);
        throw new Error(locale.no_method_error_messaage);
      }
    
    } else {
      
      const { nonce, deviceData } = await dropin.requestPaymentMethod();
      
      try {
        const payload: import("$server/defs/api/payment-methods/POST/Payload").Payload = {
          nonce: nonce,
          device_data: deviceData!,
          user_id: user_id ?? undefined,
        };

        const { payment_method }: import("$server/defs/api/payment-methods/POST/Output").Output = await _post("/api/payment-methods", payload);
        saved_methods = [...saved_methods, payment_method];
        selected_method = payment_method;
        stage = "saved";
        dropin.clearSelectedPaymentMethod();

        return payment_method._id;
        
      } catch(e: any) {
        _error(String(e?.message));
        throw e;
      }
    }
  }

  const toggle = (method: PublicPaymentMethod) => {
    if(method._id === selected_method?._id) {
      selected_method = null;
    } else {
      selected_method = method;
    }
  }
</script>

<style>
  
  .stage {
    display: flex;
    flex-direction: column;
  }
  
  .stage:not(.stage-on) {
    display: none;
  }

  .stage-title {
    font-size: 1.15rem;
    font-weight: 600;
    margin-top: 1rem;
    align-self: center;
    text-align: center;
  }

  .stage-link {
    align-self: center;
    text-align: center;
    padding: 0.5rem 0.75rem;
    cursor: pointer;
    color: #333;
    margin-bottom: 1rem;
    margin-top: 1rem;
  }

  .stage-link:hover {
    text-decoration: underline;
  }

  .dropin {
    align-self: stretch;
  }

  .saved-items {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 0.75rem;
    margin-top: 1.25rem;
  }

  .card {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }
</style>

<div class="payment-method-selector">

  <div class="stage stage-saved" class:stage-on={stage === "saved"} use:display_fly_enter={{ start: false, show: stage === "saved", duration: 200, x: -25 }}>
    <div class="stage-title">
      {locale.Select_a_payment_method}
    </div>

    <div class="saved-items">
      {#each saved_methods as method}
        <div class="card">
          <Card
            card={method}
            selected={method._id === selected_method?._id}
            on_click={() => toggle(method)}
            {locale}
          />
        </div>
      {/each}
    </div>

    <button class="stage-link new-link" on:click|preventDefault={() => stage = "new"}>
      {locale.or_add_a_new_payment_method}
    </button>
  </div>

  <div class="stage stage-dropin" class:stage-on={stage === "new"} use:display_fly_enter={{ start: false, show: stage === "new", duration: 200, x: -25 }}>

    {#if saved_methods.length}
      <div class="stage-title">
        {locale.Add_a_payment_method}
      </div>
    {/if}

    <div class="dropin">
      <Dropin bind:this={dropin} {lang} {authorization} />
    </div>
    
    {#if saved_methods.length}
      <button class="stage-link saved-link" on:click|preventDefault={() => stage = "saved"}>
        {locale.or_use_a_saved_payment_method}
      </button>
    {/if}
  </div>
</div>

