// https://github.com/codecrafters-io/build-your-own-x#build-your-own-web-server
import net from 'net';
import http from 'http';

function createServerByHttp() {
  const server = http.createServer();
  server.on('request', (req, res) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.url}`);
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hello World!');
  });
  server.listen(3000);
  return server;
}

const SERVER_NAME = 'my-web-server';
const server = net.createServer((sock) => {
  // Buffer
  sock.on('data', (data) => {});
});

// https://www.codementor.io/@ziad-saab/let-s-code-a-web-server-from-scratch-with-nodejs-streams-h4uc9utji
// curl -v localhost:3000/some/url

server.on('connection', (socket) => {
  /*
  socket.on('data', (chunk) => {
    console.log('Received chunk:\n', chunk.toString());
  });
  socket.write(
    `HTTP/1.1 200 OK\r\nServer: ${SERVER_NAME}\r\nContent-Length: 0\r\n\r\n`
  );
  */
  // Subscribe to the readable event once so we can start calling .read()
  socket.once('readable', function () {
    // Set up a buffer to hold the incoming data
    let reqBuffer = Buffer.from('');
    // Set up a temporary buffer to read in chunks
    let buf;
    let reqHeader;
    while (true) {
      // Read data from the socket
      buf = socket.read();
      // Stop if there's no more data
      if (buf === null) break;

      // Concatenate existing request buffer with new data
      reqBuffer = Buffer.concat([reqBuffer, buf]);

      // Check if we've reached \r\n\r\n, indicating end of header
      let marker = reqBuffer.indexOf('\r\n\r\n');
      if (marker !== -1) {
        // If we reached \r\n\r\n, there could be data after it. Take note.
        let remaining = reqBuffer.slice(marker + 4);
        // The header is everything we read, up to and not including \r\n\r\n
        reqHeader = reqBuffer.slice(0, marker).toString();
        // This pushes the extra data we read back to the socket's readable stream
        socket.unshift(remaining);
        break;
      }
    }
    console.log(`Request header:\n${reqHeader}`);

    // At this point, we've stopped reading from the socket and have the header as a string
    // If we wanted to read the whole request body, we would do this:

    reqBuffer = Buffer.from('');
    while ((buf = socket.read()) !== null) {
      reqBuffer = Buffer.concat([reqBuffer, buf]);
    }
    let reqBody = reqBuffer.toString();
    console.log(`Request body:\n${reqBody}`);

    // Send a generic response
    socket.end(
      'HTTP/1.1 200 OK\r\nServer: my-custom-server\r\nContent-Length: 0\r\n\r\n'
    );
  });
});

server.listen(3000);
