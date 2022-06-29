import https from 'https';

const data = JSON.stringify({
  name: 'John Doe',
  job: 'Content Writer',
});

const options = {
  hostname: 'reqres.in',
  path: '/api/users',
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    // 是否必传??
    'Content-Length': data.length,
  },
};

const req = https
  .request(options, (res) => {
    let data = '';

    console.log('Status Code:', res.statusCode);

    res.on('data', (chunk) => {
      data += chunk;
    });

    res.on('end', () => {
      console.log('Body: ', JSON.parse(data));
    });
  })
  .on('error', (err) => {
    console.log('Error: ', err.message);
  });

req.write(data);
req.end();
