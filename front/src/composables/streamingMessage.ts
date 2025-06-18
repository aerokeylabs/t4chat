import { useInterpolatingRef } from '@/composables/useInterpolatingRef';
import { acceptHMRUpdate, defineStore } from 'pinia';
import { computed, ref } from 'vue';

// const message = ref('');
// const completed = ref(true);
// const cancelled = ref(false);
// const failed = ref(false);
// const currentThreadId = ref<string | null>(null);
// const messagesContainer = ref<HTMLElement | null>(null);

// // Scroll tracking state
// const userHasScrolledUp = ref(false);
// const showScrollToBottomPill = ref(false);
// let scrollEventListener: (() => void) | null = null;

// // For character-by-character interpolation
// const pendingChunks = ref<string[]>([]);
// const isInterpolating = ref(false);

// let interpolationInterval: number | null = null;

// export function useStreamingMessage() {
//   // Clean up event listener when component is unmounted
//   onUnmounted(() => {
//     if (scrollEventListener && messagesContainer.value) {
//       messagesContainer.value.removeEventListener('scroll', scrollEventListener);
//       scrollEventListener = null;
//     }
//   });

//   /**
//    * Reset scroll tracking state when user sends a message
//    */
//   function resetScrollState() {
//     userHasScrolledUp.value = false;
//     showScrollToBottomPill.value = false;
//   }

//   function onStreamStarted(threadId: string) {
//     message.value = '';
//     completed.value = false;
//     cancelled.value = false;
//     failed.value = false;
//     currentThreadId.value = threadId;

//     // Reset interpolation state
//     pendingChunks.value = [];
//     isInterpolating.value = false;
//     if (interpolationInterval !== null) {
//       clearInterval(interpolationInterval);
//       interpolationInterval = null;
//     }
//   }

//   function onStreamCompleted() {
//     completed.value = true;
//     cancelled.value = false;
//     failed.value = false;
//     currentThreadId.value = null;

//     // Ensure any remaining text is displayed
//     if (pendingChunks.value.length > 0) {
//       const remainingText = pendingChunks.value.join('');
//       message.value += remainingText;
//       pendingChunks.value = [];
//     }

//     // Clean up interpolation
//     isInterpolating.value = false;
//     if (interpolationInterval !== null) {
//       clearInterval(interpolationInterval);
//       interpolationInterval = null;
//     }

//     // Final scroll to bottom when message is complete
//     scrollToBottom();
//   }

//   function onStreamCancelled() {
//     cancelled.value = true;
//     completed.value = true;
//     failed.value = false;
//     currentThreadId.value = null;

//     // Clean up interpolation
//     pendingChunks.value = [];
//     isInterpolating.value = false;
//     if (interpolationInterval !== null) {
//       clearInterval(interpolationInterval);
//       interpolationInterval = null;
//     }
//   }

//   function onStreamFailed() {
//     failed.value = true;
//     completed.value = true;
//     cancelled.value = false;
//     currentThreadId.value = null;

//     // Clean up interpolation
//     pendingChunks.value = [];
//     isInterpolating.value = false;
//     if (interpolationInterval !== null) {
//       clearInterval(interpolationInterval);
//       interpolationInterval = null;
//     }
//   }

//   /**
//    * Add a new chunk of text to be interpolated character by character
//    */
//   function addChunk(chunk: string, reasoning: boolean) {
//     if (chunk.length === 0) return;

//     // replace \\n with actual newline
//     const processedChunk = chunk.replace(/\\n/g, '\n');

//     // If we have too many pending chunks, append directly and clear the queue
//     if (pendingChunks.value.length >= MAX_PENDING_CHUNKS) {
//       if (interpolationInterval !== null) {
//         clearInterval(interpolationInterval);
//         interpolationInterval = null;
//       }

//       // Append all pending chunks directly
//       message.value += pendingChunks.value.join('') + processedChunk;
//       pendingChunks.value = [];
//       isInterpolating.value = false;
//     } else {
//       // Add the new chunk to the pending queue
//       pendingChunks.value.push(processedChunk);

//       // Start interpolation if not already running
//       if (!isInterpolating.value) {
//         startInterpolation();
//       }
//     }

//     if (!userHasScrolledUp.value) {
//       scrollToBottom();
//     } else if (!showScrollToBottomPill.value) {
//       showScrollToBottomPill.value = true;
//     }
//   }

//   /**
//    * Start the character-by-character interpolation process
//    */
//   function startInterpolation() {
//     if (isInterpolating.value || pendingChunks.value.length === 0) return;

//     isInterpolating.value = true;

//     interpolationInterval = window.setInterval(() => {
//       if (pendingChunks.value.length === 0) {
//         isInterpolating.value = false;
//         clearInterval(interpolationInterval as number);
//         interpolationInterval = null;
//         return;
//       }

//       let currentChunk = pendingChunks.value[0];

//       // Adjust speed based on number of pending chunks
//       const currentSpeed = Math.max(
//         MIN_INTERPOLATION_SPEED,
//         BASE_INTERPOLATION_SPEED - pendingChunks.value.length * 0.2,
//       );

//       // Process multiple characters per tick when we have many pending chunks
//       const charsToProcess = Math.ceil(BASE_INTERPOLATION_SPEED / currentSpeed);

