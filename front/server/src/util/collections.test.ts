import { clone, equals, diff } from './collections';
import { describe, it, expect } from "@jest/globals";

describe('collections: clone', () => {
  it('should return null when null is passed in', () => {
    expect(clone(null)).toStrictEqual(null);
  });

  it('should return undefined when undefined is passed in', () => {
    expect(clone(undefined)).toStrictEqual(undefined);
  });

  it('should return the same number when a number is passed in', () => {
    const n = 123;
    expect(clone(n)).toStrictEqual(n);
  });

  it('should return the same string when a string is passed in', () => {
    const s = 'hello';
    expect(clone(s)).toStrictEqual(s);
  });

  it('should return the same function when a function is passed in', () => {
    const fn = () => console.log('hello');
    expect(clone(fn)).toStrictEqual(fn);
  });

  it('should return a new instance of Date when a Date is passed in', () => {
    const d = new Date();
    const cloned = clone(d);
    expect(Object.is(d, cloned)).toBe(false);
    expect(cloned).toEqual(d);
  });

  it('should return a new array with cloned elements when an array is passed in', () => {
    const arr = [1, 'two', { three: 3 }];
    const cloned = clone(arr);
    expect(Object.is(arr, cloned)).toBe(false);
    expect(cloned).toEqual(arr);
    expect(Object.is(arr[2], cloned[2])).toBe(false);
    expect(cloned[2]).toEqual(arr[2]);
  });

  it('should return a new object with cloned properties when an object is passed in', () => {
    const obj = { a: 1, b: 'two', c: { three: 3 } };
    const cloned = clone(obj);
    expect(Object.is(obj, cloned)).toBe(false);
    expect(cloned).toEqual(obj);
    expect(Object.is(obj.c, cloned.c)).toBe(false);
    expect(cloned.c).toEqual(obj.c);
  });

  it('should throw an error when an unknown type is passed in', () => {
    expect(() => clone(Symbol())).toThrowError();
  });
});


describe('collections: equals', () => {
  it('returns true for equal numbers', () => {
    expect(equals(1, 1)).toBe(true);
  });

  it('returns false for different numbers', () => {
    expect(equals(1, 2)).toBe(false);
  });

  it('returns true for equal strings', () => {
    expect(equals('hello', 'hello')).toBe(true);
  });

  it('returns false for different strings', () => {
    expect(equals('hello', 'world')).toBe(false);
  });

  it('returns true for null and undefined', () => {
    expect(equals(null, undefined)).toBe(true);
  });

  it('returns true for equal arrays', () => {
    expect(equals([1, 2, 3], [1, 2, 3])).toBe(true);
  });

  it('returns false for different arrays', () => {
    expect(equals([1, 2, 3], [3, 2, 1])).toBe(false);
  });

  it('returns true for equal objects', () => {
    expect(equals({ a: 1, b: 'hello' }, { a: 1, b: 'hello' })).toBe(true);
  });

  it('returns false for different objects', () => {
    expect(equals({ a: 1, b: 'hello' }, { a: 2, b: 'world' })).toBe(false);
  });

  it('throws an error for functions', () => {
    expect(() => equals(() => {}, () => {})).toThrowError();
  });

  it('throws an error for unknown types', () => {
    expect(() => equals(Symbol(), Symbol())).toThrowError();
  });
});


describe('collections>: diff', () => {
  it('returns an empty object for equal objects', () => {
    const db = { a: 1, b: 'hello' };
    const current = { a: 1, b: 'hello' };
    const result = diff(db, current);
    expect(result).toEqual({});
  });

  it('returns a partial object for different objects', () => {
    const db = { a: 1, b: 'hello', c: [1, 2, 3] };
    const current = { a: 1, b: 'world', d: { x: 1 } };
    // @ts-ignore
    const result = diff(db, current);
    expect(result).toEqual({ b: 'world', c: undefined, d: { x: 1 } });
  });

  it('returns a partial object for different arrays', () => {
    const db = { a: [1, 2, 3] };
    const current = { a: [3, 2, 1] };
    const result = diff(db, current);
    expect(result).toEqual({ a: [3, 2, 1] });
  });

  it('adds extra properties in current object', () => {
    const db = { a: 1, b: 'hello' };
    const current = { a: 1, b: 'hello', c: [1, 2, 3] };
    const result = diff(db, current);
    expect(result).toEqual({ c: [1, 2, 3] });
  });

  it('aads undefined on extra properties in db object', () => {
    const db = { a: 1, b: 'hello', c: [1, 2, 3] };
    const current = { a: 1, b: 'hello' };
    const result = diff(db, current);
    expect(result).toEqual({ c: undefined });
  });

  it('throws an error for unknown types', () => {
    expect(() => diff({ a: Symbol() }, { a: Symbol() })).toThrowError();
  });
});