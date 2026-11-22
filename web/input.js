export const actions = Object.freeze(['Up', 'Down', 'Left', 'Right', 'Primary', 'Secondary', 'Menu', 'SoftLeft', 'SoftRight']);
const keys = new Map([['ArrowUp', 0], ['ArrowDown', 1], ['ArrowLeft', 2], ['ArrowRight', 3], ['KeyZ', 4], ['KeyX', 5], ['Enter', 6], ['KeyQ', 7], ['KeyW', 8]]);
export function keyAction(code) { return keys.get(code); }

// Cardinal sectors with a small neutral center; dragging keeps pointer capture.
export function padAction(x, y, rect) {
  const dx = (x - rect.left) / rect.width - .5;
  const dy = (y - rect.top) / rect.height - .5;
  if (Math.abs(dx) > .7 || Math.abs(dy) > .7 || Math.max(Math.abs(dx), Math.abs(dy)) < .12) return -1;
  return Math.abs(dx) > Math.abs(dy) ? (dx < 0 ? 2 : 3) : (dy < 0 ? 0 : 1);
}

export function bindControls(deck, handlers) {
  const events = new AbortController();
  const listen = (target, type, callback) => target.addEventListener(type, callback, { signal: events.signal });
  const held = new Map();
  const pointers = new Map();
  const pad = deck.querySelector('.dpad');
  const buttons = [...deck.querySelectorAll('[data-action]')];
  function update(source, action) {
    if (held.get(source) === action || (!held.has(source) && action === -1)) return;
    if (action === -1) held.delete(source); else held.set(source, action);
    handlers.input(source, action);
    for (const button of buttons) button.classList.toggle('pressed', [...held.values()].includes(Number(button.dataset.action)));
  }
  function release(event) {
    if (!pointers.has(event.pointerId)) return;
    pointers.delete(event.pointerId);
    update(`pointer:${event.pointerId}`, -1);
  }
  listen(deck, 'pointerdown', event => {
    if (!handlers.active() || (event.pointerType === 'mouse' && event.button !== 0)) return;
    const button = event.target.closest('[data-action]');
    const isPad = pad.contains(event.target);
    if ((!button && !isPad) || button?.disabled) return;
    event.preventDefault();
    const target = isPad ? pad : button;
    pointers.set(event.pointerId, { target, isPad });
    target.setPointerCapture(event.pointerId);
    update(`pointer:${event.pointerId}`, isPad ? padAction(event.clientX, event.clientY, pad.getBoundingClientRect()) : Number(button.dataset.action));
  });
  listen(deck, 'pointermove', event => {
    if (pointers.get(event.pointerId)?.isPad) {
      event.preventDefault();
      update(`pointer:${event.pointerId}`, padAction(event.clientX, event.clientY, pad.getBoundingClientRect()));
    }
  });
  for (const name of ['pointerup', 'pointercancel', 'lostpointercapture']) listen(deck, name, release);
  listen(deck, 'contextmenu', event => event.preventDefault());
  // Assistive technology and Space activation may synthesize a click alone.
  listen(deck, 'click', event => {
    if (event.detail !== 0 || !handlers.active()) return;
    const button = event.target.closest('[data-action]');
    if (!button || button.disabled) return;
    update('accessible:button', Number(button.dataset.action));
    update('accessible:button', -1);
  });
  listen(window, 'keydown', event => {
    if (!handlers.active()) return;
    // Let focused links and buttons keep their native keyboard activation.
    if (event.code === 'Enter' && event.target.closest?.('a, button')) return;
    if (event.code === 'Escape') { event.preventDefault(); if (!event.repeat) handlers.back(); return; }
    const action = keyAction(event.code);
    if (action === undefined || event.altKey || event.ctrlKey || event.metaKey) return;
    event.preventDefault();
    if (!event.repeat) update(`key:${event.code}`, action);
  });
  listen(window, 'keyup', event => {
    if (keyAction(event.code) === undefined) return;
    update(`key:${event.code}`, -1);
  });
  function clear() {
    for (const [id, { target }] of pointers) {
      if (target.hasPointerCapture(id)) target.releasePointerCapture(id);
    }
    pointers.clear();
    for (const source of [...held.keys()]) update(source, -1);
  }
  return { clear, dispose() { clear(); events.abort(); } };
}
