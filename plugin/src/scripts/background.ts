interface IMessage {
  api: string;
  method: string;
  data?: Object;
}
const DEFAULT_BASE_URL = 'http://0.0.0.0:12306';
chrome.webRequest.onResponseStarted.addListener(async (details) => {
  const { method, url, initiator, tabId, requestId, frameId } = details;
  if (method === 'GET' && isMatch(url, '.m3u8')) {
    console.log(url, initiator);
    let { m3u8 } = await chrome.storage.local.get('m3u8');
    if (!!m3u8) {
      m3u8 = JSON.parse(m3u8);
    } else {
      m3u8 = [];
    }
    m3u8.push(url);
    await chrome.storage.local.set({
      m3u8: JSON.stringify(Array.from(new Set(m3u8)))
    });
  }
  if (method === 'GET' && isMatch(url, '.mp4')) {
    await chrome.storage.local.set({
      mp4: url
    });
  }
}, {
  urls: ['<all_urls>']
});

function isMatch(url, target) {
  return url.indexOf(target) > -1;
}

chrome.runtime.onMessage.addListener((message: IMessage, sender, sendResponse) => {
  setTimeout(async () => {
    let { baseUrl } = await chrome.storage.local.get('baseUrl');
    let { method, api, data } = message;
    let result;
    const url = `${baseUrl ? baseUrl : DEFAULT_BASE_URL}${api}`;
    if (method.toUpperCase() === 'GET') {
      result = await fetch(url, {
        method: method,
        headers: {
          'Content-Type': 'application/json; charset=utf-8'
        },
      });
    } else if (method.toUpperCase() === 'POST') {
      result = await fetch(url, {
        method: method,
        body: JSON.stringify(data),
        headers: {
          'Content-Type': 'application/json; charset=utf-8'
        },
      });
    }
    const body = await result.json();
    console.log('fetch api result', body);
    sendResponse(body);
  }, 200);
  return true;
});
