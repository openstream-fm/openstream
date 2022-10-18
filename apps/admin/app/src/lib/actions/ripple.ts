const defaults = {
  color: 'currentColor',
  class: '',
  opacity: 0.065,
  centered: false,
  spreadingDuration: '.4s',
  spreadingDelay: '0s',
  spreadingTimingFunction: 'ease-in-out',
  clearingDuration: '0.7s',
  clearingDelay: '0s',
  clearingTimingFunction: 'ease-in-out',
};

export type Options = typeof defaults;

/**
 * Creates a ripple element but does not destroy it (use RippleStop for that)
 *
 * @param {Event} e
 * @param {*} options
 * @returns Ripple element
 */
const start = (e: Event & any, options: Partial<Options> = {}) => {
  e.stopImmediatePropagation();
  const opts = { ...defaults, ...options };

  const isTouchEvent = e.touches ? !!e.touches[0] : false;
  // Parent element
  const target = isTouchEvent ? e.touches[0].currentTarget : e.currentTarget;

  // Create ripple
  const ripple = document.createElement('div');
  const rippleStyle = ripple.style;

  // Adding default stuff
  ripple.className = `material-ripple ${opts.class}`;
  rippleStyle.pointerEvents = "none";
  rippleStyle.position = 'absolute';
  rippleStyle.color = 'inherit';
  rippleStyle.borderRadius = '50%';
  rippleStyle.pointerEvents = 'none';
  rippleStyle.width = '100px';
  rippleStyle.height = '100px';
  rippleStyle.marginTop = '-50px';
  rippleStyle.marginLeft = '-50px';
  target.appendChild(ripple);
  rippleStyle.opacity = String(opts.opacity);
  rippleStyle.transition = `transform ${opts.spreadingDuration} ${opts.spreadingTimingFunction} ${opts.spreadingDelay},opacity ${opts.clearingDuration} ${opts.clearingTimingFunction} ${opts.clearingDelay}`;
  rippleStyle.transform = 'scale(0) translate(0,0)';
  rippleStyle.background = opts.color;

  // Positioning ripple
  const targetRect = target.getBoundingClientRect();
  if (opts.centered) {
    rippleStyle.top = `${targetRect.height / 2}px`;
    rippleStyle.left = `${targetRect.width / 2}px`;
  } else {
    const distY = isTouchEvent ? e.touches[0].clientY : e.clientY;
    const distX = isTouchEvent ? e.touches[0].clientX : e.clientX;
    rippleStyle.top = `${distY - targetRect.top}px`;
    rippleStyle.left = `${distX - targetRect.left}px`;
  }

  // Enlarge ripple
  rippleStyle.transform = `scale(${
    Math.max(targetRect.width, targetRect.height) * 0.02
  }) translate(0,0)`;
  return ripple;
}

const stop = (ripple: HTMLElement) => {
  if (ripple) {
    ripple.addEventListener('transitionend', (e: TransitionEvent) => {
      if (e.propertyName === 'opacity') ripple.remove();
    });
    ripple.style.opacity = "0";
  }
}

export const ripple = (node: HTMLElement, _options: Partial<Options> = {}) => {
  let options = _options;
  let destroyed = false;
  let ripple: HTMLElement | null = null;
  let keyboardActive = false;
  const handleStart = (e: Event) => {
    // ripple = start(e, options);
    if(ripple) return;
    ripple = start(e, options);
  };
  const handleStop = () => {
    if (ripple != null) {
      stop(ripple);
      ripple = null;
    }
  }
  const handleKeyboardStart = (e: KeyboardEvent) => {
    if(ripple) return;
    if (!keyboardActive && (e.key === "Enter")) {
      ripple = start(e, { ...options, centered: true });
      keyboardActive = true;
    }
  };
  const handleKeyboardStop = () => {
    keyboardActive = false;
    handleStop();
  };

  function setup() {
    node.classList.add('s-ripple-container');
    node.addEventListener('pointerdown', handleStart);
    node.addEventListener('pointerup', handleStop);
    node.addEventListener('pointerleave', handleStop);
    node.addEventListener('keydown', handleKeyboardStart);
    node.addEventListener('keyup', handleKeyboardStop);
    destroyed = false;
  }

  function destroy() {
    node.classList.remove('s-ripple-container');
    node.removeEventListener('pointerdown', handleStart);
    node.removeEventListener('pointerup', handleStop);
    node.removeEventListener('pointerleave', handleStop);
    node.removeEventListener('keydown', handleKeyboardStart);
    node.removeEventListener('keyup', handleKeyboardStop);
    destroyed = true;
  }

  if (options) setup();

  return {
    update(newOptions: Options) {
      options = newOptions;
      if (options && destroyed) setup();
      else if (!(options || destroyed)) destroy();
    },
    destroy,
  };
};