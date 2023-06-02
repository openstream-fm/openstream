<script lang="ts">
	export let data: import('./$types').PageData;
	import '$share/LoginDashboard/login-page.css';

	import { ripple } from '$share/ripple';
	import Email from '$lib/components/Form/Email.svelte';
	import Password from '$lib/components/Form/Password.svelte';
	import TextField from '$lib/components/Form/TextField.svelte';
	import { action, _post } from '$share/net.client';
	import { 
		mdiAccountOutline,
		// mdiPhoneOutline
	} from '@mdi/js';
	import Validator from '$share/formy/Validator.svelte';
	import {
		_confirmation_password,
		_email,
		_new_password,
		_new_user_email,
		_string
		// _phone,
	} from '$share/formy/validate';
	import Formy from '$share/formy/Formy.svelte';
	import '$share/LoginDashboard/login-page.css';
	import { goto } from '$app/navigation';
	import Color from 'color';
	import { scale } from 'svelte/transition';
	import CircularProgress from '$share/CircularProgress.svelte';
	import { lang, locale } from '$lib/locale';
	import { invalidate_siblings } from '$lib/invalidate';
	import { logical_fly } from '$share/transition';
	import Dropin from "$share/braintree/Dropin.svelte";

	let dropin: Dropin;
	let payment_nonce: string | null = null;
	let payment_device_data: string | null = null;

	let first_name = '';
	let last_name = '';
	let account_name = '';
	// let phone = '';
	let email = '';
	let password = '';
	let confirm_password = '';

	let email_verification_code = '';

	let animations = false;
	let view: 'data' | 'code' | 'pay' = 'data';

	const back_to_data = () => {
		animations = false;
		view = 'data';
		tick().then(() => {
			animations = true;
		});
	};

	const back_to_pay = () => {
		animations = false;
		email_verification_code = '';
		view = 'pay';
		tick().then(() => {
			animations = true;
		});
	};

	let sending_data = false;
	const submit_data = action(async () => {
		animations = false;
		view = 'pay';
		tick().then(() => {
			animations = true;
		});
	});

	let sending_pay = false;
	const submit_pay = action(async () => {
		if (sending_pay) return;
		sending_pay = true;
		try {
			try {
				const payment_result = await dropin.requestPaymentMethod();
				// console.log('payment result', payment_result);
				if (typeof payment_result?.nonce !== 'string') {
					throw new Error('Payment internal error: invalid response');
				} else {
					payment_nonce = payment_result.nonce;
					payment_device_data = payment_result.deviceData || null;
				}
			} catch (e) {
				sending_pay = false;
				console.warn('dropin.requestPaymentMethod() error', e);
				// we dont log a notifier message here as it automatically shows the error in the UI
				return;
			}

			let payload: import('$api/auth/email-verification/send-code/POST/Payload').Payload = {
				email
			};

			await _post(`/api/auth/email-verification/send-code`, payload);

			sending_pay = false;

			animations = false;
			view = 'code';
			tick().then(() => {
				animations = true;
			});
		} catch (e) {
			sending_pay = false;
			throw e;
		}
	});

	let sending_code = false;
	const submit_code = action(async () => {
		if (sending_code) return;
		sending_code = true;
		try {
			const payload: Omit<import('$api/auth/user/register/POST/Payload').Payload, 'device_id'> = {
				plan_id: data.plan._id,
				first_name,
				last_name,
				account_name,
				phone: null,
				email,
				password,
				email_verification_code: email_verification_code.trim(),
				payment_method_nonce: payment_nonce!,
				payment_device_data: payment_device_data,
			};

			const { account } = await _post<import('$api/auth/user/register/POST/Output').Output>(
				'/api/auth/user/register',
				payload
			);
			sending_code = false;
			goto(`/accounts/${account._id}/welcome`, { invalidateAll: true });
			invalidate_siblings();
		} catch (e) {
			sending_code = false;
			throw e;
		}
	});

	let color: Color;
	try {
		color = new Color(data.plan.color);
	} catch (e) {
		color = new Color('#000');
	}

	const bg_color = color.alpha(0.1).toString();

	import { form } from '../../../transitions';
	import { tick } from 'svelte';

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

	.plan-back:hover {
		background: rgba(0, 0, 0, 0.05);
	}

	.code-message {
		margin-top: 2rem;
		text-align: center;
		width: min(90%, 500px);
	}

	.code-message > :global(b) {
		word-break: break-all;
	}

	.code-fields {
		margin-top: -1rem;
	}
	.code-input {
		font-size: 2rem;
		padding: 1rem;
		border-radius: 0.5rem;
		letter-spacing: 0.75rem;
		width: 6.75em;
		border: 2px solid #bbb;
		outline: 0;
		transition: border-color 200ms ease;
	}

	.code-input::placeholder {
		color: #bbb;
	}

	.code-input:focus {
		border-color: var(--blue);
	}

	.login-page-button {
		margin: 2rem 3rem 0 0;
	}

	.back-to {
		margin-top: 1rem;
		font-size: 0.9rem;
		color: #444;
	}

	.back-to:hover {
		text-decoration: underline;
	}

	.payment-message {
		text-align: center;
		max-width: 80%;
		margin-inline: auto;
		margin-block-start: -1.75rem;
		margin-block-end: 1rem;
		font-size: 0.9rem;
		color: #444;
	}
	
	.dropin-out {
		min-height: 10rem;
		padding: 0 2.5rem;
		width: 100%;
	}
