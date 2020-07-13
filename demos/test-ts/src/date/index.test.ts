import { genDateSequence } from './index'

test('gen date', () => {
  console.log(genDateSequence('year', '2019-01', '2020-02'))
  console.log(genDateSequence('month', '2019-01', '2020-02'))
  console.log(genDateSequence('date', '2019-02-02', '2019-03-01'))
})
