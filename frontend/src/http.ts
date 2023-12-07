import { parse } from 'cookie';

export class Http {
  private readonly csrf_cookie_name = 'XSRF-TOKEN';
  private readonly csrf_header_name = 'X-XSRF-TOKEN';
  private csrfToken: string | undefined;

  constructor() {
    fetch('/token/csrf', {
      method: 'HEAD',
      credentials: 'same-origin'
    }).then(() => {
      const cookieTxt = document.cookie;
      if (!cookieTxt) {
        throw new Error('No CSRF token');
      }
      const parsed = parse(cookieTxt);
      this.csrfToken = parsed[this.csrf_cookie_name];
      console.log("CSRF token: ", this.csrfToken);
    });
  }

  public async get(url: string): Promise<Response> {
    const headers = new Headers();
    if (this.csrfToken) {
      headers.append(this.csrf_header_name, this.csrfToken);
    }
    return await fetch(
      url, { method: 'GET', credentials: 'same-origin', headers, }
    );
  }
}
