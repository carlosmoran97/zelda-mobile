// Preparation never advances the game clock or unlocks audio playback.
export async function prepareAssets({ loadGame, audio, status, timeoutMs = 90000 }) {
  let game, canceled = false, timeout;
  try {
    status('Loading game…');
    const graphics = (async () => {
      const loaded = await loadGame();
      if (canceled) { loaded.free(); return; }
      game = loaded;
      status('Preparing graphics…');
      await game.preload_assets();
      if (!canceled) status('Preparing audio…');
    })();
    await Promise.race([
      Promise.all([graphics, audio.preload()]),
      new Promise((_, reject) => {
        timeout = setTimeout(() => reject(new Error('Loading timed out. Check your connection and try again.')), timeoutMs);
      }),
    ]);
    return game;
  } catch (error) {
    canceled = true;
    game?.free();
    await audio.dispose();
    throw error;
  } finally {
    clearTimeout(timeout);
  }
}
