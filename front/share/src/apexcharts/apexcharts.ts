import { equals } from "$server/util/collections";
import ApexCharts from "apexcharts-skip-ssr";
import type { ApexOptions } from "apexcharts-skip-ssr";

export const chart = (node: HTMLElement, options: ApexOptions & { hide_series?: string[] }) => {
  const { hide_series, ...opts} = options;
  
  let prev_hide_series = hide_series;
  
  const chart = new ApexCharts(node, opts);
  
  chart.render();
  
  if(hide_series != null) {
    for(const name of hide_series) {
      chart.hideSeries(name); 
    }
  }
  
  return {
    update(options: ApexOptions & { hide_series?: string[] }) {
      const { hide_series, ...opts } = options;
      
      chart.updateOptions(opts);
      
      if(!equals(prev_hide_series ?? [], hide_series ?? [])) {
        prev_hide_series = hide_series;
        chart.resetSeries();
        if(hide_series != null) {
          for(const name of hide_series) {
            chart.hideSeries(name); 
          }
        }
      }
    },
    destroy() {
      chart.destroy();
    }
  }
}