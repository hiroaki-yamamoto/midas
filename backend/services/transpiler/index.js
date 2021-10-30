const http2 = require('http2');
const cookie = require('cookie');
const fs = require('fs/promises');
const url = require('url');

const ts = require('typescript');
const yaml = require('js-yaml');
const argparse = require('argparse');

const protect_csrf = (cookieName, headerName, handler) => (req, res) => {
  console.log(req.headers);
  const cookieValue = cookie.parse(req.headers.cookie || '')[cookieName];
  const header = req.headers[headerName.toLowerCase()];
  if (cookieValue && header && cookieValue === header) {
    return handler(req, res);
  }
  res.writeHead(403);
  res.end('Permission Denied: Token Verification Failed');
};

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

(async () => {
  const parser = new argparse.ArgumentParser({
    description: 'Typescript Transpile Service',
    add_help: true,
  });
  parser.add_argument('-c', '--config', { help: 'Config file path' });

  const cmdArgs = parser.parse_args();
  const cfg = yaml.load(await fs.readFile(cmdArgs.config));
  const host = new url.URL(`http://${cfg.host}`);
  const certs = {
    key: await fs.readFile(cfg.tls.privateKey),
    cert: await fs.readFile(cfg.tls.cert),
    allowHTTP1: true,
  }

  const server = http2.createSecureServer(
    certs, protect_csrf('XSRF-TOKEN', 'X-XSRF-TOKEN', listener),
  );
  console.log('Listening port', host.port);
  server.listen(parseInt(host.port, 10));

  const graceful_shutdown = () => {
    console.log('Closing the server. bye bye!');
    server.close();
  };

  process.on('SIGTERM', graceful_shutdown);
  process.on('SIGINT', graceful_shutdown);
})();
