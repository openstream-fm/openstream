import ApexCharts from "apexcharts-skip-ssr";
import type { ApexOptions } from "apexcharts-skip-ssr";

export const chart = (node: HTMLElement, options: ApexOptions) => {
  const chart = new ApexCharts(node, options);
  chart.render();
   
  return {
    update(options: ApexOptions) {
      chart.updateOptions(options);
    },
    destroy() {
      chart.destroy();
    }
  }
}