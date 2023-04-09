export const clone = <T>(src: T): T => {
  const t = typeof src;
  if(src == null) return src;
  if(t === "number" || t === "string" || t === "function") return src;
  if(t === "object") {
    // @ts-ignore
    if(src instanceof Date) return new Date(src);
    if(src instanceof Array) {
      const target = [];
      for(const item of src) {
        target.push(clone(item));
      }
      // @ts-ignore
      return target;
    } else {
      const target = {};
      for(const key in src) {
        // @ts-ignore
        target[key] = clone(src[key]);
      }
      // @ts-ignore
      return target;
    }
  } else {
    throw new Error(`clone: cannot clone unkown type: ${t}`);
  }
}

export const equals = <T>(src: T, target: T) => {
  if(src == null && target == null) return true;
  const t = typeof src;
  if(t !== typeof target) return false;
  if(src == null) return t == target; // we treat null and undefined as the same
  if(t === "function") throw new Error("equals: Cannot compare a function");
  if(t === "number" || t === "string") return src === target;
  if(t === "object") {
    if(src instanceof Date) {
      if(target instanceof Date) {
        return +src === +target;
      } else {
        return false;
      } 
    }

    if(src instanceof Array) {
      if(target instanceof Array) {
        if(src.length !== target.length) return false;
        for(let i = 0; i < src.length; i++) {
          if(!equals(src[i], target[i])) return false;
        }
        return true;
      } else {
        return false;
      }
    }


    const keys = new Set([
      ...Object.keys(src),
      // @ts-ignore
      ...Object.keys(target),
    ])

    for(const key of keys) {
      // @ts-ignore
      if(!equals(src[key], target[key])) return false;
    }

    return true;

  }
  throw new Error(`equals: cannot compare unkown type ${t}`);
}

export const diff = <T extends Record<string, any>>(db: T, current: T): Partial<T> => {
  const diff: Partial<T> = {};
  const keys = new Set([...Object.keys(db), ...Object.keys(current)]);
  for(const key of keys) {
    if(!equals(db[key], current[key])) {
      // @ts-ignore
      diff[key] = current[key];
    }
  }
  return diff;
}