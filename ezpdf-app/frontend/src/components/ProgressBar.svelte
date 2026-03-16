<script lang="ts">
  let { visible }: { visible: boolean } = $props();
</script>

{#if visible}
  <div role="progressbar" class="progress-bar" aria-label="Running…">
    <div class="progress-bar__fill"></div>
  </div>
{/if}

<style>
  .progress-bar {
    width: 100%;
    height: 4px;
    background: var(--border, #e5e7eb);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar__fill {
    height: 100%;
    width: 40%;
    background: var(--run-btn, #2563eb);
    border-radius: 2px;
    animation: slide 1.2s ease-in-out infinite;
  }

  @keyframes slide {
    0%   { transform: translateX(-100%); }
    100% { transform: translateX(350%); }
  }
</style>
