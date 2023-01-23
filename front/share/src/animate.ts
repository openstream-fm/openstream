import { flip as _flip, FlipParams as _FlipParams } from "svelte/animate";

export type FlipParams = _FlipParams & { disabled?: boolean };

export const flip = (node: Element, rects: { from: DOMRect, to: DOMRect }, params: FlipParams = {}) => {
  const { disabled = false, ..._params } = params;
  if(disabled) return { delay: 0, duration: 0 }
  else return _flip(node, rects, _params);
}
