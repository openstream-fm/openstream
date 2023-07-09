import test from 'ava';
import { clone, equals, diff, to_str_hash, hash, hash_str } from './collections';

test('clone - should return null when null is passed in', t => {
  t.deepEqual(clone(null), null);
});

test('clone - should return undefined when undefined is passed in', t => {
  t.deepEqual(clone(undefined), undefined);
});

test('clone - should return the same number when a number is passed in', t => {
  const n = 123;
  t.deepEqual(clone(n), n);
});

test('clone - should return the same string when a string is passed in', t => {
  const s = 'hello';
  t.deepEqual(clone(s), s);
});

test('clone - should return the same function when a function is passed in', t => {
  const fn = () => console.log('hello');
  t.deepEqual(clone(fn), fn);
});

test('clone - should return a new instance of Date when a Date is passed in', t => {
  const d = new Date();
  const cloned = clone(d);
  t.not(d, cloned);
  t.deepEqual(cloned, d);
});

test('clone - should return a new array with cloned elements when an array is passed in', t => {
  const arr = [1, 'two', { three: 3 }];
  const cloned = clone(arr);
  t.not(arr, cloned);
  t.deepEqual(cloned, arr);
  t.not(cloned[2], arr[2]);
  t.deepEqual(cloned[2], arr[2]);
});

test('clone - should return a new object with cloned properties when an object is passed in', t => {
  const obj = { a: 1, b: 'two', c: { three: 3 } };
  const cloned = clone(obj);
  t.not(obj, cloned);
  t.deepEqual(cloned, obj);
  t.not(cloned.c, obj.c);
  t.deepEqual(cloned.c, obj.c);
});

test('clone - should throw an error when an unknown type is passed in', t => {
  t.throws(() => clone(Symbol()));
});

test('clone - should return the same boolean when a boolean is passed in', t => {
  t.deepEqual(clone(true), true);
  t.deepEqual(clone(false), false);
});

test('equals - returns false for object vs null', t => {
  t.false(equals({}, null));
});

test('equals - returns false for null vs object', t => {
  t.false(equals(null, {}));
});

test('equals - returns true for equal numbers', t => {
  t.true(equals(1, 1));
});

test('equals - returns false for different numbers', t => {
  t.false(equals(1, 2));
});

test('equals - returns true for equal strings', t => {
  t.true(equals('hello', 'hello'));
});

test('equals - returns false for different strings', t => {
  t.false(equals('hello', 'world'));
});

test('equals - returns true for null and undefined', t => {
  t.true(equals(null, undefined));
});

test('equals - returns true for equal arrays', t => {
  t.true(equals([1, 2, 3], [1, 2, 3]));
});

test('equals - returns false for different arrays', t => {
  t.false(equals([1, 2, 3], [3, 2, 1]));
});

test('equals - returns true for equal objects', t => {
  t.true(equals({ a: 1, b: 'hello' }, { a: 1, b: 'hello' }));
});

test('equals - returns false for different objects', t => {
  t.false(equals({ a: 1, b: 'hello' }, { a: 2, b: 'world' }));
});

test('equals - throws an error for functions', t => {
  t.throws(() => equals(() => {}, () => {}));
});

test('equals - throws an error for unknown types', t => {
  t.throws(() => equals(Symbol(), Symbol()));
});

test('equals - returns true for equal booleans', t => {
  t.true(equals(true, true));
  t.true(equals(false, false));
});

test('equals - returns false for different booleans', t => {
  t.false(equals(true, false));
  t.false(equals(false, true));
});

test('diff - returns an empty object for equal objects', t => {
  const db = { a: 1, b: 'hello' };
  const current = { a: 1, b: 'hello' };
  const result = diff(db, current);
  t.deepEqual(result, {});
});

test('diff - returns a partial object for different objects', t => {
  const db = { a: 1, b: 'hello', c: [1, 2, 3] };
  const current = { a: 1, b: 'world', d: { x: 1 } };
  // @ts-ignore
  const result = diff(db, current);
  t.deepEqual(result, { b: 'world', c: undefined, d: { x: 1 } });
});

