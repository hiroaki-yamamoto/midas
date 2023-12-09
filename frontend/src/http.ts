import { parse } from 'cookie';

export class Http {
  private readonly csrf_cookie_name = 'XSRF-TOKEN';
  private readonly csrf_header_name = 'X-XSRF-TOKEN';
  private csrfPromise: Promise<string>;

  constructor() {
    this.csrfPromise = fetch('/token/csrf', {
      method: 'HEAD',
      credentials: 'same-origin'
    }).then(() => {
      const cookieTxt = document.cookie;
      if (!cookieTxt) {
        throw new Error('No CSRF token');
      }
      const parsed = parse(cookieTxt);
      const token = parsed[this.csrf_cookie_name];
      console.log("CSRF token: ", token);
      return new Promise((ok) => { ok(token); });
    });
  }

  public async get(url: string): Promise<Response> {
    return this.csrfPromise.then((token: string) => {
      if (!token) {
        return new Promise((_, reject) => {
          reject(new Error('No CSRF token'));
        });
      }
      const headers = new Headers();
      headers.append(this.csrf_header_name, token);
      return fetch(
        url, { method: 'GET', credentials: 'same-origin', headers, }
      );
    });
  }
}
