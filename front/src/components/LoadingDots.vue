<template>
  <div class="loading-dots">
    <span></span>
    <span></span>
    <span></span>
  </div>
</template>

<style>
.loading-dots {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 8px;

  > span {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: whitesmoke;
    opacity: 0.7;
    display: inline-block;
    animation: dotBounce 1.4s infinite ease-in-out both;
  }

  > span:nth-child(1) {
    animation-delay: -0.32s;
  }

  > span:nth-child(2) {
    animation-delay: -0.16s;
  }
}

@keyframes dotBounce {
  0%,
  80%,
  100% {
    transform: translateY(0);
  }
  40% {
    transform: translateY(-8px);
  }
}
</style>
