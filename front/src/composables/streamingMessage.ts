import { computed, ref } from 'vue';

const message = ref('');
const completed = ref(true);
const cancelled = ref(false);
const failed = ref(false);
const currentThreadId = ref<string | null>(null);

// For character-by-character interpolation
const pendingChunks = ref<string[]>([]);
const isInterpolating = ref(false);
let interpolationInterval: number | null = null;
const INTERPOLATION_SPEED = 2; // ms between characters

export function useStreamingMessage() {
  function onStreamStarted(threadId: string) {
    message.value = '';
    completed.value = false;
    cancelled.value = false;
    failed.value = false;
    currentThreadId.value = threadId;
    
    // Reset interpolation state
    pendingChunks.value = [];
    isInterpolating.value = false;
    if (interpolationInterval !== null) {
      clearInterval(interpolationInterval);
      interpolationInterval = null;
    }
  }

  function onStreamCompleted() {
    completed.value = true;
    cancelled.value = false;
    failed.value = false;
    currentThreadId.value = null;
    
    // Ensure any remaining text is displayed
    if (pendingChunks.value.length > 0) {
      const remainingText = pendingChunks.value.join('');
      message.value += remainingText;
      pendingChunks.value = [];
    }
    
    // Clean up interpolation
    isInterpolating.value = false;
    if (interpolationInterval !== null) {
      clearInterval(interpolationInterval);
      interpolationInterval = null;
    }
  }

  function onStreamCancelled() {
    cancelled.value = true;
    completed.value = true;
    failed.value = false;
    currentThreadId.value = null;
    
    // Clean up interpolation
    pendingChunks.value = [];
    isInterpolating.value = false;
    if (interpolationInterval !== null) {
      clearInterval(interpolationInterval);
      interpolationInterval = null;
    }
  }

  function onStreamFailed() {
    failed.value = true;
    completed.value = true;
    cancelled.value = false;
    currentThreadId.value = null;
    
    // Clean up interpolation
    pendingChunks.value = [];
    isInterpolating.value = false;
    if (interpolationInterval !== null) {
      clearInterval(interpolationInterval);
      interpolationInterval = null;
    }
  }
  
  /**
   * Add a new chunk of text to be interpolated character by character
   */
  function addChunk(chunk: string) {
    if (chunk.length === 0) return;
    
    // Add the new chunk to the pending queue
    pendingChunks.value.push(chunk);
    
    // Start interpolation if not already running
    if (!isInterpolating.value) {
      startInterpolation();
    }
  }
  
  /**
   * Start the character-by-character interpolation process
   */
  function startInterpolation() {
    if (isInterpolating.value || pendingChunks.value.length === 0) return;
    
    isInterpolating.value = true;
    let currentChunk = pendingChunks.value[0];
    let charIndex = 0;
    
    interpolationInterval = window.setInterval(() => {
      if (charIndex < currentChunk.length) {
        // Add next character
        message.value += currentChunk[charIndex];
        charIndex++;
      } else {
        // Current chunk is done, remove it from the queue
        pendingChunks.value.shift();
        
        // If there are more chunks, start on the next one
        if (pendingChunks.value.length > 0) {
          currentChunk = pendingChunks.value[0];
          charIndex = 0;
        } else {
          // No more chunks, stop interpolation
          isInterpolating.value = false;
          clearInterval(interpolationInterval as number);
          interpolationInterval = null;
        }
      }
    }, INTERPOLATION_SPEED);
  }

  const isStreaming = computed(() => !completed.value && !cancelled.value && !failed.value);

  return {
    message,
    completed,
    cancelled,
    failed,
    currentThreadId,
    onStreamStarted,
    onStreamCompleted,
    onStreamCancelled,
    onStreamFailed,
    isStreaming,
    // New function for character-by-character interpolation
    addChunk,
  };
}