//       // Add the entire chunk at once if it's small enough
//       if (currentChunk.length <= charsToProcess) {
//         message.value += currentChunk;
//         pendingChunks.value.shift();
//       } else {
//         // Otherwise process a portion of the chunk
//         const portion = currentChunk.slice(0, charsToProcess);
//         message.value += portion;
//         pendingChunks.value[0] = currentChunk.slice(charsToProcess);
//       }
//     }, BASE_INTERPOLATION_SPEED);
//   }

//   const isStreaming = computed(() => !completed.value && !cancelled.value && !failed.value);

//   /**
//    * Check if user has scrolled up from the bottom
//    */
//   function checkIfUserScrolledUp() {
//     if (!messagesContainer.value) return;

//     const container = messagesContainer.value;
//     const scrollDifference = container.scrollHeight - container.clientHeight - container.scrollTop;

//     // A small threshold to account for small differences due to rendering
//     const scrollThreshold = 50;

//     // Determine if user has scrolled up significantly
//     userHasScrolledUp.value = scrollDifference > scrollThreshold;

//     // Show the pill if user has scrolled up significantly
//     // We now show it whenever user scrolls up, regardless of streaming state
//     showScrollToBottomPill.value = userHasScrolledUp.value;
//   }

//   /**
//    * Scroll the messages container to the bottom
//    */
//   function scrollToBottom(force: boolean = false) {
//     // Use nextTick to ensure DOM is updated before scrolling
//     nextTick(() => {
//       // If not explicitly set, find messages container in the DOM
//       if (!messagesContainer.value) {
//         messagesContainer.value = document.querySelector('.messages') as HTMLElement;
//       }

//       if (messagesContainer.value) {
//         // Only scroll if the user hasn't manually scrolled up or if force=true
//         if (!userHasScrolledUp.value || force) {
//           // Scroll to bottom with smooth animation
//           messagesContainer.value.scrollTo({
//             top: messagesContainer.value.scrollHeight,
//             behavior: 'smooth',
//           });
//         } else {
//           // User has scrolled up, show the pill instead
//           showScrollToBottomPill.value = true;
//         }
//       }
//     });
//   }

//   /**
//    * Set the messages container element reference
//    */
//   function setMessagesContainer(element: HTMLElement | null) {
//     messagesContainer.value = element;

//     // Set up scroll event listener
//     if (messagesContainer.value) {
//       // Remove any existing listener first
//       if (scrollEventListener) {
//         messagesContainer.value.removeEventListener('scroll', scrollEventListener);
//       }

//       // Create and attach new scroll listener
//       scrollEventListener = () => checkIfUserScrolledUp();
//       messagesContainer.value.addEventListener('scroll', scrollEventListener);
//     }
//   }

//   return {
//     message,
//     completed,
//     cancelled,
//     failed,
//     currentThreadId,
//     onStreamStarted,
//     onStreamCompleted,
//     onStreamCancelled,
//     onStreamFailed,
//     isStreaming,
//     addChunk,
//     scrollToBottom,
//     setMessagesContainer,
//     userHasScrolledUp,
//     showScrollToBottomPill,
//     resetScrollState,
//   };
// }

export const useStreamingMessage = defineStore('streamingMessage', () => {
  /** if stream is completed and no more data will be received */
  const completed = ref(true);
  /** if stream was cancelled */
  const cancelled = ref(false);
  /** if stream failed */
  const failed = ref(false);

  /** if waiting for first chunk to be received */
  const waitingForFirstChunk = ref(false);

  /** id of thread containing message being streamed */
  const threadId = ref<string | null>(null);

  /** reasoning message */
  const reasoning = useInterpolatingRef('');
  /** response message */
  const response = useInterpolatingRef('');

  /** if stream is currently active */
  const isStreaming = computed(() => !completed.value && !cancelled.value && !failed.value);

  function addChunk(chunk: string, isReasoning: boolean) {
    if (chunk.length === 0) return;
    waitingForFirstChunk.value = false;

    // replace \\n with actual newline
    const processedChunk = chunk.replace(/\\n/g, '\n');

    if (isReasoning) {
      reasoning.add(processedChunk);
    } else {
      response.add(processedChunk);
    }
  }

  const addResponseChunk = (chunk: string) => addChunk(chunk, false);
  const addReasoningChunk = (chunk: string) => addChunk(chunk, true);

  function clear() {
    reasoning.clear();
    response.clear();

    waitingForFirstChunk.value = true;

    threadId.value = null;
  }

  function onStreamStarted(newThreadId: string) {
    completed.value = false;
    cancelled.value = false;
    failed.value = false;

    clear();

    threadId.value = newThreadId;
  }

  function onStreamFailed() {
    failed.value = true;
    completed.value = true;
    cancelled.value = false;
  }

  function onStreamCancelled() {
    cancelled.value = true;
    completed.value = true;
    failed.value = false;
  }

  function onStreamCompleted() {
    completed.value = true;
    cancelled.value = false;
    failed.value = false;
  }

  return {
    completed,
    cancelled,
    failed,

    waitingForFirstChunk,

    isStreaming,

    threadId,

    reasoning,
    response,

    addResponseChunk,
    addReasoningChunk,

    clear,
    onStreamStarted,
    onStreamCancelled,
    onStreamFailed,
    onStreamCompleted,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStreamingMessage, import.meta.hot));
}
