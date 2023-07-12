export type JsonValue<T = never> = 
  null | 
  undefined |
  void | 
  boolean | 
  number | 
  string | 
  T |
  JsonValue<T>[] |
  { [key: string | number]: JsonValue<T> }

export const clone = <T>(src: T): T => {
  const t = typeof src;
  if(src == null) return src;
  if(t === "number" || t === "string" || t === "boolean" || t === "function") return src;
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
  if(src == null || target == null) return src == target; // we treat null and undefined as the same
  const t = typeof src;
  if(t !== typeof target) return false;
  if(t === "function") throw new Error("equals: Cannot compare a function");
  if(t === "boolean" || t === "number" || t === "string") return src === target;
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
      ...Object.keys(target),
    ])

    for(const key of keys) {
      // @ts-ignore
      if(!equals(src[key], target[key])) return false;
    }

    return true;

  }
  
  throw new Error(`equals: cannot compare unknown type ${t}`);
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


export const to_str_hash = (object: JsonValue<BigInt | Date>): string => {
  
  // we treat null and undefined as the same
  if (object == null) return 'n';
 
  const t = typeof object;
  if (t === "boolean") return object ? "t" : "f";
  if (t === "string") return `s:${(object as string).replace(/([\\\\;\:])/g, "\\$1")}`;
  if (t === "number") return `n:${object}`;
  if (t === "bigint") return `bi:${object}`;
  if (object instanceof Date) return `d:${+object}`;
  if (object instanceof Array) {
    return `a:${object.map(to_str_hash).join(";")}`;  
  }
  return `o:${
    Object.keys(object)
    // we ignore there types in objects
    .filter(k => {
      const v = (object as any)[k];
      const ty = typeof v;
      return v != null && ty !== "symbol" && ty !== "function";
    })
    .sort()
    .map(k => `${k}:${to_str_hash((object as any)[k])}`)
    .join(";")
  }`;
}

export const hash = (object: JsonValue<BigInt | Date>): number => {
  return hash_str(to_str_hash(object));
}

export const hash_str = (str: string): number => {
 let hash = 0;
 for (let i = 0, len = str.length; i < len; i++) {
    let chr = str.charCodeAt(i);
    hash = (hash << 5) - hash + chr;
    hash |= 0; // Convert to 32bit integer
 }
  return hash;
}