</style>

<svelte:head>
	<title>{$locale.pages.register.head.title}</title>
</svelte:head>

<div class="login-page-box" in:form>
	{#if view === 'data' || view === 'pay'}
		<div class="login-page-title" in:logical_fly|local={{ duration: 250, x: -25 }}>
			{$locale.pages.register.title}
		</div>

		<div
			class="plan"
			style:--bg-color={bg_color}
			style:--color={color.toString()}
			in:logical_fly|local={{ duration: 250, x: -25 }}
		>
			<div class="plan-pretitle">
				{$locale.pages.register.plan.selected_plan}
			</div>
			<div class="plan-title">
				{data.plan.display_name}
			</div>
			<div class="plan-price">
				{$locale.pages.register.plan.n_per_month.replace('@n', format_price(data.plan.price))}
			</div>
			<div class="plan-features">
				<div class="plan-feature">
					<b>{data.plan.limits.stations}</b>
					{data.plan.limits.stations === 1
						? $locale.pages.register.plan.limits.station
						: $locale.pages.register.plan.limits.stations}
				</div>
				<div class="plan-feature">
					<b>
						{new Intl.NumberFormat().format(data.plan.limits.listeners)}
					</b>
					{$locale.pages.register.plan.limits.listeners}
				</div>
				<div class="plan-feature">
					<b>{data.plan.limits.transfer / 1_000_000_000_000} TB</b>
					{$locale.pages.register.plan.limits.transfer}
				</div>
				<div class="plan-feature">
					<b>{data.plan.limits.storage / 1_000_000_000} GB</b>
					{$locale.pages.register.plan.limits.storage}
				</div>
			</div>

			<a href="/plans" class="na plan-back ripple-container" use:ripple>
				{$locale.pages.register.plan.links.plans}
			</a>
		</div>
	{/if}

	<Formy action={submit_data} let:submit>
		<form
			novalidate
			on:submit={submit}
			class="view view-data"
			class:animations
			class:active={view === 'data'}
		>
			<h2>{$locale.pages.register.form.title}</h2>

			<div class="login-page-fields">
				<div class="login-page-field">
					<TextField
						label={$locale.pages.register.form.fields.first_name}
						trim
						icon={mdiAccountOutline}
						autocomplete="given-name"
						bind:value={first_name}
					/>
					<Validator value={first_name} fn={_string({ required: true, maxlen: 50 })} />
				</div>
				<div class="login-page-field">
					<TextField
						label={$locale.pages.register.form.fields.last_name}
						trim
						icon={mdiAccountOutline}
						autocomplete="family-name"
						bind:value={last_name}
					/>
					<Validator value={last_name} fn={_string({ required: true, maxlen: 50 })} />
				</div>
				<div class="login-page-field">
					<TextField
						label={$locale.pages.register.form.fields.account_name}
						trim
						icon={mdiAccountOutline}
						autocomplete="off"
						bind:value={account_name}
					/>
					<div class="org-explain">
						{$locale.pages.register.form.account_name_comment}
					</div>
					<Validator value={account_name} fn={_string({ required: true, maxlen: 50 })} />
				</div>
				<!-- <div class="login-page-field">
					<TextField
						type="tel"
						label={$locale.pages.register.form.fields.phone}
						icon={mdiPhoneOutline}
						autocomplete="tel"
					/>
						bind:value={phone}
					<Validator value={phone} fn={_phone({ required: true })} />
				</div> -->
				<div class="login-page-field">
					<Email label={$locale.pages.register.form.fields.email} bind:value={email} />
					<Validator value={email} fn={_new_user_email()} />
				</div>
				<div class="login-page-field">
					<Password
						label={$locale.pages.register.form.fields.password}
						autocomplete="new-password"
						bind:value={password}
					/>
					<Validator value={password} fn={_new_password({ minlen: 8, maxlen: 50 })} />
				</div>
				<div class="login-page-field">
					<Password
						label={$locale.pages.register.form.fields.confirm_password}
						autocomplete="new-password"
						bind:value={confirm_password}
					/>
					<Validator value={{ password, confirm_password }} fn={_confirmation_password()} />
				</div>
			</div>

			<button
				type="submit"
				class="ripple-container login-page-button"
				class:sending={sending_data}
				use:ripple
			>
				{#if sending_data}
					<div class="login-page-btn-sending-progress" transition:scale|local={{ duration: 300 }}>
						<CircularProgress />
					</div>
				{/if}
				{$locale.pages.register.form.next}
			</button>
		</form>
	</Formy>

	<Formy action={submit_pay} let:submit>
		<form
			novalidate
			on:submit={submit}
			class="view view-pay"
			class:animations
			class:active={view === 'pay'}
		>
			<h2>{$locale.pages.register.pay.title}</h2>

			<div class="payment-message">
				{$locale.pages.register.pay.message}
			</div>

			<div class="dropin-out">
				<Dropin authorization="sandbox_d58xyrp3_xbw6cq92jcgfmzdh" bind:this={dropin} lang={$lang} />
			</div>

			<button class="back-to ripple-container" use:ripple on:click|preventDefault={back_to_data}>
				{$locale.pages.register.back}
			</button>

			<button
				type="submit"
				class="ripple-container login-page-button"
				class:sending={sending_pay}
				use:ripple
			>
				{#if sending_pay}
					<div class="login-page-btn-sending-progress" transition:scale|local={{ duration: 300 }}>
						<CircularProgress />
					</div>
				{/if}
				{$locale.pages.register.form.next}
			</button>
		</form>
	</Formy>

	<Formy action={submit_code} let:submit>
		<form
			novalidate
			on:submit={submit}
			class="view view-code"
			class:active={view === 'code'}
			class:animations
		>
			<h2>
				{$locale.pages.register.verification.title}
			</h2>

			<div class="code-fields">
				<input
					type="text"
					class="code-input"
					bind:value={email_verification_code}
					placeholder="XXXXXX"
					maxlength={6}
				/>
				<Validator value={email_verification_code.trim()} fn={_string({ required: true })} />
			</div>

			<div class="code-message">
				{@html $locale.pages.register.verification.message_html.replace('@email', email)}
			</div>

			<button class="back-to ripple-container" use:ripple on:click|preventDefault={back_to_pay}>
				{$locale.pages.register.back}
			</button>

			<button
				type="submit"
				class="ripple-container login-page-button code-submit-btn"
				class:sending={sending_code}
				use:ripple
			>
				{#if sending_code}
					<div class="login-page-btn-sending-progress" transition:scale|local={{ duration: 300 }}>
						<CircularProgress />
					</div>
				{/if}
				{$locale.pages.register.verification.submit}
			</button>
		</form>
	</Formy>

	<div class="login-page-switch-box">
		<span class="login-page-comment">
			{$locale.pages.register.links.login_comment}
		</span>
		<a class="na login-page-link sign-in" href="/login">
			{$locale.pages.register.links.login_link}
		</a>
	</div>
</div>
