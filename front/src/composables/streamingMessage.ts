import { computed, nextTick, onUnmounted, ref } from 'vue';

const message = ref('');
const completed = ref(true);
const cancelled = ref(false);
const failed = ref(false);
const currentThreadId = ref<string | null>(null);
const messagesContainer = ref<HTMLElement | null>(null);

// Scroll tracking state
const userHasScrolledUp = ref(false);
const showScrollToBottomPill = ref(false);
let scrollEventListener: (() => void) | null = null;

// For character-by-character interpolation
const pendingChunks = ref<string[]>([]);
const isInterpolating = ref(false);

let interpolationInterval: number | null = null;

const BASE_INTERPOLATION_SPEED = 2; // Base ms between characters
const MIN_INTERPOLATION_SPEED = 0.5; // Minimum ms between characters
const MAX_PENDING_CHUNKS = 10; // Maximum pending chunks before direct append

export function useStreamingMessage() {
  // Clean up event listener when component is unmounted
  onUnmounted(() => {
    if (scrollEventListener && messagesContainer.value) {
      messagesContainer.value.removeEventListener('scroll', scrollEventListener);
      scrollEventListener = null;
    }
  });

  /**
   * Reset scroll tracking state when user sends a message
   */
  function resetScrollState() {
    userHasScrolledUp.value = false;
    showScrollToBottomPill.value = false;
  }

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

    // Final scroll to bottom when message is complete
    scrollToBottom();
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

    // replace \\n with actual newline
    const processedChunk = chunk.replace(/\\n/g, '\n');

    // If we have too many pending chunks, append directly and clear the queue
    if (pendingChunks.value.length >= MAX_PENDING_CHUNKS) {
      if (interpolationInterval !== null) {
        clearInterval(interpolationInterval);
        interpolationInterval = null;
      }

      // Append all pending chunks directly
      message.value += pendingChunks.value.join('') + processedChunk;
      pendingChunks.value = [];
      isInterpolating.value = false;
    } else {
      // Add the new chunk to the pending queue
      pendingChunks.value.push(processedChunk);

      // Start interpolation if not already running
      if (!isInterpolating.value) {
        startInterpolation();
      }
    }

    if (!userHasScrolledUp.value) {
      scrollToBottom();
    } else if (!showScrollToBottomPill.value) {
      showScrollToBottomPill.value = true;
    }
  }

  /**
   * Start the character-by-character interpolation process
   */
  function startInterpolation() {
    if (isInterpolating.value || pendingChunks.value.length === 0) return;

    isInterpolating.value = true;

    interpolationInterval = window.setInterval(() => {
      if (pendingChunks.value.length === 0) {
        isInterpolating.value = false;
        clearInterval(interpolationInterval as number);
        interpolationInterval = null;
        return;
      }

      let currentChunk = pendingChunks.value[0];

      // Adjust speed based on number of pending chunks
      const currentSpeed = Math.max(
        MIN_INTERPOLATION_SPEED,
        BASE_INTERPOLATION_SPEED - pendingChunks.value.length * 0.2,
      );

      // Process multiple characters per tick when we have many pending chunks
      const charsToProcess = Math.ceil(BASE_INTERPOLATION_SPEED / currentSpeed);

      // Add the entire chunk at once if it's small enough
      if (currentChunk.length <= charsToProcess) {
        message.value += currentChunk;
        pendingChunks.value.shift();
      } else {
        // Otherwise process a portion of the chunk
        const portion = currentChunk.slice(0, charsToProcess);
        message.value += portion;
        pendingChunks.value[0] = currentChunk.slice(charsToProcess);
      }
    }, BASE_INTERPOLATION_SPEED);
  }

  const isStreaming = computed(() => !completed.value && !cancelled.value && !failed.value);

  /**
   * Check if user has scrolled up from the bottom
   */
  function checkIfUserScrolledUp() {
    if (!messagesContainer.value) return;

    const container = messagesContainer.value;
    const scrollDifference = container.scrollHeight - container.clientHeight - container.scrollTop;

    // A small threshold to account for small differences due to rendering
    const scrollThreshold = 50;

    // Determine if user has scrolled up significantly
    userHasScrolledUp.value = scrollDifference > scrollThreshold;

    // Show the pill if user has scrolled up significantly
    // We now show it whenever user scrolls up, regardless of streaming state
    showScrollToBottomPill.value = userHasScrolledUp.value;
  }

  /**
   * Scroll the messages container to the bottom
   */
  function scrollToBottom(force: boolean = false) {
    // Use nextTick to ensure DOM is updated before scrolling
    nextTick(() => {
      // If not explicitly set, find messages container in the DOM
      if (!messagesContainer.value) {
        messagesContainer.value = document.querySelector('.messages') as HTMLElement;
      }

      if (messagesContainer.value) {
        // Only scroll if the user hasn't manually scrolled up or if force=true
        if (!userHasScrolledUp.value || force) {
          // Scroll to bottom with smooth animation
          messagesContainer.value.scrollTo({
            top: messagesContainer.value.scrollHeight,
            behavior: 'smooth',
          });
        } else {
          // User has scrolled up, show the pill instead
          showScrollToBottomPill.value = true;
        }
      }
    });
  }

  /**
   * Set the messages container element reference
   */
  function setMessagesContainer(element: HTMLElement | null) {
    messagesContainer.value = element;

    // Set up scroll event listener
    if (messagesContainer.value) {
      // Remove any existing listener first
      if (scrollEventListener) {
        messagesContainer.value.removeEventListener('scroll', scrollEventListener);
      }

      // Create and attach new scroll listener
      scrollEventListener = () => checkIfUserScrolledUp();
      messagesContainer.value.addEventListener('scroll', scrollEventListener);
    }
  }

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
    addChunk,
    scrollToBottom,
    setMessagesContainer,
    userHasScrolledUp,
    showScrollToBottomPill,
    resetScrollState,
  };
}
