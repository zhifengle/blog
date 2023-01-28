const { writeFileSync } = require('fs');
// npm i ip2location-nodejs
const { IPTools, IP2Location } = require('ip2location-nodejs');

let ip2location = new IP2Location();
let tools = new IPTools();

// https://lite.ip2location.com/
ip2location.open('./IP2LOCATION-LITE-DB3.BIN');

// https://d7uri8nf7uskq.cloudfront.net/tools/list-cloudfront-ips
const ips = require('./cft.json').CLOUDFRONT_GLOBAL_IP_LIST;
const results = [];

for (const cidr of ips) {
  const ip = cidr.replace(/\/\d+/, '');
  const res = ip2location.getAll(ip);
  if (res.countryShort !== 'CN') {
    results.push(cidr);
  } else {
    console.log(res.countryLong);
  }
}

writeFileSync('cft.txt', results.join('\n'), 'utf-8')


ip2location.close();