test('diff - returns a partial object for different arrays', t => {
  const db = { a: [1, 2, 3] };
  const current = { a: [3, 2, 1] };
  const result = diff(db, current);
  t.deepEqual(result, { a: [3, 2, 1] });
});

test('diff - adds extra properties in current object', t => {
  const db = { a: 1, b: 'hello' };
  const current = { a: 1, b: 'hello', c: [1, 2, 3] };
  const result = diff(db, current);
  t.deepEqual(result, { c: [1, 2, 3] });
});

test('diff - adds undefined on extra properties in db object', t => {
  const db = { a: 1, b: 'hello', c: [1, 2, 3] };
  const current = { a: 1, b: 'hello' };
  const result = diff(db, current);
  t.deepEqual(result, { c: undefined });
});

test('diff - throws an error for unknown types', t => {
  t.throws(() => diff({ a: Symbol() }, { a: Symbol() }));
});

test('diff - returns an empty object for equal booleans', t => {
  const db = { a: true, b: false };
  const current = { a: true, b: false };
  const result = diff(db, current);
  t.deepEqual(result, {});
});

test('diff - returns a partial object for different booleans', t => {
  const db = { a: true, b: false };
  const current = { a: false, b: true };
  const result = diff(db, current);
  t.deepEqual(result, { a: false, b: true });
});


test('hash - to_str returns "n" for null or undefined input', t => {
  t.is(to_str_hash(null), 'n');
  t.is(to_str_hash(undefined), 'n');
});

test('hash - to_str correctly converts boolean input', t => {
  t.is(to_str_hash(true), 't');
  t.is(to_str_hash(false), 'f');
});

test('hash - to_str correctly converts string input', t => {
  t.is(to_str_hash('hello'), 's:hello');
  t.is(to_str_hash('hello;world'), 's:hello\\;world'); // test escaping of semicolon
});

test('hash - to_str correctly converts number input', t => {
  t.is(to_str_hash(42), 'n:42');
});

test('hash - to_str correctly converts BigInt input', t => {
  t.is(to_str_hash(123n), 'bi:123');
});

test('hash - to_str correctly converts Date input', t => {
  const date = new Date('2022-01-01');
  t.is(to_str_hash(date), `d:${+date}`);
});

test('hash - to_str correctly converts Array input', t => {
  const arr = ['hello', 42, { foo: 'bar' }];
  t.is(to_str_hash(arr), 'a:s:hello;n:42;o:foo:s:bar');
});

test('hash - to_str correctly converts Object input', t => {
  const obj = { foo: 'bar', baz: 42, qux: { quux: true } };
  t.is(to_str_hash(obj), 'o:baz:n:42;foo:s:bar;qux:o:quux:t');
});

test('hash - hash_str generates expected hash from empty string', t => {
  t.is(hash_str(''), 0);
});

test('hash - hash_str generates different hash for different strings', t => {
  const hash1 = hash_str('hello');
  const hash2 = hash_str('world');
  t.not(hash1, hash2);
});

test('hash - hash_str generates same hash for same string', t => {
  const hash1 = hash_str('hello');
  const hash2 = hash_str('hello');
  t.is(hash1, hash2);
});

test('hash - hash generates expected hash for primitive input', t => {
  t.is(hash(42), hash_str("n:42"));
  t.is(hash('hello'), hash_str('s:hello'));
  t.is(hash(true), hash_str('t'));
  t.is(hash(null), hash_str('n'));
});

test('hash - hash generates expected hash for object input', t => {
  const obj = { foo: 'bar', baz: 42, qux: { quux: true } };
  const expectedHash = hash_str('o:baz:n:42;foo:s:bar;qux:o:quux:t');
  t.is(hash(obj), expectedHash);
});

test('hash - hash generates same hash for same object', t => {
  const obj = { foo: 'bar', baz: 42, qux: { quux: true } };
  const hash1 = hash(obj);
  const hash2 = hash(obj);
  t.is(hash1, hash2);
});

test('hash - hash generates different hash for different objects', t => {
  const obj1 = { foo: 'bar', baz: 42, qux: { quux: true } };
  const obj2 = { foo: 'baz', qux: { quux: false }, quu: ['hello', 'world'] };
  t.not(hash(obj1), hash(obj2));
});