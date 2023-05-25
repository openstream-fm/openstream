import { browser } from "$app/environment";
import { invalidateAll as _invalidateAll } from "$app/navigation";
import { intertab } from "$share/intertab";
import { default_logger } from "$share/logger";

const logger = default_logger.scoped("invalidate");

const store = intertab<number>("studio.invalidate.all");

export const invalidateSiblings = () => {
  logger.info("invalidateSiblings called from this tab");
  store.set(Math.random());
}

export const invalidateAll = async () => {
  logger.info("invalidateAll called from this tab");
  store.set(Math.random());
  await _invalidateAll().catch((e: any) => {
    logger.warn("error calling invalidateAll from this tab")
    logger.error(e);
  });
}

if(browser) {
  store.watch(async () => {
    logger.info("invalidateAll called from other tab");
    try {
      await _invalidateAll();
    } catch(e: any) {
      logger.warn("error calling invalidaAll from other tab");
      logger.error(e);
    }
  })
}