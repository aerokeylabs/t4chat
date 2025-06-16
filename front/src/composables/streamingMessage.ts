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
const INTERPOLATION_SPEED = 2; // ms between characters

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
    
    // Add the new chunk to the pending queue
    pendingChunks.value.push(chunk);
    
    // Start interpolation if not already running
    if (!isInterpolating.value) {
      startInterpolation();
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
            behavior: 'smooth'
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
    // New function for character-by-character interpolation
    addChunk,
    // Scroll to bottom functionality
    scrollToBottom,
    setMessagesContainer,
    // Scroll pill functionality
    userHasScrolledUp,
    showScrollToBottomPill,
    resetScrollState,
  };
}
