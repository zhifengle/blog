import { addBinary } from './addBinary';

test('add binary', () => {
  expect(addBinary('11', '1')).toEqual('100');
  expect(addBinary('1010', '1011')).toEqual('10101');
});
