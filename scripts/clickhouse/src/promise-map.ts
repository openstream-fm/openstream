export type AwaitRecord<R extends Record<string, any>> = {
  [P in keyof R]: Awaited<R[P]>
}

export const promise_map = async<R extends Record<string, any>>(record: R): Promise<AwaitRecord<R>> => {
  const target = {};
  await Promise.all(Object.entries(record).map(async ([key, value]) => {
    // @ts-ignore
    target[key] = await value;
  }));

  return target as any as AwaitRecord<R>;
}