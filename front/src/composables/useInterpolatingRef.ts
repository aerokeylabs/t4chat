import { ref } from 'vue';

const BASE_INTERPOLATION_SPEED = 2; // Base ms between characters
const MIN_INTERPOLATION_SPEED = 0.5; // Minimum ms between characters
const MAX_PENDING_CHUNKS = 10; // Maximum pending chunks before direct append

/** ref that smoothly adds text added character by character */
export function useInterpolatingRef(initialValue: string) {
  let value = ref(initialValue);
  let pendingChunks: string[] = [];
  let isInterpolating = false;
  let interpolationInterval: number | null = null;
  let speed = BASE_INTERPOLATION_SPEED;

  function startInterpolation() {
    if (isInterpolating || pendingChunks.length === 0) return;

    isInterpolating = true;

    interpolationInterval = window.setInterval(() => {
      if (pendingChunks.length === 0) {
        isInterpolating = false;
        clearInterval(interpolationInterval as number);
        interpolationInterval = null;
        return;
      }

      let currentChunk = pendingChunks[0];

      // Adjust speed based on number of pending chunks
      speed = Math.max(MIN_INTERPOLATION_SPEED, BASE_INTERPOLATION_SPEED - pendingChunks.length * 0.2);

      // Process multiple characters per tick when we have many pending chunks
      const charsToProcess = Math.ceil(BASE_INTERPOLATION_SPEED / speed);

      // Add the entire chunk at once if it's small enough
      if (currentChunk.length <= charsToProcess) {
        value.value += currentChunk;
        pendingChunks.shift();
      } else {
        // Otherwise process a portion of the chunk
        const portion = currentChunk.slice(0, charsToProcess);
        value.value += portion;
        pendingChunks[0] = currentChunk.slice(charsToProcess);
      }
    }, speed);
  }

  function add(newValue: string) {
    if (newValue.length === 0) return;

    // If we have too many pending chunks, append directly and clear the queue
    if (pendingChunks.length >= MAX_PENDING_CHUNKS) {
      if (interpolationInterval !== null) {
        clearInterval(interpolationInterval);
        interpolationInterval = null;
      }

      // Append all pending chunks directly
      value.value += pendingChunks.join('') + newValue;
      pendingChunks = [];
      isInterpolating = false;
    } else {
      // Add the new chunk to the pending queue
      pendingChunks.push(newValue);

      // Start interpolation if not already running
      if (!isInterpolating) {
        startInterpolation();
      }
    }
  }

  function clear() {
    value.value = '';
    pendingChunks = [];
    isInterpolating = false;

    if (interpolationInterval !== null) {
      clearInterval(interpolationInterval);
      interpolationInterval = null;
    }
  }

  return {
    value,
    add,
    clear,
  };
}
