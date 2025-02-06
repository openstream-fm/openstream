<style>
  @keyframes CircularProgressCircleIndeterminate{
    0% {
      stroke-dasharray: 1px, 200px;
      stroke-dashoffset: 0px;
    }
  
    50% {
        stroke-dasharray: 100px, 200px;
        stroke-dashoffset: -15px;
    }
  
    100% {
        stroke-dasharray: 100px, 200px;
        stroke-dashoffset: -125px;
    }
  }
  
  @keyframes CircularProgressRotate{
    100% {
      transform: rotate(360deg);
    }
  }
  
  .circular-progress{
    display: flex;
    color: var(--progress-color, var(--primary-color));
    animation: CircularProgressRotate 1.4s linear infinite;
    width: var(--icon-size, 1em);
    height: var(--icon-size, 1em);
  }
  
  .circle.indeterminate{
    animation: CircularProgressCircleIndeterminate 1.4s ease-in-out infinite;
    stroke-dasharray: 80px, 200px;
    stroke-dashoffset: 0px;
  }

  svg {
    width: 100%;
    height: 100%;
  }
  </style>
  
  <script lang="ts">
  
  let className = "";
  export {className as class};
  
  export let style: string = "";
  export let value: number | null = null;
  export let variant: "indeterminate" | "static" | "determinate" = "indeterminate";
  
  $: if(value != null && variant === "indeterminate") variant = "static";
  </script>
  
  <div class="circular-progress {variant} {className}" {style}>
    <svg viewBox="22 22 44 44">
      <circle class="circle indeterminate" cx="44" cy="44" r="20.2" fill="none" stroke="currentColor" stroke-width="3.6"></circle>
    </svg>
  </div>