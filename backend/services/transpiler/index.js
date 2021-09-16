const http = require('http');
const ts = require('typescript');

const listener = (req, res) => {
  if (req.method.toLowerCase() !== 'post') {
    res.writeHead(405);
    res.end('Method not allowed');
  }
  let buf = [];
  req.on('data', (chunk) => {
    buf = buf.concat(chunk);
  });
  req.on('end', () => {
    const payload = buf.join('');
    const transpiled = ts.transpileModule(payload, {
      compilerOptions: {
        module: ts.ModuleKind.None,
        noLib: true,
      },
    });
    if (transpiled.diagnostics && transpiled.diagnostics.length > 0) {
      res.writeHead(417);
      res.end(JSON.stringify(
        { diagnostics: transpiled.diagnostics },
        undefined, 2,
      ));
    } else {
      res.writeHead(200);
      res.end(transpiled.outputText);
    }
  });
};

const server = http.createServer(listener);
const port = 50505;
console.log('Listening port ', port);
server.listen(port);

const graceful_shutdown = () => {
  console.log('Closing the server. bye bye!');
  server.close();
};

process.on('SIGTERM', graceful_shutdown);
process.on('SIGINT', graceful_shutdown);
