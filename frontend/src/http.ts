import { parse } from 'cookie';

export class Http {
  private readonly csrf_cookie_name = 'XSRF-TOKEN';
  private readonly csrf_header_name = 'X-XSRF-TOKEN';
  private csrfToken: string | undefined;
  public csrfPromise: Promise<void>;

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
      this.csrfToken = parsed[this.csrf_cookie_name];
      console.log("CSRF token: ", this.csrfToken);
    });
  }

  public hasCSRFToken(): boolean {
    return !!this.csrfToken;
  }

  public async get(url: string): Promise<Response> {
    if (!this.csrfToken) {
      return new Promise((_, reject) => {
        reject(new Error('No CSRF token'));
      });
    }
    const headers = new Headers();
    headers.append(this.csrf_header_name, this.csrfToken);
    return await fetch(
      url, { method: 'GET', credentials: 'same-origin', headers, }
    );
  }
}
