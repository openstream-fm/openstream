import test from 'ava';
import { clone, equals, diff } from './collections';

test('collections: clone - should return null when null is passed in', t => {
  t.deepEqual(clone(null), null);
});

test('collections: clone - should return undefined when undefined is passed in', t => {
  t.deepEqual(clone(undefined), undefined);
});

test('collections: clone - should return the same number when a number is passed in', t => {
  const n = 123;
  t.deepEqual(clone(n), n);
});

test('collections: clone - should return the same string when a string is passed in', t => {
  const s = 'hello';
  t.deepEqual(clone(s), s);
});

test('collections: clone - should return the same function when a function is passed in', t => {
  const fn = () => console.log('hello');
  t.deepEqual(clone(fn), fn);
});

test('collections: clone - should return a new instance of Date when a Date is passed in', t => {
  const d = new Date();
  const cloned = clone(d);
  t.not(d, cloned);
  t.deepEqual(cloned, d);
});

test('collections: clone - should return a new array with cloned elements when an array is passed in', t => {
  const arr = [1, 'two', { three: 3 }];
  const cloned = clone(arr);
  t.not(arr, cloned);
  t.deepEqual(cloned, arr);
  t.not(cloned[2], arr[2]);
  t.deepEqual(cloned[2], arr[2]);
});

test('collections: clone - should return a new object with cloned properties when an object is passed in', t => {
  const obj = { a: 1, b: 'two', c: { three: 3 } };
  const cloned = clone(obj);
  t.not(obj, cloned);
  t.deepEqual(cloned, obj);
  t.not(cloned.c, obj.c);
  t.deepEqual(cloned.c, obj.c);
});

test('collections: clone - should throw an error when an unknown type is passed in', t => {
  t.throws(() => clone(Symbol()));
});

test('collections: equals - returns true for equal numbers', t => {
  t.true(equals(1, 1));
});

test('collections: equals - returns false for different numbers', t => {
  t.false(equals(1, 2));
});

test('collections: equals - returns true for equal strings', t => {
  t.true(equals('hello', 'hello'));
});

test('collections: equals - returns false for different strings', t => {
  t.false(equals('hello', 'world'));
});

test('collections: equals - returns true for null and undefined', t => {
  t.true(equals(null, undefined));
});

test('collections: equals - returns true for equal arrays', t => {
  t.true(equals([1, 2, 3], [1, 2, 3]));
});

test('collections: equals - returns false for different arrays', t => {
  t.false(equals([1, 2, 3], [3, 2, 1]));
});

test('collections: equals - returns true for equal objects', t => {
  t.true(equals({ a: 1, b: 'hello' }, { a: 1, b: 'hello' }));
});

test('collections: equals - returns false for different objects', t => {
  t.false(equals({ a: 1, b: 'hello' }, { a: 2, b: 'world' }));
});

test('collections: equals - throws an error for functions', t => {
  t.throws(() => equals(() => {}, () => {}));
});

test('collections: equals - throws an error for unknown types', t => {
  t.throws(() => equals(Symbol(), Symbol()));
});

test('collections: diff - returns an empty object for equal objects', t => {
  const db = { a: 1, b: 'hello' };
  const current = { a: 1, b: 'hello' };
  const result = diff(db, current);
  t.deepEqual(result, {});
});

test('collections: diff - returns a partial object for different objects', t => {
  const db = { a: 1, b: 'hello', c: [1, 2, 3] };
  const current = { a: 1, b: 'world', d: { x: 1 } };
  // @ts-ignore
  const result = diff(db, current);
  t.deepEqual(result, { b: 'world', c: undefined, d: { x: 1 } });
});

test('collections: diff - returns a partial object for different arrays', t => {
  const db = { a: [1, 2, 3] };
  const current = { a: [3, 2, 1] };
  const result = diff(db, current);
  t.deepEqual(result, { a: [3, 2, 1] });
});

test('collections: diff - adds extra properties in current object', t => {
  const db = { a: 1, b: 'hello' };
  const current = { a: 1, b: 'hello', c: [1, 2, 3] };
  const result = diff(db, current);
  t.deepEqual(result, { c: [1, 2, 3] });
});

test('collections: diff - adds undefined on extra properties in db object', t => {
  const db = { a: 1, b: 'hello', c: [1, 2, 3] };
  const current = { a: 1, b: 'hello' };
  const result = diff(db, current);
  t.deepEqual(result, { c: undefined });
});

test('collections: diff - throws an error for unknown types', t => {
  t.throws(() => diff({ a: Symbol() }, { a: Symbol() }));
});