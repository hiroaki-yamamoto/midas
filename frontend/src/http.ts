import { parse } from 'cookie';

export class Http {
  private readonly csrf_cookie_name = 'XSRF-TOKEN';
  private readonly csrf_header_name = 'X-XSRF-TOKEN';
  private csrfToken: string | undefined;

  constructor() {
    fetch('/token/csrf', {
      method: 'HEAD',
      credentials: 'same-origin'
    }).then((resp) => {
      const cookieTxt = resp.headers.get('set-cookie');
      if (!cookieTxt) {
        throw new Error('No CSRF token');
      }
      const parsed = parse(cookieTxt);
      this.csrfToken = parsed[this.csrf_cookie_name];
    });
  }
}
