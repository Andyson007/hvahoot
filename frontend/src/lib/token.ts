export function isLoggedIn() {
  const parsedCookies = document.cookie.split(';').reduce<{[cookie: string]: string}>((acc, current) => {
    const eqSplit = current.split('=');
    acc[eqSplit[0]] = eqSplit.slice(1).join('=');
    return acc;
  }, {});

  if (parsedCookies['token']) return true;
  return false;
}